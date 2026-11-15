#!/bin/bash
S=$1; B=./author-29-bun.sh; A=animals/hedgehog
$B author-41-hedgehog.ts $S --write && ./author-41-thermal-hedgehog.sh $S --write 2>&1 | grep -E "MODEL" | cut -c1-170
[ "$S" = "100" ] && $B author-41-hedgehog.ts 100 --extra
if [ "$S" = "50" ]; then PLY=0.058 python3 author-41-emit.py $A 50 plume-back 0.05 0.0075 0.45 0.0 >/dev/null; else PLY=0.058 python3 author-41-emit.py $A 100 plume-back 0.05 0.0078 0.45 0.0078 >/dev/null; fi
$B kit-structure.ts $A --scale $S --write >/dev/null 2>&1
[ -f author-41-skel-hedgehog.json ] && $B author-41-skel.ts $A $S author-41-skel-hedgehog.json --write
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-700 | grep -v "^ok"
