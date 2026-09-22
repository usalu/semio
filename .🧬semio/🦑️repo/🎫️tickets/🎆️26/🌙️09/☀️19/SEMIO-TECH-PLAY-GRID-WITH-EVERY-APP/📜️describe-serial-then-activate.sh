#!/usr/bin/env bash
# 🧾️ Refreshes the committed descriptor of each given plugin ONE mutex hold at a time (peer rule 33: one wasm cargo per hold),
# then activates play's 28 lanes in ONE hold (nx --parallel=1 so at most one wasm cargo runs), then recycles :6033.
# usage: describe-serial-then-activate.sh <plugin …>   (plugin = animate | architect | … ; empty list = activation only)
set -u
cd /Users/ueli/Documents/semio || exit 1
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP"
M="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/📜️wasm-build-mutex-play-stamped.sh"; export PLAY_MUTEX_STAMP="${PLAY_MUTEX_STAMP:-20260922110250}"
export DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
log() { echo "== $(date '+%H:%M:%S') $*"; }
failed=""
for p in "$@"; do
  log "describe $p (mutex hold)"
  if zsh "$M" play -- bun nx run "@semio-tech/$p-plugin:describe" --parallel=1; then log "describe $p ok"; else failed="$failed $p"; log "describe $p FAILED"; fi
done
log "describe phase done; failed:${failed:- none}"
log "activate-dev (mutex hold, --parallel=1)"
zsh "$M" play -- bun nx run @semio-tech/semio-tech-play:activate-dev --parallel=1; rc=$?
log "activate-dev rc=$rc"
if [ "$rc" = 0 ]; then
  rm -f "$T/🗑️generated/activate.request"/* 2>/dev/null
  touch "$T/🗑️generated/serve-restart.request"
  log "activation requests cleared, :6033 recycle requested"
fi
exit $rc
