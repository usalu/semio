#!/bin/zsh
# 📐 Prints the measured outer area of every thermal region at one scale (areaM2 probed at 100 m2 so each region reports itself).
K="${0:A:h}"
BASS_AREA_PROBE=1 "$K/author-validate.sh" --volumes --scale "$1" --rules thermal-coverage --no-print 2>&1 | grep -o 'region "[a-z-]*" at 1:[0-9]* is [0-9.]* m²\|sum[^.]*'
python3 "$K/author-double-bass.py" > /dev/null
