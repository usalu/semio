#!/usr/bin/env python3
"""🔁 Switches a mirror between the live base and the lane's batch for a file list: swap.py <mirror> <list.txt> base|batch
base: saves the mirror (batch) bytes under C/🗑️generated/cargo2/swap/<list>/ and copies the live bytes (removing files new in the batch); batch: restores the saved bytes."""
import os, shutil, sys
mirror, listing, mode = sys.argv[1], sys.argv[2], sys.argv[3]
here = os.path.dirname(os.path.abspath(__file__)); live = here
while not os.path.isdir(os.path.join(live, "🧰️framework")): live = os.path.dirname(live)
store = os.path.join(os.path.dirname(here), "🗑️generated", "cargo2", "swap", os.path.basename(listing))
rels = [line.strip() for line in open(listing, encoding="utf8") if line.strip()]
for index, rel in enumerate(rels):
    saved, target, base = os.path.join(store, "%02d" % index), os.path.join(mirror, rel), os.path.join(live, rel)
    if mode == "base":
        os.makedirs(store, exist_ok=True); shutil.copyfile(target, saved)
        if os.path.exists(base): shutil.copyfile(base, target)
        else: os.remove(target)
    else:
        os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copyfile(saved, target)
print(mode, len(rels))
