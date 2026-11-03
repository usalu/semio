"""🧭 Handle write paths against edit rules: every `[id={..}]`/`{}` path literal in the scene dirs, normalised to `[]`, must be a SetFieldRule path, a member-relative suffix of one, or the entity container of one.

`python -I handle-paths-check.py <artifact>...`
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "leaf-kinds"))
from common import *
EMOJI = {"en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1993": "🔩️", "en1994": "🧩️", "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️", "en1998": "🫨️", "en1999": "🪶️"}
for short in sys.argv[1:]:
    root = NORM + "/🗿️artifacts/" + EMOJI[short] + short + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
    rules = "".join(rd(os.path.join(d, f)) for d, _, fs in os.walk(root + "/🧬️schema/🧬️mutations/🧭️edit-rules") for f in fs if f.endswith(".rs"))
    covered = set(re.findall(r'SetFieldRule \{ path: "([^"]+)"', rules))
    heads = {c[i + 2:] for c in covered for i in [m.start() for m in re.finditer(r"\]\.", c)]} | covered
    def ok(p):
        return any(c == p or c.startswith(p + ".") or c.startswith(p + "[].") for c in heads)
    found = {}
    for d, _, fs in os.walk(root):
        if "🎬" not in d and "scene" not in d: continue
        if "🧪" in d: continue
        for f in fs:
            if not f.endswith(".rs"): continue
            text = open(os.path.join(d, f), encoding="utf-8").read()
            for m in re.finditer(r'"([A-Za-z]+\[(?:id|index)?=?\{[^}]*\}\](?:\.[A-Za-z]+(?:\[[^\]]*\])?)+)"', text):
                p = re.sub(r"\[[^\]]*\]", "[]", m.group(1))
                if not ok(p): found.setdefault(p, os.path.join(d, f))
    print(short, len(found), "uncovered handle paths")
    for p, f in sorted(found.items()): print("   ", p, f.split("subsets/")[-1])
    if found: sys.exit(1)
