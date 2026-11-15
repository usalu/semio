#!/bin/bash
# 🛎 Runs round 5 of the worker readiness protocol on g1-m01: up to ten tries of the native bootstrap, two minutes apart after a fleet refusal or a try that ended because the graph service was replaced, three minutes after a hard stop, with 30 s samples (three largest processes with parent pid and full command). Usage: run-round-5.sh [first try]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
samples="$here/round-5.samples.txt"
refused() { [ "$(tail -1 "$1")" = "native_rc=75" ] && return 0; grep -aqiE "graph-wanted|wait(ed|ing) for the fleet|the fleet was asked for" "$1" && return 0; local ref; ref=$(grep -aoE "reference [0-9a-f]{8}" "$1" | tail -1 | cut -d" " -f2); [ -n "$ref" ] && cat "${TMPDIR%/}"/semio-repo/$(id -u)/host-*.log | grep -a "$ref" | grep -aq "graph-wanted"; }
replaced() { grep -aqE "macOS denied the resident service|superseded|Restarting with the current implementation|code changed" "$1"; }
descendants() { local pid; for pid in $(pgrep -P "$1"); do echo "$pid"; descendants "$pid"; done; }
try=${1:-1}; replacedruns=0
while [ "$try" -le 10 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-5.log"; else log="$HOME/native-bootstrap-round-5-try-$try.log"; fi
  printf '%s TRY %s start, log %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$log" >> "$samples"
  bash "$here/../../native-bootstrap.sh" "$log" &
  runner=$!
  stopped=""; twos=0
  while kill -0 "$runner" 2>/dev/null; do
    sleep 30
    pressure=$(sysctl -n kern.memorystatus_vm_pressure_level 2>/dev/null)
    printf '%s pressure=%s\n%s\n' "$(date -u +%H:%M:%SZ)" "$pressure" "$(ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-400)" >> "$samples"
    [ -n "$stopped" ] && continue
    if [ "${pressure:-1}" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    family=$(descendants "$runner" | tr '\n' ',' | sed 's/,$//')
    peak=0; [ -n "$family" ] && peak=$(ps -o rss= -p "$family" 2>/dev/null | sort -nr | head -1 | tr -d ' ')
    printf '  run peak rss=%s KiB\n' "${peak:-0}" >> "$samples"
    reason=""
    [ "${peak:-0}" -gt 8388608 ] && reason="process of the run above 8 GB resident ($peak KiB)"
    [ "${pressure:-1}" -ge 4 ] && reason="memory pressure level $pressure"
    [ "$twos" -ge 5 ] && reason="memory pressure level 2 in five samples in a row"
    if [ -n "$reason" ]; then
      stopped="$reason"
      printf '%s HARD STOP: %s\n' "$(date -u +%H:%M:%SZ)" "$reason" >> "$samples"
      for pid in $(descendants "$runner"); do kill -INT "$pid" 2>/dev/null; done
    fi
  done
  wait "$runner"
  result=$(tail -1 "$log")
  printf '%s TRY %s end: %s; hard stop: %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$result" "${stopped:-none}" >> "$samples"
  [ "$result" = "native_rc=0 marker written" ] && break
  pause=120
  if [ -n "$stopped" ]; then pause=180; replacedruns=0
  elif refused "$log"; then replacedruns=0
  elif replaced "$log" && [ "$replacedruns" -lt 1 ]; then replacedruns=$((replacedruns+1))
  else break; fi
  try=$((try+1))
  [ "$try" -le 10 ] && sleep "$pause"
done
echo "[DEBUG] round 5 finished"; grep -a 'TRY\|HARD STOP' "$samples"
