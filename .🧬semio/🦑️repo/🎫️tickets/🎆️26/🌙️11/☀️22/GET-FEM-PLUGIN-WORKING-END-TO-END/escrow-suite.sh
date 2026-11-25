#!/usr/bin/env bash
# Runs the whole job context and return budget suites natively after the escrow fix.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated/escrow"; mkdir -p "$G"
cd /c/semio/🧰️framework || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\escrow' RUST_MIN_STACK=134217728 RUSTC_WRAPPER=
cargo test -j 6 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-framework-job-context -p semio-framework-job-return-budget -p semio-framework-job-return --features semio-framework-job-context/testkit --no-fail-fast > "$G/$1.log" 2>&1
echo "rc=$?" >> "$G/$1.log"
