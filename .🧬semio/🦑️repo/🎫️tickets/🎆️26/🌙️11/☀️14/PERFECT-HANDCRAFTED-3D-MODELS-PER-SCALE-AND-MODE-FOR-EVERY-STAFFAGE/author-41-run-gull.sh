#!/bin/bash
S=$1; B=./author-29-bun.sh; A=animals/herring-gull
$B author-41-gull.ts $S --write && ./author-41-thermal-gull.sh $S --write 2>&1 | grep -E "MODEL" | cut -c1-170
[ "$S" != "50" ] && $B author-41-gull.ts $S --extra
case $S in 50) R=0.0;; 100) R=0.0078;; 200) R=0.0155;; esac
python3 author-41-emit.py $A $S plume-back 0.07 0.0075 0.45 $R >/dev/null
$B kit-structure.ts $A --scale $S --write >/dev/null 2>&1
[ -f author-41-skel-gull.json ] && $B author-41-skel.ts $A $S author-41-skel-gull.json --write
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-700 | grep -v "^ok"
