#!/bin/sh
# 🔁 Full rebuild of the candidate of humans/runner-with-blade-prosthesis: volumes source, dimensions from the 1:50 body, 2D geometry, candidate document; then its validation with the measured volume rules.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-runner-figure.ts && "$BUN" r3-runner-roots.ts --volumes > /dev/null && "$BUN" r3-runner-bbox.ts --write && "$BUN" r3-runner-draw2d.ts && "$BUN" r3-runner-roots.ts --volumes > /dev/null && "$BUN" r3-validate.ts --file "🗑️generated/r3-runner-with-blade-prosthesis/🔣️.json" --volumes
