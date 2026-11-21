#!/bin/sh
# 👴 Round 5 rebuild of humans/elderly-with-cane: volumes source, 2D redraw from the body, candidate document, validation of the candidate.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-elderly-with-cane-build.ts
"$BUN" r3-elderly-with-cane-draw2d.ts > /dev/null
python3 r3-elderly-with-cane-edit.py
"$BUN" r3-validate.ts --file "🗑️generated/r3-elderly-with-cane/🔣️.json" "$@" || true
