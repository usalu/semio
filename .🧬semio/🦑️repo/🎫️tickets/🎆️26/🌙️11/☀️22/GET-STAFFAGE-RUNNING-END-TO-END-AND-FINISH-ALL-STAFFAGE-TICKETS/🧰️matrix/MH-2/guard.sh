#!/bin/zsh
# 🛡️ Runs a command under a hard ceiling of resident memory and measures its peak: guard.sh <limit-MB> <peak-file> <command> <args…> starts the command, sums every two seconds the resident size of the command and all its descendants (`ps -o rss`), keeps the largest sum in <peak-file> (megabytes, with the time) and, when the sum passes the limit, stops the whole tree and exits 137 with `GUARD KILLED` in <peak-file>. Made so that a run reserves in the memory queue exactly what it is allowed to use: the reservation can then be the measured need instead of the 7000 MB that the queue of this host (9 GB held by unmanaged processes) cannot grant.
limit="$1"; peakfile="$2"; shift 2
"$@" & root=$!
tree() { local p; for p in $(pgrep -P "$1" 2>/dev/null); do print -r -- "$p"; tree "$p"; done }
peak=0
while kill -0 $root 2>/dev/null; do
  pids=($root $(tree $root)); kb=$(ps -o rss= -p ${(j:,:)pids} 2>/dev/null | awk '{s+=$1} END {print s+0}'); mb=$((kb/1024))
  (( mb > peak )) && { peak=$mb; print -r -- "peak $peak MB of limit $limit MB at $(date +%H:%M:%S), ${#pids} processes" > "$peakfile"; }
  if (( mb > limit )); then kill -TERM $pids 2>/dev/null; sleep 2; kill -KILL $pids 2>/dev/null; print -r -- "GUARD KILLED at $mb MB of limit $limit MB at $(date +%H:%M:%S)" >> "$peakfile"; exit 137; fi
  sleep 2
done
wait $root
