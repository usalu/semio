#!/bin/sh
# 🖼️ Renders one r3-sheet per argument string for components/external-venetian-blind, one process at a time.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for args in "$@"; do
  $BUN r3-sheet.ts components/external-venetian-blind $args 2>&1 | tail -1 | sed 's/.*r3-sheet\//r3-sheet\//'
done
