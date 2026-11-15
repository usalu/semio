#!/bin/bash
# 🛰️ Runs one protocol round of the native bootstrap on this worker: starts W/native-bootstrap.sh in its own process group, samples resident sizes every 30 s and sends one SIGINT on a hard-stop condition. Usage: run-round.sh <round>
set -u
n=$1; here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd)
log="$HOME/native-bootstrap-round-$n.log"; samples="$here/round-$n.samples.txt"; note="$here/round-$n.monitor.txt"
: > "$samples"; : > "$note"
perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
pid=$!
echo "started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
stopped=0
while kill -0 "$pid" 2>/dev/null; do
  { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$(sysctl -n kern.memorystatus_vm_pressure_level)"; ps -axo rss,command | sort -nr | head -3 | cut -c1-220; } >> "$samples"
  if [ "$stopped" = 0 ]; then
    reason=""
    top=$(ps -axo rss | sort -nr | head -1 | tr -d ' ')
    [ "${top:-0}" -gt 8388608 ] && reason="process above 8 GB resident ($top KiB)"
    [ "$(sysctl -n kern.memorystatus_vm_pressure_level)" != 1 ] && reason="memory pressure level $(sysctl -n kern.memorystatus_vm_pressure_level)"
    g=$(grep -a -o -E 'graph: refreshing [0-9]+ projects' "$log" 2>/dev/null | grep -o -E '[0-9]+' | sort -n | tail -1)
    [ -n "$g" ] && [ "$g" -gt 300 ] && reason="graph: refreshing $g projects"
    i=$(grep -a -o -E 'Inferring projects: .* of [0-9]+ projects' "$log" 2>/dev/null | grep -o -E 'of [0-9]+ projects' | grep -o -E '[0-9]+' | sort -n | tail -1)
    [ -n "$i" ] && [ "$i" -gt 300 ] && reason="Inferring projects: of $i projects"
    if [ -n "$reason" ]; then
      echo "hard stop $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"
      kill -INT -- "-$pid"; stopped=1
    fi
  fi
  sleep 30
done
wait "$pid"; echo "ended $(date -u +%Y-%m-%dT%H:%M:%SZ) wrapper_rc=$? stopped=$stopped" >> "$note"
