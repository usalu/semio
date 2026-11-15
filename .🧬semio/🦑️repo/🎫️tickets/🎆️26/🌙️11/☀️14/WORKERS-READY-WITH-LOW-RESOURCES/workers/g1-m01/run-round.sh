#!/bin/bash
# 🛎 Runs one round of the worker readiness protocol on g1-m01: starts the native bootstrap once, samples the three largest processes every 30 s and sends one SIGINT when a hard stop condition of the round is met. Usage: run-round.sh <round>
set -u
round="$1"
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
log="$HOME/native-bootstrap-round-$round.log"
samples="$here/round-$round.samples.txt"
: > "$samples"
bash "$here/../../native-bootstrap.sh" "$log" &
runner=$!
stopped=""
while kill -0 "$runner" 2>/dev/null; do
  sleep 30
  top=$(ps -axo rss,command | sort -nr | head -3 | cut -c1-160)
  pressure=$(sysctl -n kern.memorystatus_vm_pressure_level 2>/dev/null)
  printf '%s pressure=%s\n%s\n' "$(date -u +%H:%M:%SZ)" "$pressure" "$top" >> "$samples"
  [ -n "$stopped" ] && continue
  peak=$(printf '%s\n' "$top" | head -1 | awk '{print $1}')
  reason=""
  [ "${peak:-0}" -gt 8388608 ] && reason="process above 8 GB resident ($peak KiB)"
  [ "${pressure:-1}" -gt 1 ] && reason="memory pressure level $pressure"
  line=$(grep -aoE 'graph: refreshing [0-9]+ projects|Inferring projects: .* of [0-9]+ projects' "$log" | tail -1)
  number=$(printf '%s' "$line" | grep -oE '[0-9]+ projects' | grep -oE '[0-9]+' | tail -1)
  [ -n "$number" ] && [ "$number" -gt 300 ] && reason="$line"
  if [ -n "$reason" ]; then
    stopped="$reason"
    printf '%s HARD STOP: %s\n' "$(date -u +%H:%M:%SZ)" "$reason" >> "$samples"
    pkids=$(pgrep -P "$runner" | tr '\n' ' ')
    for pid in $pkids; do for child in $(pgrep -P "$pid"); do kill -INT "$child" 2>/dev/null; done; kill -INT "$pid" 2>/dev/null; done
  fi
done
wait "$runner"
echo "[DEBUG] round $round finished; hard stop: ${stopped:-none}; $(tail -1 "$log")"
