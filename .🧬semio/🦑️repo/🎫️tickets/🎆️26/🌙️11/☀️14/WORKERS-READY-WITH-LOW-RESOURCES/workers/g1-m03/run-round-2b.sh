#!/bin/bash
# 🛰️ Continues protocol round 2 on this worker from try <k>: tries of W/native-bootstrap.sh 5 minutes apart while the run ends with the fleet refusal (the refusal text is in the log of the resident graph service, the bootstrap log only names its reference), sampled every 30 s, hard-stop rules of ROUND.md round 2. Usage: run-round-2b.sh <first-try> <seconds-before-first-try>
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-2.monitor.txt"
service=$(ls -t "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log 2>/dev/null | head -1)
fleetline='^graph: .*fleet|waiting for the fleet|asked the fleet|graph-wanted'
sleep "$2"
k=$1
while [ "$k" -le 6 ]; do
  log="$HOME/native-bootstrap-round-2-try-$k.log"; samples="$here/round-2.try-$k.samples.txt"; : > "$samples"
  before=0; [ -n "$service" ] && before=$(wc -l < "$service" | tr -d ' ')
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ) log=$log service=$service from line $before" >> "$note"
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
      m=$(cat "$log" <(tail -n +"$((before+1))" "${service:-/dev/null}") 2>/dev/null | grep -a -o -m1 -E 'Inferring projects: .* of [0-9]+ projects' | grep -o -E 'of [0-9]+ projects' | grep -o -E '[0-9]+')
      if [ "${m:-0}" -gt 300 ] && ! cat "$log" <(tail -n +"$((before+1))" "${service:-/dev/null}") 2>/dev/null | grep -a -q -E "$fleetline"; then reason="Inferring projects: of $m projects with no fleet line before it"; fi
      if [ -n "$reason" ]; then echo "try $k hard stop $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1; fi
    fi
    sleep 30
  done
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$log" | tail -1 | cut -d= -f2)
  tail -n +"$((before+1))" "${service:-/dev/null}" 2>/dev/null | grep -a -E '^graph:|request-failed|Restarting with|restored' | awk '!s[substr($0,1,70)]++' | cut -c1-400 > "$here/round-2.try-$k.service-lines.txt"
  refusal=0; grep -a -q -E "$fleetline" "$log" "$here/round-2.try-$k.service-lines.txt" 2>/dev/null && grep -a -q -E 'graph-wanted|nothing to wait for any longer|for the fleet' "$log" "$here/round-2.try-$k.service-lines.txt" 2>/dev/null && refusal=1
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=${rc:-unknown} stopped=$stopped fleet_refusal=$refusal" >> "$note"
  [ "$stopped" = 1 ] && break; [ "${rc:-1}" = 0 ] && break; [ "$refusal" = 1 ] || break
  [ "$k" = 6 ] && break
  echo "try $k fleet refusal: waiting 300 s $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"; sleep 300; k=$((k+1))
done
echo "round ended $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
