"""🔁 Re-syncs changed staged files into the live tree after the one-step apply (only files this carve owns)."""
import os, shutil, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
changed = []
for dp, dn, fn in os.walk(STAGE):
    for f in fn:
        rel = os.path.relpath(os.path.join(dp, f), STAGE)
        if rel.endswith(".deleted"):
            if os.path.exists(REPO + rel[:-8]): changed.append(rel)
            continue
        live = REPO + rel
        if not os.path.exists(live) or open(live, "rb").read() != open(STAGE + rel, "rb").read(): changed.append(rel)
for rel in changed: print("changed:", rel.replace(R, ""))
if "--write" in sys.argv:
    for rel in changed:
        if rel.endswith(".deleted"): os.remove(REPO + rel[:-8]); continue
        os.makedirs(os.path.dirname(REPO + rel), exist_ok=True)
        shutil.copy2(STAGE + rel, REPO + rel)
    print("synced", len(changed))
