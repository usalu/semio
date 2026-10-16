"""Own-closure census (files, bytes, active lines) of each target crate: baseline snapshot vs live tree."""
import os, sys
sys.path.insert(0, os.path.dirname(__file__))
from rcrlib import *
from reach import reach
from snapshot import snap_dir

def active(text):
    return sum(1 for l in text.split("\n") if l.strip() and not l.strip().startswith("//"))

def census(files):
    b = a = 0
    for p in files:
        t = read(p)
        b += len(t.encode("utf-8")); a += active(t)
    return len(files), b, a

print(f"{'crate':<14} {'files base->now':>16} {'bytes base->now':>22} {'active base->now':>18}")
for n, t in TARGETS.items():
    order_b, _, _, _ = reach(f"{snap_dir(n)}/{t['root']}")
    # snapshot closure: mounts are relative to the snapshot tree, same layout
    order_n, _, _, _ = reach(f"{ROOT}/{t['root']}")
    fb, bb, ab = census(order_b); fn, bn, an = census(order_n)
    print(f"{n:<14} {fb:>7} -> {fn:<6} {bb:>10} -> {bn:<9} {ab:>8} -> {an:<7}")
