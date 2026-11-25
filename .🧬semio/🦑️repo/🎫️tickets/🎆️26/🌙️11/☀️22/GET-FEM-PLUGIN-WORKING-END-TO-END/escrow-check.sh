#!/usr/bin/env bash
# Checks the given packages natively (tests included): $1 = log name, rest = cargo package names.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated/escrow"; mkdir -p "$G"; log="$1"; shift
cd "${WS:-/c/semio/🧰️framework}" || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\escrow' RUST_MIN_STACK=134217728 RUSTC_WRAPPER=
args=(); for p in "$@"; do args+=(-p "$p"); done
cargo check -j 6 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "${args[@]}" ${TESTS:+--tests} --message-format short > "$G/$log.log" 2>&1
echo "rc=$?" >> "$G/$log.log"
