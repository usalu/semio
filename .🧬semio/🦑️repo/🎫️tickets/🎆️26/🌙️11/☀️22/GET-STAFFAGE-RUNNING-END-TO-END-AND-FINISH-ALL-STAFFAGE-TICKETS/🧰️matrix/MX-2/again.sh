#!/bin/zsh
# 🔁️ Runs a chain script until the queue admitted it: `repo capacity run` gives a request up after 2100 s when the host stayed full; again.sh <log> <command…> asks again (at most 12 times) and appends to the log.
log="$1"; shift
for i in {1..12}; do
  "$@" >> "$log" 2>&1
  grep -aq 'admitted command' "$log" && break
  echo "[DEBUG] asked again $(date +%H:%M:%S)" >> "$log"
done
