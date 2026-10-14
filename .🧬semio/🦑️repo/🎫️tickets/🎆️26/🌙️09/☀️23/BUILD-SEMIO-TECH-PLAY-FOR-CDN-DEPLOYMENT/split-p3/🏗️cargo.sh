#!/bin/bash
# 🏗️ One P3 cargo invocation inside a fleet slot with the private P3 target and build dirs; logs to `⚡️cache/p3/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P3="$ROOT/.🧬semio/🦑️repo/⚡️cache/p3"; mkdir -p "$P3/logs"
export CARGO_TARGET_DIR="$P3/target" CARGO_BUILD_BUILD_DIR="$P3/build" CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p3-$TAG" -- nice -n 0 cargo "$@" > "$P3/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P3/logs/$TAG.log") warnings=$(grep -c '^warning' "$P3/logs/$TAG.log")"
exit $rc
