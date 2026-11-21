#!/bin/sh
# ✅ Round 5 V03: validates one two-wheeler's catalog asset with its volumes source merged (schema, validateAsset, print, measured volume rules) and runs the product volume check; output to 🗑️generated/r5-<slug>/check.txt. Usage: sh r5-v03-check.sh vehicles/<slug>
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A="$1"; SLUG="${A#*/}"; O="🗑️generated/r5-$SLUG/check.txt"; mkdir -p "$(dirname "$O")"
"$BUN" r3-validate.ts "$A" > "$O" 2>&1
"$BUN" volumes-check.ts "$A" --product 2>&1 | grep -v "^\[check\]" >> "$O"
grep "VALID\|PASS\|FAIL" "$O" | cut -c1-220
