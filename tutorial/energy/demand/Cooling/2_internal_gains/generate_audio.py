"""Generate TTS audio for Scene 2 (Internal Gains / Hidden Heat)."""

import sys
from pathlib import Path

NOWIGETIT_ROOT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT_ROOT))

from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent

NARRATIONS = {
    "beat_1_audio": (
        "Before the sun even touches our building, we have a massive heat problem "
        "already brewing inside. In commercial buildings, offices, or university "
        "lecture halls, the heat generated internally is often our primary cooling load."
    ),
    "beat_2_audio": (
        "Think of a human body as a biological heater. We calculate this load, "
        "Q-dot Personen, by multiplying the number of people, n, by their specific "
        "heat emission, q-dot p. A single person sitting at a desk outputs roughly "
        "100 W. Pack 50 students into a lecture hall, and you essentially have a "
        "5,000 W space heater running constantly."
    ),
    "beat_3_audio": (
        "But it doesn't stop with people. Every laptop, coffee machine, and server "
        "rack converts its electrical power, P el, directly into heat. We multiply "
        "this by a usage factor, f N, because not every device runs simultaneously. "
        "Add in the installed power of our ceiling lighting, P Licht, and the room "
        "starts to bake from the inside out."
    ),
    "beat_4_audio": (
        "In building physics, we sum all of these individual factors together to "
        "calculate our total internal cooling load, Q-dot internal. On a hot summer "
        "day, this internal heat trap means our cooling system has to work overtime, "
        "even if all the blinds are perfectly drawn."
    ),
}


def main():
    print("=== Scene 2 TTS Audio Generation (Internal Gains) ===\n")
    for name, text in NARRATIONS.items():
        out_path = BASE_DIR / f"{name}.mp3"
        print(f"Generating: {name} ...")
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
