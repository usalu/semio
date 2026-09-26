#!/bin/zsh
# 🔎️ H12: cargo check of semio-hub (lib + bin + tests, default features) at nice 10 in the fleet-b build dir (rule 26); capture → <capture>.
# usage: check.sh <capture> [extra cargo args…]
OUT=$1; shift
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target
echo "=== start $(date +%T) args: $*" > "$OUT"
nice -n 10 cargo check -p semio-hub --tests "$@" >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
