#!/bin/zsh
# [DEBUG] 🛡️ Runs one heavy command under host-wide limits so the 16 GB build host cannot be driven into disk exhaustion or memory collapse again (kernel panic 2026-11-25 01:00).
# Usage: claude-resource-guard.sh <command> [args...]. Admits at most SLOTS concurrent guarded jobs, waits until disk >= MIN_DISK_GIB free and memory >= MIN_MEM_PCT free, and kills the job if disk drops below ABORT_DISK_GIB.
SLOTS=${SLOTS:-1}; MIN_DISK_GIB=${MIN_DISK_GIB:-30}; MIN_MEM_PCT=${MIN_MEM_PCT:-30}; ABORT_DISK_GIB=${ABORT_DISK_GIB:-12}
ROOT=/Users/ueli/Documents/semio; LOCKS=$ROOT/.cache/b/tmp/claude-resource-guard; mkdir -p $LOCKS
disk() { df -g $ROOT | tail -1 | awk '{print $4}'; }
mem() { memory_pressure | tail -1 | grep -oE '[0-9]+'; }
slot=""
while [ -z "$slot" ]; do
  for s in $(seq 1 $SLOTS); do
    f=$LOCKS/slot-$s; p=$(cat $f 2>/dev/null)
    if [ -n "$p" ] && kill -0 $p 2>/dev/null && ! ps -o stat= -p $p | grep -q Z; then continue; fi
    if [ "$(disk)" -ge $MIN_DISK_GIB ] && [ "$(mem)" -ge $MIN_MEM_PCT ]; then echo $$ > $f; slot=$f; break; fi
  done
  [ -z "$slot" ] && { echo "[DEBUG] guard waiting: disk $(disk) GiB, mem $(mem)% free, slots busy or limits unmet"; sleep 60; }
done
echo "[DEBUG] guard admitted $slot: disk $(disk) GiB, mem $(mem)% free"
"$@" & job=$!
while kill -0 $job 2>/dev/null; do
  if [ "$(disk)" -lt $ABORT_DISK_GIB ]; then echo "[DEBUG] guard abort: disk $(disk) GiB free"; pkill -TERM -P $job; kill -TERM $job; fi
  sleep 20
done
wait $job; code=$?; rm -f $slot; exit $code
