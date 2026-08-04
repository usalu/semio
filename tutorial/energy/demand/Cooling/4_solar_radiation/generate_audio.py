"""Generate TTS audio for Scene 4 (Solare Strahlung & Verglasung) using the Now I Get It TTS pipeline."""

import sys
from pathlib import Path

# Now I Get It provides backend.pipeline.tts (OpenRouter / Gemini speech).
NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))

from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent

NARRATIONS = {
    "beat_1_audio": (
        "Now we tackle the most significant summer heat source: direct solar radiation. "
        "Everything starts with the maximum solar irradiance, I S max, measured in watts "
        "per square meter. As the solar chart shows, this maximum value changes drastically "
        "depending on the time of day and whether the surface is horizontal, or facing "
        "North, South, East, or West."
    ),
    "beat_2_audio": (
        "To calculate the cooling load, we start with the gross area of the window opening, A. "
        "However, glass doesn't cover the entire opening. We must multiply by F F, the "
        "dimensionless frame factor. This mathematically isolates the effective transparent "
        "area by subtracting the opaque window frames that physically block the sun."
    ),
    "beat_3_audio": (
        "Next, we deploy our sun protection. This is a vertical section through the facade. "
        "With no shading, the full beam strikes the glass and passes straight into the room: "
        "a shading factor, F V, of one point zero. Now an external Raffstore drops in front. "
        "Its slats intercept the beam and reflect the heat back outside, before it ever reaches "
        "the glass. Only a small residual gets through, so F V falls to about zero point one five."
    ),
    "beat_4_audio": (
        "Finally, the remaining light hits the glass pane itself. We multiply by g tot, the "
        "total solar energy transmittance. Academically, this is the sum of direct solar "
        "transmission, tau e, and the secondary inward heat emission, q i, from the glass "
        "absorbing the radiation. It tells us exactly what fraction of that heat successfully "
        "penetrates into the room."
    ),
    "beat_5_audio": (
        "By multiplying the raw solar irradiance by our building's gross area, and then applying "
        "our three dimensionless reduction filters, the frame factor, the shading factor, and "
        "the glass transmittance, we arrive at our answer. This is Q dot S t r, the final "
        "transmission cooling load. It is the precise thermal wattage our mechanical system must "
        "actively remove to prevent the room from overheating."
    ),
}


def main():
    print("=== Scene 4 TTS Audio Generation ===\n")
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
