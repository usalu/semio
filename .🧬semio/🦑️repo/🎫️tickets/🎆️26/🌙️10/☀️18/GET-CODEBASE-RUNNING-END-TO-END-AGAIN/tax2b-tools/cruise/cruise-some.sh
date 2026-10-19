#!/bin/zsh
# usage: cruise-some.sh <tag> <area>...   (areas from areas.txt)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
T="$C/tax2b-tools/cruise"; R="$C/🗑️generated/tax2b"
tag="$1"; shift
for want in "$@"; do
  while IFS='|' read -r name rest; do
    [[ "$name" == "$want" ]] || continue
    paths=("${(@s:|:)rest}")
    echo "[$(date +%H:%M:%S)] start $name"
    "$T/cruise-area.sh" "$R/nct-$tag-$name.json" "${paths[@]}" 2>&1 | tail -2
  done < "$T/areas.txt"
done
echo "[$(date +%H:%M:%S)] DONE"
