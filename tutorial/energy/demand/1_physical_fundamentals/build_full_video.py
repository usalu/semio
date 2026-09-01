"""Render, mux, and compose Physical Fundamentals into a single video."""

import subprocess
import sys
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
SEMIO_ROOT = next(
    p for p in Path(__file__).resolve().parents
    if (p / ".venv").is_dir() or (p / "package.json").is_file()
)
# region Intro
INTRO_SCRIPT = SEMIO_ROOT / "tutorial" / "intro" / "intro_scene.py"
INTRO_SCENE = "Demo_Intro_PhysikalischeGrundlagen"
# endregion
sys.path.insert(0, str(NOWIGETIT_ROOT))

from backend.pipeline.compose import probe_duration


def _ffmpeg_concat(clips: list[Path], output: Path, list_path: Path) -> None:
    """🎞️ Hard-cut concat — no crossfade overlap between scenes."""
    lines = []
    for clip in clips:
        escaped = str(clip.resolve()).replace("'", r"'\''")
        lines.append(f"file '{escaped}'")
    list_path.parent.mkdir(parents=True, exist_ok=True)
    list_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(
        ["ffmpeg", "-y", "-f", "concat", "-safe", "0", "-i", str(list_path), "-c", "copy", str(output)],
        check=True,
    )


def _mux_hq(video_path: Path, audio_path: Path | None, output_path: Path, *, fps: int = 60) -> str | None:
    """🔊 Sync narration to video at 1080p60 — pad to max(video, audio), never -shortest."""
    video = Path(video_path)
    if not video.exists():
        return None
    audio = Path(audio_path) if audio_path and Path(audio_path).exists() else None
    v_dur = probe_duration(video)
    a_dur = probe_duration(audio) if audio else 0.0
    target = max(v_dur, a_dur, 0.5) + (0.15 if audio else 0.0)
    output_path.parent.mkdir(parents=True, exist_ok=True)

    if audio is None:
        silent = (
            f"anullsrc=channel_layout=stereo:sample_rate=44100,apad=whole_dur={target:.3f},"
            f"atrim=0:{target:.3f}[a]"
        )
        vf = f"[0:v]tpad=stop_mode=clone:stop_duration={max(0.0, target - v_dur):.3f},fps={fps},format=yuv420p[v]"
        cmd = [
            "ffmpeg", "-y", "-i", str(video),
            "-filter_complex", f"{vf};{silent}",
            "-map", "[v]", "-map", "[a]",
            "-c:v", "libx264", "-preset", "veryfast", "-crf", "18",
            "-c:a", "aac", "-b:a", "192k", "-ar", "44100",
            "-movflags", "+faststart", str(output_path),
        ]
    else:
        vf = f"[0:v]tpad=stop_mode=clone:stop_duration={max(0.0, target - v_dur):.3f},fps={fps},format=yuv420p[v]"
        af = (
            "[1:a]aformat=sample_fmts=fltp:sample_rates=44100:channel_layouts=stereo,"
            f"apad=whole_dur={target:.3f},atrim=0:{target:.3f}[a]"
        )
        cmd = [
            "ffmpeg", "-y", "-i", str(video), "-i", str(audio),
            "-filter_complex", f"{vf};{af}",
            "-map", "[v]", "-map", "[a]",
            "-c:v", "libx264", "-preset", "veryfast", "-crf", "18",
            "-c:a", "aac", "-b:a", "192k", "-ar", "44100",
            "-movflags", "+faststart", str(output_path),
        ]
    res = subprocess.run(cmd, capture_output=True, text=True)
    if res.returncode != 0 or not output_path.exists():
        print(res.stderr[-1500:] if res.stderr else "mux failed")
        return None
    return str(output_path)


