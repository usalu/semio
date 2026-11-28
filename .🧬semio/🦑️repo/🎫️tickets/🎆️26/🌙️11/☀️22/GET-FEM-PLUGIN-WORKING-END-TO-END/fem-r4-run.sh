#!/usr/bin/env bash
# 🧊️ Runs the R4 keep-results laws natively in private r4 dirs (2D+3D results window and 2D session lib tests).
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/r4" /c/t/r4tmp
export CARGO_BUILD_BUILD_DIR='C:\t\r4' CARGO_TARGET_DIR='C:\t\r4' TMP='C:\t\r4tmp' TEMP='C:\t\r4tmp' TMPDIR='C:\t\r4tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$T/🗑️generated/r4/run.log"; : > "$LOG"
run() { (cd "$A/$1" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$2" --lib -- "$3") >> "$LOG" 2>&1; echo "rc[$2]=$?" >> "$LOG"; }
run 🧊️3d semio-s-artifact-fem-3d-editor-results-window display_setting_edit
run 🧊️3d semio-s-artifact-fem-3d-editor-results-window mode_wiring
echo done >> "$LOG"
