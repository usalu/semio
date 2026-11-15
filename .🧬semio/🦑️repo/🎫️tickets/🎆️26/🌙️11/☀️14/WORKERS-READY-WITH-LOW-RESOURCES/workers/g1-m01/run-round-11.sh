#!/bin/bash
# 🛎 Runs round 11 of the worker readiness protocol on g1-m01: up to ten tries of the native bootstrap with 30 s samples, launcher identity (inode, mtime, link, CDHash) before and after every try, one SIGINT after 15 minutes without a new progress line outside a fleet wait, and the retry rules and hard stops of the round (exit 4 and reference 3420bc16 end the round; a store or cache that is not reachable waits 5 minutes and does not count). A missing build-input-closures record is tried again once, then once more after one graph-inference gc (rule of round 6). Usage: run-round-11.sh [first try] [closure failures so far]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
worktree="$HOME/Documents/semio"
samples="$here/round-11.samples.txt"
launcherfacts="$here/round-11.launcher.txt"
consent="$here/round-11.consent.txt"
launcher="$HOME/.local/share/semio/launcher/bin/repo"
access="$HOME/Library/Application Support/semio/access"
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$worktree" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
clean() { LC_ALL=C sed $'s/\x1b\\[[0-9;]*[A-Za-z]//g' "$1" | LC_ALL=C grep -av '^\[script\] {'; }
refused() { [ "$(tail -1 "$1")" = "native_rc=75" ] && return 0; grep -aqiE "the fleet was asked for|waited for the fleet" "$1"; }
failed_tasks() { clean "$1" | awk '/Failed tasks:/{f=1; next} f && /^ *- /{print $2; next} f && NF && !/^ *- /{f=0}' | sort -u | tr '\n' ' '; }
descendants() { local pid; for pid in $(pgrep -P "$1"); do echo "$pid"; descendants "$pid"; done; }
launcher_facts() {
  printf '\n##### %s %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" >> "$launcherfacts"
  ls -li "$launcher" "$HOME/.local/bin/repo" >> "$launcherfacts" 2>&1
  [ -L "$launcher" ] && { echo "readlink: $(readlink "$launcher")"; ls -liL "$launcher"; } >> "$launcherfacts" 2>&1
  codesign -dvvv "$launcher" 2>&1 | grep -E "^(Identifier|CDHash)" >> "$launcherfacts"
}
replica_up() { if launchctl print "gui/$(id -u)/tech.semio.fleet.replica" 2>&1 | grep -qE "^[[:space:]]*state = not running"; then printf '%s REPLICA KICKSTART (state = not running)\n' "$(date -u +%H:%M:%SZ)" >> "$samples"; launchctl kickstart -k "gui/$(id -u)/tech.semio.fleet.replica" >> "$samples" 2>&1; sleep 60; fi; }
receipts() { for file in "$access"/*/launcher.json "$access"/*/service.json "$access"/*/client.json; do [ -f "$file" ] && printf '%s ' "$(basename "$file" .json)=$(grep -oE '"state":"[a-z]+"' "$file" | cut -d'"' -f4)"; done; }
hostlogs="${TMPDIR%/}/semio-repo/$(id -u)"
signature() { local newest; newest=$(ls -t "$hostlogs"/host-*.log 2>/dev/null | head -1); printf '%s|%s' "$(tail -c 30000 "$1" | clean /dev/stdin | grep -a "Reading the project graph slice\|Planning\|Loading\|Running\|nx run" | tail -1 | sed -E 's/\([0-9]+ s\)//; s/, [0-9]+, [0-9]+\)/)/g; s/[0-9]+ s[,)]//g' | cut -c1-200)" "$([ -n "$newest" ] && tail -c 60000 "$newest" | grep -a "^graph:" | tail -1 | cut -c1-200)"; }
fleet_wait() { local newest; newest=$(ls -t "$hostlogs"/host-*.log 2>/dev/null | head -1); [ -n "$newest" ] && tail -c 60000 "$newest" | grep -a "^graph:" | tail -1 | grep -q "waiting for the fleet"; }
stall_facts() {
  local newest; newest=$(ls -t "$hostlogs"/host-*.log | head -1)
  { printf '\n##### %s stall in try %s; last 60 lines of %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" "$newest"; tail -60 "$newest" | cut -c1-400; printf '\n##### ps of the service and its children (pid, ppid, etime, rss, command)\n'; ps -axo pid,ppid,etime,rss,args | LC_ALL=C grep -aE "service host|bun --eval|bun -- " | grep -v grep | cut -c1-300; } >> "$here/round-11.stall.txt"
}
missing_closure() { local ref; ref=$(grep -aoE "reference [0-9a-f]{8}" "$1" | tail -1 | cut -d" " -f2); [ -n "$ref" ] && cat "$hostlogs"/host-*.log | grep -a "request-failed $ref" | grep -aq "ENOENT.*build-input-closures"; }
try=${1:-1}; previous=""; free=0; closures=${2:-0}
while [ "$try" -le 10 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-11.log"; else log="$HOME/native-bootstrap-round-11-try-$try.log"; fi
  replica_up
  launcher_facts "before try $try"
  printf '%s TRY %s start, log %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$log" >> "$samples"
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
    [ "$same" -ge 30 ] && ! fleet_wait && { reason="stall: no new progress line for 15 minutes"; stall_facts "$try"; }
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
  result=$(tail -1 "$log")
  tasks=$(failed_tasks "$log")
  printf '%s TRY %s end: %s; hard stop: %s; failed tasks: %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$result" "${stopped:-none}" "${tasks:-none}" >> "$samples"
  launcher_facts "after try $try"
  [ "$result" = "native_rc=0 marker written" ] && break
  pause=120
  [ "$result" = "native_rc=4" ] && break
  grep -aq "3420bc16" "$log" && break
  if grep -aqiE "store (is )?not reachable|timeout of the cache|cache.*timed out|store.*unreachable" "$log" && [ -z "$stopped" ] && [ "$free" -lt 3 ]; then free=$((free+1)); printf '%s try %s not counted (store or cache not reachable); waiting 5 minutes\n' "$(date -u +%H:%M:%SZ)" "$try" >> "$samples"; mv "$log" "$log.not-counted-$free"; sleep 300; continue
  elif [ -n "$stopped" ]; then pause=180; previous=""
  elif refused "$log"; then previous=""
  elif grep -aqiE "source-differs|closure\.moving|retry" "$log" && [ -n "$tasks" ]; then previous=""
  elif [ -n "$tasks" ]; then [ "$tasks" = "$previous" ] && break; previous="$tasks"
  elif missing_closure "$log" && [ "$closures" -lt 2 ]; then
    closures=$((closures+1)); previous=""
    if [ "$closures" = 2 ]; then
      printf '%s GC start: repo cache gc --area=graph-inference\n' "$(date -u +%H:%M:%SZ)" >> "$samples"
      (cd "$worktree" && PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" repo cache gc --area=graph-inference) > "$here/round-11.gc.txt" 2>&1
      printf '%s GC end: exit %s\n' "$(date -u +%H:%M:%SZ)" "$?" >> "$samples"
      pause=0
    fi
  else break; fi
  try=$((try+1))
  [ "$try" -le 10 ] && sleep "$pause"
done
echo "[DEBUG] round 11 finished"; grep -a 'TRY\|HARD STOP\|KICKSTART\|GC \|not counted' "$samples"
