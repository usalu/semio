#!/bin/sh
# 🧪 Rebuilds the candidate document of humans/wheelchair-user (roots, volumes of every scale, 2D redrawn from them), writes the volumes source and validates the candidate; extra arguments go to r3-validate.ts (e.g. --volumes).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-wheelchair-user-draw2d.ts | grep -v "^\(50\|100\|200\|500\) " || true
"$BUN" r3-wheelchair-user-build.ts --source
"$BUN" r3-validate.ts --file "🗑️generated/r3-wheelchair-user/🔣️.json" "$@" | tail -40
