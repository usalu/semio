#!/bin/sh
# [mem2] Copies the instrumented harness into a mirror and captures its snapshot: prep.sh <mirror root> <image dir> [extra patches.json ...]. Writes loader.js + loader.marked.js. Not a library file.
set -e
T=$(cd "$(dirname "$0")" && pwd); M=$(cd "$1" && pwd -P); O=$2; shift 2
BUN=$(ls -d /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
mkdir -p "$M/fiharness" "$O" && cp "$T/harness/entry.ts" "$M/fiharness/🟦️.ts" && cp "$T/harness/run.ts" "$M/fiharness/run.ts"
python3 -I "$T/harness/mkpatches.py" "$O/patches.json" "$@"
cd "$M" && "$BUN" "$T/mkimage.ts" "$M" "fiharness/🟦️.ts" "$O" "$O/patches.json"
python3 -I "$T/loadermarks.py" "$O"
