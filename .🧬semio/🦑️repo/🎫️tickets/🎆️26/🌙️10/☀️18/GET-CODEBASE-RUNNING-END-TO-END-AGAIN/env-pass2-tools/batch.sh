#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
LIST="$1"; BUDGET="${2:-480}"
START=$(date +%s)
touch "$C/env-pass2-tools/summary.tsv"
run(){ N="$1"; F="$2"; [[ -n "$(command grep -a -F -- "$N	" "$C/env-pass2-tools/summary.tsv")" ]] && return; (( $(date +%s)-START > BUDGET )) && return; "$C/env-pass2-tools/one.sh" "$F" "$N"; }
i=0; PIDS=()
while IFS= read -r F; do
  i=$((i+1)); N=$(printf "%s%03d" "$3" $i)
  while (( ${#PIDS[@]} >= 2 )); do
    NP=(); for p in $PIDS; do kill -0 $p 2>/dev/null && NP+=($p); done; PIDS=($NP); (( ${#PIDS[@]} >= 2 )) && sleep 1
  done
  run "$N" "$F" & PIDS+=($!)
done < "$LIST"
wait
