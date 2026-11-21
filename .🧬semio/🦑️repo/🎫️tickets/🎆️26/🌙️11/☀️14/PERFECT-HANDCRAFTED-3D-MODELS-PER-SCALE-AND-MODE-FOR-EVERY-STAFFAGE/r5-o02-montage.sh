#!/bin/sh
# 🧩 Round 5 O02: montages of one asset's rendered sheets (per coarse scale: body, thermal cut, structure, 2D; 1:50 lists; poses) in 🗑️generated/r5-<slug>/<tag>/. Usage: sh r5-o02-montage.sh <slug> <tag> "<pose sheet suffixes>"
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
D="🗑️generated/r5-$1/$2"; P="$D/$1"
for N in 100 200 500; do F=""; for x in body thermal.cut structure; do [ -f $P.s$N.$x.png ] && F="$F $P.s$N.$x.png"; done; [ -f $P.s$N.png ] && F="$F $P.s$N.png"; $BUN r5-seated-montage.ts $D/m$N.png $F >/dev/null; done
$BUN r5-seated-montage.ts $D/m50.png $P.s50.body.png $P.s50.thermal.cut.png $P.s50.structure.png $P.s50.png >/dev/null
F=""; for x in $3; do F="$F $P.s50.body.$x.png"; done; $BUN r5-seated-montage.ts $D/mposes.png $F >/dev/null
ls $D/m*.png
