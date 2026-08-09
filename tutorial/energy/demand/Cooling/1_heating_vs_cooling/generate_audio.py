"""Generate TTS audio for Scene 1 (Heizlast vs. Kühllast). Narration lives in scene_1.py."""

import sys
from pathlib import Path

NOWIGETIT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT))
from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(BASE_DIR))

from scene_1 import Beat1_WinterGains, Beat2_SummerOverheat, Beat3_CoolingSystem  # noqa: E402
from manim_visuals import narration_text  # noqa: E402

BEATS = [Beat1_WinterGains, Beat2_SummerOverheat, Beat3_CoolingSystem]


def main():
    print("=== Scene 1 TTS Audio Generation (Heizlast vs. Kühllast) ===\n")
    for i, cls in enumerate(BEATS, start=1):
        text = narration_text(cls.NARRATION)
        out_path = BASE_DIR / f"beat_{i}_audio.mp3"
        print(f"Generating: {cls.__name__} → {out_path.name} ...")
        try:
            result_path, skipped = synthesize_narration(text, out_path)
            if skipped:
                print("  ⚠ Skipped (no TTS API key configured or empty text)")
            elif result_path:
                print(f"  ✓ Saved → {result_path}")
            else:
                print("  ✗ Failed (no output)")
        except Exception as e:
            print(f"  ✗ Error: {e}")
    print("\nDone!")


if __name__ == "__main__":
    main()
