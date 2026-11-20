#!/bin/sh
# 🩼 Rebuilds the candidate asset document of humans/person-on-crutches: volumes, then the 2D redraw from them, then the candidate with both; validates it.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-person-on-crutches-build.ts
"$BUN" r3-person-on-crutches-draw2d.ts
"$BUN" r3-person-on-crutches-build.ts > /dev/null
"$BUN" r3-validate.ts --file "🗑️generated/r3-person-on-crutches/🔣️.json" "$@" || true
