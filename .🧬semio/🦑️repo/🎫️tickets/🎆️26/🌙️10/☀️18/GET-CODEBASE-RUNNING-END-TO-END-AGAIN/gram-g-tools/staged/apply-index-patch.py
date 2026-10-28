#!/usr/bin/env python3
"""[gram-g] STAGED (apply after 11:30, one compile-atomic batch): fact index patch re-partitions only when the partition outgrew its bound AND grew by `compactionGrowth` since its base. Files: 🕰️generations/{🟨️.mjs, 🟨️.d.mts, 🧬️schema/🔣️.json} + new law 🌱️patch-growth. Refuses when an anchor drifted. usage: apply-index-patch.py [--write]"""
import os, shutil, sys
ROOT = "/Users/ueli/Documents/semio"
G = ROOT + "/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations"
HERE = os.path.dirname(os.path.abspath(__file__))
edits = {
    G + "/🟨️.mjs": [
        ("return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor?refineFactIndexV1(result):result;",
         "return result.classes.length>P.compactionFactor*result.owners.length+P.compactionFloor&&result.classes.length>P.compactionGrowth*index.classes.length?refineFactIndexV1(result):result;"),
        (" compactionFactor:2,compactionFloor:1024,", " compactionFactor:2,compactionFloor:1024,compactionGrowth:1.25,"),
    ],
    G + "/🟨️.d.mts": [("readonly compactionFloor:number;", "readonly compactionFloor:number;readonly compactionGrowth:number;")],
    G + "/🧬️schema/🔣️.json": [('    "compactionFloor": 1024,\n', '    "compactionFloor": 1024,\n    "compactionGrowth": 1.25,\n')],
}
write = "--write" in sys.argv
for path, rows in edits.items():
    text = open(path, encoding="utf8").read()
    for old, new in rows:
        if new in text and old not in text:
            print("already applied:", os.path.basename(path), old[:50]); continue
        if text.count(old) != 1:
            sys.exit("anchor drift in " + path + ": " + old[:60])
        text = text.replace(old, new)
    if write:
        open(path, "w", encoding="utf8").write(text)
target = G + "/🧪️tests/🌱️patch-growth"
if write:
    os.makedirs(target, exist_ok=True)
    shutil.copy(HERE + "/files/patch-growth.test.ts", target + "/🟦️.ts")
print("written" if write else "check ok (nothing written)")
