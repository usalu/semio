#!/bin/zsh
# 🔬️ H14: release build of the std-only origin probe (`wp-h14/origin-probe`) through the native lane (preamble 14 rule 3);
# capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-h14/origin-probe
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c 'echo "=== start $(date +%T)"; nice -n 15 cargo build --release; echo "=== EXIT $? $(date +%T)"' >> "$OUT" 2>&1
