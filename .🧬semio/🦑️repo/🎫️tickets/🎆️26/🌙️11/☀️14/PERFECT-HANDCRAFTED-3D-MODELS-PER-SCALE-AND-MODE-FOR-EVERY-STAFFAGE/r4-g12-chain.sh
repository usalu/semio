#!/bin/sh
# 🔗 Runs one after another for a human asset of group G12: the given sheets (scale:list[:extra flags joined by +]), then the product volume check and the validation; output under 🗑️generated/r4-g12/. Usage: ./r4-g12-chain.sh <slug> [check] <scale:list[:flags]>...
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="humans/$1"; shift
O="🗑️generated/r4-g12"; mkdir -p "$O"
CHECK=0
for spec in "$@"; do
  if [ "$spec" = "check" ]; then CHECK=1; continue; fi
  scale=$(echo "$spec" | cut -d: -f1); list=$(echo "$spec" | cut -d: -f2); flags=$(echo "$spec" | cut -s -d: -f3 | tr '+' ' ')
  if [ "$list" = "2d" ]; then out=$($BUN r3-2d.ts "$A" --scale "$scale" $flags 2>&1 | grep -o '🗑️generated/r3-2d/.*png' | tail -1)
  else out=$($BUN r3-sheet.ts "$A" --scale "$scale" --list "$list" --no-rig $flags 2>&1 | grep -o '🗑️generated/r3-sheet/.*png' | tail -1); fi
  [ -n "$out" ] && sips -Z 1800 "$out" --out "$O/$(basename "$out")" > /dev/null && echo "SHEET $O/$(basename "$out")"
done
if [ $CHECK = 1 ]; then
  $BUN volumes-check.ts "$A" --product > "$O/$(basename "$A").check.txt" 2>&1; grep -E "FAIL|PASS|fail" "$O/$(basename "$A").check.txt" | cut -c1-240 | tail -20
  $BUN r3-validate.ts "$A" 2>&1 | cut -c1-240 | tail -8
fi
