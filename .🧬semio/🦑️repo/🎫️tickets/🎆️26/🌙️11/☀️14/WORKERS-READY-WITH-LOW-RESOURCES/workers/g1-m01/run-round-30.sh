#!/bin/bash
# 🪜 Runs round 30 of the worker readiness protocol on g1-m01 (rounds 14 to 30 folded): up to forty tries of the native bootstrap, five minutes apart (rounds 28 and 29: no try limit; forty bounds an unattended run), copying every `cut smaller` line and the highest `step k of 6` per try, each with the read-only slice worker sampler of ROUND-NOTE-P12.md (5 s) and 30 s samples of pressure and the three largest processes. Only hard stop: a process of the run above 8 GB resident. Stall: no new `graph:` line for 15 minutes during the graph step, no new progress line for 15 minutes after it. native_rc=4 ends the series only three times in a row. Copies per try the service log lines the rounds ask for and writes state.json after every try. No file name contains a colon. Usage: run-round-30.sh [first try]
set -u
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
worktree="$HOME/Documents/semio"
samples="$here/round-30.samples.txt"
launcherfacts="$here/round-30.launcher.txt"
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
  local out="$1" sampled=0 ts fts row pid rssmb fp last log
  while [ ! -e "$out.stop" ]; do
    ts=$(date -u +%FT%TZ); fts=$(date -u +%Y%m%dT%H%M%SZ); log=$(newest)
    row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | head -1)
    pid=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
    if [ -n "$pid" ]; then
      fp=$(footprint -p "$pid" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
      last=$(tail -n 600 "$log" 2>/dev/null | grep -aE 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
      echo "$ts pid=$pid rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$out"
      if [ "$rssmb" -gt 4096 ] && [ "$sampled" = 0 ]; then sampled=1; sample "$pid" 5 -file "${out%.txt}.sample-$fts.txt" >/dev/null 2>&1 & fi
    else
      echo "$ts no slice worker" >> "$out"
    fi
    sleep 5
  done
}
signature() { local log; log=$(newest); printf '%s|%s' "$(tail -c 30000 "$1" | clean /dev/stdin | grep -a "Reading the project graph slice\|Planning\|Loading\|nx run" | tail -1 | sed -E 's/\([0-9]+ s\)//; s/, [0-9]+, [0-9]+\)/)/g; s/[0-9]+ s[,)]//g' | cut -c1-200)" "$([ -n "$log" ] && tail -c 60000 "$log" | grep -a "^graph:" | tail -1 | cut -c1-200)"; }
fleet_wait() { local log; log=$(newest); [ -n "$log" ] && tail -c 60000 "$log" | grep -a "^graph:" | tail -1 | grep -q "waiting for the fleet"; }
service_since() { local log; log=$(newest); [ -n "$log" ] && tail -n +"$1" "$log"; }
try=${1:-1}; previous=""; other=""; rcfour=0
export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$worktree" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24
while [ "$try" -le 40 ]; do
  if [ "$try" = 1 ]; then log="$HOME/native-bootstrap-round-30.log"; else log="$HOME/native-bootstrap-round-30-try-$try.log"; fi
  replica_up
  launcher_facts "before try $try"
  p12="$here/round-30.p12-try-$try.txt"; : > "$p12"; rm -f "$p12.stop"
  servicelog=$(newest); mark=1; [ -n "$servicelog" ] && mark=$(( $(wc -l < "$servicelog") + 1 ))
  echo "service log at start: $servicelog (line $mark on)" >> "$p12"; echo "closure entries before: $(entries)" >> "$p12"
  sampler "$p12" & samplerpid=$!
  printf '%s TRY %s start, log %s; service log %s from line %s\n' "$(date -u +%H:%M:%SZ)" "$try" "$log" "$(basename "${servicelog:-none}")" "$mark" >> "$samples"
  bash "$here/../../native-bootstrap.sh" "$log" &
  runner=$!
  touch "$HOME/.round-30-try-start"
  stopped=""; twos=0; fours=0; last=""; same=0; holds=0; storewait=""
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
    if tail -c 3000 "$log" | clean /dev/stdin | tail -1 | grep -aqE "Reading the project graph slice|Waiting for the repo graph service|Starting the repo graph service"; then now="graph|$(newest)|$(grep -acE "^graph:|Inferring projects: [0-9]+ of" "$(newest)" 2>/dev/null)"; else now=$(signature "$log"); fi
    if [ "$now" = "$last" ]; then same=$((same+1)); else same=0; last="$now"; fi
    reason=""
    if tail -c 4000 "$log" | grep -aq "Another process holds the dependency store"; then holds=$((holds+1)); else holds=0; fi
    if [ "$holds" -ge 10 ]; then
      reason="dependency store wait of 5 minutes"; storewait=1
      { printf '\n##### %s dependency store wait in try %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$try"; tail -c 4000 "$log" | clean /dev/stdin | grep -a "Another process holds" | tail -1 | cut -c1-600; ps -axo pid,ppid,etime,command | grep -aE "bun|nx|repo" | grep -v grep | cut -c1-260; } >> "$here/round-30.store.txt"
    fi
    if [ "$same" -ge 30 ]; then
      reason="stall: no new graph: line (graph step) or progress line for 15 minutes"
      { printf '\n##### %s stall in try %s; last 60 lines of %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$try" "$(newest)"; tail -60 "$(newest)" | cut -c1-400; } >> "$here/round-30.stall.txt"
    fi
    [ "${peak:-0}" -gt 8388608 ] && reason="process of the run above 8 GB resident ($peak KiB)"
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
  { printf '\n##### try %s %s\n' "$try" "$result"; clean "$log" | tr '\r' '\n' | grep -a "cut smaller" | sed -E 's/\([0-9]+ s\)/(0 s)/' | awk '!seen[$0]++' | cut -c1-700; printf 'highest step: %s\n' "$(clean "$log" | tr '\r' '\n' | grep -aoE "step [0-9] of 6" | sort | tail -1)"; printf 'largest fleet wait: %s\n' "$(clean "$log" | tr '\r' '\n' | grep -aoE "waiting for the fleet: [0-9]+ of [0-9]+" | sort -t' ' -k5 -n | tail -1)"; printf 'slice worker peak rss: %s MB\n' "$(grep -aoE "rss=[0-9]+MB" "$p12" | tr -dc '0-9\n' | sort -n | tail -1)"; printf 'final lines:\n'; clean "$log" | tr '\r' '\n' | grep -av "^Reading the project graph slice ([0-9]* s)\.$" | tail -6 | cut -c1-900; } >> "$here/round-30.steps.txt"
  code=$(printf '%s' "$result" | grep -oE '[0-9]+' | head -1); ready=false; [ "$result" = "native_rc=0 marker written" ] && [ -f "$worktree/.cache/f/native-bootstrap.done" ] && ready=true
  failedjson=$(printf '%s' "$tasks" | tr ' ' '\n' | grep -v '^$' | sed 's/.*/"&"/' | paste -sd, -)
  printf '{"origin":"g1-m01","round":30,"try":%s,"native_rc":%s,"ready":%s,"failed":[%s],"at":"%s"}\n' "$try" "${code:-1}" "$ready" "$failedjson" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$here/state.json"
  if grep -aq "about 0 MB per project" "$log"; then { printf '%s try %s says 0 MB per project; vm_stat and memory_pressure:\n' "$(date -u +%H:%M:%SZ)" "$try"; vm_stat; memory_pressure | tail -3; } >> "$here/round-30.zero.txt"; fi
  [ "$result" = "native_rc=0 marker written" ] && break
  extract="$here/round-30.service-try-$try.txt"; : > "$extract"
  for f in $(find "$hostlogs" -name 'host-*.log' -newer "$HOME/.round-30-try-start" 2>/dev/null) "$servicelog"; do
    [ -f "$f" ] || continue; grep -qF "##### $f" "$extract" && continue
    from=1; [ "$f" = "$servicelog" ] && from=$mark
    { echo "##### $f from line $from"; tail -n +"$from" "$f" | grep -aE "access-proof|closure-missing|settled from|waiting for memory headroom|waiting for the fleet|plan stage|per project|taken from the fleet|inferred here|too heavy|cut smaller|step [0-9] of 6|restored |where: |EPERM|refused|request-failed|graph-plan|generation" | cut -c1-700 | awk '!seen[$0]++' | awk '{a[NR]=$0} END{if(NR<=300){for(i=1;i<=NR;i++)print a[i]}else{for(i=1;i<=150;i++)print a[i]; print "… " NR-300 " lines left out …"; for(i=NR-149;i<=NR;i++)print a[i]}}'; } >> "$extract"
    if tail -n +"$from" "$f" | grep -aq "EPERM"; then { echo "--- EPERM lines with the 20 lines before:"; tail -n +"$from" "$f" | grep -a -B20 "EPERM" | cut -c1-700 | tail -120; } >> "$extract"; fi
  done
  if grep -aq "EPERM\|denied access" "$extract" "$log"; then { echo "--- after the access failure:"; ls -laO@ "$worktree/.cache/b" | head -20; echo "mount | grep -c semio: $(mount | grep -c semio)"; ps -axo pid,ppid,etime,command | grep "service host" | grep -v grep | cut -c1-300; } >> "$extract" 2>&1; fi
  if [ "$result" = "native_rc=1" ] || [ "$result" = "native_rc=4" ]; then { printf '\n##### try %s %s: last 40 lines of the bootstrap log\n' "$try" "$result"; clean "$log" | tail -40 | cut -c1-900; } >> "$here/round-30.bootstrap-tails.txt"; fi
  if [ "$result" = "native_rc=4" ]; then rcfour=$((rcfour+1)); else rcfour=0; fi
  [ "$rcfour" -ge 3 ] && break
  if [ -n "$stopped" ] || [ -n "$storewait" ] || [ "$result" = "native_rc=75" ] || [ "$result" = "native_rc=4" ] || clean "$log" | tail -8 | grep -aqiE "denied access|EPERM|try again|kept changing"; then other=""; previous=""
  elif [ -n "$tasks" ] && grep -aqiE "source-differs|closure\.moving|retry" "$log"; then other=""; previous=""
  elif { [ "$(newest)" = "$servicelog" ] && service_since "$mark" | grep -aq "closure-missing"; } || { [ "$(newest)" != "$servicelog" ] && grep -aq "closure-missing" "$(newest)"; } || grep -aq "closure-missing" "$log"; then other=""; previous=""
  else
    now="${tasks:-}${reference:-}$(clean "$log" | grep -av '^end \|^native_rc' | tail -1 | sed -E 's/[0-9]+//g' | cut -c1-160)"
    [ "$now" = "$other" ] && break
    other="$now"
  fi
  try=$((try+1))
  [ "$try" -le 40 ] && sleep 300
done
echo "[DEBUG] round 30 finished"; grep -a 'TRY\|HARD STOP\|KICKSTART' "$samples"
