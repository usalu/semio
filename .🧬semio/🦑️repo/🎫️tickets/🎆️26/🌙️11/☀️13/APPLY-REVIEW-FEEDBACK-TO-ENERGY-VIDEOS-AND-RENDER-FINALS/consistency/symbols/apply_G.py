"""🌡️ Unify the degree-day symbol to G_{t,20/15} and the heating-period factor to F_{Gt}."""
import re

ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy"
PY = [
    r"📊️demand\🎬️clip\⚖️regulations\🐍️.py",
    r"📊️demand\🎬️clip\🔁️recap\🧪️tests\🧮️balance\🐍️.py",
    r"📊️demand\🎬️clip\🗺️roadmap\🐍️.py",
    r"🔥️heating\🎬️clip\🧮️calculation\🐍️.py",
]
MD = [
    r"📊️demand\🎬️clip\README.md",
    r"📖️concepts\📝️.md",
]
NEW = "G_{t,20/15}"
PY_RULES = [
    (re.compile(r"G_\{\\mathrm\{t20/15\}\}"), NEW),
    (re.compile(r"G_\{\\mathrm\{t\}\}"), NEW),
    (re.compile(r"G_\{\{\\mathrm\{\{t\}\}\}\}"), "G_{{t,20/15}}"),
    (re.compile(r"(?<![A-Za-z_{\\])G_t20/15"), "G_{t,20/15}"),
    (re.compile(r"(?<![A-Za-z_{\\])G_t(?![A-Za-z0-9_{,/])"), NEW),
    (re.compile(r"degree days Gt20/15"), "degree days G_t,20/15"),
    (re.compile(r"Gradtagzahl Gt20/15"), "Gradtagzahl G_t,20/15"),
    (re.compile(r"F_\{\\mathrm\{Gt\}\}"), "F_{Gt}"),
    (re.compile(r"F_\{\{\\mathrm\{\{Gt\}\}\}\}"), "F_{{Gt}}"),
]
MD_RULES = [
    (re.compile(r"G_t20/15"), NEW),
    (re.compile(r"(?<![A-Za-z_{])G_t(?![A-Za-z0-9_{,/])"), NEW),
]


def apply(rel, rules):
    path = ROOT + "\\" + rel
    with open(path, encoding="utf-8", newline="") as h:
        text = h.read()
    new = text
    for pattern, repl in rules:
        new = pattern.sub(lambda m, r=repl: r, new)
    if new != text:
        with open(path, "w", encoding="utf-8", newline="") as h:
            h.write(new)
        print("changed", rel)


for f in PY:
    apply(f, PY_RULES)
for f in MD:
    apply(f, MD_RULES)
