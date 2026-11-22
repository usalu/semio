#!/bin/sh
# 👀 Renders body, thermal and structure sheets and the 2D drawing of one asset at the given scales, one tool process at a time.
# Usage: sh rework-walls-look.sh <category/slug> <scale> [<scale> …]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
ASSET=$1
shift
for SCALE in "$@"; do
  for LIST in body thermal structure; do
    "$BUN" r3-sheet.ts "$ASSET" --scale "$SCALE" --list "$LIST" 2>&1 | tail -1
  done
  "$BUN" r3-2d.ts "$ASSET" --scale "$SCALE" 2>&1 | tail -1
done
