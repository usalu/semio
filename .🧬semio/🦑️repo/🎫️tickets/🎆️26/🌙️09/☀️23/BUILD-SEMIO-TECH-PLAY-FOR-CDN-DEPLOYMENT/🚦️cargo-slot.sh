#!/bin/bash
# Runs one command while holding one of SLOTS fleet cargo slots (atomic mkdir); waits FIFO-ish for a free slot.
AGENT="$1"; shift; [ "$1" = "--" ] && shift
DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/slots"; SLOTS="${SLOTS:-6}"; mkdir -p "$DIR"
while :; do
  for i in $(seq 1 "$SLOTS"); do
    S="$DIR/slot-$i"
    if [ -d "$S" ] && ! kill -0 "$(cat "$S/pid" 2>/dev/null)" 2>/dev/null; then rm -rf "$S"; fi
    if mkdir "$S" 2>/dev/null; then
      echo $$ > "$S/pid"; echo "$AGENT $(date +%H:%M:%S)" > "$S/owner"
      trap 'rm -rf "$S"' EXIT INT TERM
      "$@"; exit $?
    fi
  done
  sleep 5
done
