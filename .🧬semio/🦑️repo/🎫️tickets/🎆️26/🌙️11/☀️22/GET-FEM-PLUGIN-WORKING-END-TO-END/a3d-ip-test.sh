#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\3dy' CARGO_BUILD_BUILD_DIR='C:\t\3dyb' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
G="$T/🗑️generated"
cargo test -j 6 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor-interaction -p semio-s-artifact-fem-3d-editor-panels --lib --no-fail-fast --message-format short > "$G/ip-test.log" 2>&1
echo "rc=$?" >> "$G/ip-test.log"
