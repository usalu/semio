#!/usr/bin/env bash
# Runs the FEM 3D session's production numerical-child demo solves natively.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
mkdir -p /c/t/tmp
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\3d' CARGO_BUILD_BUILD_DIR='C:\t\3db' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
cargo test -j 3 -p semio-s-artifact-fem-3d-editor-session --lib fem3d_production_numerical_child -- --nocapture > "$T/🗑️generated/a3d-session-demo.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/a3d-session-demo.log"
