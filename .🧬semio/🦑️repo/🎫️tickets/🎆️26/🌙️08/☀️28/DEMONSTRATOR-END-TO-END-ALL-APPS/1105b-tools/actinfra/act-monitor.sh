#!/bin/bash
# usage: act-monitor.sh <label> [interval-seconds]  — prints one line per change of an activation run (log/status written by 1105b-tools/act.sh <label>)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"; L="$G/$1.log"; S="$G/$1.status"; I="${2:-60}"
last=""; restarts=0; errors=0; prev=0; prevd=0
while :; do
 [ -f "$L" ] || { sleep "$I"; continue; }
 v=$(grep -a -o "Verifying input closures: [0-9]* of [0-9]*" "$L" | tail -1 | sed 's/Verifying input closures: //')
 b=$(grep -a -E "^\[[0-9,]+ / [0-9,]+\]" "$L" | tail -1 | cut -c1-90)
 r=$(grep -a -c "Restarting with the current implementation" "$L"); e=$(grep -a -c "^ERROR:" "$L")
 w=$(grep -a "waiting .*s for run\|Waiting for access" "$L" | tail -1 | cut -c1-110)
 line="verify=[${v:--}] bazel=[${b:--}] restarts=$r errors=$e"
 [ "$r" -gt "$restarts" ] && { echo "ALARM restart #$r: $(grep -a -B2 'Restarting with the current implementation' "$L" | tail -3 | tr '\n' ' ' | cut -c1-300)"; restarts=$r; }
 [ "$e" -gt "$errors" ] && { echo "ALARM new ERROR: $(grep -a '^ERROR:' "$L" | tail -1 | cut -c1-300)"; errors=$e; }
n=${v%% of *}; d=${v##* of }; if [ -n "$v" ] && [ -z "$b" ]; then if [ "$d" = "$prevd" ] && [ "$n" -lt "$prev" ]; then echo "ALARM verify counter dropped $prev -> $n of $d while bazel not started (restart loop)"; fi; prev=$n; prevd=$d; fi
 if [ "$line" != "$last" ]; then echo "$(date +%T) $line ${w:+| $w}"; last="$line"; fi
 if grep -q '^exit ' "$S" 2>/dev/null; then echo "$(date +%T) DONE $(tail -1 "$S")"; exit 0; fi
 sleep "$I"
done
