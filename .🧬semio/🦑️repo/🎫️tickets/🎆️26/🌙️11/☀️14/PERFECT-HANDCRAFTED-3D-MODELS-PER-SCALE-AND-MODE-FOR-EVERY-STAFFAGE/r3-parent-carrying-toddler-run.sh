#!/bin/sh
# 🔁 Rebuilds the volumes source, the 2D shapes and the candidate asset of humans/parent-carrying-toddler and validates the candidate.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-parent-carrying-toddler-build.ts && "$BUN" r3-parent-carrying-toddler-draw2d.ts | grep -E "DEBUG|error"
python3 r3-parent-carrying-toddler-roots.py --with-volumes > /dev/null && "$BUN" r3-validate.ts --file "🗑️generated/r3-parent-carrying-toddler/🔣️.json" | tail -25
