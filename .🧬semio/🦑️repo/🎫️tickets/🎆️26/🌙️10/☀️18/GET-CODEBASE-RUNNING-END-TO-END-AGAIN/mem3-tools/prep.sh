#!/bin/sh
# [mem3] like mem2-tools/prep.sh with the mem3 harness (heap snapshot hook M3_SNAP). usage: prep.sh <mirror root> <image dir> (REROOT=<live root> in the environment)
set -e
T=$(cd "$(dirname "$0")" && pwd); T2="$T/../mem2-tools"; M=$(cd "$1" && pwd -P); O=$2; shift 2
BUN=$(ls -d /Users/ueli/Documents/semio/.cache/b/t/m/bun/*/bun | head -1)
mkdir -p "$M/fiharness" "$O" && cp "$T/harness/entry.ts" "$M/fiharness/🟦️.ts" && cp "$T/harness/run.ts" "$M/fiharness/run.ts"
python3 -I "$T2/harness/mkpatches.py" "$O/patches.json" "$@"
cd "$M" && "$BUN" "$T2/mkimage.ts" "$M" "fiharness/🟦️.ts" "$O" "$O/patches.json"
python3 -I "$T/loadermarks.py" "$O"
