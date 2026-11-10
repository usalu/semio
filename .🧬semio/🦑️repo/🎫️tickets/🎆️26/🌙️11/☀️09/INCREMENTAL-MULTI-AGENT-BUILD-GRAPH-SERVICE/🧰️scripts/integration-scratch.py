#!/usr/bin/env python3
"""Prints the scratch kind (`checkout` or `sandbox`) of a build-library test file: the one its own test envelope declares, else the one the suite table declares (default `sandbox`)."""
import json,re,sys
B="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
table=json.load(open(B+"🗂️suites/🔣️.json",encoding="utf8"))
file=sys.argv[1].removeprefix(B);best=("sandbox",-1)
declared=re.search(r'declareLibraryBuildTestEnvelopeV1\(\{[^}]*scratch:"(checkout|sandbox)"',open(sys.argv[1],encoding="utf8").read())
if declared:print(declared.group(1));sys.exit(0)
for suite in table["suites"]:
    kind=suite.get("scratch","sandbox")
    for tree in suite.get("trees",[]):
        if (file.startswith(tree+"/") or tree=="") and len(tree)>best[1]:best=(kind,len(tree))
    for name in suite.get("files",[]):
        if file==name:best=(kind,10**6)
print(best[0])
