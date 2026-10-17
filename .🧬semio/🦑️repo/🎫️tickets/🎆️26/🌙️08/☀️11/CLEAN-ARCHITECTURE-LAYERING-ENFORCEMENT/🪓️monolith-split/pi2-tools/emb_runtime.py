#!/usr/bin/env python3
"""📖️ Turns one oversized `include_str!` (expression context) of a test into a run-time read (the ftc precedent: embedded inputs became run-time reads so the closure counter stops counting them).
usage: emb_runtime.py <file relative to PLUG> <line> [--apply]"""
import sys, os, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tsplit import *
import ed
f = PLUG + "/" + sys.argv[1]
line = int(sys.argv[2])
text = read(f)
lines = text.split("\n")
clean = blank(text)
start = sum(len(l) + 1 for l in lines[:line - 1])
end = start + len(lines[line - 1])
seg = text[start:end]
new, n = [], 0
last = 0
for m in EMB_RX.finditer(seg):
    if m.group(1) != "include_str!":
        continue
    cseg = clean[start:end]
    if cseg[m.start():m.start() + 12] != "include_str!":
        continue
    if m.group(2):
        tgt = os.path.normpath(MANIFEST_DIR + "/" + m.group(2).lstrip("/"))
    else:
        tgt = os.path.normpath(os.path.join(os.path.dirname(f), m.group(3)))
    relp = os.path.relpath(tgt, MANIFEST_DIR)
    rep = '&std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/' + relp + '")).unwrap()'
    new.append(seg[last:m.start()]); new.append(rep); last = m.end(); n += 1
new.append(seg[last:])
assert n == 1, n
out = text[:start] + "".join(new) + text[end:]
print(sys.argv[1], line, "->", "".join(new)[:240])
if "--apply" in sys.argv:
    ed.put(f, out)
    print("applied")
