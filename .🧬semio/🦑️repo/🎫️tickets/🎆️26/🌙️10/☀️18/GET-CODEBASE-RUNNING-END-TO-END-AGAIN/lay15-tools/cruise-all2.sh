#!/bin/zsh
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"
while IFS='|' read -r name rest; do
  paths=("${(@s:|:)rest}")
  echo "[$(date +%H:%M:%S)] start $name"
  "$C/lay15-tools/cruise-area.sh" "$C/🗑️generated/lay15-run/lay15$tag-$name.json" "${paths[@]}" 2>&1 | tail -3
done < "$C/lay15-tools/${AREAS:-areas.txt}"
echo "[$(date +%H:%M:%S)] ALL DONE $tag"
