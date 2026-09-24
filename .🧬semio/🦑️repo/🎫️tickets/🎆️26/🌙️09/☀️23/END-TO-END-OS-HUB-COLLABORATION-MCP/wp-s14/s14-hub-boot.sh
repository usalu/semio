#!/bin/zsh
set -u
WP=/Users/ueli/Documents/semio/.tmp-ticket/wp-s14
GEN=$WP/generated
PORT=${1:-7730}
DATA=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-boot
BIN=$WP/bin/os-hub
mkdir -p "$DATA" "$GEN"
chmod 700 "$DATA"
LOG=$GEN/s14-hub-hold.txt
: > "$LOG"
# macOS: no setsid; detach via nohup + background in a subshell that exits
(
  nohup bun "$WP/s14-hub-hold.ts" "$PORT" "$DATA" "$BIN" >> "$LOG" 2>&1 &
  echo $! > "$GEN/s14-hub-hold-pid.txt"
)
echo "started pid=$(cat $GEN/s14-hub-hold-pid.txt) port=$PORT"
