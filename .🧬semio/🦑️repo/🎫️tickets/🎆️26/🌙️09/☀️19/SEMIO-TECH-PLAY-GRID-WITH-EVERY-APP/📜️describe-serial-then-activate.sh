#!/usr/bin/env bash
# 🧾️ Refreshes the committed descriptor of each given plugin ONE mutex hold at a time (peer rule 33: one wasm cargo per hold),
# then activates play's 28 lanes in ONE hold (nx --parallel=1 so at most one wasm cargo runs), then recycles :6033.
# A step that fails because a FRAMEWORK crate does not compile (a peer's in-flight refactor) is retried after 5 min
# (mutex released while waiting), up to 18 times; any other failure moves on.
# usage: describe-serial-then-activate.sh <plugin …>   (plugin = animate | architect | … ; empty list = activation only)
set -u
cd /Users/ueli/Documents/semio || exit 1
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP"
M="$T/📜️wasm-build-mutex-play-stamped.sh"; export PLAY_MUTEX_STAMP="${PLAY_MUTEX_STAMP:-20260922110250}"
export DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
STEPS="${PLAY_CHAIN_STEPS:-/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/activation/steps}"; mkdir -p "$STEPS"
log() { echo "== $(date '+%H:%M:%S') $*"; }
framework_broken() { grep -qE 'could not compile `semio-framework-[a-z-]*`' "$1"; }
run_step() { # name, cmd…
  local name="$1"; shift; local try=0
  while true; do
    try=$((try+1)); local out="$STEPS/$name-$(date +%H%M%S).txt"
    log "$name (mutex hold, try $try) → $out"
    if zsh "$M" play -- "$@" > "$out" 2>&1; then log "$name ok"; return 0; fi
    if framework_broken "$out" && [ $try -lt 18 ]; then
      log "$name blocked: $(grep -oE 'could not compile `semio-framework-[a-z-]*`' "$out" | sort -u | tr '\n' ' ') — retry in 5 min"
      sleep 300; continue
    fi
    log "$name FAILED (see $out)"; return 1
  done
}
activate() {
  run_step "activate-dev" bun nx run @semio-tech/semio-tech-play:activate-dev --parallel=1; local rc=$?
  log "activate-dev rc=$rc"
  if [ "$rc" = 0 ]; then
    rm -f "$T/🗑️generated/activate.request"/* 2>/dev/null
    touch "$T/🗑️generated/serve-restart.request"
    log "activation requests cleared, :6033 recycle requested"
  fi
  return $rc
}
if [ -n "${PLAY_ACTIVATE_FIRST:-}" ]; then activate; fi
failed=""
for p in "$@"; do
  run_step "describe-$p" bun nx run "@semio-tech/$p-plugin:describe" --parallel=1 || failed="$failed $p"
done
log "describe phase done; failed:${failed:- none}"
activate; exit $?
