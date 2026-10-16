#!/usr/bin/env python3
# 🛠️ Anchored edit helpers of the RENDERER-INTEGRATION-1 pass: every replacement asserts a unique anchor, every cut asserts its text.
import os, sys
REPO = "/Users/ueli/Documents/semio"
E = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T = E + "/🎯️targets/🧊️wgpu"
R = T + "/🧊️renderer"
EL = E + "/🧱️elements"
SPLIT = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SP = "/private/tmp/claude-501/-Users-ueli-Documents-semio/11914954-c49c-43e1-9499-97ab9f20751b/scratchpad"

def read(p):
    with open(p, encoding="utf8") as h:
        return h.read()

def write(p, t):
    os.makedirs(os.path.dirname(p), exist_ok=True)
    with open(p, "w", encoding="utf8") as h:
        h.write(t)

def sub1(p, old, new):
    t = read(p)
    n = t.count(old)
    assert n == 1, (p, n, old[:80])
    write(p, t.replace(old, new))

def cut_between(p, start, end, repl="", end_inclusive=False):
    t = read(p)
    assert t.count(start) == 1, (p, "start", t.count(start), start[:60])
    i = t.index(start)
    j = t.index(end, i)
    assert t.count(end, i) >= 1
    if end_inclusive:
        j += len(end)
    write(p, t[:i] + repl + t[j:])
    return t[i:j]
