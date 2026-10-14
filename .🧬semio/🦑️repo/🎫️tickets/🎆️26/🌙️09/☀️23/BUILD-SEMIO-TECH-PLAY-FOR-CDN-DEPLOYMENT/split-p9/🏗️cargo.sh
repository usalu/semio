#!/bin/bash
# 🏗️ One P9 cargo invocation inside a fleet slot with the private P9 target and build dirs; logs to `⚡️cache/p9/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P9="$ROOT/.🧬semio/🦑️repo/⚡️cache/p9"; mkdir -p "$P9/logs"
export CARGO_TARGET_DIR="$P9/target" CARGO_BUILD_BUILD_DIR="$P9/build" CARGO_TERM_COLOR=never CARGO_INCREMENTAL=0
cd "$ROOT" || exit 2
s=$(date +%s)
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p9-$TAG" -- cargo "$@" > "$P9/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P9/logs/$TAG.log") warnings=$(grep -c '^warning' "$P9/logs/$TAG.log")"
exit $rc
