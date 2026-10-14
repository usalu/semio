#!/bin/bash
# 🏗️ One P7 cargo invocation through the issued launcher (`🚦️cargo-slot.sh`, shared store); logs to `⚡️cache/p7/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P7="$ROOT/.🧬semio/🦑️repo/⚡️cache/p7"; mkdir -p "$P7/logs"
unset CARGO_TARGET_DIR CARGO_BUILD_BUILD_DIR
export CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p7-$TAG" -- cargo "$@" > "$P7/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P7/logs/$TAG.log") warnings=$(grep -c '^warning' "$P7/logs/$TAG.log") log=$P7/logs/$TAG.log"
exit $rc
