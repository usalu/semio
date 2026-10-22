#!/bin/sh
# backend-contract: re-cuts only the literal ratchet of the neutral build files against the tree the chain produced (mkratchet.py), re-renders the ratchet test and refreshes it in the tree. usage: bkc-ratchet.sh <backend-contract copy> <root>
set -e
BC="$1"; ROOT="$2"
cd "$BC"
export BKC_ROOT="$ROOT"
nice -n 10 python3 mkratchet.py "$ROOT" --write
nice -n 10 python3 build.py | head -1
nice -n 10 python3 - <<PY
import json,os,shutil
m=json.load(open("staged/manifest.json",encoding="utf8"))["backends_test.ts"]
shutil.copyfile("staged/backends_test.ts",os.path.join("$ROOT",m["destination"]))
print("ratchet test refreshed in $ROOT")
PY
