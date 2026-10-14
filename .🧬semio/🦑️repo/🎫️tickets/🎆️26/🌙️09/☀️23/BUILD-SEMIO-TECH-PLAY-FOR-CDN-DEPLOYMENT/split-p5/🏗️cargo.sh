#!/bin/bash
# 🏗️ One P5 cargo invocation through the issued repository Cargo launcher (`🚦️cargo-slot.sh`, shared store); logs to `⚡️cache/p5/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P5="$ROOT/.🧬semio/🦑️repo/⚡️cache/p5"; mkdir -p "$P5/logs"
unset CARGO_TARGET_DIR CARGO_BUILD_BUILD_DIR; export CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p5-$TAG" -- cargo "$@" > "$P5/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P5/logs/$TAG.log") warnings=$(grep -c '^warning' "$P5/logs/$TAG.log")"
exit $rc
