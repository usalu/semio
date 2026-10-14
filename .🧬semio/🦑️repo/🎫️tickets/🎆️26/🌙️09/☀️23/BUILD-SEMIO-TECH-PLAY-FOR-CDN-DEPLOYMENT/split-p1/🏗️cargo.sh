#!/bin/bash
# 🏗️ One lane-P1 cargo invocation through the fleet usage line (`🚦️cargo-slot.sh`, the issued K0 launcher on the shared
# store); the full log goes to `split-p1/🗑️generated/cargo/<tag>.log` and a one-line summary (rc, wall, error and warning
# counts) to stdout. usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
LOG="$T/split-p1/🗑️generated/cargo"; mkdir -p "$LOG"
cd "$ROOT" || exit 2
s=$(date +%s)
env -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR CARGO_TERM_COLOR=never bash "$T/🚦️cargo-slot.sh" "p1-$TAG" -- cargo "$@" > "$LOG/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$LOG/$TAG.log") warnings=$(grep -c '^warning' "$LOG/$TAG.log")"
exit $rc
