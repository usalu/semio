#!/bin/sh
# 🩺️ Full check of one generated asset file: validateAsset, the silhouette gate and checkVolumes, the staffage check CLI. Logs go to 🗑️generated/HW-03.
# check.sh <file.json> <asset-id>
B=/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun
H="$(cd "$(dirname "$0")" && pwd)"
S="$H/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
F="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
G="$H/../../🗑️generated/HW-03"
cd "$H" && $B validate.ts "$F" > "$G/[DEBUG]-v-$2.txt" 2>&1 &
cd "$H" && $B measure.ts "$F" > "$G/[DEBUG]-m-$2.txt" 2>&1 &
cd "$S" && $B 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$F" --samples 9 > "$G/[DEBUG]-c-$2.txt" 2>&1 &
wait
tail -3 "$G/[DEBUG]-v-$2.txt"; grep -v "^ *at " "$G/[DEBUG]-m-$2.txt" | tail -12; tail -12 "$G/[DEBUG]-c-$2.txt"
