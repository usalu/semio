#!/bin/bash
# usage: waitm.sh asset-id  -- waits (max 20 min) until the watcher rewrote m-<id>.txt after work/<id>.json
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/CVF-V"
for i in $(seq 1 240); do
  if [ "$G/m-$1.txt" -nt "$G/work/$1.json" ]; then break; fi
  sleep 5
done
cat "$G/m-$1.txt"
