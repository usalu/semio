#!/bin/sh
# 🔎 Lombardy poplar author: renders one r3 sheet of a candidate document (${DOC:-pre}.json in 🗑️generated/rework-lombardy-poplar/lombardy-poplar) and keeps a reduced copy: rework-lombardy-poplar-look.sh <name> <width px> <r3-sheet arguments…>
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-lombardy-poplar/lombardy-poplar"
name=$1; width=$2; shift 2
out=$("$BUN" r3-sheet.ts vegetation/lombardy-poplar --file "$G/${DOC:-pre}.json" "$@" 2>&1 | tail -1)
[ -f "$out" ] || { echo "FAILED $out"; exit 1; }
sips -Z "$width" "$out" --out "$G/$name.png" >/dev/null 2>&1
rm -f "$out"
echo "$G/$name.png"
