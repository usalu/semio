"""Shared helpers of the EN 1990 v7 cascade (ticket-local input script)."""
import json, os, re, sys
SEMIO = "/home/deck/semio"
ART = SEMIO + "/✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990"
SUB = ART + "/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCH = SUB + "/🧬️schema"
SNAP = SCH + "/📸️snapshot"
DIFF = SCH + "/🔺️diff"
MUT = SCH + "/🧬️mutations"
SQ = SUB + "/🚪️io/🪶️sqlite/📸️snapshot"
TXT = SUB + "/🚪️io/📝️text"
INF = SCH + "/💡️inferences"
rd = lambda p: open(p, encoding="utf8").read()

def wr(p, t):
    assert len(p) < 256, "path too long " + p
    open(p, "w", encoding="utf8").write(t)

def edit(p, fn):
    t = rd(p)
    n = fn(t)
    if n != t:
        wr(p, n)
        print("  edited", p.replace(SUB, "")[:110])
    return n

def rep(t, old, new, count=1, guard=None):
    if guard is not None and guard in t:
        return t
    assert t.count(old) == count, ("anchor", old[:110], t.count(old))
    return t.replace(old, new)

def sub(t, pattern, repl, count=1, flags=0):
    out, n = re.subn(pattern, repl, t, flags=flags)
    assert n == count, ("pattern", pattern[:110], n)
    return out
