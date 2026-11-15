#!/bin/bash
# 🛰️ Runs protocol round 3 on this worker: up to eight tries of W/native-bootstrap.sh, 3 minutes apart while a try ends with native_rc=75 or the fleet refusal text, each in its own process group, sampled every 30 s with parent pids, hard-stop rules of ROUND.md round 3. Usage: run-round-3.sh
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-3.monitor.txt"; : > "$note"
refusaltext='graph-wanted|nothing to wait for any longer|asked the fleet for'
servicelog(){ ls -t "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log 2>/dev/null | head -1; }
k=1
while [ "$k" -le 8 ]; do
  if [ "$k" = 1 ]; then log="$HOME/native-bootstrap-round-3.log"; else log="$HOME/native-bootstrap-round-3-try-$k.log"; fi
  samples="$here/round-3.try-$k.samples.txt"; chains="$here/round-3.try-$k.git-parents.txt"; : > "$samples"; : > "$chains"
  touch "$here/.round-3.try-$k.start"
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ) log=$log" >> "$note"
  stopped=0; twos=0
  while kill -0 "$pid" 2>/dev/null; do
    level=$(sysctl -n kern.memorystatus_vm_pressure_level)
    { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-600; } >> "$samples"
    g=$(ps -axo rss,pid,ppid,comm | awk '$1>1048576 && $4 ~ /\/git$/ {print $2; exit}')
    if [ -n "$g" ] && [ "$(wc -l < "$chains")" -lt 60 ]; then
      { date -u +"%Y-%m-%dT%H:%M:%SZ git pid $g"; p=$g; while [ -n "$p" ] && [ "$p" != 0 ]; do ps -o pid=,ppid=,rss=,args= -p "$p" | cut -c1-300; p=$(ps -o ppid= -p "$p" | tr -d ' '); [ "$p" = 1 ] && { ps -o pid=,ppid=,args= -p 1 | cut -c1-100; break; }; done; } >> "$chains"
    fi
    if [ "$level" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    if [ "$stopped" = 0 ]; then
      reason=""
      top=$(ps -axo pgid,rss | awk -v g="$pid" '$1==g && $2>m {m=$2} END {print m+0}')
      [ "$top" -gt 8388608 ] && reason="process of the run above 8 GB resident ($top KiB)"
      [ "$level" = 4 ] && reason="memory pressure level 4"
      [ "$twos" -ge 3 ] && reason="memory pressure level 2 in three samples in a row"
      if [ -n "$reason" ]; then echo "try $k hard stop $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1; fi
    fi
    sleep 30
  done
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$log" | tail -1 | cut -d= -f2)
  : > "$here/round-3.try-$k.service-lines.txt"
  for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do [ "$s" -nt "$here/.round-3.try-$k.start" ] && { echo "== $s"; grep -a -n -E '^graph:|request-failed|Restarting|restored|adopted|serving|arrived|wanted' "$s" | awk -F: '!s[substr($0,index($0,":")+1,48)]++' | cut -c1-420; } >> "$here/round-3.try-$k.service-lines.txt"; done
  refusal=0; { [ "${rc:-1}" = 75 ] || grep -a -q -E "$refusaltext" "$log" "$here/round-3.try-$k.service-lines.txt" 2>/dev/null; } && refusal=1
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=${rc:-unknown} stopped=$stopped fleet_refusal=$refusal" >> "$note"
  [ "$stopped" = 1 ] && break; [ "${rc:-1}" = 0 ] && break; [ "$refusal" = 1 ] || break
  [ "$k" = 8 ] && break
  echo "try $k refusal: waiting 180 s $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"; sleep 180; k=$((k+1))
done
echo "round ended $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
