#!/bin/zsh
# ⚖️ H12: every hub bin law whose name mentions a socket (regression guard for the document-socket closing handshake)
# through the native mutex in build-fleet-b; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target
echo "=== start $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h12 -- nice -n 15 cargo test --no-fail-fast -p semio-hub --bin os-hub -- socket presence welcome close --test-threads 4 >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
