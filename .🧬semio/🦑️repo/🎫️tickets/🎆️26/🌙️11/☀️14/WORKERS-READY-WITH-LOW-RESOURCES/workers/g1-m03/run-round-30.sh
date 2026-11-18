#!/bin/bash
# 🛰️ Runs protocol round 30 on this worker: once `repo tools setup all` (step 0 again, as rounds 19 to 28 ask of machines where it has not ended with exit 0), then tries of W/native-bootstrap.sh five minutes apart without a try limit, each in its own process group, with the 30 s samples and the P12 sampler. Stop rules: a process of the run above 8 GB resident (one SIGINT); no new progress line and no new graph line for 15 minutes (one SIGINT); the dependency store wait above 5 minutes (one SIGINT). The series ends on native_rc=0, on three native_rc=4 in a row, or on two tries in a row with the same other error. Usage: run-round-30.sh <first-try> [skip-tools]
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-30.monitor.txt"; export TZ=UTC
wt="$HOME/Documents/semio"; sd="${TMPDIR%/}/semio-repo/$(id -u)"
strip(){ sed 's/\x1b\[[0-9;]*m//g' "$@"; }
newestlog(){ ls -t "$sd"/host-*.log 2>/dev/null | head -1; }
closures(){ find "$wt"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' '; }
now(){ date -u +%Y-%m-%dT%H:%M:%SZ; }
tools_setup(){
  tlog="$HOME/tools-setup-round-30.log"; rep="$here/round-30.tools-setup.txt"; : > "$tlog"
  echo "step 0 again in round 30: repo tools setup all, started $(now) from $wt (environment as in W/native-bootstrap.sh)" > "$rep"; t0=$(date +%s)
  ( cd "$wt" && export SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$wt" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24 PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" && unset SEMIO_REMOTE_CACHE_ENDPOINT SEMIO_REMOTE_CACHE_ACCESS SEMIO_REMOTE_CACHE_INSTANCE SEMIO_REMOTE_CACHE_TOKEN SEMIO_FLEET_CACHE_ENDPOINT && exec perl -e 'setpgrp(0,0); exec @ARGV' bash -c 'repo tools setup all 2>&1 | tee "$0"; echo "tools_rc=${PIPESTATUS[0]}" >> "$0"' "$tlog" ) < /dev/null > /dev/null 2>&1 &
  tp=$!; echo "step 0 (repo tools setup all) started pid=$tp $(now)" >> "$note"; sent=0
  while kill -0 "$tp" 2>/dev/null; do
    w=$(strip "$tlog" | grep -a 'holds the dependency store' | tail -1); ws=$(printf '%s' "$w" | grep -o -E '\(([0-9]+) s of' | grep -o -E '[0-9]+')
    if [ "$sent" = 0 ] && [ -n "$ws" ] && [ "$ws" -ge 120 ]; then
      { echo "-- dependency store wait of $ws s at $(now); the wait line:"; echo "$w"; echo "-- ps -axo pid,ppid,etime,command | grep -E \"bun|repo\""; ps -axo pid,ppid,etime,command | grep -E "bun|repo" | grep -v -e grep -e Claude -e claude | sed -E 's#/Users/ueli/Library/Caches/semio/runtimes/bun/[0-9a-f]+/bun#bun#' | cut -c1-420; echo "-- one SIGINT to process group $tp"; } >> "$rep" 2>&1
      kill -INT -- "-$tp"; sent=1
    fi
    sleep 10
  done
  t1=$(date +%s); trc=$(grep -a -o -E '^tools_rc=[0-9]+' "$tlog" | tail -1 | cut -d= -f2)
  { echo "ended $(now), exit code ${trc:-unknown (interrupted)}, wall time $((t1-t0)) s, SIGINT sent: $sent"; echo "-- per-scope restored / installed lines"; strip "$tlog" | grep -a -i -E 'restored|installed' | grep -a -v '^\[script\] {' | awk '!s[$0]++' | cut -c1-400 | head -40; echo "-- last 20 lines of ~/tools-setup-round-30.log (without JSON progress lines)"; strip "$tlog" | grep -a -v -e '^\[script\] {' -e '^tools_rc=' | tail -20 | cut -c1-500; } >> "$rep"
  echo "step 0 ended $(now) exit=${trc:-interrupted} wall=$((t1-t0))s sigint=$sent" >> "$note"
}
k=$1; previous=""; fours=0
[ "${2:-}" = skip-tools ] || tools_setup
while [ "$k" -le 60 ]; do
  if [ "$k" = 1 ]; then log="$HOME/native-bootstrap-round-30.log"; else log="$HOME/native-bootstrap-round-30-try-$k.log"; fi
  samples="$here/round-30.try-$k.samples.txt"; mark="$here/.round-30.try-$k.start"; out="$here/round-30.try-$k.p12-samples.txt"
  : > "$samples"; touch "$mark"; : > "$out"; rm -f "$out.stop"
  echo "service log: $(newestlog)" >> "$out"; echo "closure entries before: $(closures)" >> "$out"
  for s in "$sd"/host-*.log; do [ -f "$s" ] && echo "$(wc -l < "$s" | tr -d ' ') $s"; done > "$here/.round-30.try-$k.before"
  (
    sampled=0
    while [ ! -e "$out.stop" ]; do
      ts=$(date -u +%FT%TZ); slog=$(newestlog)
      row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | sort -k2,2nr | head -1)
      p=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
      if [ -n "$p" ]; then
        fp=$(footprint -p "$p" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
        last=$(tail -n 600 "$slog" 2>/dev/null | grep -E 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
        echo "$ts pid=$p rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$out"
        if [ "$rssmb" -gt 4096 ] && [ "$sampled" = 0 ]; then sampled=1; sample "$p" 5 -file "$out.sample-$(date -u +%Y%m%dT%H%M%SZ).txt" >/dev/null 2>&1 & fi
      else echo "$ts no slice worker" >> "$out"; fi
      sleep 5
    done
  ) &
  sampler=$!
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(now) log=$log" >> "$note"
  stopped=0; same=0; lastsig=""; twos=0; fourlvl=0
  while kill -0 "$pid" 2>/dev/null; do
    level=$(sysctl -n kern.memorystatus_vm_pressure_level)
    { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-600; } >> "$samples"
    sigline=$( { strip "$log" | grep -a -o -E 'owners [0-9]+ of [0-9]+|^(✔|✖|>) +nx run [^ ]+|Inferring projects: [0-9]+ of [0-9]+|step [0-9] of 6|^\[semio\] .{0,60}' | tail -1; strip "$log" | grep -a -v -e 'remains active' -e 'still running' -e 'still working' | wc -l | tr -d ' '; for s in "$sd"/host-*.log; do [ "$s" -nt "$mark" ] && grep -a -c '^graph: ' "$s"; done; } | tr '\n' '|')
    if [ "$sigline" = "$lastsig" ]; then same=$((same+1)); else same=0; lastsig="$sigline"; fi
    if [ "$stopped" = 0 ]; then
      w=$(strip "$log" | grep -a 'holds the dependency store' | tail -1); ws=$(printf '%s' "$w" | grep -o -E '\(([0-9]+) s of' | grep -o -E '[0-9]+')
      top=$(ps -axo pgid,rss | awk -v g="$pid" '$1==g && $2>m {m=$2} END {print m+0}')
      if [ -n "$ws" ] && [ "$ws" -ge 300 ]; then
        echo "try $k dependency store wait of $ws s $(now): recording the processes, one SIGINT to process group $pid" >> "$note"
        { echo "== try $k dependency store wait, $(now)"; strip "$log" | grep -a 'holds the dependency store' | head -1; echo "$w"; ps -axo pid,ppid,etime,command | grep -E 'bun|nx|repo' | grep -v -e grep -e Claude -e claude | sed -E 's#/Users/ueli/Library/Caches/semio/runtimes/bun/[0-9a-f]+/bun#bun#' | cut -c1-420; } >> "$here/round-30.store-wait.txt" 2>&1
        kill -INT -- "-$pid"; stopped=3
      elif [ "$top" -gt 8388608 ]; then echo "try $k hard stop $(now): process of the run above 8 GB resident ($top KiB); one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1
      elif [ "$same" -ge 30 ]; then
        echo "try $k no new progress line and no new graph line for 15 minutes (last: $sigline) $(now); one SIGINT to process group $pid" >> "$note"
        { echo "== try $k stalled $(now), last progress: $sigline"; for s in "$sd"/host-*.log; do [ "$s" -nt "$mark" ] && { echo "-- last 60 lines of $s"; tail -60 "$s" | cut -c1-500; }; done; echo "-- last 20 lines of the bootstrap log"; strip "$log" | grep -a -v '^\[script\] {' | tail -20 | cut -c1-400; } >> "$here/round-30.stall.txt" 2>&1
        kill -INT -- "-$pid"; stopped=2
      fi
    fi
    sleep 30
  done
  touch "$out.stop"; wait "$sampler" 2>/dev/null; echo "closure entries after: $(closures)" >> "$out"
  cleanlog="$here/.round-30.try-$k.clean"; strip "$log" | grep -a -v '^\[script\] {' > "$cleanlog"
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$cleanlog" | tail -1 | cut -d= -f2)
  svc="$here/round-30.try-$k.service.txt"; : > "$svc"
  for s in "$sd"/host-*.log; do
    [ -f "$s" ] && [ "$s" -nt "$mark" ] || continue
    b=$(grep -F " $s" "$here/.round-30.try-$k.before" | awk '{print $1}'); b=${b:-0}
    { echo "== $s (lines after $b)"; tail -n +"$((b+1))" "$s" | grep -a -h -E 'access-proof|where: root=|launchd route|closure-missing|settled|graph-plan-settled|graph-plan-too-heavy|cut smaller|graph: plan stage|graph: refreshing|graph: [0-9]+ stale projects|memory readings|taken from the fleet|inferred here|too heavy|restored|nothing to wait|needs [0-9]+ MB' | awk '!s[$0]++' | cut -c1-900 | head -400; echo "-- waiting for the fleet: first and last line, largest x"; tail -n +"$((b+1))" "$s" | grep -a 'waiting for the fleet' | head -1 | cut -c1-300; tail -n +"$((b+1))" "$s" | grep -a 'waiting for the fleet' | tail -1 | cut -c1-300; echo "largest x: $(tail -n +"$((b+1))" "$s" | grep -a -o -E 'waiting for the fleet: [0-9]+ of [0-9]+' | awk '{print $5" of "$7}' | sort -n | tail -1)"; echo "-- waiting for memory headroom: first and last line"; tail -n +"$((b+1))" "$s" | grep -a 'waiting for memory headroom' | head -1 | cut -c1-300; tail -n +"$((b+1))" "$s" | grep -a 'waiting for memory headroom' | tail -1 | cut -c1-300; echo "-- request-failed entries with 12 lines after"; tail -n +"$((b+1))" "$s" | grep -a -A12 'request-failed' | cut -c1-500 | head -60; echo "-- last 30 lines when the try ended"; tail -30 "$s" | cut -c1-500; } >> "$svc"
  done
  { echo "-- bootstrap log: cut smaller lines"; grep -a -E 'cut smaller' "$cleanlog" | awk '!s[$0]++' | cut -c1-600; echo "highest step: $(grep -a -o -E 'step [0-9] of 6' "$cleanlog" | sort | tail -1)"; echo "-- bootstrap log: last 40 lines"; tail -40 "$cleanlog" | cut -c1-500; } >> "$svc"
  failed=$(awk '/^Failed tasks:/{f=1;next} f&&/^- /{print $2} f&&/^$/{n++} f&&n>1{exit}' "$cleanlog" | sort | tr '\n' ' ')
  other=$(grep -a -E '^Internal error|unexpected failure|could not be read' "$cleanlog" | tail -1 | sed -E 's/reference [0-9a-f]+//; s/[0-9]+ MB/N MB/g; s/[0-9]+ s/N s/g' | cut -c1-140)
  flist=$(printf '%s' "$failed" | awk '{for(i=1;i<=NF;i++) printf "%s\"%s\"", (i>1?",":""), $i}')
  printf '{"origin":"g1-m03","round":30,"try":%s,"native_rc":%s,"ready":false,"failed":[%s],"at":"%s"}\n' "$k" "${rc:-1}" "$flist" "$(now)" > "$here/state.json"
  echo "try $k ended $(now) native_rc=${rc:-unknown} stopped=$stopped failed=[$failed] other=[$other]" >> "$note"
  [ "${rc:-1}" = 0 ] && break
  if [ "${rc:-1}" = 4 ]; then fours=$((fours+1)); [ "$fours" -ge 3 ] && { echo "try $k: three native_rc=4 in a row: series ends" >> "$note"; break; }; else fours=0; fi
  if [ "$stopped" = 0 ] && [ "${rc:-1}" != 75 ] && [ "${rc:-1}" != 4 ] && ! grep -a -q -i -E 'source-differs|closure\.moving|closure-missing|retry|try again|run the command again' "$cleanlog"; then
    sigerr="$failed|$other"
    if [ -n "$sigerr" ] && [ "$sigerr" = "$previous" ]; then echo "try $k: the same other error as the try before: series ends" >> "$note"; break; fi
    previous="$sigerr"
  else previous=""; fi
  echo "try $k: next try after 300 s" >> "$note"; sleep 300; k=$((k+1))
done
echo "series ended $(now)" >> "$note"
