"""🎬 Cooling full series with synced German subtitle VO (1080p60 HQ).

Pipeline:
  1. synth   — per-clause TTS + vo_timing.json (skippable if clauses exist)
  2. render  — HQ section re-render with VO_TRACE=1
  3. align   — place each clause on its traced subtitle start
  4. mux     — embed section audio (slow/x264 HQ), concat intro + sections
  5. noaudio — concat silent HQ sections for NoAudio deliverable

Ticket: 2026/09/12/RENDER-FULL-COOLING-WITH-AUDIO
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
COOLING = Path("/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio/tutorial/energy/demand/Cooling")
ROOT = Path("/Users/niloufarghandehariyoon/Documents/Master LUH/Hiwi/semio")
TUTORIAL = ROOT / "tutorial"
sys.path.insert(0, str(TUTORIAL))

from tts_pipeline import (  # noqa: E402
    assemble_aligned_track,
    probe_duration,
    read_vo_trace,
)

PARTS = [
    ("1_heating_vs_cooling", "scene_1.py", "Cooling_01_HeatingVsCooling"),
    ("2_internal_gains", "scene_2.py", "Cooling_02_InternalGains"),
    ("3_transmission_humidity", "scene_3.py", "Cooling_03_TransmissionHumidity"),
    ("4_solar_radiation", "scene_4.py", "Cooling_04_SolarRadiation"),
    ("5_systemauslegung", "scene_5.py", "Cooling_05_Systemauslegung"),
    ("6_lueftungssysteme", "scene_6.py", "Cooling_06_Lueftungssysteme"),
]

SERIES_INTRO = "Demo_Intro_Kuehllast"
LOG = TICKET / "pipeline.log"


def _log(msg: str) -> None:
    line = msg.rstrip() + "\n"
    print(line, end="", flush=True)
    with LOG.open("a", encoding="utf-8") as fh:
        fh.write(line)


def _run(cmd: list[str], *, env: dict | None = None, cwd: Path | None = None) -> int:
    _log(f"$ {' '.join(cmd)}")
    proc = subprocess.run(
        cmd,
        cwd=str(cwd or ROOT),
        env=env or os.environ.copy(),
        check=False,
    )
    _log(f"  exit={proc.returncode}")
    return proc.returncode


def _beat_index(name: str) -> int:
    match = re.search(r"Beat(\d+)", name)
    return int(match.group(1)) if match else 0


def _clauses_ready() -> bool:
    for folder, _, _ in PARTS:
        found = list((COOLING / folder).glob("beat_*_clauses/clause_*.mp3"))
        if len(found) < 3:
            return False
    return True


def step_synth(*, force: bool = False) -> None:
    _log("\n=== 1) SYNTH all Cooling parts ===")
    if not force and _clauses_ready():
        _log("  skip synth — clause mp3s already present")
        return
    py = str(ROOT / ".venv" / "bin" / "python")
    for folder, _scene, _section in PARTS:
        script = COOLING / folder / "generate_audio.py"
        code = _run([py, str(script)], cwd=COOLING / folder)
        if code != 0:
            raise SystemExit(f"synth failed for {folder}")


def step_render_trace() -> None:
    _log("\n=== 2) HQ re-render with VO_TRACE (measured holds + subtitle marks) ===")
    env = {**os.environ, "VO_TRACE": "1"}
    series_trace = COOLING / "vo_trace.json"
    if series_trace.is_file():
        series_trace.unlink()
    code = _run(
        [
            str(ROOT / ".venv" / "bin" / "python"),
            str(COOLING / "full_cooling_video.py"),
            "-q",
            "h",
            "--no-play",
            "--force",
        ],
        env=env,
    )
    if code != 0:
        raise SystemExit("full cooling render failed")


def _load_part_beats(folder: str, scene_file: str):
    import importlib.util

    path = COOLING / folder / scene_file
    name = f"vo_mod_{folder}"
    spec = importlib.util.spec_from_file_location(name, path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[name] = mod
    assert spec.loader is not None
    spec.loader.exec_module(mod)
    beats = []
    for attr in dir(mod):
        if not re.match(r"^Beat\d+", attr):
            continue
        cls = getattr(mod, attr)
        if isinstance(cls, type) and getattr(cls, "NARRATION", None):
            beats.append(cls)
    beats.sort(key=lambda c: _beat_index(c.__name__))
    return beats


def _merge_traces() -> dict:
    merged: dict = {}
    for path in [COOLING / "vo_trace.json", *[COOLING / f / "vo_trace.json" for f, _, _ in PARTS]]:
        for beat, marks in read_vo_trace(path).items():
            merged[beat] = marks
    return merged


def _find_section_mp4(section: str) -> Path:
    media = COOLING / "media" / "videos" / "full_cooling_video" / "1080p60" / f"{section}.mp4"
    if media.is_file() and media.stat().st_size > 1000:
        return media
    matches = sorted(
        (COOLING / "media" / "videos").rglob(f"{section}.mp4"),
        key=lambda p: p.stat().st_mtime,
        reverse=True,
    )
    for match in matches:
        if match.stat().st_size > 1000:
            return match
    raise FileNotFoundError(section)


def _find_intro_mp4() -> Path:
    for root in (COOLING / "media" / "videos", TUTORIAL / "intro" / "media" / "videos"):
        matches = sorted(root.rglob(f"{SERIES_INTRO}.mp4"), key=lambda p: p.stat().st_mtime, reverse=True)
        for match in matches:
            if "1080p60" in str(match) and match.stat().st_size > 1000:
                return match
    raise FileNotFoundError(SERIES_INTRO)


def _mux(video: Path, audio: Path | None, output: Path) -> Path:
    """🎞 HQ remux: slow x264 + high AAC for final delivery quality."""
    output.parent.mkdir(parents=True, exist_ok=True)
    v_dur = probe_duration(video)
    a_dur = probe_duration(audio) if audio and audio.is_file() else 0.0
    target = max(v_dur, a_dur, 0.5) + (0.05 if audio else 0.0)
    vf = (
        f"[0:v]tpad=stop_mode=clone:stop_duration={max(0.0, target - v_dur):.3f},"
        f"fps=60,format=yuv420p[v]"
    )
    if audio is None or not audio.is_file():
        af = (
            f"anullsrc=channel_layout=stereo:sample_rate=48000,"
            f"apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
        )
        inputs = ["-i", str(video)]
    else:
        af = (
            "[1:a]aformat=sample_fmts=fltp:sample_rates=48000:channel_layouts=stereo,"
            f"apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
        )
        inputs = ["-i", str(video), "-i", str(audio)]
    cmd = [
        "ffmpeg",
        "-y",
        *inputs,
        "-filter_complex",
        f"{vf};{af}",
        "-map",
        "[v]",
        "-map",
        "[a]",
        "-c:v",
        "libx264",
        "-preset",
        "slow",
        "-crf",
        "15",
        "-pix_fmt",
        "yuv420p",
        "-c:a",
        "aac",
        "-b:a",
        "320k",
        "-ar",
        "48000",
        "-movflags",
        "+faststart",
        str(output),
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, check=False)
    if proc.returncode != 0 or not output.is_file():
        raise RuntimeError(proc.stderr[-800:] if proc.stderr else "mux failed")
    return output


def _concat_copy(clips: list[Path], list_path: Path, output: Path) -> Path:
    lines = []
    for clip in clips:
        escaped = str(clip.resolve()).replace("'", r"'\''")
        lines.append(f"file '{escaped}'")
    list_path.parent.mkdir(parents=True, exist_ok=True)
    list_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    output.parent.mkdir(parents=True, exist_ok=True)
    cmd = [
        "ffmpeg",
        "-y",
        "-f",
        "concat",
        "-safe",
        "0",
        "-i",
        str(list_path),
        "-c",
        "copy",
        str(output),
    ]
    proc = subprocess.run(cmd, capture_output=True, text=True, check=False)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr[-800:])
    return output


def step_align_mux_concat() -> Path:
    _log("\n=== 3) Align section audio to subtitle traces + mux + concat ===")
    trace = _merge_traces()
    if not trace:
        raise SystemExit("no vo_trace.json — render with VO_TRACE=1 first")
    (TICKET / "vo_trace_merged.json").write_text(
        json.dumps({"beats": trace}, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    out_dir = COOLING / "rendered" / "with_audio"
    out_dir.mkdir(parents=True, exist_ok=True)
    clips: list[Path] = []
    silent_clips: list[Path] = []

    intro = _find_intro_mp4()
    intro_mux = _mux(intro, None, out_dir / "00_intro_with_audio.mp4")
    clips.append(intro_mux)
    silent_clips.append(intro)
    _log(f"  intro silent-pad → {intro_mux.name} ({probe_duration(intro_mux):.1f}s)")

    for folder, scene_file, section in PARTS:
        beats = _load_part_beats(folder, scene_file)
        video = _find_section_mp4(section)
        silent_clips.append(video)
        video_seconds = probe_duration(video)
        placements: list[tuple[Path, float]] = []
        missing = []
        for cls in beats:
            marks = trace.get(cls.__name__)
            if not marks:
                missing.append(cls.__name__)
                continue
            index = _beat_index(cls.__name__)
            clause_dir = COOLING / folder / f"beat_{index}_clauses"
            clause_paths = {p.stem: p for p in clause_dir.glob("clause_*")}
            for order, (section_key, _, text_de) in enumerate(cls.NARRATION):
                if not text_de.strip():
                    continue
                path = clause_paths.get(f"clause_{order:02d}")
                mark = marks.get(section_key)
                if not path or not mark:
                    missing.append(f"{cls.__name__}:{section_key}")
                    continue
                placements.append((path, float(mark["start"])))
        if missing:
            _log(f"  ⚠ {section}: missing marks/files: {missing[:8]}{'…' if len(missing) > 8 else ''}")
        if not placements:
            raise SystemExit(f"no placements for {section}")

        audio = out_dir / f"{section}_vo.mp3"
        assemble_aligned_track(placements, audio, total_duration=video_seconds)
        muxed = _mux(video, audio, out_dir / f"{section}_with_audio.mp4")
        clips.append(muxed)
        _log(
            f"  {section}: {len(placements)} clauses · video {video_seconds:.1f}s "
            f"→ {muxed.name} ({probe_duration(muxed):.1f}s)"
        )

    final_audio = COOLING / "rendered" / "Full_Cooling_Demand_WithAudio_1080p60.mp4"
    final_alias = COOLING / "rendered" / "Full_Cooling_Demand_1080p60.mp4"
    _concat_copy(clips, out_dir / "concat_list.txt", final_audio)
    shutil.copy2(final_audio, final_alias)

    final_noaudio = COOLING / "rendered" / "Full_Cooling_Demand_NoAudio_1080p60.mp4"
    _concat_copy(silent_clips, COOLING / "rendered" / "noaudio_concat_list.txt", final_noaudio)

    _log(f"\n✅ WithAudio → {final_audio}")
    _log(f"   alias → {final_alias}")
    _log(f"   NoAudio → {final_noaudio}")
    _log(f"   WithAudio duration {probe_duration(final_audio):.1f}s")
    _log(f"   NoAudio duration {probe_duration(final_noaudio):.1f}s")
    return final_audio


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--skip-synth", action="store_true", help="Reuse existing clause mp3s")
    parser.add_argument("--force-synth", action="store_true", help="Re-run TTS even if clauses exist")
    parser.add_argument("--align-only", action="store_true", help="Skip synth+render; mux from existing HQ")
    parser.add_argument("--render-only", action="store_true", help="Only HQ VO_TRACE render")
    args = parser.parse_args()

    LOG.write_text("", encoding="utf-8")
    if not args.align_only:
        if not args.render_only:
            step_synth(force=args.force_synth)
        step_render_trace()
        if args.render_only:
            return 0
    step_align_mux_concat()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
