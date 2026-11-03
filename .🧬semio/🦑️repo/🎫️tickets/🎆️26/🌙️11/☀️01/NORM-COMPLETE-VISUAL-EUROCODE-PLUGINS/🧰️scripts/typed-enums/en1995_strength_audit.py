#!/usr/bin/env python3
"""Audit of en1995 strength-class values in data files: compares the ordered strength-class tokens of every data file with its committed version (HEAD, else the index) and lists files without any committed version.
Usage: python3 -I en1995_strength_audit.py [--list-none]"""
import os, re, subprocess, sys
REPO = "/home/deck/semio"
ROOT = REPO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any"
TOK = re.compile(r"(?<![\w.])((?:C|D)\d{2}|GL\d{2}[hc]|LVL\d{2}|CLT\d{3})(?![\w])", re.I)
EXT = (".json", ".semio", ".feature", ".sql")
def show(spec):
    r = subprocess.run(["git", "-C", REPO, "show", spec], capture_output=True)
    return r.stdout.decode("utf-8", "replace") if r.returncode == 0 else None
def toks(t):
    return [m.group(1).lower() for m in TOK.finditer(t)]
bad, none = [], []
for base, dirs, files in os.walk(ROOT):
    dirs[:] = [d for d in dirs if d not in ("target", "node_modules")]
    for f in files:
        if not f.endswith(EXT):
            continue
        p = os.path.join(base, f)
        t = open(p, encoding="utf-8", errors="replace").read()
        cur = toks(t)
        if not cur:
            continue
        rel = os.path.relpath(p, REPO)
        h = show("HEAD:" + rel)
        if h is None:
            h = show(":" + rel)
        if h is None:
            none.append(rel)
        elif toks(h) != cur:
            bad.append((rel, toks(h), cur))
print("no committed version:", len(none))
if "--list-none" in sys.argv:
    for r in none: print("  none:", r.split("any/")[1])
print("differs from committed:", len(bad))
for r, a, b in bad:
    print("  diff:", r.split("any/")[1], a[:8], "->", b[:8])
