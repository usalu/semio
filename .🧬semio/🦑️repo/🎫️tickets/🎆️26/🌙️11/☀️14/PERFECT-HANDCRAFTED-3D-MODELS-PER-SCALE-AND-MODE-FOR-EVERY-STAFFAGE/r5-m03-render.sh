#!/bin/sh
# 🖼️ Round 5 M03: renders every sheet of one machinery asset (body, structure, thermal cut per authored scale, every pose at 1:50, one clip frame, the 2D per scale). Usage: r5-m03-render.sh <slug> "<scales>" "<poses>" "<clip> <t>" [what]
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=machinery/$1
WHAT=${5:-all}
for s in $2; do
  case $WHAT in all|sheets)
    $BUN r3-sheet.ts $A --scale $s 2>&1 | grep -E "NOT DRAWN|rror" | head -3
    $BUN r3-sheet.ts $A --scale $s --list structure 2>&1 | grep -E "NOT DRAWN|rror" | head -3
    $BUN r3-sheet.ts $A --scale $s --list thermal --cut 2>&1 | grep -E "NOT DRAWN|rror" | head -3;;
  esac
  case $WHAT in all|2d) $BUN r3-2d.ts $A --scale $s 2>&1 | grep -E "rror" | head -3;; esac
done
case $WHAT in all|poses)
  for p in $3; do $BUN r3-sheet.ts $A --scale 50 --pose $p 2>&1 | grep -E "NOT DRAWN|rror" | head -3; done
  set -- $4
  [ -n "$1" ] && $BUN r3-sheet.ts $A --scale 50 --clip $1 --t $2 2>&1 | grep -E "NOT DRAWN|rror" | head -3;;
esac
echo done
