#!/bin/sh
# 🔁 Round 3, furniture/office-swivel-chair: builds the candidate, projects the outlines of its bodies, redraws the 2D from them and validates the candidate document.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
C="🗑️generated/r3-office-swivel-chair"
rm -f "$C/outlines.json"
python3 r3-office-swivel-chair-build.py > /dev/null
$BUN r3-office-swivel-chair-outline.ts outline "$C/🔣️.json" "$C/outline-requests.json" "$C/outlines.json"
python3 r3-office-swivel-chair-build.py
$BUN r3-validate.ts --file "$C/🔣️.json" 2>&1 | cut -c1-420 | tail -25
