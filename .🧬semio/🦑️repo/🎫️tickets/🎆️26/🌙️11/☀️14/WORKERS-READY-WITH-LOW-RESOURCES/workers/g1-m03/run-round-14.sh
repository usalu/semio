#!/bin/bash
# 🛰️ Runs protocol round 14 on this worker from try <first> on: up to twelve tries of W/native-bootstrap.sh, 3 minutes apart, each in its own process group, with the P12 sampler (slice worker every 5 s), the 30 s samples, the hard stops of round 6 and the 15 minute stall rule. Next try after native_rc=75, a hard stop, a stall, task failures with source-differs, closure.moving or retry wording, or closure-missing; stop after exit 0, exit 4 or two tries in a row with the same other error. Usage: run-round-14.sh <first-try>
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-14.monitor.txt"; export TZ=UTC
wt="$HOME/Documents/semio"; sd="${TMPDIR%/}/semio-repo/$(id -u)"
newestlog(){ ls -t "$sd"/host-*.log 2>/dev/null | head -1; }
closures(){ find "$wt"/.cache/b/g/m/inference/selections/*/ce/e -type f 2>/dev/null | wc -l | tr -d ' '; }
k=$1; previous=""
while [ "$k" -le 12 ]; do
  if [ "$k" = 1 ]; then log="$HOME/native-bootstrap-round-14.log"; else log="$HOME/native-bootstrap-round-14-try-$k.log"; fi
  samples="$here/round-14.try-$k.samples.txt"; : > "$samples"; mark="$here/.round-14.try-$k.start"; touch "$mark"
  out="$here/round-14.try-$k.p12-samples.txt"; : > "$out"; rm -f "$out.stop"
  echo "service log: $(newestlog)" >> "$out"; echo "closure entries before: $(closures)" >> "$out"
  for s in "$sd"/host-*.log; do [ -f "$s" ] && echo "$(wc -l < "$s" | tr -d ' ') $s"; done > "$here/.round-14.try-$k.before"
  (
    sampled=0
    while [ ! -e "$out.stop" ]; do
      ts=$(date -u +%FT%TZ); slog=$(newestlog)
      row=$(ps -axo pid=,rss=,args= | grep -F slice-pool | grep -F worker.ts | grep -v grep | head -1)
      p=$(echo "$row" | awk '{print $1}'); rssmb=$(echo "$row" | awk '{printf "%d", $2/1024}')
      if [ -n "$p" ]; then
        fp=$(footprint -p "$p" 2>/dev/null | grep -m1 -i 'footprint' | sed 's/^ *//' | cut -c1-120)
        last=$(tail -n 600 "$slog" 2>/dev/null | grep -E 'Reading the project graph slice|graph: plan stage|Graph: Plan-Stufe|^graph: ' | tail -1 | cut -c1-220)
        echo "$ts pid=$p rss=${rssmb}MB footprint=[$fp] last=[$last]" >> "$out"
        if [ "$rssmb" -gt 4096 ] && [ "$sampled" = 0 ]; then sampled=1; sample "$p" 5 -file "$out.sample-$ts.txt" >/dev/null 2>&1 & fi
      else echo "$ts no slice worker" >> "$out"; fi
      sleep 5
    done
  ) &
  sampler=$!
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ) log=$log" >> "$note"
  stopped=0; twos=0; fours=0; same=0; lastsig=""
  while kill -0 "$pid" 2>/dev/null; do
    level=$(sysctl -n kern.memorystatus_vm_pressure_level)
    { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-600; } >> "$samples"
    if [ "$level" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    if [ "$level" = 4 ]; then fours=$((fours+1)); else fours=0; fi
    sigline=$( { sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -o -E 'owners [0-9]+ of [0-9]+|^(✔|✖|>) +nx run [^ ]+|Inferring projects: [0-9]+ of [0-9]+|^\[semio\] [A-Za-z:]+' | tail -1; for s in "$sd"/host-*.log; do [ "$s" -nt "$mark" ] && grep -a -o -E 'waiting for the fleet: [0-9]+ of [0-9]+|[0-9]+ of [0-9]+ projects inferred|graph: plan stage [a-z-]+ (starts|took)|restored [^ ]+' "$s" | tail -1; done; } | tr '\n' '|')
    if [ "$sigline" = "$lastsig" ]; then same=$((same+1)); else same=0; lastsig="$sigline"; fi
    if [ "$stopped" = 0 ]; then
      w=$(sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a 'holds the dependency store' | tail -1)
      ws=$(printf '%s' "$w" | grep -o -E '\(([0-9]+) s of' | grep -o -E '[0-9]+')
      if [ -n "$ws" ] && [ "$ws" -ge 300 ]; then
        echo "try $k dependency store wait of $ws s $(date -u +%Y-%m-%dT%H:%M:%SZ): recording the processes, one SIGINT to process group $pid" >> "$note"
        { echo "== try $k dependency store wait, $(date -u +%Y-%m-%dT%H:%M:%SZ)"; echo "-- the full wait lines (first, last)"; sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a 'holds the dependency store' | head -1; echo "$w"; echo "-- ps -axo pid,ppid,etime,command of every bun/nx/repo process"; ps -axo pid,ppid,etime,command | grep -E 'bun|nx|repo' | grep -v -e grep -e Claude -e claude | sed -E 's#/Users/ueli/Library/Caches/semio/runtimes/bun/[0-9a-f]+/bun#bun#' | cut -c1-420; } >> "$here/round-14.store-wait.txt" 2>&1
        kill -INT -- "-$pid"; stopped=3
      fi
    fi
    if [ "$stopped" = 0 ]; then
      reason=""
      top=$(ps -axo pgid,rss | awk -v g="$pid" '$1==g && $2>m {m=$2} END {print m+0}')
      [ "$top" -gt 8388608 ] && reason="process of the run above 8 GB resident ($top KiB)"
      [ "$fours" -ge 2 ] && reason="memory pressure level 4 in two samples in a row"
      [ "$twos" -ge 6 ] && reason="memory pressure level 2 in six samples in a row"
      if [ -n "$reason" ]; then echo "try $k hard stop $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1
      elif [ "$same" -ge 30 ]; then
        echo "try $k no new progress line for 15 minutes (last: $sigline) $(date -u +%Y-%m-%dT%H:%M:%SZ); one SIGINT to process group $pid" >> "$note"
        { echo "== try $k stalled $(date -u +%Y-%m-%dT%H:%M:%SZ), last progress: $sigline"; for s in "$sd"/host-*.log; do [ "$s" -nt "$mark" ] && { echo "-- last 60 lines of $s"; tail -60 "$s" | cut -c1-500; }; done; echo "-- last 20 lines of the bootstrap log"; sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -v '^\[script\] {' | tail -20 | cut -c1-400; } >> "$here/round-14.stall.txt" 2>&1
        kill -INT -- "-$pid"; stopped=2
      fi
    fi
    sleep 30
  done
  touch "$out.stop"; wait "$sampler" 2>/dev/null; echo "closure entries after: $(closures)" >> "$out"
  bash "$here/launcher-facts.sh" "after try $k" >> "$here/round-14.launcher.txt" 2>&1
  cleanlog="$here/.round-14.try-$k.clean"; sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -v '^\[script\] {' > "$cleanlog"
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$cleanlog" | tail -1 | cut -d= -f2)
  svc="$here/round-14.try-$k.service.txt"; : > "$svc"
  for s in "$sd"/host-*.log; do
    [ -f "$s" ] && [ "$s" -nt "$mark" ] || continue
    b=$(grep -F " $s" "$here/.round-14.try-$k.before" | awk '{print $1}'); b=${b:-0}
    { echo "== $s (lines after $b)"; tail -n +"$((b+1))" "$s" | grep -a -h -E 'graph: plan stage|Graph: Plan-Stufe|graph-plan-too-heavy|graph: refreshing|graph: [0-9]+ stale projects|taken from the fleet|inferred here|too heavy|restored|\[graph-phase\]|closure-missing|closureRepaired|waiting for the fleet|nothing to wait|request-failed|where: root=' | awk '!s[$0]++' | cut -c1-900; echo "-- last 30 lines when the try ended"; tail -30 "$s" | cut -c1-500; } >> "$svc"
  done
  failed=$(awk '/^Failed tasks:/{f=1;next} f&&/^- /{print $2} f&&/^$/{n++} f&&n>1{exit}' "$cleanlog" | sort | tr '\n' ' ')
  retryword=0; [ -n "$failed" ] && grep -a -q -i -E 'source-differs|closure\.moving|retry|try again|run the command again' "$cleanlog" && retryword=1
  missing=0; grep -a -q 'closure-missing' "$cleanlog" "$svc" 2>/dev/null && missing=1
  other=$(grep -a -E '^Internal error|unexpected failure' "$cleanlog" | tail -1 | sed -E 's/reference [0-9a-f]+//' | cut -c1-160)
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=${rc:-unknown} stopped=$stopped retry_wording=$retryword closure_missing=$missing failed=[$failed] other=[$other]" >> "$note"
  flist=$(printf '%s' "$failed" | awk '{for(i=1;i<=NF;i++) printf "%s\"%s\"", (i>1?",":""), $i}')
  printf '{"origin":"g1-m03","round":14,"try":%s,"native_rc":%s,"ready":false,"failed":[%s],"at":"%s"}\n' "$k" "${rc:-1}" "$flist" "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$here/state.json"
  [ "${rc:-1}" = 0 ] && break
  if [ "${rc:-1}" = 4 ]; then printf '{"origin":"g1-m03","round":14,"native_rc":4,"ready":false,"failed":[],"consent":true,"at":"%s"}\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$here/state.json"; echo "try $k native_rc=4: stop" >> "$note"; break; fi
  [ "$k" = 12 ] && break
  if [ "$stopped" != 0 ] || [ "${rc:-1}" = 75 ] || [ "$retryword" = 1 ] || [ "$missing" = 1 ]; then echo "try $k: next try after 180 s" >> "$note"
  else
    sigerr="$failed|$other"
    if [ "$sigerr" = "$previous" ]; then echo "try $k: the same other error as the try before: no further try" >> "$note"; break; fi
    previous="$sigerr"; echo "try $k other error (first time): comparison try after 180 s" >> "$note"
  fi
  sleep 180; k=$((k+1))
done
echo "round ended $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
