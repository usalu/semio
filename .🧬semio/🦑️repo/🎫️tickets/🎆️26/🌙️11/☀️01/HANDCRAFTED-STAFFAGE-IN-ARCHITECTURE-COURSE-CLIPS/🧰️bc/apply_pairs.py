"""🪄 Apply exact en/de pair rewrites and generic capitalisation fixes with a fresh read and a validity check per file."""
import json
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent / "🧰️bl"))
import audit_bilingual as a

EVENTS = {
    ("heel-l on", "heel-l auf"): ("Left heel contact", "Linker Fersenkontakt"),
    ("ball-r on", "ball-r auf"): ("Right ball contact", "Rechter Fußballenkontakt"),
    ("ball-l on", "ball-l auf"): ("Left ball contact", "Linker Fußballenkontakt"),
    ("ball-r off", "ball-r ab"): ("Right ball lifts", "Rechter Fußballen hebt ab"),
    ("heel-l off", "heel-l ab"): ("Left heel lifts", "Linke Ferse hebt ab"),
    ("heel-r on", "heel-r auf"): ("Right heel contact", "Rechter Fersenkontakt"),
    ("ball-l off", "ball-l ab"): ("Left ball lifts", "Linker Fußballen hebt ab"),
    ("heel-r off", "heel-r ab"): ("Right heel lifts", "Rechte Ferse hebt ab"),
}
PAIRS = dict(EVENTS)
for n in range(1, 5):
    for tag, en_t, de_t in (("lo", "lower", "unten"), ("hi", "upper", "oben")):
        PAIRS[(f"Wind drag on fol-{n}-{tag}", f"Winddruck auf fol-{n}-{tag}")] = (f"Wind drag on leaf mat {n}, {en_t}", f"Winddruck auf Blattteppich {n}, {de_t}")
        PAIRS[(f"Snow on the leaf mat fol-{n}-{tag}", f"Schnee auf dem Blattteppich fol-{n}-{tag}")] = (f"Snow on leaf mat {n}, {en_t}", f"Schnee auf Blattteppich {n}, {de_t}")
PAIRS.update({
    ("Switched off", "Aus"): ("Switched off", "Ausgeschaltet"),
    ("Denim jeans", "Denim"): ("Denim jeans", "Jeans"),
    ("Breath from the nostrils", "Atem aus den Nasenöffnungen"): ("Breath from the nostrils", "Atem aus den Nasenlöchern"),
    ("Socks and sneakers", "Socken und Turnschuhe"): ("Socks and sneakers", "Socken und Sneaker"),
    ("Jogger", "Jogger:in"): ("Jogger", "Joggende Person"),
    ("Skateboarder", "Skateboarder:in"): ("Skateboarder", "Skateboardfahrende Person"),
    ("Kneeling gardener", "Kniende Gärtnerin"): ("Kneeling Gardener", "Kniende Gärtnerin"),
    ("Parent carrying a toddler", "Elternteil trägt ein Kleinkind"): ("Parent Carrying a Toddler", "Elternteil trägt ein Kleinkind"),
    ("Seated guitarist", "Sitzender Gitarrist"): ("Seated Guitarist", "Sitzender Gitarrist"),
    ("the 0.5 mm drops and the 3 mm trough rim", "die Tropfen und der 3 mm dicke Rinnenrand"): ("The 0.5 mm drops and the 3 mm trough rim", "Die 0,5 mm großen Tropfen und der 3 mm dicke Rinnenrand"),
})
DE_ONLY = {
    "Fahrschleife start": "Fahrschleife Start",
    "Linke Fersenabhebung": "Linke Ferse hebt ab",
    "Rechte Fersenabhebung": "Rechte Ferse hebt ab",
}
TREES = {"apple-tree", "japanese-maple", "london-plane-pollarded", "small-leaved-lime"}
LIMIT = re.compile(r'("limitingFeature": \{\s*"en": ")([a-zäöü])(.*?",\s*"de": ")([a-zäöü])')
TITLE_EN = re.compile(r'("title": \{\s*"en": ")([a-z])')


def pair_rx(en, de):
    return re.compile(r'("en": )"' + re.escape(en) + r'"(,\s*"de": )"' + re.escape(de) + '"')


def fix(text, tree):
    n = 0
    for (en, de), (nen, nde) in PAIRS.items():
        text, c = pair_rx(en, de).subn(lambda m: f'{m.group(1)}"{nen}"{m.group(2)}"{nde}"', text)
        n += c
    for old, new in DE_ONLY.items():
        c = text.count(f'"de": "{old}"')
        text = text.replace(f'"de": "{old}"', f'"de": "{new}"')
        n += c
    text, c = LIMIT.subn(lambda m: m.group(1) + m.group(2).upper() + m.group(3) + m.group(4).upper(), text)
    n += c
    if tree:
        text, c = TITLE_EN.subn(lambda m: m.group(1) + m.group(2).upper(), text)
        n += c
    return text, n


def main():
    total = 0
    for f in sorted(a.CATALOG.rglob("🔣️.json")):
        text = f.read_text(encoding="utf8")
        new, n = fix(text, f.parent.name in TREES)
        if not n or new == text:
            continue
        json.loads(new)
        if f.read_text(encoding="utf8") != text:
            print("SKIP (changed meanwhile)", f.parent.name)
            continue
        try:
            f.write_text(new, encoding="utf8", newline="")
        except OSError as e:
            print("WRITE FAILED", f.parent.name, e)
            continue
        total += n
        print(f.parent.name, n)
    print("replacements", total)


if __name__ == "__main__":
    main()
