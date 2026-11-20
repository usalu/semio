#!/bin/sh
# 🔁 Rebuilds the volumes source, the 2D shapes and the candidate asset of humans/walking-commuter and validates the candidate.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-walking-commuter-build.ts && "$BUN" r3-walking-commuter-draw2d.ts | grep DEBUG
python3 r3-walking-commuter-root.py --with-volumes > /dev/null && "$BUN" r3-validate.ts --file "🗑️generated/r3-walking-commuter/🔣️.json" | tail -12
