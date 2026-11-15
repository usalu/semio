#!/bin/bash
# author 41: full rebuild of one goat scale. Usage: ./author-41-run-goat.sh <50|100|200|500>
S=$1; B=./author-29-bun.sh; A=animals/goat
$B author-41-goat.ts $S --write && ./author-41-thermal-goat.sh $S --write 2>&1 | grep -E "MODEL|NO SOLID" | cut -c1-170
[ "$S" = "500" ] && $B author-41-goat.ts 500 --extra
case $S in
50|100) python3 author-41-emit.py $A $S plume-back 0.12 0.0075 0.2 0.0078 >/dev/null;;
200) python3 author-41-emit.py $A 200 plume-back 0.12 0.016 0.2 0.0165 >/dev/null;;
500) python3 author-41-emit.py $A 500 plume-back 0.12 0.04 0.2 0.0376 >/dev/null;;
esac
$B kit-structure.ts $A --scale $S --write >/dev/null 2>&1
$B author-41-skel.ts $A $S author-41-skel-goat.json --write
$B volumes-check.ts $A --product --scale $S --quiet 2>&1 | grep -v "^\[check" | cut -c1-700 | grep -v "^ok"
