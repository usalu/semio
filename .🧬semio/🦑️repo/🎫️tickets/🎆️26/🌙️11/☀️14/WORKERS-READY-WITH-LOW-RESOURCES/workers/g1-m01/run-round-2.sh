#!/bin/bash
# 🛎 Runs round 2 of the worker readiness protocol on g1-m01: up to six tries of the native bootstrap, five minutes apart after a fleet refusal, with 30 s samples and the hard stops of the round (process of the run above 8 GB, pressure level 4, level 2 in three samples in a row, inference of more than 300 projects without a fleet line before it).
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
samples="$here/round-2.samples.txt"
service=$(ls -t "${TMPDIR%/}"/semio-repo/$(id -u)/host-*.log 2>/dev/null | head -1)
refused() { grep -aqiE "graph-wanted|wait(ed|ing) for the fleet|the fleet was asked for" "$1" && return 0; local ref; ref=$(grep -aoE "reference [0-9a-f]{8}" "$1" | tail -1 | cut -d" " -f2); [ -n "$ref" ] && cat "${TMPDIR%/}"/semio-repo/$(id -u)/host-*.log | grep -a "$ref" | grep -aq "graph-wanted"; }
descendants() { local pid; for pid in $(pgrep -P "$1"); do echo "$pid"; descendants "$pid"; done; }
try=${1:-1}
while [ "$try" -le 6 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-2.log"; else log="$HOME/native-bootstrap-round-2-try-$try.log"; fi
  printf '%s TRY %s start, log %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$log" >> "$samples"
  bash "$here/../../native-bootstrap.sh" "$log" &
  runner=$!
  stopped=""; twos=0
  while kill -0 "$runner" 2>/dev/null; do
    sleep 30
    pressure=$(sysctl -n kern.memorystatus_vm_pressure_level 2>/dev/null)
    printf '%s pressure=%s\n%s\n' "$(date -u +%H:%M:%SZ)" "$pressure" "$(ps -axo rss,command | sort -nr | head -3 | cut -c1-160)" >> "$samples"
    [ -n "$stopped" ] && continue
    if [ "${pressure:-1}" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    family=$(descendants "$runner" | tr '\n' ',' | sed 's/,$//')
    peak=0; [ -n "$family" ] && peak=$(ps -o rss= -p "$family" 2>/dev/null | sort -nr | head -1 | tr -d ' ')
    printf '  run peak rss=%s KiB\n' "${peak:-0}" >> "$samples"
    reason=""
    [ "${peak:-0}" -gt 8388608 ] && reason="process of the run above 8 GB resident ($peak KiB)"
    [ "${pressure:-1}" -ge 4 ] && reason="memory pressure level $pressure"
    [ "$twos" -ge 3 ] && reason="memory pressure level 2 in three samples in a row"
    inferring=$(grep -anE 'Inferring projects: .* of [0-9]+ projects' "$log" | tail -1)
    if [ -n "$inferring" ]; then
      total=$(printf '%s' "$inferring" | grep -oE 'of [0-9]+ projects' | grep -oE '[0-9]+')
      line=${inferring%%:*}
      fleet=$(head -n "$line" "$log" | grep -aci 'fleet')
      [ "${total:-0}" -gt 300 ] && [ "$fleet" = 0 ] && reason="inference of $total projects with no fleet line before it"
    fi
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
  [ "$try" -le 6 ] && sleep 300
done
echo "[DEBUG] round 2 finished"; grep -a 'TRY\|HARD STOP' "$samples"
