#!/usr/bin/env bash
# Runs one escrow law natively: $1 = cargo package, $2 = test target, $3 = filter, $4 = log name.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated/escrow"; mkdir -p "$G"
cd "${WS:-/c/semio/🧰️framework}" || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\escrow' RUST_MIN_STACK=134217728 RUSTC_WRAPPER=
cargo test -j 6 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$1" --features testkit --test "$2" -- "$3" --nocapture > "$G/$4.log" 2>&1
echo "rc=$?" >> "$G/$4.log"
