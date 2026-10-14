#!/bin/bash
# 🏗️ One lane-P2 cargo invocation through `🚦️cargo-slot.sh` (the issued repository launcher, shared store); the full log goes to
# `split-p2/🗑️generated/logs/<tag>.log` and a one-line summary (rc, wall, error and warning counts) to stdout.
# usage: 🏗️cargo.sh <tag> <cargo subcommand> <args...>   (pass --manifest-path)
TAG=$1; shift
HERE="$(cd "$(dirname "$0")" && pwd)"; LOGS="$HERE/🗑️generated/logs"; mkdir -p "$LOGS"
ROOT="/Users/ueli/Documents/semio"
cd "$ROOT" || exit 2
s=$(date +%s)
env -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR CARGO_TERM_COLOR=never bash "$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p2-$TAG" -- cargo "$@" > "$LOGS/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -cE '(^|: )error(\[|:)' "$LOGS/$TAG.log") warnings=$(grep -cE '(^|: )warning(\[|:)' "$LOGS/$TAG.log")"
exit $rc
