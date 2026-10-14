#!/bin/bash
# 🏗️ One P5 cargo invocation inside a fleet slot with the private P5 target and build dirs; logs to `⚡️cache/p5/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P5="$ROOT/.🧬semio/🦑️repo/⚡️cache/p5"; mkdir -p "$P5/logs"
export CARGO_TARGET_DIR="$P5/target" CARGO_BUILD_BUILD_DIR="$P5/build" CARGO_TERM_COLOR=never CARGO_INCREMENTAL=0
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p5-$TAG" -- cargo "$@" > "$P5/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P5/logs/$TAG.log") warnings=$(grep -c '^warning' "$P5/logs/$TAG.log")"
exit $rc
