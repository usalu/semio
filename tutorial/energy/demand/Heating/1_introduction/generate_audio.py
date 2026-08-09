"""Generate TTS audio for Heating Module 1 (Einführung) using the Now I Get It TTS pipeline.

Narration text is not re-typed here — it's imported from each Beat's NARRATION
in scene_1.py, per the generate-manim-tutorial skill.
"""

import sys
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))
from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(BASE_DIR))

# Importing scene_1 runs its own _TUTORIAL_ROOT sys.path bootstrap, which is
# what makes the next import (manim_visuals) resolve.
from scene_1 import (  # noqa: E402
    Beat1_DreiWegeDerWaerme,
    Beat2_Waermeleitung,
    Beat3_Konvektion,
    Beat4_Strahlung,
    Beat5_Zusammenfassung,
    Beat6_VonWegenZuZahlen,
    Beat7_Waermedurchlasswiderstand,
    Beat8_UWert,
    Beat9_WaermestromFormel,
)
from manim_visuals import narration_text  # noqa: E402

BEATS = [
    Beat1_DreiWegeDerWaerme,
    Beat2_Waermeleitung,
    Beat3_Konvektion,
    Beat4_Strahlung,
    Beat5_Zusammenfassung,
    Beat6_VonWegenZuZahlen,
    Beat7_Waermedurchlasswiderstand,
    Beat8_UWert,
    Beat9_WaermestromFormel,
]


def main():
    print("=== Modul 1 (Einführung) TTS Audio Generation ===\n")
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
