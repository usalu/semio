#!/usr/bin/env bash
# Runs a lib test of a workspace-excluded crate standalone: $1 = crate dir (holding Cargo.toml), $2 = filter, $3 = log name. Removes the nested lock it creates.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated/escrow"; mkdir -p "$G"
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\escrow-standalone' RUST_MIN_STACK=134217728 RUSTC_WRAPPER=
had=0; [ -f "$1/Cargo.lock" ] && had=1
cargo test --offline -j 6 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' --manifest-path "$1/Cargo.toml" --lib -- "$2" --nocapture > "$G/$3.log" 2>&1
echo "rc=$?" >> "$G/$3.log"
[ $had = 0 ] && rm -f "$1/Cargo.lock"
