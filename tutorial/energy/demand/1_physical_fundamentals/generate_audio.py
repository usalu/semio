"""🔊 Subtitle voiceover for Physical Fundamentals — synthesize, align and verify.

The spoken text is the German subtitle already on screen, so audio and captions
come from one source (``scene_1.py`` NARRATION). Workflow:

    synth   → one audio file per clause + vo_timing.json (measured lengths)
    render  → VO_TRACE=1 manim run records when each subtitle appears
    align   → per-beat track with every clause laid on its own timestamp
    report  → per-clause drift between speech and subtitle
"""

import argparse
import subprocess
import sys
from pathlib import Path

BASE_DIR = Path(__file__).resolve().parent
_TUTORIAL_ROOT = next(p for p in BASE_DIR.parents if (p / "manim_fonts.py").is_file())
if str(_TUTORIAL_ROOT) not in sys.path:
    sys.path.insert(0, str(_TUTORIAL_ROOT))
if str(BASE_DIR) not in sys.path:
    sys.path.insert(0, str(BASE_DIR))

from scene_1 import (  # noqa: E402
    Beat1_UnsichtbareDimension,
    Beat2_KraftUndArbeit,
    Beat3_ArbeitZuLeistung,
    Beat4_Kilowattstunde,
    Beat5_Groessenordnungen,
    Beat6_Energieerhaltung,
    Beat7_Waermepumpe,
    Beat8_Ausblick,
)
from tts_pipeline import (  # noqa: E402
    assemble_aligned_track,
    configure_tutorial_tts,
    probe_duration,
    read_vo_trace,
    subtitle_narration_text,
    synthesize_clause_audio,
    write_vo_timing_manifest,
)

BEATS = [
    Beat1_UnsichtbareDimension,
    Beat2_KraftUndArbeit,
    Beat3_ArbeitZuLeistung,
    Beat4_Kilowattstunde,
    Beat5_Groessenordnungen,
    Beat6_Energieerhaltung,
    Beat7_Waermepumpe,
    Beat8_Ausblick,
]

SCENE_FILE = BASE_DIR / "scene_1.py"
TIMING_MANIFEST = BASE_DIR / "vo_timing.json"
TRACE_MANIFEST = BASE_DIR / "vo_trace.json"
CLAUSE_GAP = 0.25


def _clause_dir(index: int) -> Path:
    return BASE_DIR / f"beat_{index}_clauses"


def _clause_files(index: int) -> list[Path]:
    return sorted(_clause_dir(index).glob("clause_*"))


def _rendered_clip(beat_cls) -> Path | None:
    candidates = [
        p for p in (BASE_DIR / "media" / "videos").rglob(f"{beat_cls.__name__}.mp4")
        if p.stat().st_size > 1000
    ]
    if not candidates:
        return None
    return max(candidates, key=lambda p: p.stat().st_mtime)


def _selected(beat_arg: int | None):
    if beat_arg is None:
        return list(enumerate(BEATS, start=1))
    if not 1 <= beat_arg <= len(BEATS):
        raise SystemExit(f"Beat must be 1–{len(BEATS)}")
    return [(beat_arg, BEATS[beat_arg - 1])]


def cmd_synth(beat_arg: int | None) -> int:
    configure_tutorial_tts()
    print("=== Subtitle voiceover — per-clause synthesis ===\n")
    manifest_rows = []
    for index, cls in _selected(beat_arg):
        print(f"Beat {index}: {cls.__name__}")
        print(f"  {subtitle_narration_text(cls.NARRATION)[:70]}…")
        clauses = synthesize_clause_audio(cls.NARRATION, _clause_dir(index))
        spoken = [(section, path, seconds) for section, path, seconds in clauses if path]
        if not spoken:
            print("  ✗ No audio produced (missing TTS key?)\n")
            continue
        total = sum(seconds for _, _, seconds in spoken)
        print(f"  ✓ {len(spoken)} clauses · {total:.1f}s speech")
        manifest_rows.append((cls.__name__, clauses))

        placements, cursor = [], 0.0
        for _section, path, seconds in clauses:
            if not path:
                continue
            placements.append((path, cursor))
            cursor += seconds + CLAUSE_GAP
        assemble_aligned_track(placements, BASE_DIR / f"beat_{index}_audio.mp3", total_duration=cursor)
        print(f"  ✓ Draft track → beat_{index}_audio.mp3\n")

    if manifest_rows:
        write_vo_timing_manifest(manifest_rows, TIMING_MANIFEST)
        print(f"📋 Measured clause lengths → {TIMING_MANIFEST.name}")
        print("Next: run `--trace` to re-render with subtitle marks, then `--align`.")
    return 0


