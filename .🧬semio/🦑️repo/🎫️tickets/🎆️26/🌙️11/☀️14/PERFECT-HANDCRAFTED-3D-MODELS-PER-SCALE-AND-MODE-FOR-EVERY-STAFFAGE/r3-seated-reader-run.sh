#!/bin/sh
# 🧪 Rebuilds the candidate document of humans/seated-reader (roots, volumes of every scale, 2D redrawn from them), writes the volumes source and validates the candidate; `--volumes` adds the measured volume rules. A failing redraw stops the run.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-seated-reader-draw2d.ts > "🗑️generated/r3-seated-reader/draw2d.txt" 2>&1 || { tail -5 "🗑️generated/r3-seated-reader/draw2d.txt"; exit 1; }
grep "DEBUG" "🗑️generated/r3-seated-reader/draw2d.txt" || true
"$BUN" r3-seated-reader-build.ts --source
"$BUN" r3-validate.ts --file "🗑️generated/r3-seated-reader/🔣️.json" "$@" | tail -40
