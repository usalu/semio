#!/bin/sh
# 🧩 Half-size 2x2 montages of one object asset's grading sheets (thermal/structure per coarse scale, poses, 2D) for looking at all of them.
set -u
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
A=$1; S=${A#*/}
G="🗑️generated/r3-sheet/objects"; D="🗑️generated/r3-2d/objects"; O="🗑️generated/r5-$S"; mkdir -p "$O"
$BUN r3-street-lamp-montage.ts "$O/m-ts100-200.png" 2 $G/$S.s100.thermal.cut.png $G/$S.s100.structure.png $G/$S.s200.thermal.cut.png $G/$S.s200.structure.png
$BUN r3-street-lamp-montage.ts "$O/m-ts500-body.png" 2 $G/$S.s500.thermal.cut.png $G/$S.s500.structure.png $G/$S.s200.body.png $G/$S.s100.body.png
$BUN r3-street-lamp-montage.ts "$O/m-2d.png" 2 $D/$S.s50.png $D/$S.s100.png $D/$S.s200.png $D/$S.s500.png
ls $G/$S.s50.body.*.png | grep -v -e three -e back -e front -e right | head -8 > "$O/poses.txt"
$BUN r3-street-lamp-montage.ts "$O/m-poses.png" 3 $(cat "$O/poses.txt")
