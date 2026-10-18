#!/bin/sh
# C5A: full-workspace census, retried while a peer's manifest is mid-edit (duplicate key / parse error), at most 12 tries one minute apart.
cd /Users/ueli/Documents/semio
ulimit -n 65536
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
n=0
while [ $n -lt 12 ]; do
  n=$((n+1))
  nice -n 10 bun "$F/c5a-tools/cgc-census.ts" > "$F/🗑️generated/c5a-cgc-census.out" 2>&1
  if /usr/bin/grep -q "readerRefusedCrates\|oversized" "$F/🗑️generated/c5a-cgc-census.out"; then echo "done try $n" >> "$F/🗑️generated/c5a-cgc-loop.log"; break; fi
  echo "try $n failed" >> "$F/🗑️generated/c5a-cgc-loop.log"
  sleep 60
done
