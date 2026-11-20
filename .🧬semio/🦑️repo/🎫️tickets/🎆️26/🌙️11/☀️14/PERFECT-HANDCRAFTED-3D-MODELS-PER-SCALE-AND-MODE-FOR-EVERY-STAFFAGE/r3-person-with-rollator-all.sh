#!/bin/sh
# 🧵 Rebuilds everything of humans/person-with-rollator in order: volumes source, 2D redraw from it, candidate document; then validates the candidate (pass --volumes for the measured volume rules).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-person-with-rollator-build.ts > /dev/null
"$BUN" r3-person-with-rollator-draw2d.ts | grep DEBUG || true
python3 r3-person-with-rollator-candidate.py > /dev/null
"$BUN" r3-validate.ts --file "🗑️generated/r3-person-with-rollator/🔣️.json" "$@" || true
