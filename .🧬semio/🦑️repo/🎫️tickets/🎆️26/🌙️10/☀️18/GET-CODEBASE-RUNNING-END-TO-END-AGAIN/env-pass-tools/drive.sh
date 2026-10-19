#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OUT="$C/🗑️generated/env-pass"; mkdir -p "$OUT"
budget=$1; per=$2; shift 2
deadline=$((SECONDS+budget))
for t in "$@"; do
  left=$((deadline-SECONDS)); (( left < 45 )) && { echo "SKIP(budget) $t"; continue; }
  a=$(( left < per ? left : per ))
  log="$OUT/${t//[:\/]/_}.log"
  s=$SECONDS
  perl -e 'alarm shift; exec @ARGV' -- $a bun repo run "$t" 2>&1 | grep -v '^\[script\] {"owner"' > "$log"
  rc=${pipestatus[1]}
  q=$(command grep -c 'still waits' "$log")
  p=$(command grep -Eo '[0-9]+ pass' "$log" | tail -1); f=$(command grep -Eo '[0-9]+ fail' "$log" | tail -1)
  echo "$t rc=$rc t=$((SECONDS-s))s waits=$q $p $f" | tee -a "$OUT/status.tsv"
done
