#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
E="🧰️framework/🔨️modules/🧵️job/⏱️context/📦️packages/🦀️rust/🦀️.rs"
for i in $(seq 1 16); do
  r=$(bun "$C/1105b-tools/bundle/closure.ts" "$E" 2>&1 | tail -1)
  f=$(echo "$r" | awk '{print $1}'); b=$(echo "$r" | awk '{print $3}')
  if [ -n "$b" ] && [ "$b" -le 262144 ] 2>/dev/null && [ "$f" -le 32 ]; then echo "FITS $r"; exit 0; fi
  sleep 30
done
echo "STILL $r"; exit 1
