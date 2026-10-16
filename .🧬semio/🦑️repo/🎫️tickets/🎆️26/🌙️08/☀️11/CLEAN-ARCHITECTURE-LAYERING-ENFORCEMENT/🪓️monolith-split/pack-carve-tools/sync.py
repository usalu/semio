"""🔁 Re-syncs changed staged files into the live tree after the one-step apply (only files this carve owns)."""
import os, shutil, sys, filecmp
from lib import *
changed = []
for dp, dn, fn in os.walk(STAGE):
    for f in fn:
        rel = os.path.relpath(os.path.join(dp, f), STAGE)
        live = REPO + rel
        if not os.path.exists(live) or open(live, "rb").read() != open(STAGE + rel, "rb").read():
            changed.append(rel)
for rel in changed: print("changed:", rel)
if "--write" in sys.argv:
    for rel in changed:
        os.makedirs(os.path.dirname(REPO + rel), exist_ok=True)
        shutil.copy2(STAGE + rel, REPO + rel)
    print("synced", len(changed))
