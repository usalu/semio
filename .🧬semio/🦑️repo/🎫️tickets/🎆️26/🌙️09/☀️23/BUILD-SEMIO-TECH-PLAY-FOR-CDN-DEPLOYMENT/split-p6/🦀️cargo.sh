#!/bin/bash
# 🦀️ P6 cargo call: one fleet slot, private target/build dirs under ⚡️cache/p6, log under ⚡️cache/p6/logs/<tag>.log.
# usage: 🦀️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; C="$ROOT/.🧬semio/🦑️repo/⚡️cache/p6"; mkdir -p "$C/logs"
export CARGO_TARGET_DIR="$C/target" CARGO_BUILD_BUILD_DIR="$C/build" CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p6-$TAG" -- cargo "$@" > "$C/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$C/logs/$TAG.log") warnings=$(grep -c '^warning' "$C/logs/$TAG.log")"
exit $rc
