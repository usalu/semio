#!/bin/bash
# 🛰️ Runs protocol round 6 on this worker: up to eight tries of W/native-bootstrap.sh, each in its own process group, sampled every 30 s. Next try after a refusal (120 s), a hard stop (180 s), nx failures with a source-differs (120 s) or a surviving closure ENOENT (after one `repo cache gc --area=graph-inference`); no next try after exit 4, exit 0, or two tries in a row with the same failed tasks and no source-differs. Usage: run-round-6.sh
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-6.monitor.txt"; : > "$note"
refusaltext='graph-wanted|nothing to wait for any longer|could not be read on this machine'
want='taken from the fleet|inferred here|waiting for the fleet|restored|closureRepaired|refreshing [0-9]+ projects|asking the host for|stale facts|access-denied|consent'
k=1; previous=""; enoent_before=0; gc_done=0
while [ "$k" -le 8 ]; do
  if [ "$k" = 1 ]; then log="$HOME/native-bootstrap-round-6.log"; else log="$HOME/native-bootstrap-round-6-try-$k.log"; fi
  samples="$here/round-6.try-$k.samples.txt"; : > "$samples"; mark="$here/.round-6.try-$k.start"; touch "$mark"
  for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do [ -f "$s" ] && echo "$(wc -l < "$s" | tr -d ' ') $s"; done > "$here/.round-6.try-$k.before"
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ) log=$log" >> "$note"
  stopped=0; twos=0; fours=0
  while kill -0 "$pid" 2>/dev/null; do
    level=$(sysctl -n kern.memorystatus_vm_pressure_level)
    { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-600; } >> "$samples"
    if [ "$level" = 2 ]; then twos=$((twos+1)); else twos=0; fi
    if [ "$level" = 4 ]; then fours=$((fours+1)); else fours=0; fi
    if [ "$stopped" = 0 ]; then
      reason=""
      top=$(ps -axo pgid,rss | awk -v g="$pid" '$1==g && $2>m {m=$2} END {print m+0}')
      [ "$top" -gt 8388608 ] && reason="process of the run above 8 GB resident ($top KiB)"
      [ "$fours" -ge 2 ] && reason="memory pressure level 4 in two samples in a row"
      [ "$twos" -ge 6 ] && reason="memory pressure level 2 in six samples in a row"
      if [ -n "$reason" ]; then echo "try $k hard stop $(date -u +%Y-%m-%dT%H:%M:%SZ): $reason; one SIGINT to process group $pid" >> "$note"; kill -INT -- "-$pid"; stopped=1; fi
    fi
    sleep 30
  done
  cleanlog="$here/.round-6.try-$k.clean"; sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -v '^\[script\] {' > "$cleanlog"
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$cleanlog" | tail -1 | cut -d= -f2)
  out="$here/round-6.try-$k.service.txt"; : > "$out"
  for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do
    [ -f "$s" ] && [ "$s" -nt "$mark" ] || continue
    b=$(grep -F " $s" "$here/.round-6.try-$k.before" | awk '{print $1}'); b=${b:-0}
    { echo "== $s (lines after $b)"; echo "-- graph lines"; tail -n +"$((b+1))" "$s" | grep -a -E "$want" | awk '!s[$0]++' | cut -c1-500; echo "-- last 30 lines when the try ended"; tail -30 "$s" | cut -c1-500; } >> "$out"
  done
  refusal=0; { [ "${rc:-1}" = 75 ] || grep -a -q -E "$refusaltext" "$cleanlog"; } && refusal=1
  failed=$(awk '/^Failed tasks:/{f=1;next} f&&/^- /{print $2} f&&/^$/{n++} f&&n>1{exit}' "$cleanlog" | sort | tr '\n' ' ')
  differs=0; grep -a -q 'source-differs' "$cleanlog" && differs=1
  enoent=0; grep -a -q -E 'ENOENT.*build-input-closures' "$cleanlog" && enoent=1
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=${rc:-unknown} stopped=$stopped fleet_refusal=$refusal source_differs=$differs closure_enoent=$enoent failed=[$failed]" >> "$note"
  [ "${rc:-1}" = 0 ] && break
  [ "${rc:-1}" = 4 ] && { echo "try $k exit 4: consent, no further try" >> "$note"; break; }
  [ "$k" = 8 ] && break
  if [ "$stopped" = 1 ]; then echo "try $k hard stop: waiting 180 s" >> "$note"; sleep 180
  elif [ "$refusal" = 1 ]; then echo "try $k refusal: waiting 120 s" >> "$note"; sleep 120
  elif [ "$enoent" = 1 ] && [ "$enoent_before" = 1 ] && [ "$gc_done" = 0 ]; then
    echo "try $k: the closure ENOENT survived a try; running repo cache gc --area=graph-inference once $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
    ( cd "$HOME/Documents/semio" && SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$HOME/Documents/semio" PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" repo cache gc --area=graph-inference > "$here/round-6.cache-gc.txt" 2>&1; echo "exit=$?" >> "$here/round-6.cache-gc.txt" ); gc_done=1
  elif [ -n "$failed" ] && [ "$differs" = 1 ]; then echo "try $k nx failures with source-differs: waiting 120 s" >> "$note"; sleep 120
  elif [ "$enoent" = 1 ] && [ "$gc_done" = 0 ]; then echo "try $k closure ENOENT (first time): waiting 120 s for the next try" >> "$note"; sleep 120
  elif [ -n "$failed" ] && [ "$failed" = "$previous" ]; then echo "try $k: same failed tasks as the try before and no source-differs, no further try" >> "$note"; break
  elif [ -n "$failed" ]; then echo "try $k nx failures without source-differs (first time with this list): waiting 120 s for one comparison try" >> "$note"; sleep 120
  else echo "try $k ended without a rule for a further try (native_rc=${rc:-unknown})" >> "$note"; break; fi
  previous="$failed"; enoent_before=$enoent; k=$((k+1))
done
echo "round ended $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
