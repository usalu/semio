#!/bin/bash
# 🔁 Round 5 vehicles: renders the named sheets of one asset after its changes into 🗑️generated/r5-<slug>/after/. Usage: r5-vehicles-after.sh <category/slug> "<scale> <list> [--cut]" ... ; a spec "2d <scale>" renders the 2D.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; shift; SLUG=${A#*/}; CAT=${A%/*}; O="🗑️generated/r5-$SLUG/after"; mkdir -p "$O"
for spec in "$@"; do
  set -- $spec
  if [ "$1" = 2d ]; then $BUN r3-2d.ts $A --scale $2 >/dev/null 2>&1 && cp "🗑️generated/r3-2d/$CAT/$SLUG.s$2.png" "$O/2d.s$2.png" || echo "2d $2 failed"; continue; fi
  S=$1; L=$2; X=$3
  $BUN r3-sheet.ts $A --scale $S --list $L $X --no-labels 2>&1 | grep -i "not drawn"
  cp "🗑️generated/r3-sheet/$CAT/$SLUG.s$S.$L${X:+.cut}.png" "$O/s$S.$L${X:+.cut}.png"
done
ls "$O"
