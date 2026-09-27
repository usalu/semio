#!/bin/zsh
# ⚖️ H12: the guest-landing laws (puzzle initial-snapshot cost, puzzle codec parity, plugin codec-no-app) through the
# native mutex in build-fleet-b (rule 30); capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target
echo "=== start $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h12 -- nice -n 15 cargo test --no-fail-fast -p semio-framework-plugin -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-3d -p semio-s-plugin-puzzle --lib --features semio-s-artifact-puzzle-2d/component-app-assembly,semio-s-artifact-puzzle-3d/component-app-assembly -- the_initial_snapshot_costs_only_its_document guest_codec_tables_answer_like_the_declared_native_codecs codec_calls_construct_no_app >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
