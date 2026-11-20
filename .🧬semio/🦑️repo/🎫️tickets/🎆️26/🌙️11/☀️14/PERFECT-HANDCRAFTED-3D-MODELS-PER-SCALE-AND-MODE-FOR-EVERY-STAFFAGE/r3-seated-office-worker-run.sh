#!/bin/sh
# 🧪 Rebuilds the candidate document of humans/seated-office-worker (roots, volumes of every scale, 2D redrawn from them), writes the volumes source and validates the candidate; `--volumes` adds the measured volume rules.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-seated-office-worker-draw2d.ts | grep -v "^\(50\|100\|200\|500\) " || true
"$BUN" r3-seated-office-worker-build.ts --source
"$BUN" r3-validate.ts --file "🗑️generated/r3-seated-office-worker/🔣️.json" "$@" | tail -40
