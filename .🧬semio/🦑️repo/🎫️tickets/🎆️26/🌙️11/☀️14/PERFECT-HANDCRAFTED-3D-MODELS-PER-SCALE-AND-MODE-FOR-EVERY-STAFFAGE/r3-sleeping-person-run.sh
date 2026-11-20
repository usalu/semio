#!/bin/sh
# 🧪 Rebuilds the candidate document of humans/sleeping-person (roots, volumes of every scale, 2D redrawn from them), writes the volumes source and validates the candidate; extra arguments go to r3-validate.ts (e.g. --volumes).
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" r3-sleeping-person-draw2d.ts | grep -v "^\(50\|100\|200\|500\) " || true
"$BUN" r3-sleeping-person-build.ts --source
"$BUN" r3-validate.ts --file "🗑️generated/r3-sleeping-person/🔣️.json" "$@" | cut -c1-260 | tail -40
