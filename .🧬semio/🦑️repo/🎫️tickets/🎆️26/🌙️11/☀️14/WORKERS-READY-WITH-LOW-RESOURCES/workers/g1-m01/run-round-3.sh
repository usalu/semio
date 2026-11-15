#!/bin/bash
# 🛎 Runs round 3 of the worker readiness protocol on g1-m01: up to eight tries of the native bootstrap, three minutes apart after a fleet refusal, with 30 s samples (three largest processes with parent pid and full command, parent chain of a large git process) and the hard stops of the round. Usage: run-round-3.sh [first try]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
samples="$here/round-3.samples.txt"
refused() { [ "$(tail -1 "$1")" = "native_rc=75" ] && return 0; grep -aqiE "graph-wanted|wait(ed|ing) for the fleet|the fleet was asked for" "$1" && return 0; local ref; ref=$(grep -aoE "reference [0-9a-f]{8}" "$1" | tail -1 | cut -d" " -f2); [ -n "$ref" ] && cat "${TMPDIR%/}"/semio-repo/$(id -u)/host-*.log | grep -a "$ref" | grep -aq "graph-wanted"; }
descendants() { local pid; for pid in $(pgrep -P "$1"); do echo "$pid"; descendants "$pid"; done; }
chain() { local pid="$1"; while [ -n "$pid" ] && [ "$pid" -gt 0 ]; do ps -o pid=,ppid=,args= -p "$pid" | cut -c1-300; pid=$(ps -o ppid= -p "$pid" | tr -d ' '); done; }
try=${1:-1}
while [ "$try" -le 8 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-3.log"; else log="$HOME/native-bootstrap-round-3-try-$try.log"; fi
  printf '%s TRY %s start, log %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$log" >> "$samples"
  bash "$here/../../native-bootstrap.sh" "$log" &
  runner=$!
  stopped=""; twos=0
  while kill -0 "$runner" 2>/dev/null; do
    sleep 30
    pressure=$(sysctl -n kern.memorystatus_vm_pressure_level 2>/dev/null)
    top=$(ps -axo rss,pid,ppid,args | sort -nr | head -3)
    printf '%s pressure=%s\n%s\n' "$(date -u +%H:%M:%SZ)" "$pressure" "$(printf '%s\n' "$top" | cut -c1-400)" >> "$samples"
    printf '%s\n' "$top" | awk '$1>1048576 && $4 ~ /git$/ {print $2}' | while read -r gitpid; do printf '  GIT CHAIN of %s:\n' "$gitpid" >> "$samples"; chain "$gitpid" | sed 's/^/    /' >> "$samples"; done
    [ -n "$stopped" ] && continue
    if [ "${pressure:-1}" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    family=$(descendants "$runner" | tr '\n' ',' | sed 's/,$//')
    peak=0; [ -n "$family" ] && peak=$(ps -o rss= -p "$family" 2>/dev/null | sort -nr | head -1 | tr -d ' ')
    printf '  run peak rss=%s KiB\n' "${peak:-0}" >> "$samples"
    reason=""
    [ "${peak:-0}" -gt 8388608 ] && reason="process of the run above 8 GB resident ($peak KiB)"
    [ "${pressure:-1}" -ge 4 ] && reason="memory pressure level $pressure"
    [ "$twos" -ge 3 ] && reason="memory pressure level 2 in three samples in a row"
    if [ -n "$reason" ]; then
      stopped="$reason"
      printf '%s HARD STOP: %s\n' "$(date -u +%H:%M:%SZ)" "$reason" >> "$samples"
      for pid in $(descendants "$runner"); do kill -INT "$pid" 2>/dev/null; done
    fi
  done
  wait "$runner"
  result=$(tail -1 "$log")
  printf '%s TRY %s end: %s; hard stop: %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$result" "${stopped:-none}" >> "$samples"
  [ -n "$stopped" ] && break
  [ "$result" = "native_rc=0 marker written" ] && break
  refused "$log" || break
  try=$((try+1))
  [ "$try" -le 8 ] && sleep 180
done
echo "[DEBUG] round 3 finished"; grep -a 'TRY\|HARD STOP' "$samples"
