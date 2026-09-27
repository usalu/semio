#!/bin/zsh
# 🧪️ S17 native gate phases B (demonstrator) + C (stdio, hub) of the VersionPin set, build-fleet-b, nice 10, keep-going.
# usage: zsh s17-check-pin-bc.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
echo "PHASE BC START $(date '+%T')" >> "$1"
nice -n 10 cargo check -p semio-s-plugin-demonstrator -p semio-s-plugin-stdio -p semio-hub --lib --tests --keep-going --message-format short >> "$1" 2>&1
echo "PHASE BC EXIT=$? $(date '+%T')" >> "$1"
echo "ALL_DONE $(date '+%T')" >> "$1"
