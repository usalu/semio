#!/bin/bash
# 🛰️ Runs protocol round 5 on this worker: up to ten tries of W/native-bootstrap.sh, each in its own process group, sampled every 30 s with parent pids. A refusal (native_rc=75 or refusal text) or a try ended by a code change is followed by the next try after 120 s, a hard stop by the next try after 180 s; a try that reaches nx tasks ends the round. Usage: run-round-5.sh
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-5.monitor.txt"; : > "$note"
refusaltext='graph-wanted|nothing to wait for any longer|asked for [0-9]+ and delivered|could not be read on this machine'
want='waiting for the fleet|taken from the fleet|inferred here|refreshing [0-9]+ projects|asking the host for|stale facts'
k=1
while [ "$k" -le 10 ]; do
  if [ "$k" = 1 ]; then log="$HOME/native-bootstrap-round-5.log"; else log="$HOME/native-bootstrap-round-5-try-$k.log"; fi
  samples="$here/round-5.try-$k.samples.txt"; : > "$samples"; mark="$here/.round-5.try-$k.start"; touch "$mark"
  for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do [ -f "$s" ] && echo "$(wc -l < "$s" | tr -d ' ') $s"; done > "$here/.round-5.try-$k.before"
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ) log=$log" >> "$note"
  stopped=0; twos=0
  while kill -0 "$pid" 2>/dev/null; do
    level=$(sysctl -n kern.memorystatus_vm_pressure_level)
    { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-600; } >> "$samples"
    if [ "$level" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    if [ "$stopped" = 0 ]; then
      reason=""
      top=$(ps -axo pgid,rss | awk -v g="$pid" '$1==g && $2>m {m=$2} END {print m+0}')
      [ "$top" -gt 8388608 ] && reason="process of the run above 8 GB resident ($top KiB)"
      [ "$level" = 4 ] && reason="memory pressure level 4"
      [ "$twos" -ge 5 ] && reason="memory pressure level 2 in five samples in a row"
      if [ -n "$reason" ]; then echo "try $k hard stop $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1; fi
    fi
    sleep 30
  done
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$log" | tail -1 | cut -d= -f2)
  out="$here/round-5.try-$k.service.txt"; : > "$out"
  for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do
    [ -f "$s" ] && [ "$s" -nt "$mark" ] || continue
    b=$(grep -F " $s" "$here/.round-5.try-$k.before" | awk '{print $1}'); b=${b:-0}
    { echo "== $s (lines after $b)"; echo "-- lines with: $want"; tail -n +"$((b+1))" "$s" | grep -a -E "$want" | awk '!s[$0]++' | cut -c1-500; echo "-- last 30 lines when the try ended"; tail -30 "$s" | cut -c1-500; } >> "$out"
  done
  refusal=0; { [ "${rc:-1}" = 75 ] || sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -q -E "$refusaltext"; } && refusal=1
  tasks=0; sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -q -E '^(✔|✖) +nx run|^Failed tasks' && tasks=1
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=${rc:-unknown} stopped=$stopped fleet_refusal=$refusal reached_tasks=$tasks" >> "$note"
  [ "${rc:-1}" = 0 ] && break
  [ "$k" = 10 ] && break
  if [ "$stopped" = 1 ]; then echo "try $k hard stop: waiting 180 s" >> "$note"; sleep 180
  elif [ "$refusal" = 1 ]; then echo "try $k refusal: waiting 120 s" >> "$note"; sleep 120
  elif [ "$tasks" = 1 ]; then break
  else echo "try $k ended without refusal and before the nx tasks (code change or other): waiting 120 s" >> "$note"; sleep 120; fi
  k=$((k+1))
done
echo "round ended $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
