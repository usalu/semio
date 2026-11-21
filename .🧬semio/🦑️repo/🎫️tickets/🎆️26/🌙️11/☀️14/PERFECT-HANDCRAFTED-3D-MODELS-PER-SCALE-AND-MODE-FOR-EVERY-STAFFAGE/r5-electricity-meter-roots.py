"""🖊️ Round 5, appliances/electricity-meter: the print root fix. The rotor disc is drawn edge-on as its true 1.2 mm sheet; with the 0.18 mm body pen it measured 0.196 mm at 1:75, the shared end of the 1:50 and 1:100 bands (office-laser minimum 0.2 mm). A heavier pen on the disc would cover the 2 mm window margin beside the dial plate (then 0.196 mm itself), so the disc of 1:50 and 1:100 is drawn, and modelled, at 1.6 mm (real discs are 1 to 2 mm thick): 0.021 + 0.18 = 0.201 mm at 1:75. The hub of 1:50 follows the new disc faces; nothing else changes.

Usage: python3 r5-electricity-meter-roots.py [--apply]   (without --apply: writes the candidate to 🗑️generated/r5-electricity-meter/🔣️.json)
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *[".."] * 7))
CATALOG = os.path.join(ROOT, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🗂️catalog", "🔌️appliances", "electricity-meter", "🔣️.json")
CANDIDATE = os.path.join(HERE, "🗑️generated", "r5-electricity-meter", "🔣️.json")


FACES = (("0.0194,", "0.0192,"), ("0.0206,", "0.0208,"))


def thicker(text, ident, count, after):
    """📐 In the first `count` shapes `ident` of bone disc after offset `after`, move the disc faces from 1.2 to 1.6 mm inside their points (the file keeps its own layout)."""
    at = after
    for _ in range(count):
        at = text.index(f'"id": "{ident}",', at)
        assert text.index('"bone": "disc"', at) < text.index('"closed"', at)
        end = text.index('"closed"', at)
        block = text[at:end]
        for old, new in FACES:
            block = block.replace(old, new)
        text = text[:at] + block + text[end:]
        at = end
    return text


def main():
    """📝 Re-read the catalog file right before writing; change the disc outline of the 1:50 and 1:100 drawings (the first two disc shapes under `representations`; 1:200 keeps its 1.2 mm disc with the 0.2 mm pen) and the 1:50 hub."""
    with open(CATALOG, encoding="utf-8") as handle:
        text = handle.read()
    reps = text.index('"representations"')
    text = thicker(thicker(text, "disc", 2, reps), "hub", 1, reps)
    target = CATALOG if "--apply" in sys.argv else CANDIDATE
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(text)
    print("written", target)


if __name__ == "__main__":
    main()
