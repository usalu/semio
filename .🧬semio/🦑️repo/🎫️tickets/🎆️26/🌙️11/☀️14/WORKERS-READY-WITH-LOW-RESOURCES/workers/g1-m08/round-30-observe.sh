#!/bin/bash
# 🔭 Observes the automatic ready runner (sidecar role `ready`) on g1-m08 in round 30 without starting any repo command: every 30 s the fleet status row, the memory pressure level and the three largest processes, every 5 s the slice worker of the graph read (ROUND-NOTE-P12.md section 1). Ends when the completion marker exists, when the runner reports a failed stage (backoff) or after the given hours. Usage: round-30-observe.sh [hours]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
worktree="$HOME/Documents/semio"
status="/Volumes/semio-store/v1/origins/g1-m08/ready.json"
marker="$worktree/.cache/f/native-bootstrap.done"
samples="$here/round-30.samples.txt"
p12="$here/round-30.p12-auto.txt"
stages="$here/round-30.stages.txt"
deadline=$(( $(date +%s) + ${1:-6} * 3600 ))
field() { sed -nE "s/.*\"$1\":(\"[^\"]*\"|[^,}]*).*/\1/p" "$status" 2>/dev/null | tr -d '"' | head -1; }
sampler() {
  local sampled=0 row pid rssmb fp
  while [ ! -e "$p12.stop" ]; do
    row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | sort -k2 -nr | head -1)
    pid=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
    if [ -n "$pid" ]; then
      fp=$(footprint -p "$pid" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
      echo "$(date -u +%FT%TZ) pid=$pid rss=${rssmb}MB footprint=[$fp] stage=$(field stage)" >> "$p12"
      if [ "$rssmb" -gt 4096 ] && [ "$sampled" = 0 ]; then sampled=1; sample "$pid" 5 -file "${p12%.txt}.sample-$(date -u +%Y%m%dT%H%M%SZ).txt" >/dev/null 2>&1 & fi
    fi
    sleep 5
  done
}
rm -f "$p12.stop"; sampler & samplerpid=$!
trap 'touch "$p12.stop"; wait "$samplerpid" 2>/dev/null; rm -f "$p12.stop"' EXIT
last=""; outcome="time limit"
while [ "$(date +%s)" -lt "$deadline" ]; do
  now="$(field state)|$(field stage)|$(field attempts)|$(field exit)"
  if [ "$now" != "$last" ]; then printf '%s state=%s stage=%s (%s of %s) attempts=%s exit=%s peak=%s MB\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$(field state)" "$(field stage)" "$(field stageIndex)" "$(field stageCount)" "$(field attempts)" "$(field exit)" "$(field peakMegabytes)" >> "$stages"; last="$now"; fi
  printf '%s pressure=%s %s\n%s\n' "$(date -u +%H:%M:%SZ)" "$(sysctl -n kern.memorystatus_vm_pressure_level 2>/dev/null)" "$now" "$(ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-300)" >> "$samples"
  if [ -f "$marker" ]; then outcome="ready"; break; fi
  if [ "$(field state)" = backoff ]; then outcome="backoff"; break; fi
  sleep 30
done
printf '%s OBSERVER END: %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$outcome" >> "$stages"
echo "[DEBUG] observer ended: $outcome"; cat "$stages"; cut -c1-1800 "$status"
