"""Generate a short English VO clip for the NGS intro card (optional mux)."""

import sys
from pathlib import Path

NOWIGETIT = Path("/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt")
sys.path.insert(0, str(NOWIGETIT))

from backend.pipeline.tts import synthesize_narration

BASE_DIR = Path(__file__).resolve().parent

NARRATIONS = {
    "intro_audio": (
        "Welcome to this tutorial from Sustainable Building Systems "
        "at Leibniz University Hannover. "
        "In this video, we explain the topic shown on screen."
    ),
    "intro_physical_fundamentals": (
        "Welcome to this tutorial from Sustainable Building Systems "
        "at Leibniz University Hannover. "
        "This video is the physical foundation: force, work, power and energy — "
        "the language of heating and cooling demand."
    ),
}


def main():
    for name, text in NARRATIONS.items():
        out_path = BASE_DIR / f"{name}.mp3"
        result_path, skipped = synthesize_narration(text, out_path)
        print(name, "skipped" if skipped else result_path)


if __name__ == "__main__":
    main()
