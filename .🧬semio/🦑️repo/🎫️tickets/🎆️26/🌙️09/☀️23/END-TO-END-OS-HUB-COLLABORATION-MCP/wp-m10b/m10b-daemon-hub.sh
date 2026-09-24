#!/bin/zsh
# Double-fork style detach so Cursor shell exit cannot reap the hold.
set -eu
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-m10b
DATA=$(readlink "$W/links/hc1-boot")
PORT=${1:-7681}
BIN=$W/target-hub/debug/os-hub
LOG=$W/generated/hub-${PORT}-daemon.txt
PIDF=$W/generated/hub-${PORT}.pids
STAT=$W/generated/hub-${PORT}-status.txt
: > "$LOG"
: > "$STAT"
rm -f "$W/generated/hub-${PORT}-status-ready.json" "$W/generated/hub-${PORT}-capture.txt"
# Outer nohup; inner bun hold keeps hub pipe alive.
nohup /Users/ueli/.bun/bin/bun "$W/m10b-hub-hold.ts" "$PORT" "$DATA" "$BIN" "$PIDF" "$STAT" >>"$LOG" 2>&1 </dev/null &
echo $! > "$W/generated/hub-${PORT}-daemon.pid"
disown || true
echo "daemon_pid=$(cat "$W/generated/hub-${PORT}-daemon.pid") log=$LOG"
