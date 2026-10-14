#!/bin/bash
# 🏗️ One P9 cargo invocation through the fleet slot (the issued launcher, shared store); logs to `⚡️cache/p9/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo subcommand> <args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P9="$ROOT/.🧬semio/🦑️repo/⚡️cache/p9"; mkdir -p "$P9/logs"
unset CARGO_TARGET_DIR CARGO_BUILD_BUILD_DIR
export CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p9-$TAG" -- cargo "$@" > "$P9/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P9/logs/$TAG.log") warnings=$(grep -c '^warning' "$P9/logs/$TAG.log")"
exit $rc
