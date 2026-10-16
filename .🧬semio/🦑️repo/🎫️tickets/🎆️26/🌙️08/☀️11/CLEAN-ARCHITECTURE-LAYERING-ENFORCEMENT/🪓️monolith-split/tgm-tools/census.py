"""Lib (non-test) and test closure census of one crate: files / bytes / active lines, before (snapshot) and after (live)."""
import json, os, re, sys
sys.path.insert(0, os.path.dirname(__file__))
from tgmlib import *
from modtree import mounts_of


def walk(lib_abs):
    lib_files, all_files = {}, {}
    stack = [(lib_abs, False)]
    seen = {}
    while stack:
        f, tonly = stack.pop()
        if f in seen and (seen[f] is False or tonly):
            continue
        seen[f] = tonly if f not in seen else (seen[f] and tonly)
        if not os.path.isfile(f):
            continue
        text = read(f)
        lines = text.split("\n")
        d = os.path.dirname(f)
        for kind, p, inl in mounts_of(text):
            if kind == "plain":
                continue
            tgt = os.path.normpath(os.path.join(d, *inl, p)) if kind == "path" else os.path.normpath(os.path.join(d, p))
            if not tgt.endswith(".rs"):
                continue
            idx = next((i for i, l in enumerate(lines) if p in l and ("#[path" in l or "include!" in l)), None)
            gated = False
            if idx is not None:
                attrs = [lines[idx]]
                k = idx - 1
                while k >= 0 and (lines[k].strip().startswith("#[") or lines[k].strip().startswith("//")):
                    attrs.append(lines[k])
                    k -= 1
                gated = bool(re.search(r"cfg\((?:any\(|all\()?test\b", "\n".join(attrs)))
            if "🧪️tests" in tgt:
                gated = True
            stack.append((tgt, tonly or gated))
    res = {}
    for f, tonly in seen.items():
        if os.path.isfile(f):
            res[f] = tonly
    return res


def stats(files):
    b = l = 0
    for f in files:
        tx = read(f)
        b += len(tx.encode("utf-8"))
        l += len([x for x in tx.split("\n") if x.strip() and not x.strip().startswith("//")])
    return dict(files=len(files), bytes=b, active_lines=l)


def main():
    out = {}
    for name, t in TARGETS.items():
        r = {}
        for tag, base in (("before", snap_dir(name)), ("after", ROOT)):
            w = walk(os.path.join(base, t["lib"]))
            lib = [f for f, to in w.items() if not to]
            r[tag] = dict(lib=stats(lib), all=stats(list(w)))
        out[name] = r
    print(json.dumps(out, ensure_ascii=False, indent=1))
    write(F + "/🗑️generated/tgm-census.json", json.dumps(out, ensure_ascii=False, indent=1))


main()
