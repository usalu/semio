#!/bin/sh
# ✂️ Crops a region of a rendered sheet and enlarges it for a close look: r5-runners-crop.sh <png> <x> <y> <w> <h> <out-name>; writes 🗑️generated/r5-runners/crop/<out-name>.png
cd "$(dirname "$0")" || exit 1
mkdir -p "🗑️generated/r5-runners/crop"
o="🗑️generated/r5-runners/crop/$6.png"
sips -c "$5" "$4" --cropOffset "$3" "$2" "$1" --out "$o" >/dev/null && sips -Z 1400 "$o" >/dev/null && echo "$o"
