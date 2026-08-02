"""Generate TTS audio for Scene 3 (Transmission & Feuchtigkeit) using the Now I Get It TTS pipeline."""

import sys
from pathlib import Path

# Now I Get It provides backend.pipeline.tts (OpenRouter / Gemini speech).
NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))

from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent

NARRATIONS = {
    "beat_1_audio": (
        "Next, we have to look outside. Just like in winter, heat travels through "
        "solid walls and roofs. We calculate this transmission load, Q-dot T, using "
        "the U-value and area of the wall. But in summer, a dark roof baking under "
        "the midday sun absorbs massive energy, so we use an equivalent temperature "
        "difference, Delta-T equivalent, to account for that extreme solar heating "
        "on the surface."
    ),
    "beat_2_audio": (
        "Because building materials like concrete and brick have high thermal mass, "
        "they store this heat. They soak it up during the day and slowly release it "
        "into the room hours later. This means your equivalent temperature difference "
        "and your peak cooling demand might actually hit in the late evening, long "
        "after the sun has set."
    ),
    "beat_3_audio": (
        "Now let's examine ventilation heat and moisture. Looking at a cross-section of the house, "
        "we can clearly see the airflow through the open window. Cool, conditioned air escapes "
        "outward, while warm, humid outdoor air streams inside. The total ventilation load, "
        "Q-dot L, must handle both of these effects: it is the sum of the sensible heat load, "
        "Q-dot sens, and the latent humidity load, Q-dot lat."
    ),
    "beat_4_audio": (
        "Let's break down these formulas. On the left, the sensible cooling load, Q-dot sens, "
        "represents the energy needed just to lower the air temperature. It depends on air density, "
        "specific heat capacity, the temperature difference Delta-Theta, and the air volume flow. "
        "On the right, we have the latent cooling load, Q-dot lat. This is the energy required to "
        "remove humidity from the air. It depends on air density, the latent heat of vaporization r, "
        "the absolute moisture difference Delta-x, and the air volume flow. Removing moisture requires "
        "massive phase change energy, meaning dealing with summer humidity, 'Feuchtigkeit', is often "
        "the most energy-intensive part of air conditioning."
    ),
}


def main():
    print("=== Scene 3 TTS Audio Generation ===\n")
    for name, text in NARRATIONS.items():
        out_path = BASE_DIR / f"{name}.mp3"
        print(f"Generating: {name} ...")
        try:
            result_path, skipped = synthesize_narration(text, out_path)
            if skipped:
                print(f"  ⚠ Skipped (no TTS API key configured or empty text)")
            elif result_path:
                print(f"  ✓ Saved → {result_path}")
            else:
                print(f"  ✗ Failed (no output)")
        except Exception as e:
            print(f"  ✗ Error: {e}")
    print("\nDone!")


if __name__ == "__main__":
    main()
