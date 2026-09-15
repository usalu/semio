#!/bin/zsh
# 🔁 (semio-91) Restage the puzzle3d react RELEASE guest (component-release → support → materialize → prepare → activate) for the
# fill e2e gate, retrying through peer cargo churn. Log: 🗑️generated/W5-mac-react-e2e/w6-restage-release.txt
LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/🗑️generated/W5-mac-react-e2e/w6-restage-release.txt"
cd /Users/ueli/Documents/semio
export NX_DAEMON=false NX_SKIP_NX_CACHE=true
for attempt in $(seq 1 4); do
  date >> "$LOG"; echo "attempt=$attempt" >> "$LOG"
  bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-react-release --skip-nx-cache >> "$LOG" 2>&1; rc=$?
  echo "EXIT=$rc attempt=$attempt" >> "$LOG"
  [ $rc -eq 0 ] && break
  sleep 120
done
date >> "$LOG"
echo "RESTAGE-DONE" >> "$LOG"
