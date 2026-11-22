#!/bin/sh
# 🔭 Renders one view of one asset at the four scales for one list and tiles them: rework-18-look.sh <asset> <list> [extra r3-sheet args]; env VIEW (default three-quarter), SCALES.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
asset=$1; list=$2; shift 2
view=${VIEW:-three-quarter}
files=""
for s in ${SCALES:-50 100 200 500}; do
  f=$($BUN r3-sheet.ts "$asset" --scale $s --list "$list" --only $view --no-labels "$@" 2>&1 | grep '\.png' | tail -1)
  cp "$f" "🗑️generated/rework-18/tile-$s.png"; files="$files 🗑️generated/rework-18/tile-$s.png"
done
$BUN rework-18-montage.ts "🗑️generated/rework-18/$(basename "$asset").$list.$view.png" $files
