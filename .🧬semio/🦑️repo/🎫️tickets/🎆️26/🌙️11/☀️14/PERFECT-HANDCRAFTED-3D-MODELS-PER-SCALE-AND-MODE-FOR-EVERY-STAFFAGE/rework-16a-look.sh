#!/bin/sh
# 👀 Renders every sheet of the loop for one appliance: look.sh <slug> "<pose ...>" "<clip:t ...>"
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=appliances/$1
for s in 50 100 200 500; do for l in body thermal structure; do $BUN r3-sheet.ts $A --scale $s --list $l 2>&1 | grep -i 'not drawn\|error'; done; done
$BUN r3-sheet.ts $A --scale 50 --list thermal --cut >/dev/null 2>&1
for p in $2; do $BUN r3-sheet.ts $A --scale 50 --pose $p 2>&1 | grep -i 'not drawn\|error'; done
for c in $3; do $BUN r3-sheet.ts $A --scale 50 --clip ${c%%:*} --t ${c##*:} 2>&1 | grep -i 'not drawn\|error'; done
for s in 50 100 200 500; do $BUN r3-2d.ts $A --scale $s 2>&1 | grep -i 'error'; done
ls "🗑️generated/r3-sheet/appliances" | grep "^$1\." | wc -l
