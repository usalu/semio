#!/bin/zsh
G="$1"; n="$2"; sleep 180
head -1 "$G/../../ROUND.md" > "$G/round-file-at-try-$n.txt"
cd /Users/ueli/Documents/semio && python3 "$G/run-observed.py" "$3" "$n"
