#!/bin/sh
# 📐 Author 17: the measured outer area of every thermal region per scale (product numbers).
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
$BUN volumes-check.ts "appliances/$1" --product --rules volumes-thermal-coverage 2>&1 | grep "volumes-thermal-coverage" | while read -r line; do echo "$line" | awk '{print $3}'; echo "$line" | tr ' ' '\n' | grep "^info:outerAreaM2\|^info:cavityAreaM2" | sed 's/info://' | awk -F= '{printf "  %s=%.4f", $1, $2}'; echo; done
