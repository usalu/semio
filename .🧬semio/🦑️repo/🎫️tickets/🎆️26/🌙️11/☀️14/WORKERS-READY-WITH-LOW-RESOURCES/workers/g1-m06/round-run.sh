#!/bin/bash
# 🩺 Runs the native bootstrap of one protocol round once on g1-m06, samples the three largest resident processes with parent and full command and the memory pressure level every 30 s, records the parent chain of a large git process, sends one SIGINT on a hard stop condition (a process of the run or of the graph service of this host above 8 GB resident, pressure level 4 in two samples in a row, level 2 in six samples in a row) and restarts the replica agent when launchd leaves it down so the report reaches the coordinator.
# Usage: bash round-run.sh <log>
# @see ../../PROTOCOL.md
set -u
log=$1
samples=${log%.log}.samples.txt
ticket=$(cd "$(dirname "$0")/../.." && pwd)
label=gui/$(id -u)/tech.semio.fleet.replica
: > "$samples"
bash "$ticket/native-bootstrap.sh" "$log" &
run=$!
descendants() { for child in $(pgrep -P "$1"); do echo "$child"; descendants "$child"; done; }
chain() { pid=$1; while [ "${pid:-0}" -gt 1 ]; do ps -o pid=,ppid=,args= -p "$pid" | cut -c1-300; pid=$(ps -o ppid= -p "$pid" | tr -d ' '); done; }
stopped=
warned=0
critical=0
while kill -0 "$run" 2>/dev/null; do
  now=$(date -u +%Y-%m-%dT%H:%M:%SZ)
  level=$(sysctl -n kern.memorystatus_vm_pressure_level)
  family=$(descendants "$run" | tr '\n' ',' | sed 's/,$//')
  own=$(ps -o rss=,pid=,ppid=,args= -p "${family:-$run}" | sort -nr | head -3 | cut -c1-400)
  all=$(ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-400)
  printf '%s pressure_level=%s\nall:\n%s\nrun:\n%s\n' "$now" "$level" "$all" "$own" >> "$samples"
  printf '%s\n' "$all" | awk '$1 > 1000000 && $4 ~ /(^|\/)git$/ {print $2}' | while read -r big; do printf 'chain of git %s:\n%s\n' "$big" "$(chain "$big")" >> "$samples"; done
  largest=$(ps -axo rss,args | grep -E 'semio/runtimes/bun|Documents/semio' | sort -nr | head -1 | awk '{print $1}')
  if [ "$level" = 2 ]; then warned=$((warned+1)); else warned=0; fi
  if [ "$level" = 4 ]; then critical=$((critical+1)); else critical=0; fi
  reason=
  [ "${largest:-0}" -gt 8388608 ] && reason="process of the run above 8 GB resident ($largest KB)"
  [ "$warned" -ge 6 ] && reason="memory pressure level 2 in six samples in a row"
  [ "$critical" -ge 2 ] && reason="memory pressure level 4 in two samples in a row"
  if [ -n "$reason" ] && [ -z "$stopped" ]; then
    stopped=$reason
    printf '%s HARD STOP: %s\n' "$now" "$reason" >> "$samples"
    for pid in $(descendants "$run"); do ps -o args= -p "$pid" | grep -q 'repo run ' && kill -INT "$pid"; done
  fi
  if launchctl print "$label" 2>/dev/null | grep -q 'state = not running'; then
    sleep 20
    if launchctl print "$label" 2>/dev/null | grep -q 'state = not running'; then launchctl kickstart "$label"; printf '%s replica kickstart %s\n' "$now" "$?" >> "$samples"; fi
  fi
  sleep 30
done
wait "$run"
printf '%s runner ended, hard stop: %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "${stopped:-none}" >> "$samples"
