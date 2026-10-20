#!/usr/bin/env python3
"""Anchored all-or-nothing apply of the staged law/fixture edits. Default dry-run; --write applies; refuses when an anchor drifted."""
import hashlib, json, os, sys
R = "/Users/ueli/Documents/semio"
T = os.path.dirname(os.path.abspath(__file__))
def sha(b): return hashlib.sha256(b).hexdigest()
def plan(op):
    p = R + "/" + op["rel"]
    if op["kind"] == "create":
        if not os.path.exists(p): return "apply", None
        return ("done", None) if sha(open(p, "rb").read()) == op["after"] else ("drift", "exists with other content")
    if not os.path.exists(p): return "drift", "missing"
    text = open(p, encoding="utf8").read()
    if sha(text.encode("utf8")) == op["after"]: return "done", None
    for h in op["hunks"]:
        if text.count(h["old"]) == 1: text = text.replace(h["old"], h["new"], 1)
        elif text.count(h["new"]) >= 1 and text.count(h["old"]) == 0: continue
        else: return "drift", "anchor " + str(text.count(h["old"])) + "x: " + h["old"][:80]
    return "apply", text
def main():
    m = json.load(open(T + "/staged/manifest.json"))
    plans = [(op, *plan(op)) for op in m["operations"]]
    bad = [(op["rel"], why) for op, st, why in [(a, b, c) for a, b, c in [(x[0], x[1], x[2]) for x in plans]] if st == "drift"]
    print(len(plans), "operations;", sum(1 for _, s, _w in [(x[0], x[1], x[2]) for x in plans] if s == "done"), "already applied;", len(bad), "drifted")
    for r, w in bad: print("DRIFT", r[-90:], w)
    if bad: sys.exit(1)
    if "--write" not in sys.argv: print("dry-run: no writes"); return
    for op, st, text in plans:
        if st != "apply": continue
        p = R + "/" + op["rel"]
        os.makedirs(os.path.dirname(p), exist_ok=True)
        tmp = p + ".tfl"
        if op["kind"] == "create": open(tmp, "wb").write(open(T + "/staged/" + op["file"], "rb").read())
        else: open(tmp, "w", encoding="utf8").write(text)
        os.replace(tmp, p)
    print("applied")
main()
