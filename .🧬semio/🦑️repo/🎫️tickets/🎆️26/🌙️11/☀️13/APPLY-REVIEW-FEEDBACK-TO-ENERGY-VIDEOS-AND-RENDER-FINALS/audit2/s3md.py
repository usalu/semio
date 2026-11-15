import re, os
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = "C:/git/semio"
files = [l.strip() for l in open(os.path.join(HERE, "files.txt"), encoding="utf-8") if l.strip()]
UNI = re.compile("[ΔθϑΦρληΣψΘΛΩ]")
SPELLED = re.compile(r"(?<![\\A-Za-z_])(Delta|Theta|Rho|Phi|Lambda|Eta|Sigma|Q-Punkt|Q Punkt|Q-Strich|c-Luft|Delta-Theta|Theta-i|Eta-h|Phi-V)(?![A-Za-z_])")
DOLLAR = re.compile(r"\$[^$]*\$")
CODE = re.compile(r"`[^`]*`")
out = []
for f in files:
    if not (f.endswith(".md") or f.endswith(".json")):
        continue
    if "tests" in f or "lexicon" in f or "/🧮️tex/" in f:
        continue
    lines = open(os.path.join(ROOT, f), encoding="utf-8").read().splitlines()
    for i, l in enumerate(lines, 1):
        t = CODE.sub("", DOLLAR.sub("", l))
        a = UNI.search(t)
        b = SPELLED.search(t)
        if a or b:
            out.append(f"{f}:{i}: [{'UNI' if a else ''}{'SPELL' if b else ''}] {l.strip()[:200]}")
print("\n".join(out))
print("TOTAL", len(out))
