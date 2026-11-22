#!/bin/bash
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for args in "$@"; do "$BUN" r3-sheet.ts objects/shopping-trolley $args 2>&1 | grep -E "png|NOT DRAWN|rror" | sed 's/.*r3-sheet\//r3-sheet\//'; done
