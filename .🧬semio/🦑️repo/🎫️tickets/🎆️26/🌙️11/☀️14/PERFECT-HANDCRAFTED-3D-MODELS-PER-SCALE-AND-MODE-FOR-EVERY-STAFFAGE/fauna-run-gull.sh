#!/bin/bash
S=$1; B=./author-29-bun.sh; A=animals/herring-gull
$B fauna-gull.ts $S --write || exit 1
M=$(python3 fauna-map.py $A $S fauna-rules-gull.json)
$B kit-thermal.ts $A --scale $S --map "$M" --write 2>&1 | grep -E "MODEL|uncovered|error" | cut -c1-170
[ "$S" != "50" ] && $B author-41-gull.ts $S --extra
case $S in 50) R=0.0;; 100) R=0.0078;; 200) R=0.0155;; esac
python3 author-41-emit.py $A $S plume-back 0.07 0.0075 0.45 $R >/dev/null
$B kit-structure.ts $A --scale $S --write >/dev/null 2>&1
$B author-41-skel.ts $A $S author-41-skel-gull.json --write
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-900 | grep -v "^ok"
