#!/bin/zsh
# usage: tt6-ptboth.sh <phase> ; runs <phase> list now, then its HEAD copies, removes the copies
T=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split
G=$T/🗑️generated
python3 $T/tt6-tools/tt6_pertest.py ${1}N $G/tt6-pt-list${1}.txt 3
python3 $T/tt6-tools/tt6_pertest.py ${1}H $G/tt6-pt-list${1}H.txt 3
cd /Users/ueli/Documents/semio; while IFS= read -r f; do rm -f "$f"; done < $G/tt6-pt-list${1}H.txt
echo done > $G/tt6-pt-${1}.both
