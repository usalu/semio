#!/bin/bash
# 🛎 Runs round 13 of the worker readiness protocol on g1-m01: up to twelve tries of the native bootstrap, three minutes apart, each with the read-only slice worker sampler of ROUND-NOTE-P12.md (5 s), 30 s samples of the three largest processes, launcher identity before and after, the hard stops and the stall rule of the round. Usage: run-round-13.sh [first try]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
worktree="$HOME/Documents/semio"
samples="$here/round-13.samples.txt"
launcherfacts="$here/round-13.launcher.txt"
launcher="$HOME/.local/share/semio/launcher/bin/repo"
hostlogs="${TMPDIR%/}/semio-repo/$(id -u)"
clean() { LC_ALL=C sed $'s/\x1b\\[[0-9;]*[A-Za-z]//g' "$1" | LC_ALL=C grep -av '^\[script\] {'; }
newest() { ls -t "$hostlogs"/host-*.log 2>/dev/null | head -1; }
entries() { find "$worktree"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' '; }
failed_tasks() { clean "$1" | awk '/Failed tasks:/{f=1; next} f && /^ *- /{print $2; next} f && NF && !/^ *- /{f=0}' | sort -u | tr '\n' ' '; }
descendants() { local pid; for pid in $(pgrep -P "$1"); do echo "$pid"; descendants "$pid"; done; }
launcher_facts() {
  printf '\n##### %s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" >> "$launcherfacts"
  ls -li "$launcher" "$HOME/.local/bin/repo" >> "$launcherfacts" 2>&1
  [ -L "$launcher" ] && { echo "readlink: $(readlink "$launcher")"; ls -liL "$launcher"; } >> "$launcherfacts" 2>&1
  codesign -dvvv "$launcher" 2>&1 | grep -E "^(Identifier|CDHash)" >> "$launcherfacts"
}
replica_up() { if launchctl print "gui/$(id -u)/tech.semio.fleet.replica" 2>&1 | grep -qE "^[[:space:]]*state = not running"; then printf '%s REPLICA KICKSTART (state = not running)\n' "$(date -u +%H:%M:%SZ)" >> "$samples"; launchctl kickstart -k "gui/$(id -u)/tech.semio.fleet.replica" >> "$samples" 2>&1; sleep 60; fi; }
sampler() {
  local out="$1" sampled=0 ts row pid rssmb fp last log
  while [ ! -e "$out.stop" ]; do
    ts=$(date -u +%FT%TZ); log=$(newest)
    row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | head -1)
    pid=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
    if [ -n "$pid" ]; then
      fp=$(footprint -p "$pid" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
      last=$(tail -n 600 "$log" 2>/dev/null | grep -aE 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
      echo "$ts pid=$pid rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$out"
      if [ "$rssmb" -gt 4096 ] && [ "$sampled" = 0 ]; then sampled=1; sample "$pid" 5 -file "$out.sample-$ts.txt" >/dev/null 2>&1 & fi
    else
      echo "$ts no slice worker" >> "$out"
    fi
    sleep 5
  done
}
signature() { local log; log=$(newest); printf '%s|%s' "$(tail -c 30000 "$1" | clean /dev/stdin | grep -a "Reading the project graph slice\|Planning\|Loading\|nx run" | tail -1 | sed -E 's/\([0-9]+ s\)//; s/, [0-9]+, [0-9]+\)/)/g; s/[0-9]+ s[,)]//g' | cut -c1-200)" "$([ -n "$log" ] && tail -c 60000 "$log" | grep -a "^graph:" | tail -1 | cut -c1-200)"; }
fleet_wait() { local log; log=$(newest); [ -n "$log" ] && tail -c 60000 "$log" | grep -a "^graph:" | tail -1 | grep -q "waiting for the fleet"; }
service_since() { local log; log=$(newest); [ -n "$log" ] && tail -n +"$1" "$log"; }
try=${1:-1}; previous=""; other=""
while [ "$try" -le 12 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-13.log"; else log="$HOME/native-bootstrap-round-13-try-$try.log"; fi
  replica_up
  launcher_facts "before try $try"
  p12="$here/round-13.p12-try-$try.txt"; : > "$p12"; rm -f "$p12.stop"
  servicelog=$(newest); mark=1; [ -n "$servicelog" ] && mark=$(( $(wc -l < "$servicelog") + 1 ))
  echo "service log at start: $servicelog (line $mark on)" >> "$p12"; echo "closure entries before: $(entries)" >> "$p12"
  sampler "$p12" & samplerpid=$!
  printf '%s TRY %s start, log %s; service log %s from line %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$log" "$(basename "${servicelog:-none}")" "$mark" >> "$samples"
  bash "$here/../../native-bootstrap.sh" "$log" &
  runner=$!
  stopped=""; twos=0; fours=0; last=""; same=0
  while kill -0 "$runner" 2>/dev/null; do
    sleep 30
    pressure=$(sysctl -n kern.memorystatus_vm_pressure_level 2>/dev/null)
    printf '%s pressure=%s\n%s\n' "$(date -u +%H:%M:%SZ)" "$pressure" "$(ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-400)" >> "$samples"
    [ -n "$stopped" ] && continue
    if [ "${pressure:-1}" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    if [ "${pressure:-1}" -ge 4 ]; then fours=$((fours+1)); else fours=0; fi
    family=$(descendants "$runner" | tr '\n' ',' | sed 's/,$//')
    peak=0; [ -n "$family" ] && peak=$(ps -o rss= -p "$family" 2>/dev/null | sort -nr | head -1 | tr -d ' ')
    printf '  run peak rss=%s KiB\n' "${peak:-0}" >> "$samples"
    now=$(signature "$log"); if [ "$now" = "$last" ]; then same=$((same+1)); else same=0; last="$now"; fi
    reason=""
    if [ "$same" -ge 30 ] && ! fleet_wait; then
      reason="stall: no new progress line for 15 minutes"
      { printf '\n##### %s stall in try %s; last 60 lines of %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$try" "$(newest)"; tail -60 "$(newest)" | cut -c1-400; } >> "$here/round-13.stall.txt"
    fi
    [ "${peak:-0}" -gt 8388608 ] && reason="process of the run above 8 GB resident ($peak KiB)"
    [ "$fours" -ge 2 ] && reason="memory pressure level 4 in two samples in a row"
    [ "$twos" -ge 6 ] && reason="memory pressure level 2 in six samples in a row"
    if [ -n "$reason" ]; then
      stopped="$reason"
      printf '%s HARD STOP: %s\n' "$(date -u +%H:%M:%SZ)" "$reason" >> "$samples"
      for pid in $(descendants "$runner"); do kill -INT "$pid" 2>/dev/null; done
    fi
  done
  wait "$runner"
  touch "$p12.stop"; wait "$samplerpid" 2>/dev/null; rm -f "$p12.stop"
  echo "closure entries after: $(entries)" >> "$p12"
  result=$(tail -1 "$log")
  tasks=$(failed_tasks "$log")
  reference=$(grep -aoE "reference [0-9a-f]{8}" "$log" | tail -1 | cut -d" " -f2)
  printf '%s TRY %s end: %s; hard stop: %s; failed tasks: %s; reference: %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$result" "${stopped:-none}" "${tasks:-none}" "${reference:-none}" >> "$samples"
  launcher_facts "after try $try"
  [ "$result" = "native_rc=0 marker written" ] && break
  [ "$result" = "native_rc=4" ] && break
  if [ -n "$stopped" ] || [ "$result" = "native_rc=75" ]; then other=""; previous=""
  elif [ -n "$tasks" ] && grep -aqiE "source-differs|closure\.moving|retry" "$log"; then other=""; previous=""
  elif { [ "$(newest)" = "$servicelog" ] && service_since "$mark" | grep -aq "closure-missing"; } || { [ "$(newest)" != "$servicelog" ] && grep -aq "closure-missing" "$(newest)"; } || grep -aq "closure-missing" "$log"; then other=""; previous=""
  else
    now="${tasks:-}${reference:-}$(clean "$log" | grep -av '^end \|^native_rc' | tail -1 | sed -E 's/[0-9]+//g' | cut -c1-160)"
    [ "$now" = "$other" ] && break
    other="$now"
  fi
  try=$((try+1))
  [ "$try" -le 12 ] && sleep 180
done
echo "[DEBUG] round 13 finished"; grep -a 'TRY\|HARD STOP\|KICKSTART' "$samples"
