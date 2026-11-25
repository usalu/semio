#!/bin/zsh
# ⌛️ Waits (at most 170 s) for a new line in $GEN/all.out, then prints its last lines and the last line of the queue log: w.sh [lines]
source "${0:A:h}/env.sh"
n0=$(wc -l < "$GEN/all.out" 2>/dev/null); t=0
while [[ "$(wc -l < "$GEN/all.out" 2>/dev/null)" == "$n0" && $t -lt 170 ]]; do sleep 5; t=$((t+5)); done
date +%H:%M; tail -${1:-4} "$GEN/all.out" | cut -c1-220; tail -1 "$GEN/capacity.log" 2>/dev/null | cut -c1-160
