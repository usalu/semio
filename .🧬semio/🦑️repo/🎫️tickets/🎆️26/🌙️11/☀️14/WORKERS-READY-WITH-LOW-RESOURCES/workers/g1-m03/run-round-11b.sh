#!/bin/bash
# 🛰️ Runs protocol round 11 on this worker: up to ten tries of W/native-bootstrap.sh, each in its own process group, sampled every 30 s, launcher facts after every try. After native_rc=4 it waits for the owner's consent (repo preflight every 60 s, at most 30 minutes) and tries again; after a refusal 120 s; after a hard stop 180 s; after task failures with source-differs, closure.moving or retry wording 120 s; it stops after exit 0 or two tries in a row with the same failed tasks of another kind. Usage: run-round-11.sh
set -u
here=$(cd "$(dirname "$0")" && pwd); W=$(cd "$here/../.." && pwd); note="$here/round-11.monitor.txt"
export TZ=UTC
refusaltext='graph-wanted|nothing to wait for any longer|could not be read on this machine'
want='taken from the fleet|inferred here|waiting for the fleet|restored|closureRepaired|where: root=|fstatat failed|falls back|fall back|fallback|caller.s access|under the|refreshing [0-9]+ projects|asking the host for|stale facts|access-denied|consent'
repoenv(){ ( cd "$HOME/Documents/semio" && SEMIO_LANGUAGE=en SEMIO_WORKSPACE_ROOT="$HOME/Documents/semio" SEMIO_FLEET_GROUP=g1 SEMIO_FLEET_NFS_HOST=192.168.178.70 SEMIO_FLEET_NETWORK=192.168.178.0/24 PATH="$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" "$@" ); }
k=4; previous=""; uncounted=0
while [ "$k" -le 10 ]; do
  if [ "$k" = 1 ]; then log="$HOME/native-bootstrap-round-11.log"; else log="$HOME/native-bootstrap-round-11-try-$k.log"; fi
  samples="$here/round-11.try-$k.samples.txt"; : > "$samples"; mark="$here/.round-11.try-$k.start"; touch "$mark"
  for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do [ -f "$s" ] && echo "$(wc -l < "$s" | tr -d ' ') $s"; done > "$here/.round-11.try-$k.before"
  perl -e 'setpgrp(0,0); exec @ARGV' bash "$W/native-bootstrap.sh" "$log" < /dev/null > /dev/null 2>&1 &
  pid=$!; echo "try $k started pid=$pid $(date -u +%Y-%m-%dT%H:%M:%SZ) log=$log" >> "$note"
  stopped=0; twos=0; fours=0; same=0; lastsig=""
  while kill -0 "$pid" 2>/dev/null; do
    level=$(sysctl -n kern.memorystatus_vm_pressure_level)
    { date -u +"%Y-%m-%dT%H:%M:%SZ pressure=$level"; ps -axo rss,pid,ppid,args | sort -nr | head -3 | cut -c1-600; } >> "$samples"
    sigline=$( { sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -o -E 'owners [0-9]+ of [0-9]+|^(✔|✖|>) +nx run [^ ]+|Inferring projects: [0-9]+ of [0-9]+|^\[semio\] [A-Za-z:]+' | tail -1; for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do [ "$s" -nt "$mark" ] && grep -a -o -E 'waiting for the fleet: [0-9]+ of [0-9]+|[0-9]+ of [0-9]+ projects inferred' "$s" | tail -1; done; } | tr '\n' '|')
    if [ "$sigline" = "$lastsig" ]; then same=$((same+1)); else same=0; lastsig="$sigline"; fi
    if [ "$stopped" = 0 ] && [ "$same" -ge 30 ]; then
      echo "try $k no new progress line for 15 minutes (last: $sigline) $(date -u +%Y-%m-%dT%H:%M:%SZ); one SIGINT to process group $pid" >> "$note"
      { echo "== try $k stalled $(date -u +%Y-%m-%dT%H:%M:%SZ), last progress: $sigline"; echo "-- ps lines of the graph service and its children"; for sp in $(ps -axo pid,args | grep -E 'service host' | grep -v grep | awk '{print $1}'); do ps -axo pid,ppid,etime,rss,args | awk -v p="$sp" '$1==p || $2==p' | cut -c1-400; done; for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do [ "$s" -nt "$mark" ] && { echo "-- last 60 lines of $s"; tail -60 "$s" | cut -c1-500; }; done; } >> "$here/round-11.stall.txt" 2>&1
      kill -INT -- "-$pid"; stopped=2
    fi
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
  bash "$here/launcher-facts.sh" "after try $k" >> "$here/round-11.launcher.txt" 2>&1
  cleanlog="$here/.round-11.try-$k.clean"; sed 's/\x1b\[[0-9;]*m//g' "$log" | grep -a -v '^\[script\] {' > "$cleanlog"
  rc=$(grep -a -o -E '^native_rc=[0-9]+' "$cleanlog" | tail -1 | cut -d= -f2)
  out="$here/round-11.try-$k.service.txt"; : > "$out"
  for s in "${TMPDIR%/}"/semio-repo/"$(id -u)"/host-*.log; do
    [ -f "$s" ] && [ "$s" -nt "$mark" ] || continue
    b=$(grep -F " $s" "$here/.round-11.try-$k.before" | awk '{print $1}'); b=${b:-0}
    { echo "== $s (lines after $b)"; echo "-- graph lines"; tail -n +"$((b+1))" "$s" | grep -a -E "$want" | awk '!s[$0]++' | cut -c1-500; echo "-- last 30 lines when the try ended"; tail -30 "$s" | cut -c1-500; } >> "$out"
  done
  refusal=0; { [ "${rc:-1}" = 75 ] || grep -a -q -E "$refusaltext" "$cleanlog"; } && refusal=1
  failed=$(awk '/^Failed tasks:/{f=1;next} f&&/^- /{print $2} f&&/^$/{n++} f&&n>1{exit}' "$cleanlog" | sort | tr '\n' ' ')
  retryword=0; grep -a -q -i -E 'source-differs|closure\.moving|retry|try again|run the command again' "$cleanlog" && [ -n "$failed" ] && retryword=1
  echo "try $k ended $(date -u +%Y-%m-%dT%H:%M:%SZ) native_rc=${rc:-unknown} stopped=$stopped fleet_refusal=$refusal retry_wording=$retryword failed=[$failed]" >> "$note"
  [ "${rc:-1}" = 0 ] && break
  [ "$k" = 10 ] && break
  if grep -a -q '3420bc16' "$cleanlog"; then echo "try $k: reference 3420bc16 appeared again: stop and report the full stack" >> "$note"; break; fi
  if [ "$stopped" = 2 ]; then echo "try $k stalled: next try after 120 s" >> "$note"; [ "$k" = 10 ] && break; sleep 120; previous="$failed"; k=$((k+1)); continue; fi
  if [ "${rc:-1}" = 4 ]; then
    printf '{"origin":"g1-m03","round":11,"native_rc":4,"ready":false,"failed":[],"consent":true,"at":"%s"}\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" > "$here/state.json"
    echo "try $k consent (native_rc=4): neither the launcher nor the caller can read the worktree; no further try" >> "$note"; break
  elif grep -a -q -i -E 'store (is )?not reachable|store .*unreachable|cache .*timed? ?out|timeout of the cache|ETIMEDOUT|ECONNREFUSED|ESTALE' "$cleanlog"; then
    echo "try $k ended with a cache timeout or an unreachable store (server move window?): waiting 300 s, not counted" >> "$note"; sleep 300; previous="$failed"; uncounted=$((uncounted+1)); [ "$uncounted" -le 6 ] && continue
  elif [ "$stopped" = 1 ]; then echo "try $k hard stop: waiting 180 s" >> "$note"; sleep 180
  elif [ "$refusal" = 1 ]; then echo "try $k refusal: waiting 120 s" >> "$note"; sleep 120
  elif [ "$retryword" = 1 ]; then echo "try $k task failures with source-differs, closure.moving or retry wording: waiting 120 s" >> "$note"; sleep 120
  elif [ -n "$failed" ] && [ "$failed" = "$previous" ]; then echo "try $k: same failed tasks as the try before, of another kind: no further try" >> "$note"; break
  elif [ -n "$failed" ]; then echo "try $k task failures of another kind (first time with this list): waiting 120 s for the comparison try" >> "$note"; sleep 120
  else echo "try $k ended without a rule for a further try (native_rc=${rc:-unknown})" >> "$note"; break; fi
  previous="$failed"; k=$((k+1))
done
echo "round ended $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$note"
