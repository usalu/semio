#!/bin/sh
# 🕰️ Reruns the Draw component retry driver until it builds, waiting out a peer's in-flight build-infrastructure edit.
cd "$(dirname "$0")/../../../../../../.."
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/DRAW-PLUGIN-END-TO-END"
for round in $(seq 1 20); do
  sh "$T/component-retry-26.sh" "$1w$round" >/dev/null 2>&1
  grep -aq "built, described and materialized" "$T/🗑️generated/draw-native-component-r"*"-$1w$round.log" 2>/dev/null && { echo "[DEBUG] built round $round $(date +%T)" >> "$T/🗑️generated/wait-$1.status"; exit 0; }
  echo "[DEBUG] round $round failed $(date +%T)" >> "$T/🗑️generated/wait-$1.status"
  sleep 150
done
exit 1
