"""🔠 Match the heating family: the internal-gains index is an upright lower-case i inside every load symbol."""
import re

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy"
TARGETS = [
    r"❄️cooling\🎬️clip\⚖️heating-comparison\🐍️.py",
    r"❄️cooling\🎬️clip\👥️internal-gains\🐍️.py",
    r"❄️cooling\🎬️clip\🗺️roadmap\🐍️.py",
    r"🌬️ventilation\🎬️clip\❓️need\🐍️.py",
]
SINGLE = re.compile(r"(\\dot\{Q\}_\{)(i(?:,[a-z]+)?)(\})")
DOUBLE = re.compile(r"(\\dot\{\{Q\}\}_\{\{)(i(?:,[a-z]+)?)(\}\})")


def main():
    for rel in TARGETS:
        path = ROOT + "\\" + rel
        with open(path, encoding="utf-8", newline="") as h:
            text = h.read()
        new = SINGLE.sub(lambda m: m.group(1) + "\\mathrm{" + m.group(2) + "}" + m.group(3), text)
        new = DOUBLE.sub(lambda m: m.group(1) + "\\mathrm{{" + m.group(2) + "}}" + m.group(3), new)
        if new != text:
            try:
                with open(path, "w", encoding="utf-8", newline="") as h:
                    h.write(new)
                print("changed", rel)
            except OSError as error:
                print("FAILED", rel, error)


main()
