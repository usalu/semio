#!/bin/bash
# 🏗️ One lane-P1 cargo invocation inside a fleet slot with private target and build dirs under `⚡️cache/p1`; the full log
# goes to `⚡️cache/p1/logs/<tag>.log` and a one-line summary (rc, wall, error and warning counts) to stdout.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P1="$ROOT/.🧬semio/🦑️repo/⚡️cache/p1"; mkdir -p "$P1/logs"
export CARGO_TARGET_DIR="$P1/target" CARGO_BUILD_BUILD_DIR="$P1/build" CARGO_TERM_COLOR=never CARGO_INCREMENTAL=0
cd "$ROOT" || exit 2
s=$(date +%s)
bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p1-$TAG" -- cargo "$@" > "$P1/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P1/logs/$TAG.log") warnings=$(grep -c '^warning' "$P1/logs/$TAG.log")"
exit $rc
