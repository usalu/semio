#!/bin/bash
# 🦀️ One P4 cargo invocation: one fleet slot, private target/build dirs under `⚡️cache/p4`, a deadline (exit 124 resumes on
# rerun from the private build dir), log at `⚡️cache/p4/logs/<tag>.log` and a one-line summary.
# usage: 🦀️cargo.sh <tag> <seconds> <cargo args...>
TAG=$1; LIMIT=$2; shift 2
ROOT="/Users/ueli/Documents/semio"; P4="$ROOT/.🧬semio/🦑️repo/⚡️cache/p4"; mkdir -p "$P4/logs"
T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT"
export CARGO_TARGET_DIR="$P4/target" CARGO_BUILD_BUILD_DIR="$P4/build" CARGO_TERM_COLOR=never CARGO_INCREMENTAL=0
cd "$ROOT" || exit 2
s=$(date +%s)
bash "$T/🚦️cargo-slot.sh" "p4-$TAG" -- bash "$T/split-p4/⏱️bounded.sh" "$LIMIT" -- cargo "$@" > "$P4/logs/$TAG.log" 2>&1
rc=$?
echo "rc=$rc wall=$(( $(date +%s) - s ))s errors=$(grep -c '^error' "$P4/logs/$TAG.log") warnings=$(grep -c '^warning' "$P4/logs/$TAG.log")"
exit $rc
