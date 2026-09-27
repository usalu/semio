#!/bin/zsh
# 🔎️ H12: one cargo check of semio-hub (lib + bin + tests) and puzzle 2d/3d (lib + tests, component-app-assembly) at nice 10
# in the landing build dir (rule 27, H12 guest landing handed over by LA); capture → <capture>.
OUT=$1; shift
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-landing" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h12/target
echo "=== start $(date +%T)" > "$OUT"
nice -n 10 cargo check --keep-going -p semio-framework-plugin -p semio-hub -p semio-s-artifact-puzzle-2d -p semio-s-artifact-puzzle-3d -p semio-s-plugin-puzzle --lib --bins --tests --features semio-s-artifact-puzzle-2d/component-app-assembly,semio-s-artifact-puzzle-3d/component-app-assembly "$@" >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
