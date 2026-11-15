"""🔠 Lower-case the internal-gains index in every shown symbol."""
import re
import sys

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy"
TARGETS = [
    r"❄️cooling\🎬️clip\README.md",
    r"❄️cooling\🎬️clip\⚖️heating-comparison\🐍️.py",
    r"❄️cooling\🎬️clip\👥️internal-gains\🐍️.py",
    r"❄️cooling\🎬️clip\🗺️roadmap\🐍️.py",
    r"🌬️ventilation\🎬️clip\❓️need\🐍️.py",
    r"🔥️heating\🎬️clip\README.md",
    r"🔥️heating\🎬️clip\🗺️roadmap\🐍️.py",
]
MATHRM = re.compile(r"\\mathrm\{\{I((?:,[a-z]+)?)\}\}|\\mathrm\{I((?:,[a-z]+)?)\}")
UNI = re.compile(r"(Q\u0307?_)I(?![A-Za-z0-9_])")


def main():
    for rel in TARGETS:
        path = ROOT + "\\" + rel
        with open(path, encoding="utf-8", newline="") as h:
            text = h.read()
        new = MATHRM.sub(lambda m: "i" + (m.group(1) or m.group(2) or ""), text)
        if rel.endswith(".md"):
            new = UNI.sub(r"\1i", new)
        if new != text:
            with open(path, "w", encoding="utf-8", newline="") as h:
                h.write(new)
            print("changed", rel)


main()
