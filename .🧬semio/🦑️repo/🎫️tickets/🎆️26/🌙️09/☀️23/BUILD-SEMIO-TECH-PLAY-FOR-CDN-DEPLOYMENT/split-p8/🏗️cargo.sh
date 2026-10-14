#!/bin/bash
# 🏗️ One P8 cargo invocation inside a fleet slot with private target/build dirs under `⚡️cache/p8`; log in `⚡️cache/p8/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P8="$ROOT/.🧬semio/🦑️repo/⚡️cache/p8"; mkdir -p "$P8/logs"
export CARGO_TARGET_DIR="$P8/target" CARGO_BUILD_BUILD_DIR="$P8/build" CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p8-$TAG" -- nice -n 0 cargo "$@" > "$P8/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P8/logs/$TAG.log") warnings=$(grep -c '^warning' "$P8/logs/$TAG.log")"
exit $rc
