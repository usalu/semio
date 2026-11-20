#!/bin/sh
# 🔁 Rebuilds everything of humans/person-cooking in order: candidate roots, volumes, 2D redraw, roots with shapes and volumes, validation of the candidate.
set -e
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN r3-person-cooking-roots.ts "$@" | tail -1
$BUN r3-person-cooking-build.ts > /dev/null
$BUN r3-person-cooking-draw2d.ts | grep DEBUG || true
$BUN r3-person-cooking-roots.ts --with-volumes | tail -1
$BUN r3-validate.ts --file "🗑️generated/r3-person-cooking/🔣️.json" | tail -12 | cut -c1-400
