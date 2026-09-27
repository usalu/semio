#!/bin/zsh
# 🧊️ G11: wasm32-wasip2 `cargo check --lib` of guest-linked crates, through the fleet wasm mutex, build-fleet-b.
# usage: zsh g11-wasm-check.sh <crate>…
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
P=(); for a in "$@"; do P+=(-p "$a"); done
date; S=$(date +%s)
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm g11 -- nice -n 10 cargo check "${P[@]}" --lib --target wasm32-wasip2 --message-format short 2>&1
RC=$?
echo "WASM_CHECK_RC=$RC secs=$(( $(date +%s)-S ))"; date
