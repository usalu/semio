#!/bin/sh
# 🏗️ Builds the round-3 candidate of humans/worker-carrying-box: root fixes (draft), volumes, 2D redraw from the volumes, root fixes again with the new shapes, volumes again, then validates the candidate document.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
python3 r3-worker-carrying-box-roots.py > /dev/null
python3 r3-worker-carrying-box-model.py
$BUN r3-worker-carrying-box-draw2d.ts
python3 r3-worker-carrying-box-roots.py > /dev/null
python3 r3-worker-carrying-box-model.py > /dev/null
$BUN r3-validate.ts --file "🗑️generated/r3-worker-carrying-box/🔣️.json" | tail -${TAIL:-25}
