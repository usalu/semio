#!/bin/bash
# 🛎 Runs round 6 of the worker readiness protocol on g1-m01: up to eight tries of the native bootstrap with 30 s samples, the read-only macOS checks of ROUND-NOTE-P6.md after the first try, and the retry rules of the round (refusal, hard stop, source-differs, one graph-inference gc after a surviving ENOENT; no retry after exit 4 or the same task failures twice). Usage: run-round-6.sh [first try]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
worktree="$HOME/Documents/semio"
samples="$here/round-6.samples.txt"
checks="$here/round-6.checks.txt"
hostlogs="${TMPDIR%/}/semio-repo/$(id -u)"
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$worktree" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
clean() { LC_ALL=C sed $'s/\x1b\\[[0-9;]*[A-Za-z]//g' "$1" | LC_ALL=C grep -av '^\[script\] {'; }
refused() { [ "$(tail -1 "$1")" = "native_rc=75" ] && return 0; grep -aqiE "the fleet was asked for|waited for the fleet" "$1"; }
missing_closure() { local ref; ref=$(grep -aoE "reference [0-9a-f]{8}" "$1" | tail -1 | cut -d" " -f2); grep -aqE "ENOENT.*build-input-closures" "$1" && return 0; [ -n "$ref" ] && cat "$hostlogs"/host-*.log | grep -a "request-failed $ref" | grep -aq "ENOENT.*build-input-closures"; }
failed_tasks() { clean "$1" | awk '/Failed tasks:/{f=1; next} f && /^ *- /{print $2; next} f && NF && !/^ *- /{f=0}' | sort -u | tr '\n' ' '; }
descendants() { local pid; for pid in $(pgrep -P "$1"); do echo "$pid"; descendants "$pid"; done; }
section() { printf '\n##### %s\n' "$1" >> "$checks"; }
mac_checks() {
  printf '%s checks after try %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$1" >> "$checks"
  section "1: launchctl list | grep tech.semio.repo.service"; launchctl list | grep tech.semio.repo.service >> "$checks" 2>&1 || echo "(no line)" >> "$checks"
  for label in $(launchctl list | awk '/tech.semio.repo.service/{print $3}'); do section "1: launchctl print gui/$(id -u)/$label"; launchctl print "gui/$(id -u)/$label" 2>&1 | grep -E "program|arguments|working directory|state|last exit" >> "$checks"; launchctl print "gui/$(id -u)/$label" 2>&1 | awk '/arguments = \{/{f=1} f{print} /\}/{f=0}' | cut -c1-300 >> "$checks"; done
  section "1: SEMIO_SERVICE_LAUNCHD"; echo "${SEMIO_SERVICE_LAUNCHD:-(unset)}" >> "$checks"
  section "1: ps of service processes (pid, ppid, command)"; ps -axo pid,ppid,command | LC_ALL=C grep -aE "service host|bun --eval" | grep -v grep | cut -c1-300 >> "$checks"
  section "1: ls -l /usr/local/bin/repo"; ls -l /usr/local/bin/repo >> "$checks" 2>&1
  section "1: first lines of the newest service log"; newest=$(ls -t "$hostlogs"/host-*.log | head -1); echo "$newest" >> "$checks"; clean "$newest" | head -12 | cut -c1-300 >> "$checks"
  section "2: bootVersion"; grep bootVersion "$HOME/Library/Application Support/semio/fleet/entry/entry.json" >> "$checks" 2>&1
  section "2: boot chain"; ps -axo pid,ppid,command | LC_ALL=C grep -aE "boot.sh|boot.mjs|supervisor" | grep -v grep | cut -c1-300 >> "$checks"
  section "2: entry/shim.txt (last 12 lines)"; tail -12 "$HOME/Library/Application Support/semio/fleet/entry/shim.txt" 2>&1 | cut -c1-300 >> "$checks"
  section "4: access receipts"; ls "$HOME/Library/Application Support/semio/access/"*/ >> "$checks" 2>&1; cat "$HOME/Library/Application Support/semio/access/"*/service.json >> "$checks" 2>&1; echo >> "$checks"
  section "4: repo preflight --lang en"; (cd "$worktree" && PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" repo preflight --lang en) 2>&1 | clean /dev/stdin | cut -c1-500 >> "$checks"; echo "preflight exit ${PIPESTATUS[0]}" >> "$checks"
  section "codesign"; codesign -dvvv "$HOME/.local/share/semio/launcher/bin/repo" "$HOME/.local/bin/repo" 2>&1 | grep -E "Identifier|CDHash" >> "$checks"
}
consent_checks() {
  section "3: tcc-attribution --last 30m (after try $1)"
  python3 "$worktree/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️13/FLEET-ONE-MACHINE-SHARED-HOT-CACHE-AND-REMOTE-EXECUTION/🧰️scripts/P6/tcc-attribution.py" --last 30m 2>&1 | cut -c1-400 >> "$checks"
}
try=${1:-1}; previous=""; collected=""; checked=""
while [ "$try" -le 8 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-6.log"; else log="$HOME/native-bootstrap-round-6-try-$try.log"; fi
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
  [ -z "$checked" ] && { mac_checks "$try"; checked=1; }
  if [ "$result" = "native_rc=4" ] || grep -aqE "macOS (denied|does not let)|access-denied|e07192d7" "$log"; then consent_checks "$try"; fi
  [ "$result" = "native_rc=0 marker written" ] && break
  [ "$result" = "native_rc=4" ] && break
  pause=120
  if [ -n "$stopped" ]; then pause=180; previous=""
  elif refused "$log"; then previous=""
  elif grep -aq "source-differs" "$log"; then previous=""
  elif [ -n "$tasks" ]; then [ "$tasks" = "$previous" ] && break; previous="$tasks"
  elif missing_closure "$log" && [ -z "$collected" ]; then
    collected=1; previous=""
    printf '%s GC start: repo cache gc --area=graph-inference\n' "$(date -u +%H:%M:%SZ)" >> "$samples"
    (cd "$worktree" && PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" repo cache gc --area=graph-inference) > "$here/round-6.gc.txt" 2>&1
    printf '%s GC end: exit %s\n' "$(date -u +%H:%M:%SZ)" "$?" >> "$samples"
    pause=0
  else break; fi
  try=$((try+1))
  [ "$try" -le 8 ] && sleep "$pause"
done
echo "[DEBUG] round 6 finished"; grep -a 'TRY\|HARD STOP\|GC ' "$samples"
