"""Generate TTS audio for Physical Fundamentals (scene_1.py)."""

import sys
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))
from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent
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
from manim_visuals import narration_text  # noqa: E402

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


def main():
    print("=== Physical Fundamentals TTS Audio Generation ===\n")
    for i, cls in enumerate(BEATS, start=1):
        text = narration_text(cls.NARRATION)
        out_path = BASE_DIR / f"beat_{i}_audio.mp3"
        print(f"Generating: {cls.__name__} -> beat_{i}_audio ...")
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
