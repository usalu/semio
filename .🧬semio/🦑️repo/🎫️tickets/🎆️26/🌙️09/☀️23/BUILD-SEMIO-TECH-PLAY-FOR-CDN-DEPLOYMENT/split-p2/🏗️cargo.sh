#!/bin/bash
# 🏗️ One lane-P2 cargo invocation inside a fleet slot with the private P2 target and build dirs; logs to `⚡️cache/p2/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>   (run from any cwd; pass --manifest-path)
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P2="$ROOT/.🧬semio/🦑️repo/⚡️cache/p2"; mkdir -p "$P2/logs"
export CARGO_TARGET_DIR="$P2/target" CARGO_BUILD_BUILD_DIR="$P2/build" CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p2-$TAG" -- cargo "$@" > "$P2/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P2/logs/$TAG.log") warnings=$(grep -c '^warning' "$P2/logs/$TAG.log")"
exit $rc
