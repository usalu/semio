#!/bin/zsh
# usage: suite.sh OVERLAY OUTFILE FILE... ; one line per test file: pass/fail/skip tallies + sorted fail names -> OUTFILE (append). Files are repo-relative.
OV="$1"; OUT="$2"; shift 2
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/storage-seam-tools"
for f in "$@"; do
  [ -f "$OV/$f" ] || { echo "MISSING $f" >> "$OUT"; continue; }
  s=$(date +%s)
  log=$("$T/verify/tw.sh" "$OV" "$f" 2>&1)
  e=$(( $(date +%s) - s ))
  p=$(echo "$log" | sed -nE 's/^ *([0-9]+) pass$/\1/p' | tail -1); fl=$(echo "$log" | sed -nE 's/^ *([0-9]+) fail$/\1/p' | tail -1); sk=$(echo "$log" | sed -nE 's/^ *([0-9]+) skip$/\1/p' | tail -1); p=${p:-0}; fl=${fl:-0}; sk=${sk:-0}
  names=$(echo "$log" | /usr/bin/grep -E '^\(fail\)' | sed -E 's/ \[[0-9.]+m?s\]//' | sort | tr '\n' '|')
  err=""; [ "$p$fl" = "00" ] && err=" ERR:$(echo "$log" | /usr/bin/grep -iE 'error|cannot|not found' | head -1 | cut -c1-160)"
  echo "${f: -70} | ${p}p ${fl}f ${sk}s | ${e}s | $names$err" >> "$OUT"
done
