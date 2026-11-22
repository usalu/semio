#!/bin/sh
# 👴 Author 04: rebuilds the candidate of humans/elderly-with-cane (rig, volumes, 2D redraw) and validates it; extra arguments go to r3-validate.ts (e.g. --volumes).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" rework-04-elderly-with-cane-rig.ts > /dev/null
"$BUN" rework-04-elderly-with-cane-build.ts | tail -4
"$BUN" rework-04-elderly-with-cane-draw2d.ts | grep -v "shapes," || true
"$BUN" r3-validate.ts --file "🗑️generated/rework-04/elderly-with-cane/🔣️.json" "$@" 2>&1 | cut -c1-420 | tail -60 || true
