#!/bin/sh
# 🔭 Tiles one r3-sheet view of components/cantilever-balcony for several scales of one list: rework-cantilever-balcony-look.sh <list> "<scales>" <view> [extra r3-sheet args].
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
G="🗑️generated/rework-cantilever-balcony"; mkdir -p "$G"
list=$1; scales=$2; view=$3; shift 3
files=""
for s in $scales; do
  f=$($BUN r3-sheet.ts components/cantilever-balcony --scale $s --list "$list" --only $view --no-labels "$@" 2>&1 | grep '\.png' | tail -1)
  cp "$f" "$G/t-$list-$s-$view.png"; files="$files $G/t-$list-$s-$view.png"
done
$BUN rework-18-montage.ts "$G/m-$list-$view.png" $files
