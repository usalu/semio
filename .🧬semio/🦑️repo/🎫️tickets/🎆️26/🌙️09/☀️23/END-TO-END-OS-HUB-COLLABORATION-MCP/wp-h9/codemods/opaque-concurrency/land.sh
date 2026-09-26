#!/bin/zsh
# ⚔️ Lands both halves of the opaque-concurrency patch (coordinator item (a)) in the post-publish window, then proves
# them: the db half (host), the store half (guest-linked: needs the freeze lifted), the hub fixture law.
# usage: land.sh [--dry-run]   (all three codemods are dry-run first; nothing is written unless all anchors match)
set -e
cd /Users/ueli/Documents/semio
HERE=.tmp-ticket/wp-h9/codemods/opaque-concurrency
for mod in db-opaque-concurrency store-causal-dependencies hub-vigilant-law; do python3 $HERE/$mod.py --dry-run > /dev/null; done
[ "$1" = "--dry-run" ] && { echo "dry run clean"; exit 0; }
for mod in db-opaque-concurrency store-causal-dependencies hub-vigilant-law; do python3 $HERE/$mod.py; done
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h9/target RUST_MIN_STACK=268435456
nice -n 15 cargo nextest run -p semio-framework-os-kernel-db --lib --no-fail-fast -E 'test(/artifact|conflict|durable_group|submit/)'
nice -n 15 cargo nextest run -p semio-framework-os-kernel --lib --no-fail-fast -E 'test(/store|replay|dependenc/)'
nice -n 15 cargo test -p semio-hub --bin os-hub --features native-artifact-execution,integration-fixtures --no-fail-fast -- a_vigilant_hub_refuses_an_opaque_write_authored_without_seeing_the_head a_withdrawn_delegation
zsh .tmp-ticket/📜️fleet-mutex.sh wasm h9 -- nice -n 15 cargo check -p semio-framework-os-kernel --target wasm32-unknown-unknown --profile wasm-dev
