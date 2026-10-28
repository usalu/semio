#!/usr/bin/env python3
"""🧱️ usage: compose.py <mirror root> -> applies the prerequisite batches (journal-fix, vouch, vouch-anchor module, probes) onto a mirror, base hashes checked; the stage the witness2-g batch is built on."""
import hashlib, json, os, shutil, sys
C = "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
BATCHES = [C + "/verify2-g-tools/staged/journal-fix", C + "/verify2-g-tools/staged/vouch", C + "/rev4-g-tools/staged/vouch-anchor", C + "/toolsetup-g-tools/staged/probes"]
def sha(path):
    if not os.path.exists(path): return None
    return hashlib.sha256(open(path, "rb").read()).hexdigest()
def main(root):
    for batch in BATCHES:
        bases = json.load(open(batch + "/bases.json", encoding="utf8")) if os.path.exists(batch + "/bases.json") else {}
        newdir = batch + "/new"
        for dirpath, _, names in os.walk(newdir):
            for name in names:
                source = os.path.join(dirpath, name); rel = os.path.relpath(source, newdir); target = os.path.join(root, rel)
                row = bases.get(rel); live = sha(target)
                if row is not None and live not in (row["base"], row["new"]): print("DRIFT", os.path.basename(batch), rel[-90:], (live or "absent")[:10], "base", (row["base"] or "none")[:10])
                os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copyfile(source, target)
        print("composed", os.path.basename(batch))
main(sys.argv[1])
