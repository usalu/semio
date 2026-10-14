#!/bin/bash
# 🏗️ One E3b cargo invocation through `🚦️cargo-slot.sh` on the shared store; logs to `🗑️generated/e3b/logs/<tag>.log`.
# usage: 🏗️cargo.sh <tag> <cargo args...>
TAG=$1; shift
TICKET="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"; LOGS="$TICKET/🗑️generated/e3b/logs"; mkdir -p "$LOGS"
s=$(date +%s)
env -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR CARGO_TERM_COLOR=never "$TICKET/🚦️cargo-slot.sh" "e3b-$TAG" -- cargo "$@" > "$LOGS/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$LOGS/$TAG.log") warnings=$(grep -c '^warning' "$LOGS/$TAG.log")"
exit $rc
