#!/usr/bin/env bash
# Runs the FEM 2D session tests natively (mounted Demo under a distinct host-bound operation, eigen, unit).
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
mkdir -p /c/t/tmp
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\2ds' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
cargo --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' test -j 3 -p semio-s-artifact-fem-2d-editor-session --no-fail-fast > "$T/🗑️generated/a2d-session-demo.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/a2d-session-demo.log"
