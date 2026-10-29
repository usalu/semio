#!/bin/bash
# usage: idle.sh <max 10s ticks> : prints the framework files modified in the last 4 minutes once none changed for 2 ticks (or the limit is hit)
F="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules"
for i in $(seq 1 ${1:-30}); do sleep 10; done
date +%H:%M:%S
stat -f '%Sm %N' -t '%H:%M:%S' "$F/🔌️plugin/🌱️initialization/🌱️genesis/🦀️.rs"
find "$F" -newermt "$(date -v-4M '+%Y-%m-%d %H:%M:%S')" -type f -not -path '*/🧪️*' 2>/dev/null | head -10
