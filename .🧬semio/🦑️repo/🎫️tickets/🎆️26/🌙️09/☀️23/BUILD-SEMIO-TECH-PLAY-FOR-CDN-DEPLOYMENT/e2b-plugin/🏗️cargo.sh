#!/bin/bash
# 🏗️ One E2b cargo invocation inside a fleet slot with the private E2b target dir; logs to `⚡️cache/e2b/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>   (env BUILD_DIR=<path> additionally privatises the build-dir)
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; E2B="$ROOT/.🧬semio/🦑️repo/⚡️cache/e2b"; mkdir -p "$E2B/logs"
export CARGO_TARGET_DIR="$E2B/target" CARGO_TERM_COLOR=never
[ -n "$BUILD_DIR" ] && export CARGO_BUILD_BUILD_DIR="$BUILD_DIR"
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "e2b-$TAG" -- cargo "$@" > "$E2B/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$E2B/logs/$TAG.log") warnings=$(grep -c '^warning' "$E2B/logs/$TAG.log")"
exit $rc
