"""🎬 Render, mux and concat Cooling Part 5 (Systemauslegung) into one 1080p60 video.

Self-contained: only manim + ffmpeg + this repo's ``tts_pipeline`` (no external paths).
Run ``generate_audio.py`` (synth → --trace → --align) first so ``beat_N_audio.mp3``
sit on the video clock; missing audio just yields a silent-padded clip.
"""

import subprocess
import sys
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
_TUTORIAL_ROOT = next(p for p in BASE_DIR.parents if (p / "manim_fonts.py").is_file())
SEMIO_ROOT = next(
    p for p in BASE_DIR.parents if (p / ".venv").is_dir() or (p / "package.json").is_file()
)
if str(_TUTORIAL_ROOT) not in sys.path:
    sys.path.insert(0, str(_TUTORIAL_ROOT))

from tts_pipeline import probe_duration  # noqa: E402

SCENE_FILE = BASE_DIR / "scene_5.py"
SCENES = [
    ("Beat1_MechanicalVentilation", BASE_DIR / "beat_1_audio.mp3"),
    ("Beat2_VolumeFlowEquation", BASE_DIR / "beat_2_audio.mp3"),
    ("Beat3_IsolateAirflow", BASE_DIR / "beat_3_audio.mp3"),
    ("Beat4_DuctCrossSection", BASE_DIR / "beat_4_audio.mp3"),
    ("Beat5_CalculateRadius", BASE_DIR / "beat_5_audio.mp3"),
]


def _resolve_audio(path: Path) -> Path | None:
    if path.exists():
        return path
    wav = path.with_suffix(".wav")
    return wav if wav.exists() else None


def _manim_bin() -> str:
    cand = SEMIO_ROOT / ".venv" / "bin" / "manim"
    return str(cand) if cand.is_file() else "manim"


def _mux(video: Path, audio: Path | None, output: Path, *, fps: int = 60) -> Path | None:
    """🔊 Lay narration on the clip, pad to max(video, audio) — never -shortest."""
    if not video.exists():
        return None
    v_dur = probe_duration(video)
    a_dur = probe_duration(audio) if audio else 0.0
    target = max(v_dur, a_dur, 0.5) + (0.15 if audio else 0.0)
    output.parent.mkdir(parents=True, exist_ok=True)
    vf = (
        f"[0:v]tpad=stop_mode=clone:stop_duration={max(0.0, target - v_dur):.3f},"
        f"fps={fps},format=yuv420p[v]"
    )
    if audio is None:
        af = (
            f"anullsrc=channel_layout=stereo:sample_rate=44100,"
            f"apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
        )
        inputs = ["-i", str(video)]
    else:
        af = (
            "[1:a]aformat=sample_fmts=fltp:sample_rates=44100:channel_layouts=stereo,"
            f"apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
        )
        inputs = ["-i", str(video), "-i", str(audio)]
    cmd = [
        "ffmpeg", "-y", *inputs,
        "-filter_complex", f"{vf};{af}",
        "-map", "[v]", "-map", "[a]",
        "-c:v", "libx264", "-preset", "veryfast", "-crf", "18",
        "-c:a", "aac", "-b:a", "192k", "-ar", "44100",
        "-movflags", "+faststart", str(output),
    ]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0 or not output.exists():
        print(res.stderr[-1500:] if res.stderr else "mux failed")
        return None
    return output


def _concat(clips: list[Path], output: Path, list_path: Path) -> None:
    """🎞️ Hard-cut concat — stream copy, no crossfade."""
    list_path.write_text(
        "\n".join(f"file '{str(c.resolve())}'" for c in clips) + "\n", encoding="utf-8"
    )
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["ffmpeg", "-y", "-f", "concat", "-safe", "0", "-i", str(list_path),
         "-c", "copy", str(output)],
        check=True,
    )


def main() -> int:
    output_dir = BASE_DIR / "rendered" / "hq"
    media_dir = output_dir / "media"
    output_dir.mkdir(parents=True, exist_ok=True)
    manim_bin = _manim_bin()
    muxed: list[Path] = []

    for idx, (scene_name, audio_path) in enumerate(SCENES, start=1):
        print(f"\n{'=' * 60}\n--- Rendering {scene_name} ({idx}/{len(SCENES)}) @ 1080p60 ---\n{'=' * 60}")
        res = subprocess.run(
            [manim_bin, "-qh", "--media_dir", str(media_dir), str(SCENE_FILE), scene_name],
            capture_output=True, text=True,
        )
        if res.returncode != 0:
            print(res.stderr[-2000:])
            return 1
        print("  ✓ rendered")

        rendered = media_dir / "videos" / SCENE_FILE.stem / "1080p60" / f"{scene_name}.mp4"
        if not rendered.exists():
            cands = sorted(
                media_dir.rglob(f"{scene_name}.mp4"),
                key=lambda p: p.stat().st_mtime, reverse=True,
            )
            rendered = next((p for p in cands if p.stat().st_size > 1000), None)
            if rendered is None:
                print(f"  ✗ rendered mp4 not found for {scene_name}")
                return 1

        audio = _resolve_audio(audio_path)
        if audio is None:
            print(f"  ⚠ {audio_path.name} missing — clip will be silent (run generate_audio.py)")
        out = _mux(rendered, audio, output_dir / f"beat_{idx}_with_audio.mp4")
        if out is None:
            print("  ✗ mux failed, using raw clip")
            out = rendered
        else:
            print(f"  ✓ muxed → {out.name} ({probe_duration(out):.1f}s)")
        muxed.append(out)

    final = output_dir / "Full_Cooling_05_Systemauslegung_1080p60.mp4"
    print(f"\n{'=' * 60}\n--- Concatenating {len(muxed)} clips ---\n{'=' * 60}")
    _concat(muxed, final, output_dir / "concat_list.txt")
    total = probe_duration(final)
    print(f"\n✅ {final}\n   {total:.1f}s ({total / 60:.1f} min)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