def cmd_trace(beat_arg: int | None) -> int:
    import os

    env = {**os.environ, "VO_TRACE": "1"}
    manim = _TUTORIAL_ROOT.parent / ".venv" / "bin" / "manim"
    if not manim.is_file():
        manim = Path("manim")
    print("=== Recording subtitle timestamps (VO_TRACE=1) ===\n")
    for index, cls in _selected(beat_arg):
        print(f"Beat {index}: rendering {cls.__name__} …")
        proc = subprocess.run(
            [str(manim), "-ql", str(SCENE_FILE), cls.__name__],
            cwd=str(BASE_DIR), env=env, capture_output=True, text=True, check=False,
        )
        if proc.returncode != 0:
            print(f"  ✗ render failed: {proc.stderr[-400:]}")
            continue
        print("  ✓ traced")
    print(f"\n📍 Subtitle marks → {TRACE_MANIFEST.name}")
    return 0


def cmd_align(beat_arg: int | None) -> int:
    trace = read_vo_trace(TRACE_MANIFEST)
    if not trace:
        raise SystemExit(f"{TRACE_MANIFEST.name} missing — run with --trace first")
    print("=== Aligning speech to subtitle timestamps ===\n")
    for index, cls in _selected(beat_arg):
        marks = trace.get(cls.__name__)
        if not marks:
            print(f"Beat {index}: no trace for {cls.__name__} — skipped")
            continue
        clause_paths = {p.stem: p for p in _clause_files(index)}
        clip = _rendered_clip(cls)
        video_seconds = probe_duration(clip) if clip else 0.0

        placements = []
        for order, (section, _, text_de) in enumerate(cls.NARRATION):
            if not text_de.strip():
                continue
            path = clause_paths.get(f"clause_{order:02d}")
            mark = marks.get(section)
            if not path or not mark:
                continue
            placements.append((path, float(mark["start"])))
        if not placements:
            print(f"Beat {index}: nothing to place — skipped")
            continue

        output = BASE_DIR / f"beat_{index}_audio.mp3"
        assemble_aligned_track(placements, output, total_duration=video_seconds)
        print(f"Beat {index}: {len(placements)} clauses on video clock · {video_seconds:.1f}s → {output.name}")
    return 0


def cmd_report(beat_arg: int | None) -> int:
    trace = read_vo_trace(TRACE_MANIFEST)
    print("=== Subtitle vs. speech drift ===\n")
    worst_overall = 0.0
    for index, cls in _selected(beat_arg):
        clip = _rendered_clip(cls)
        video_seconds = probe_duration(clip) if clip else 0.0
        audio_seconds = probe_duration(BASE_DIR / f"beat_{index}_audio.mp3")
        marks = trace.get(cls.__name__, {})
        clause_paths = {p.stem: p for p in _clause_files(index)}

        print(f"Beat {index} · {cls.__name__}")
        print(f"  video {video_seconds:6.1f}s | audio {audio_seconds:6.1f}s | diff {audio_seconds - video_seconds:+.1f}s")
        if not marks:
            print("  (no subtitle trace — run --trace)\n")
            continue
        worst = 0.0
        for order, (section, _, text_de) in enumerate(cls.NARRATION):
            if not text_de.strip():
                continue
            mark = marks.get(section)
            path = clause_paths.get(f"clause_{order:02d}")
            if not mark or not path:
                continue
            speech = probe_duration(path)
            overflow = speech - float(mark["window"])
            worst = max(worst, overflow)
            flag = "  ⚠" if overflow > 0.4 else "   "
            print(
                f"{flag} {section:<12} subtitle at {mark['start']:6.1f}s "
                f"window {mark['window']:5.1f}s · speech {speech:5.1f}s · overflow {overflow:+.1f}s"
            )
        worst_overall = max(worst_overall, worst)
        print(f"  worst clause overflow: {worst:+.1f}s\n")
    print(f"Worst overflow across beats: {worst_overall:+.1f}s")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--beat", type=int, help="Only this beat number (1-based)")
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--trace", action="store_true", help="Re-render with VO_TRACE=1 to record subtitle times")
    mode.add_argument("--align", action="store_true", help="Rebuild tracks on the traced subtitle clock")
    mode.add_argument("--report", action="store_true", help="Print subtitle vs. speech drift")
    args = parser.parse_args(argv)

    if args.trace:
        return cmd_trace(args.beat)
    if args.align:
        return cmd_align(args.beat)
    if args.report:
        return cmd_report(args.beat)
    return cmd_synth(args.beat)


if __name__ == "__main__":
    raise SystemExit(main())
