#!/bin/sh
# 🌿 Round 5: rebuilds the candidate asset document of humans/kneeling-gardener (round 3 root fixes, round 5 volumes of every scale, 2D redrawn from the 1:50 and 1:100 bodies) and renders the sheets named as arguments: "<scale> <list> [extra flags]" per quoted argument.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="🗑️generated/r3-kneeling-gardener/🔣️.json"
python3 r3-kneeling-gardener-roots.py --candidate > /dev/null
"$BUN" r5-kneeling-gardener-figure.ts
"$BUN" r5-kneeling-gardener-draw2d.ts > /dev/null
python3 r3-kneeling-gardener-draw2d.py --candidate
for spec in "$@"; do
  set -- $spec
  scale=$1; list=$2; shift 2
  "$BUN" r3-kneeling-gardener-sheet.ts humans/kneeling-gardener --file "$C" --scale "$scale" --list "$list" "$@" 2>&1 | tail -2
done
