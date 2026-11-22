#!/bin/sh
# 🏃 Builds, redraws, annotates and validates the candidate of humans/walking-commuter; publish with rework-walking-commuter-publish.py humans/walking-commuter.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
"$BUN" rework-walking-commuter-build.ts
"$BUN" rework-walking-commuter-draw2d.ts
python3 rework-walking-commuter-texts.py
"$BUN" r3-validate.ts --file "🗑️generated/rework-walking-commuter/🔣️.json" --volumes
