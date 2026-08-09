"""Generate TTS audio for Scene 6 (Natürliche Lüftung im Passivhaus).

Narration text lives in scene_6.py — this file only reads it via each Beat's
NARRATION and sends it to TTS.
"""

import sys
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))
from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(BASE_DIR))

from scene_6 import (  # noqa: E402
    Beat1_PassivhausIdee,
    Beat2_Fensterregeln,
    Beat3_Querlueftung,
    Beat4_Auftrieb,
    Beat5_Nachtlueftung,
    Beat6_GrenzenDerFreienLueftung,
    Beat7_MechanischeGrundtypen,
    Beat8_Waermerueckgewinnung,
    Beat9_KomfortStrategie,
)
from manim_visuals import narration_text  # noqa: E402

BEATS = [
    Beat1_PassivhausIdee,
    Beat2_Fensterregeln,
    Beat3_Querlueftung,
    Beat4_Auftrieb,
    Beat5_Nachtlueftung,
    Beat6_GrenzenDerFreienLueftung,
    Beat7_MechanischeGrundtypen,
    Beat8_Waermerueckgewinnung,
    Beat9_KomfortStrategie,
]


def main():
    print("=== Scene 6 TTS Audio Generation (Natürliche Lüftung / Passivhaus) ===\n")
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
