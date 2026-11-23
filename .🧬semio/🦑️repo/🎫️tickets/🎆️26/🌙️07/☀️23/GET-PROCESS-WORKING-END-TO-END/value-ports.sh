#!/bin/zsh
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END"
export RUSTUP_TOOLCHAIN=nightly-2026-07-20-aarch64-apple-darwin
for t in "$@"; do $T/stale-cargo.sh vp-$t test --offline --message-format short --manifest-path 🧰️framework/Cargo.toml -p semio-framework-value --test $t --no-run; done
