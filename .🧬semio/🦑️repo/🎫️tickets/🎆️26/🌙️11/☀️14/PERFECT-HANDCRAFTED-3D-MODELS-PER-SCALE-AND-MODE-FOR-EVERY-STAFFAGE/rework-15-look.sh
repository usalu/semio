#!/bin/sh
# 🔎 Renders one r3 sheet and copies it reduced to `🗑️generated/rework-15/<name>.png`: rework-15-look.sh <name> <width px> <r3-sheet arguments…>
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
name=$1; width=$2; shift 2
out=$("$BUN" r3-sheet.ts "$@" 2>&1 | tail -1)
[ -f "$out" ] || { echo "FAILED $out"; exit 1; }
mkdir -p "🗑️generated/rework-15"
sips -Z "$width" "$out" --out "🗑️generated/rework-15/$name.png" >/dev/null 2>&1
rm -f "$out"
echo "🗑️generated/rework-15/$name.png"
