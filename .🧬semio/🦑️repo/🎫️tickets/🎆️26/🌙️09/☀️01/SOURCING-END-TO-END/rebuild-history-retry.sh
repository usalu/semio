#!/bin/sh
# 🔁️ Retries the locked rebuild until the host answers ready: peers' half-edited crates fail single attempts.
# Usage: sh rebuild-history-retry.sh [attempts]   (run from the workspace root)
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/SOURCING-END-TO-END"
i=0
while [ $i -lt "${1:-12}" ]; do
  sh "$T/rebuild-history.sh" > "$T/🗑️generated/rebuild-retry-$i.txt" 2>&1
  grep -q "ready on" "$T/🗑️generated/rebuild-retry-$i.txt" && { echo ready; exit 0; }
  i=$((i+1)); sleep 120
done
exit 1
