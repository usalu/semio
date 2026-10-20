#!/usr/bin/env python3
"""Anchored all-or-nothing apply of the staged testfix-real set. Default dry run; --write applies; --root DIR targets another checkout (overlay verification); refuses when any anchor drifted. Operations: edit (exact-once hunks, sha of the result), create (hashed copy), delete (sha-anchored), move (sha-anchored rename)."""
import hashlib, json, os, shutil, sys
R = "/Users/ueli/Documents/semio"
T = os.path.dirname(os.path.abspath(__file__))
def sha(b): return hashlib.sha256(b).hexdigest()
def arg(name, default=None):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default
def plan(root, op):
    p = root + "/" + op["rel"]
    k = op["kind"]
    if k == "create":
        if not os.path.exists(p): return "apply", None
        return ("done", None) if sha(open(p, "rb").read()) == op["after"] else ("drift", "exists with other content")
    if k == "delete":
        if not os.path.exists(p): return "done", None
        return ("apply", None) if sha(open(p, "rb").read()) == op["before"] else ("drift", "content changed")
    if k == "move":
        q = root + "/" + op["to"]
        if not os.path.exists(p): return ("done", None) if os.path.exists(q) and sha(open(q, "rb").read()) == op["sha"] else ("drift", "source missing")
        if sha(open(p, "rb").read()) != op["sha"]: return "drift", "source changed"
        if os.path.exists(q): return "drift", "destination exists"
        return "apply", None
    if not os.path.exists(p): return "drift", "missing"
    text = open(p, encoding="utf8").read()
    if sha(text.encode("utf8")) == op["after"]: return "done", None
    for h in op["hunks"]:
        if text.count(h["old"]) == 1: text = text.replace(h["old"], h["new"], 1)
        else: return "drift", "anchor " + str(text.count(h["old"])) + "x: " + h["old"][:80]
    return ("apply", text) if sha(text.encode("utf8")) == op["after"] else ("drift", "result hash differs")
def main():
    root = arg("--root", R).rstrip("/")
    only = arg("--group")
    m = json.load(open(T + "/staged/manifest.json"))
    ops = [o for o in m["operations"] if only is None or o.get("group", "default") == only or (only == "all")]
    if only is None: ops = [o for o in m["operations"] if o.get("group", "default") == "default"]
    plans = [(op,) + plan(root, op) for op in ops]
    bad = [(op["rel"], why) for op, st, why in plans if st == "drift"]
    print(len(plans), "operations;", sum(1 for _, st, _w in plans if st == "done"), "already applied;", len(bad), "drifted")
    for r, w in bad: print("DRIFT", r[-110:], w)
    if bad: sys.exit(1)
    if "--write" not in sys.argv: print("dry-run: no writes"); return
    for op, st, text in plans:
        if st != "apply": continue
        p = root + "/" + op["rel"]
        k = op["kind"]
        if k == "delete": os.remove(p); continue
        if k == "move":
            q = root + "/" + op["to"]; os.makedirs(os.path.dirname(q), exist_ok=True); os.rename(p, q); continue
        os.makedirs(os.path.dirname(p), exist_ok=True)
        tmp = p + ".tfr"
        if k == "create": shutil.copyfile(T + "/staged/" + op["file"], tmp)
        else: open(tmp, "w", encoding="utf8").write(text)
        os.replace(tmp, p)
    print("applied")
main()
