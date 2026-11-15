#!/bin/bash
# author 41: full rebuild of one cat scale (body, thermal, emitters, structure). Usage: ./author-41-run-cat.sh <50|100|200>
S=$1
B=./author-29-bun.sh
A=animals/domestic-cat
$B author-41-cat.ts $S --write && ./author-41-thermal-cat.sh $S --write 2>&1 | grep -E "MODEL|NO SOLID" | cut -c1-160
[ "$S" = "200" ] && $B author-41-cat.ts 200 --extra
case $S in
50) python3 author-41-emit.py $A 50 plume-back 0.05 0.005 0.45 0.0 >/dev/null;;
100) python3 author-41-emit.py $A 100 plume-back 0.05 0.0078 0.45 0.0078 >/dev/null;;
200) JET=0.084,0.01,0.092,0.02 python3 author-41-emit.py $A 200 plume-back 0.05 0.015 0.45 0.0155 >/dev/null;;
esac
$B kit-structure.ts $A --scale $S --write >/dev/null 2>&1
[ "$S" != "200" ] && $B author-41-skel.ts $A $S author-41-skel-cat.json --write
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-500 | grep -v "^ok"
