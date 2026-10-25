"""Render, mux, and compose Heating Module 4 (Interne Wärmegewinne) into a single video."""

import sys
import subprocess
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
SEMIO_ROOT = Path(__file__).resolve().parents[5]
sys.path.insert(0, str(NOWIGETIT_ROOT))

from backend.pipeline.compose import mux_scene_audio, compose_final_video


def main():
    base_dir = Path(__file__).resolve().parent
    script_path = base_dir / "scene_4.py"
    output_dir = base_dir / "rendered"
    output_dir.mkdir(exist_ok=True)

    scenes = [
        ("Beat1_WinterInterneGewinne", base_dir / "beat_1_audio.mp3"),
        ("Beat2_PersonenPhiP", base_dir / "beat_2_audio.mp3"),
        ("Beat3_GeraetePhiE", base_dir / "beat_3_audio.mp3"),
        ("Beat4_BeleuchtungPhiL", base_dir / "beat_4_audio.mp3"),
        ("Beat5_SummeUndDichte", base_dir / "beat_5_audio.mp3"),
    ]

    def resolve_audio(path: Path) -> Path:
        if path.exists():
            return path
        wav = path.with_suffix(".wav")
        if wav.exists():
            return wav
        return path

    muxed_clips = []
    manim_bin = SEMIO_ROOT / ".venv" / "bin" / "manim"

    for idx, (scene_name, audio_path) in enumerate(scenes, start=1):
        print(f"\n{'=' * 60}")
        print(f"--- Rendering {scene_name} (Beat {idx}/{len(scenes)}) ---")
        print(f"{'=' * 60}")

        render_cmd = [
            str(manim_bin),
            "-qh",
            "--media_dir", str(output_dir / "media"),
            str(script_path),
            scene_name,
        ]

        res = subprocess.run(render_cmd, capture_output=True, text=True)
        if res.returncode != 0:
            print(f"Error rendering {scene_name}:")
            print(res.stderr[-2000:] if len(res.stderr) > 2000 else res.stderr)
            continue
        print("  ✓ Rendered successfully")

        rendered_mp4 = output_dir / "media" / "videos" / "scene_4" / "1080p60" / f"{scene_name}.mp4"
        if not rendered_mp4.exists():
            candidates = list((output_dir / "media").rglob(f"{scene_name}.mp4"))
            if candidates:
                rendered_mp4 = candidates[0]
            else:
                print(f"  ✗ Could not find rendered mp4 for {scene_name}")
                continue

        resolved_audio = resolve_audio(audio_path)
        muxed_output = output_dir / f"beat_{idx}_with_audio.mp4"
        print(f"--- Muxing audio for {scene_name} ---")
        muxed_path = mux_scene_audio(
            str(rendered_mp4),
            str(resolved_audio) if resolved_audio.exists() else None,
            muxed_output,
        )

        if muxed_path:
            muxed_clips.append(muxed_path)
            print(f"  ✓ Muxed → {muxed_path}")
        else:
            print("  ✗ Mux failed, using raw video")
            muxed_clips.append(str(rendered_mp4))

    print(f"\n{'=' * 60}")
    print("--- Composing Final Full Video ---")
    print(f"{'=' * 60}")

    final_output = base_dir / "Full_Modul4_InterneGewinne.mp4"
    result = compose_final_video(muxed_clips, final_output)

    if result:
        print("\n✅ SUCCESS! Full video composed:")
        print(f"   Path: {final_output}")
    else:
        print("\n❌ Failed to compose final video.")


if __name__ == "__main__":
    main()
