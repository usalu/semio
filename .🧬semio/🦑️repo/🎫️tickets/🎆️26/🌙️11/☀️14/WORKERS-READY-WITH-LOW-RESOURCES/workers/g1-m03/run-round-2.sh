#!/bin/bash
# 🛰️ Runs protocol round 2 on this worker: up to six tries of W/native-bootstrap.sh (5 minutes apart, only after a fleet refusal), each in its own process group, sampled every 30 s, with the hard-stop rules of ROUND.md round 2. Usage: run-round-2.sh
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-2.monitor.txt"; : > "$note"
for k in 1 2 3 4 5 6; do
  if [ "$k" = 1 ]; then log="$HOME/native-bootstrap-round-2.log"; else log="$HOME/native-bootstrap-round-2-try-$k.log"; fi
  samples="$here/round-2.try-$k.samples.txt"; : > "$samples"
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ) log=$log" >> "$note"
  stopped=0; twos=0
  while kill -0 "$pid" 2>/dev/null; do
    level=$(sysctl -n kern.memorystatus_vm_pressure_level)
    { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,command | sort -nr | head -3 | cut -c1-220; } >> "$samples"
    if [ "$level" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    if [ "$stopped" = 0 ]; then
      reason=""
      top=$(ps -axo pgid,rss | awk -v g="$pid" '$1==g && $2>m {m=$2} END {print m+0}')
      [ "$top" -gt 8388608 ] && reason="process of the run above 8 GB resident ($top KiB)"
      [ "$level" = 4 ] && reason="memory pressure level 4"
      [ "$twos" -ge 3 ] && reason="memory pressure level 2 in three samples in a row"
      line=$(grep -a -n -m1 -E 'Inferring projects: .* of [0-9]+ projects' "$log" 2>/dev/null)
      if [ -n "$line" ]; then
        m=$(printf '%s' "$line" | grep -o -E 'of [0-9]+ projects' | grep -o -E '[0-9]+'); at=${line%%:*}
        if [ "${m:-0}" -gt 300 ] && ! head -n "$at" "$log" | grep -a -q -i 'fleet'; then reason="Inferring projects: of $m projects with no fleet line before it"; fi
      fi
      if [ -n "$reason" ]; then echo "try $k hard stop $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1; fi
    fi
    sleep 30
  done
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$log" | tail -1 | cut -d= -f2)
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=${rc:-unknown} stopped=$stopped" >> "$note"
  [ "$stopped" = 1 ] && break
  [ "${rc:-1}" = 0 ] && break
  if grep -a -q -E 'graph-wanted|for the fleet' "$log"; then
    [ "$k" = 6 ] && break
    echo "try $k fleet refusal: waiting 300 s $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"; sleep 300
  else break; fi
done
echo "round ended $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