def main():
    base_dir = Path(__file__).resolve().parent
    script_path = base_dir / "scene_1.py"
    output_dir = base_dir / "rendered" / "hq"
    output_dir.mkdir(parents=True, exist_ok=True)

    intro_audio = SEMIO_ROOT / "tutorial" / "intro" / "intro_physical_fundamentals.mp3"
    scenes = [
        (INTRO_SCENE, INTRO_SCRIPT, intro_audio),
        ("Beat1_UnsichtbareDimension", script_path, base_dir / "beat_1_audio.mp3"),
        ("Beat2_KraftUndArbeit", script_path, base_dir / "beat_2_audio.mp3"),
        ("Beat3_ArbeitZuLeistung", script_path, base_dir / "beat_3_audio.mp3"),
        ("Beat4_Kilowattstunde", script_path, base_dir / "beat_4_audio.mp3"),
        ("Beat5_Groessenordnungen", script_path, base_dir / "beat_5_audio.mp3"),
        ("Beat6_Energieerhaltung", script_path, base_dir / "beat_6_audio.mp3"),
        ("Beat7_Waermepumpe", script_path, base_dir / "beat_7_audio.mp3"),
        ("Beat8_Ausblick", script_path, base_dir / "beat_8_audio.mp3"),
    ]

    def resolve_audio(path: Path) -> Path:
        if path.exists():
            return path
        wav = path.with_suffix(".wav")
        if wav.exists():
            return wav
        return path

    muxed_clips: list[Path] = []
    manim_bin = SEMIO_ROOT / ".venv" / "bin" / "manim"

    for idx, (scene_name, scene_script, audio_path) in enumerate(scenes, start=1):
        print(f"\n{'=' * 60}")
        print(f"--- Rendering {scene_name} ({idx}/{len(scenes)}) @ 1080p60 ---")
        print(f"{'=' * 60}")

        render_cmd = [
            str(manim_bin),
            "-qh",
            "--media_dir", str(output_dir / "media"),
            str(scene_script),
            scene_name,
        ]

        res = subprocess.run(render_cmd, capture_output=True, text=True)
        if res.returncode != 0:
            print(f"Error rendering {scene_name}:")
            print(res.stderr[-2000:] if len(res.stderr) > 2000 else res.stderr)
            continue
        print("  ✓ Rendered successfully")

        script_stem = scene_script.stem
        rendered_mp4 = output_dir / "media" / "videos" / script_stem / "1080p60" / f"{scene_name}.mp4"
        if not rendered_mp4.exists():
            candidates = sorted(
                (output_dir / "media").rglob(f"{scene_name}.mp4"),
                key=lambda p: p.stat().st_mtime,
                reverse=True,
            )
            rendered_mp4 = next((p for p in candidates if p.stat().st_size > 1000), None)
            if rendered_mp4 is None:
                print(f"  ✗ Could not find rendered mp4 for {scene_name}")
                continue

        resolved_audio = resolve_audio(audio_path)
        if scene_name == INTRO_SCENE and not resolved_audio.exists():
            print("  ⚠ Intro audio missing — run tutorial/intro/generate_audio.py first")
        mux_name = "intro_with_audio.mp4" if scene_name == INTRO_SCENE else f"beat_{idx - 1}_with_audio.mp4"
        muxed_output = output_dir / mux_name
        print(f"--- Muxing audio for {scene_name} ---")
        muxed_path = _mux_hq(rendered_mp4, resolved_audio if resolved_audio.exists() else None, muxed_output)

        if muxed_path:
            muxed_clips.append(Path(muxed_path))
            vd = probe_duration(muxed_output)
            print(f"  ✓ Muxed → {muxed_output} ({vd:.1f}s)")
        else:
            print("  ✗ Mux failed, using raw video")
            muxed_clips.append(rendered_mp4)

    if len(muxed_clips) != len(scenes):
        print(f"\n❌ Only {len(muxed_clips)}/{len(scenes)} clips ready — aborting concat.")
        return

    print(f"\n{'=' * 60}")
    print("--- Concatenating (hard cuts, no overlap) ---")
    print(f"{'=' * 60}")

    final_output = output_dir / "Full_Physical_Fundamentals_1080p60.mp4"
    list_path = output_dir / "concat_list.txt"
    _ffmpeg_concat(muxed_clips, final_output, list_path)

    total = probe_duration(final_output)
    print(f"\n✅ SUCCESS! Full video composed:")
    print(f"   Path: {final_output}")
    print(f"   Duration: {total:.1f}s ({total / 60:.1f} min)")


if __name__ == "__main__":
    main()
