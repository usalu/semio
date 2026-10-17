"""🧾 C4P28 public API check: the set of `pub` (exactly `pub`) top-level item names of each lib root before versus the root plus its children after, and the re-export wiring
(a child that defines a `pub` item is re-exported with `pub use self::<child>::*;`). usage: apicheck.py stage|live"""
import glob, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE + "/../rf8-tools")
from rslex import outline

REPO = "/Users/ueli/Documents/semio"
mode = sys.argv[1]
bad = 0
for path in sorted(glob.glob(HERE + "/specs/*.json")):
    n = int(os.path.basename(path).split(".")[0])
    spec = json.load(open(path, encoding="utf-8"))
    if "segments" not in spec["chunks"][0]:
        continue
    base = REPO if mode == "live" else spec["stage"]
    d = os.path.dirname(spec["file"])

    def pubs(text):
        out = set()
        for it in outline(text):
            if it["vis"] == "pub" and it["kind"] in ("fn", "struct", "enum", "trait", "const", "static", "type", "union", "mod", "macro_rules!") and it["name"]:
                out.add((it["kind"], it["name"]))
        return out

    before = pubs(open(spec["original"], encoding="utf-8").read())
    root_text = open(f"{base}/{spec['file']}", encoding="utf-8").read()
    after = pubs(root_text)
    problems = []
    for c in spec["chunks"]:
        ct = open(f"{base}/{d}/{c['folder']}/🦀️.rs", encoding="utf-8").read()
        cp = pubs(ct)
        after |= cp
        has_pub = bool(cp)
        pub_use = f"pub use self::{c['name']}::*;" in root_text
        if has_pub and not pub_use:
            problems.append(f"{c['name']}: defines pub items {sorted(cp)[:3]} but is not re-exported with pub use")
    only_b, only_a = before - after, after - before
    ok = not only_b and not only_a and not problems
    bad += 0 if ok else 1
    print(f"[{n}] pub top-level items before={len(before)} after={len(after)} lost={len(only_b)} gained={len(only_a)} {'OK' if ok else 'PROBLEM'}")
    for x in sorted(only_b):
        print("    lost", x)
    for x in sorted(only_a):
        print("    gained", x)
    for p in problems:
        print("    PROBLEM", p)
print("APICHECK", "OK" if bad == 0 else f"{bad} PROBLEM")
sys.exit(1 if bad else 0)
