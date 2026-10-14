#!/bin/bash
# 🏗️ One F1 cargo invocation inside a fleet slot with the private F1 target and build dirs; logs to `⚡️cache/f1/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; F1="$ROOT/.🧬semio/🦑️repo/⚡️cache/f1"; mkdir -p "$F1/logs"
export CARGO_TARGET_DIR="$F1/target" CARGO_BUILD_BUILD_DIR="$F1/build" CARGO_TERM_COLOR=never CARGO_INCREMENTAL=0
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "f1-$TAG" -- cargo "$@" > "$F1/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$F1/logs/$TAG.log") warnings=$(grep -c '^warning' "$F1/logs/$TAG.log")"
exit $rc
