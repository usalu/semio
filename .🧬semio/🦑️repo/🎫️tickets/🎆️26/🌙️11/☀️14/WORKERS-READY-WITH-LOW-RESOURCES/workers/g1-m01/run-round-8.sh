#!/bin/bash
# 🛎 Runs round 8 of the worker readiness protocol on g1-m01: up to ten tries of the native bootstrap with 30 s samples, launcher identity (inode, mtime, link, CDHash) before and after every try, a wait of at most 30 minutes for the owner's file access consent after exit 4, and the retry rules and hard stops of the round. Usage: run-round-8.sh [first try]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
worktree="$HOME/Documents/semio"
samples="$here/round-8.samples.txt"
launcherfacts="$here/round-8.launcher.txt"
consent="$here/round-8.consent.txt"
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
consent_wait() {
  printf '\n##### %s consent wait after try %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" >> "$consent"
  clean "$2" | grep -aE "macOS|consent|Allow" | awk '!seen[$0]++' >> "$consent"
  python3 "$worktree/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/P6/tcc-attribution.py" --last 30m 2>&1 | cut -c1-400 >> "$consent"
  printf '{"origin":"g1-m01","round":8,"native_rc":4,"ready":false,"failed":[],"consent":"consent","at":"%s"}\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$here/state.json"
  local minute=0 output code line states
  while [ "$minute" -lt 30 ]; do
    output=$( (cd "$worktree" && PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" repo preflight --lang en) 2>&1 ); code=$?
    line=$(printf '%s\n' "$output" | clean /dev/stdin | grep -aiE "access" | head -3 | cut -c1-300 | tr '\n' ' ')
    states=$(receipts)
    printf '%s preflight exit %s; access line: %s; receipts: %s\n' "$(date -u +%H:%M:%SZ)" "$code" "${line:-(none printed)}" "$states" >> "$consent"
    if [ "$code" = 0 ] && ! printf '%s' "$line" | grep -qiE "denied|waiting" && ! printf '%s' "$states" | grep -qE "waiting|denied"; then printf '%s access ok\n' "$(date -u +%H:%M:%SZ)" >> "$consent"; return 0; fi
    minute=$((minute+1)); sleep 60
  done
  printf '%s no consent within 30 minutes; next try runs anyway\n' "$(date -u +%H:%M:%SZ)" >> "$consent"
}
try=${1:-1}; previous=""
while [ "$try" -le 10 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-8.log"; else log="$HOME/native-bootstrap-round-8-try-$try.log"; fi
  replica_up
  launcher_facts "before try $try"
  printf '%s TRY %s start, log %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$log" >> "$samples"
  bash "$here/../../native-bootstrap.sh" "$log" &
  runner=$!
  stopped=""; twos=0; fours=0
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
    reason=""
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
  if [ "$result" = "native_rc=4" ]; then consent_wait "$try" "$log"; previous=""; pause=0
  elif [ -n "$stopped" ]; then pause=180; previous=""
  elif refused "$log"; then previous=""
  elif grep -aqiE "source-differs|closure\.moving|retry" "$log" && [ -n "$tasks" ]; then previous=""
  elif [ -n "$tasks" ]; then [ "$tasks" = "$previous" ] && break; previous="$tasks"
  else break; fi
  try=$((try+1))
  [ "$try" -le 10 ] && sleep "$pause"
done
echo "[DEBUG] round 8 finished"; grep -a 'TRY\|HARD STOP\|KICKSTART' "$samples"
