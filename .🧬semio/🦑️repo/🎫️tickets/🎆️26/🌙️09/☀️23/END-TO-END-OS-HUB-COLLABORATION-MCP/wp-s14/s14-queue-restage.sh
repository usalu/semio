#!/bin/zsh
set -u
WP=/Users/ueli/Documents/semio/.tmp-ticket/wp-s14
GEN=$WP/generated
MUTEX=/Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh
LOG=$GEN/s14-restage2.txt
: >> "$LOG"
(
  nohup zsh "$MUTEX" wasm s14 -- zsh "$WP/s14-restage-all.sh" >> "$LOG" 2>&1 &
  echo $! > "$GEN/s14-restage-pid.txt"
)
echo "restage_pid=$(cat $GEN/s14-restage-pid.txt)"
