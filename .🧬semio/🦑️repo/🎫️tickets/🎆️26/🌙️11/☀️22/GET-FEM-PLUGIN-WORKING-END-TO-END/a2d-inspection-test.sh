#!/usr/bin/env bash
# 🔍️ Runs the FEM 2D inspection unit tests natively in the editor crate's editor_app host (separate target dir). Usage: a2d-inspection-test.sh [filter]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
mkdir -p /c/t/tmp "$T/🗑️generated"
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\2dp' RUSTC_WRAPPER=
cargo test -j 2 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-2d-editor --test editor_app "${1:-inspection}" > "$T/🗑️generated/a2d-inspection-test.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/a2d-inspection-test.log"
