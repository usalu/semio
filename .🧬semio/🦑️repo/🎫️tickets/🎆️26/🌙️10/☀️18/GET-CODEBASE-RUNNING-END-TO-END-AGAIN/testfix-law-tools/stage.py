#!/usr/bin/env python3
"""Stages every tracked overlay edit as anchored line-hunk replacements (new files as hashed copies) into staged/manifest.json."""
import difflib, hashlib, json, os, shutil
O = "/private/tmp/testfix-law"
T = os.path.dirname(os.path.abspath(__file__))
S = T + "/staged"
def sha(b): return hashlib.sha256(b).hexdigest()
def hunks(orig, new):
    a, b = orig.split("\n"), new.split("\n")
    out = []
    for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
        if tag == "equal": continue
        k = 0 if i1 < i2 else 1
        while True:
            lo, hi = max(0, i1 - k), min(len(a), i2 + k)
            old = "\n".join(a[lo:hi])
            if (old and orig.count(old) == 1) or (lo == 0 and hi == len(a)): break
            k += 1
        rep = "\n".join(a[lo:i1] + b[j1:j2] + a[i2:hi])
        out.append({"old": old, "new": rep})
    return out
def main():
    idx = json.load(open(O + "/.orig/index.json"))
    shutil.rmtree(S, ignore_errors=True); os.makedirs(S)
    ops = []
    for rel, o in sorted(idx.items()):
        p = O + "/" + rel
        if not os.path.exists(p): continue
        data = open(p, "rb").read()
        if o["orig"] is None:
            n = str(len(ops)).zfill(4) + ".bin"
            open(S + "/" + n, "wb").write(data)
            ops.append({"rel": rel, "kind": "create", "file": n, "after": sha(data)}); continue
        orig = open(O + "/.orig/" + o["orig"], "rb").read()
        if orig == data: continue
        hs = hunks(orig.decode("utf8"), data.decode("utf8"))
        ops.append({"rel": rel, "kind": "edit", "before": sha(orig), "after": sha(data), "hunks": hs})
    json.dump({"version": 2, "operations": ops}, open(S + "/manifest.json", "w"), ensure_ascii=False, indent=1)
    print(len(ops), "operations,", sum(len(o.get("hunks", [])) for o in ops), "hunks")
main()
