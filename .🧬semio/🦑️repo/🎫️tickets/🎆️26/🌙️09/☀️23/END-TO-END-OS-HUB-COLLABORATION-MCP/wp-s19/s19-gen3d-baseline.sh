#!/bin/zsh
# 🧪️ S19 live-tree baseline (native lane, build-fleet-b) for the 8 generation3d lib tests red on the overlay binary with set
# `gen-archive-load` (6 mesh-component gumball/projection, mesh preview count, keyboard fixture) — red here too = pre-existing,
# not the set's. usage: zsh s19-gen3d-baseline.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19/target CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
echo "START $(date '+%T')" > "$1"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native s19 -- perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-fail-fast -- mesh_component mesh_preview_renders_without_a_brep the_editor_binds_every_keyboard_verb --test-threads 1 >> "$1" 2>&1
echo "END rc=$? $(date '+%T')" >> "$1"
