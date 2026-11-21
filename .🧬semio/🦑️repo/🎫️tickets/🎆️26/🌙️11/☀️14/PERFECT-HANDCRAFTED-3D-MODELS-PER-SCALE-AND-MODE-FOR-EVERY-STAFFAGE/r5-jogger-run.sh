#!/bin/sh
# 🔁 Rebuilds the round 5 candidate of humans/jogger (volumes, 2D, candidate document) and validates it; nothing in the catalog changes.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r5-jogger-figure.ts >/dev/null || exit 1
"$BUN" r5-jogger-draw2d.ts >/dev/null || exit 1
python3 r5-runners-candidate.py jogger >/dev/null || exit 1
"$BUN" r3-validate.ts --file "🗑️generated/r5-jogger/🔣️.json" $1 | cut -c1-300 | tail -${2:-15}
