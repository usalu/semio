#!/bin/bash
S=$1; B=./author-29-bun.sh; A=animals/pigeon
$B fauna-pigeon.ts $S --write || exit 1
M=$(python3 fauna-map.py $A $S fauna-rules-pigeon.json)
$B kit-thermal.ts $A --scale $S --map "$M" --write 2>&1 | grep -E "MODEL|error" | cut -c1-170
$B fauna-pigeon.ts $S --extra
python3 author-29-emit.py $A $S trunk 16 0.004 >/dev/null
$B kit-structure.ts $A --scale $S --write 2>&1 | grep -E "^kit-structure|outside" | cut -c1-140
python3 fauna-joints.py $A $S "shank-n=-0.026,tarsus-n=-0.026,toes-n=-0.026,shank-f=0.026,tarsus-f=0.026,toes-f=0.026,thigh-n=-0.026,thigh-f=0.026" >/dev/null
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-700 | grep -v "^ok"
