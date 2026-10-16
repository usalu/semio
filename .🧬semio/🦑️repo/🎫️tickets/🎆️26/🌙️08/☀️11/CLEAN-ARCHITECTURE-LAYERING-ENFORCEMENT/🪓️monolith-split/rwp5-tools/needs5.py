# 🧭️ Crate-module needs of the WP5 crates: which old renderer-root modules (`crate::interpreter`, `crate::scenes`, …) each staging crate names, directly or through the parent's `use crate::…` lines.
import os, re, sys, collections, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import cfg, ctx
from blank import blank

def flatten_uses(parent_text):
    b = blank(parent_text); out = []
    for m in re.finditer(r"(?m)^(?:#\[[^\n]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?use\s+crate::([A-Za-z_]\w*)((?:::[^;]*)?);", b):
        mod, rest = m.group(1), m.group(2)
        names = set(re.findall(r"(?<![\w:])([A-Za-z_]\w*)(?:\s+as\s+(\w+))?\s*(?=[,}]|$)", rest))
        flat = set()
        for a, al in names: flat.add(al or a)
        rest2 = rest.strip()
        if rest2.startswith("::") and "{" not in rest2:
            flat = {rest2.split("::")[-1].split(" as ")[-1].strip()}
        out.append((mod, flat))
    return out

def main(root):
    parent = open(f"{root}/{cfg.PARENT}", encoding="utf-8").read()
    uses = flatten_uses(parent)
    res = {}
    for crate in list(ctx.BRIDGE) + ["shell-view-paint"]:
        text = ""
        for slug, p in ctx.crate_files(root, crate): text += blank(open(p, encoding="utf-8").read()) + "\n"
        toks = set(re.findall(r"(?<![\w])([A-Za-z_]\w*)", text))
        mods = collections.Counter()
        for m in re.finditer(r"\bcrate::([A-Za-z_]\w*)", text): mods[m.group(1)] += 1
        for mod, names in uses:
            if any(n in toks for n in names): mods[mod] += 0
        res[crate] = dict(mods)
    return res

if __name__ == "__main__":
    r = main(sys.argv[1] if len(sys.argv) > 1 else cfg.NEW)
    for c, v in r.items(): print(c, sorted(v))
    json.dump(r, open(cfg.GEN + "/needs5.json", "w"), indent=1, ensure_ascii=False)
