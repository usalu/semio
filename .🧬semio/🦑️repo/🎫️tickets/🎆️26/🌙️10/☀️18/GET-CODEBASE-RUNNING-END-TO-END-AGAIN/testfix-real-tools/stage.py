#!/usr/bin/env python3
"""Stages the difference between the pristine overlay r0 and the working overlay r as anchored operations (edit hunks, created files as hashed copies, deletes, moves) into staged/manifest.json. Files listed in ov-ignore.txt are verification stubs and never staged. usage: stage.py"""
import difflib, hashlib, json, os, shutil, sys
O0 = "/private/tmp/testfix-real/r0"
O = "/private/tmp/testfix-real/r"
T = os.path.dirname(os.path.abspath(__file__))
S = T + "/staged"
SKIP_DIRS = {"node_modules", ".git", "target"}
def sha(b): return hashlib.sha256(b).hexdigest()
def walk(root):
    out = {}
    for dp, dns, fns in os.walk(root):
        dns[:] = [d for d in dns if d not in SKIP_DIRS]
        for f in fns:
            p = os.path.join(dp, f)
            rel = os.path.relpath(p, root)
            if rel.startswith(".tfr-") or os.path.islink(p): continue
            out[rel] = p
    return out
def hunks(orig, new):
    a, b = orig.split("\n"), new.split("\n")
    cur = list(a)
    out = []
    for tag, i1, i2, j1, j2 in reversed(difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes()):
        if tag == "equal": continue
        text = "\n".join(cur)
        k = 0 if (i1 < i2 and j1 < j2) else 1
        while True:
            lo, hi = max(0, i1 - k), min(len(cur), i2 + k)
            old = "\n".join(cur[lo:hi])
            if (old and text.count(old) == 1) or (lo == 0 and hi == len(cur)): break
            k += 1
        rep = cur[lo:i1] + b[j1:j2] + cur[i2:hi]
        assert rep or (lo == 0 and hi == len(cur))
        out.append({"old": old, "new": "\n".join(rep)})
        cur[lo:hi] = rep
    assert "\n".join(cur) == new
    return out
def main():
    ignore = set(l.strip() for l in open(T + "/ov-ignore.txt", encoding="utf8")) if os.path.exists(T + "/ov-ignore.txt") else set()
    w0, w1 = walk(O0), walk(O)
    shutil.rmtree(S, ignore_errors=True); os.makedirs(S)
    edits, creates, deletes = [], [], []
    for rel, p in w1.items():
        if rel in ignore: continue
        d = open(p, "rb").read()
        if rel not in w0: creates.append((rel, d)); continue
        o = open(w0[rel], "rb").read()
        if o != d: edits.append((rel, o, d))
    for rel, p in w0.items():
        if rel in ignore: continue
        if rel not in w1: deletes.append((rel, open(p, "rb").read()))
    ops = []
    created = {sha(d): (rel, d) for rel, d in creates}
    moved_from = {}
    for rel, o in deletes:
        h = sha(o)
        if h in created and created[h][0] not in moved_from.values():
            ops.append({"kind": "move", "rel": rel, "to": created[h][0], "sha": h}); moved_from[rel] = created[h][0]
        else: ops.append({"kind": "delete", "rel": rel, "before": h})
    moved_to = set(moved_from.values())
    for rel, d in creates:
        if rel in moved_to: continue
        n = str(len(ops)).zfill(4) + ".bin"
        open(S + "/" + n, "wb").write(d)
        ops.append({"kind": "create", "rel": rel, "file": n, "after": sha(d)})
    for rel, o, d in edits:
        ops.append({"kind": "edit", "rel": rel, "before": sha(o), "after": sha(d), "hunks": hunks(o.decode("utf8"), d.decode("utf8"))})
    gp = T + "/ov-groups.json"
    groups = json.load(open(gp, encoding="utf8")) if os.path.exists(gp) else []
    for op in ops:
        for g in groups:
            if any(op["rel"].startswith(x) or op.get("to", "").startswith(x) for x in g["prefixes"]): op["group"] = g["group"]; break
        op.setdefault("group", "default")
    json.dump({"version": 3, "operations": ops}, open(S + "/manifest.json", "w"), ensure_ascii=False, indent=1)
    c = {}
    for op in ops: c[op["kind"]] = c.get(op["kind"], 0) + 1
    print(len(ops), "operations", c, "hunks", sum(len(o.get("hunks", [])) for o in ops))
main()
