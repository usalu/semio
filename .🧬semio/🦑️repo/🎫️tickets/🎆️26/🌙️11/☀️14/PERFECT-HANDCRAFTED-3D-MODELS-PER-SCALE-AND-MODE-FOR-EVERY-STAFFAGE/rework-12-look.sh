#!/bin/sh
# 🔎 Author 12: renders one r3 sheet of a candidate document and keeps it reduced: rework-12-look.sh <slug> <name> <width px> <r3-sheet arguments…> (document: 🗑️generated/rework-12/<slug>/${DOC:-pre}.json)
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
slug=$1; name=$2; width=$3; shift 3
out=$("$BUN" r3-sheet.ts --file "🗑️generated/rework-12/$slug/${DOC:-pre}.json" "$@" 2>&1 | tail -1)
[ -f "$out" ] || { echo "FAILED $out"; exit 1; }
sips -Z "$width" "$out" --out "🗑️generated/rework-12/$slug/$name.png" >/dev/null 2>&1
rm -f "$out"
echo "🗑️generated/rework-12/$slug/$name.png"
