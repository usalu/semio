#!/bin/bash
# 🏗️ Like 🏗️cargo.sh but with `--message-format=json` diagnostics in `⚡️cache/p5/logs/<tag>.json.log`.
TAG=$1; shift
ROOT="/Users/ueli/Documents/semio"; P5="$ROOT/.🧬semio/🦑️repo/⚡️cache/p5"; mkdir -p "$P5/logs"
export CARGO_TARGET_DIR="$P5/target" CARGO_BUILD_BUILD_DIR="$P5/build" CARGO_TERM_COLOR=never CARGO_INCREMENTAL=0
cd "$ROOT" || exit 2
"$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" "p5-$TAG" -- cargo "$@" --message-format=json > "$P5/logs/$TAG.json.log" 2>"$P5/logs/$TAG.stderr.log"
rc=$?
echo "rc=$rc errors=$(grep -c '"level":"error"' "$P5/logs/$TAG.json.log")"
exit $rc
