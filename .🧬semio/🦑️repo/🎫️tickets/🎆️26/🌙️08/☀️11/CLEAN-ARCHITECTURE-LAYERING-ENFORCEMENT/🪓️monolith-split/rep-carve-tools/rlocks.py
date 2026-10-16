"""🔒 Refreshes the hand-derived lock entries of the carved packages from the current staged manifests (idempotent)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rregistry as G, rapply
changed = 0
targets = [("🧰️framework/Cargo.lock", True), ("Cargo.lock", False)] + [(l, False) for l in rapply.LOCKS]
for rel, dev in targets:
    t = open(REPO + rel, encoding="utf8").read()
    try:
        n = G.apply_lock(t, G.new_blocks(dev))
    except AssertionError as e:
        print("skipped", rel); continue
    if n != t:
        open(REPO + rel, "w", encoding="utf8").write(n); changed += 1; print("updated", rel)
print("locks updated", changed)
