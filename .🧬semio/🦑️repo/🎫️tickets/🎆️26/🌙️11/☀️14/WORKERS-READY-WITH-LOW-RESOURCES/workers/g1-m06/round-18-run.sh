#!/bin/bash
# 🛰️ Runs the native bootstrap of one try of round 18 once on g1-m06 with the slice worker sampler of ROUND-NOTE-P12 (every 5 s) and a 30 s sampler of the three largest processes and the memory pressure level; sends one SIGINT on a process above 8 GB resident, on pressure level 4 in two samples in a row, on a graph step without a new progress line for 15 minutes and on a dependency store wait above 5 minutes (each with a capture of the service log tail and the process list), records pressure level 2 without stopping, and restarts the replica agent when launchd leaves it down.
# Usage: bash round-18-run.sh <log>
# @see ../../PROTOCOL.md
set -u
log=$1
samples=${log%.log}.samples.txt
ticket=$(cd "$(dirname "$0")/../.." && pwd)
label=gui/$(id -u)/tech.semio.fleet.replica
: > "$samples"
p12=${log%.log}.p12.txt
: > "$p12"; rm -f "$p12.stop"
root=$HOME/Documents/semio
service=$(ls -t "$TMPDIR"semio-repo/"$(id -u)"/host-*.log 2>/dev/null | head -1)
printf 'service log: %s\nclosure entries before: %s\n' "$service" "$(find "$root"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' ')" >> "$p12"
(
  sampled=0
  while [ ! -e "$p12.stop" ]; do
    ts=$(date -u +%FT%TZ)
    row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | head -1)
    pid=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
    if [ -n "$pid" ]; then
      fp=$(footprint -p "$pid" 2>/dev/null | grep -m1 -i footprint | sed 's/^ *//' | cut -c1-120)
      newest=$(ls -t "$TMPDIR"semio-repo/"$(id -u)"/host-*.log 2>/dev/null | head -1)
      last=$(tail -n 600 "$newest" 2>/dev/null | grep -E 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
      echo "$ts pid=$pid rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$p12"
      if [ "$rssmb" -gt 4096 ] && [ "$sampled" = 0 ]; then sampled=1; sample "$pid" 5 -file "$p12.sample-$ts.txt" >/dev/null 2>&1 & fi
    else
      echo "$ts no slice worker" >> "$p12"
    fi
    sleep 5
  done
) &
sampler=$!
bash "$ticket/native-bootstrap.sh" "$log" &
run=$!
capture() { { printf 'last 60 lines of the newest graph service log:\n'; tail -n 60 "$(ls -t "$TMPDIR"semio-repo/"$(id -u)"/host-*.log | head -1)" | cut -c1-400; printf 'processes (bun, nx, repo):\n'; ps -axo pid,ppid,etime,rss,command | grep -E 'bun|nx|repo' | grep -v grep | cut -c1-300; } > "${log%.log}.stall.txt"; }
descendants() { for child in $(pgrep -P "$1"); do echo "$child"; descendants "$child"; done; }
chain() { pid=$1; while [ "${pid:-0}" -gt 1 ]; do ps -o pid=,ppid=,args= -p "$pid" | cut -c1-300; pid=$(ps -o ppid= -p "$pid" | tr -d ' '); done; }
stopped=
warned=0
critical=0
still=0
last=
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
  line=$(tail -n 5 "$log" | grep 'Reading the project graph slice' | tail -n 1 | sed -E 's/\([0-9]+ s\)//')
  if [ -n "$line" ] && [ "$line" = "$last" ]; then still=$((still+1)); else still=0; fi
  last=$line
  reason=
  held=$(tail -n 40 "$log" | grep 'Another process holds the dependency store' | tail -n 1)
  heldfor=$(printf '%s' "$held" | sed -n 's/.*(\([0-9]*\) s of.*/\1/p')
  if [ "$still" -ge 30 ] && [ -z "$stopped" ]; then reason="no new progress line of the graph step for 15 minutes"; capture; fi
  if [ "${heldfor:-0}" -ge 300 ] && [ -z "$stopped" ]; then reason="dependency store wait above 5 minutes: $held"; capture; fi
  [ "${largest:-0}" -gt 8388608 ] && reason="process of the run above 8 GB resident ($largest KB)"
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
touch "$p12.stop"; wait "$sampler" 2>/dev/null
printf 'closure entries after: %s\n' "$(find "$root"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' ')" >> "$p12"
printf '%s runner ended, hard stop: %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "${stopped:-none}" >> "$samples"
