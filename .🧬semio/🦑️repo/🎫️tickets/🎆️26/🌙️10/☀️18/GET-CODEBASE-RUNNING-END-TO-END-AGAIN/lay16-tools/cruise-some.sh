#!/bin/zsh
# usage: cruise-some.sh <tag> <area>...   runs areas two at a time
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/lay16-tools"; R="$C/🗑️generated/lay16-run"
tag="$1"; shift
run() { local want="$1"; while IFS='|' read -r name rest; do [[ "$name" == "$want" ]] || continue; paths=("${(@s:|:)rest}"); echo "[$(date +%H:%M:%S)] start $name"; "$T/cruise-area.sh" "$R/$tag-$name.json" "${paths[@]}" 2>&1 | tail -2; done < "$T/areas.txt"; }
n=0
for a in "$@"; do run "$a" & n=$((n+1)); (( n % ${PAR:-2} == 0 )) && wait; done
wait
echo "[$(date +%H:%M:%S)] DONE $tag"
