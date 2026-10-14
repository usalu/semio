#!/bin/bash
# 🏗️ One F1 cargo invocation through `🚦️cargo-slot.sh` (issued launcher, shared store); logs to `⚡️cache/f1/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo subcommand> <args...>
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; F1="$ROOT/.🧬semio/🦑️repo/⚡️cache/f1"; mkdir -p "$F1/logs"
unset CARGO_TARGET_DIR CARGO_BUILD_BUILD_DIR
export CARGO_TERM_COLOR=never
cd "$ROOT" || exit 2
s=$(date +%s)
bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "f1-$TAG" -- cargo "$@" > "$F1/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -cE '^error|: error' "$F1/logs/$TAG.log") warnings=$(grep -cE '^warning|: warning' "$F1/logs/$TAG.log")"
exit $rc
