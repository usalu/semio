#!/bin/bash
# usage: waitm.sh asset-id  -- waits until the watcher rewrote m-<asset>.txt after the asset json was last modified
G="/home/deck/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️29/ARCHITECTURAL-STAFFAGE-ARCHIVE/🗑️generated/NW-H2"
J="/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/$1/🔣️.json"
for i in $(seq 1 90); do
  if [ "$G/m-$1.txt" -nt "$J" ]; then break; fi
  sleep 2
done
grep -c "^   gap" "$G/m-$1.txt" | sed 's/^/gaps total: /'
