#!/usr/bin/env bash
# 🧪️ Runs every test target of the FEM 3D editor crate (lib, editor_app, editor_app_2, ...) natively, no fail-fast. Usage: full-3d-editor.sh <tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/full" /c/t/fulltmp
export CARGO_TARGET_DIR='C:\t\full' CARGO_BUILD_BUILD_DIR='C:\t\full' TMP='C:\t\fulltmp' TEMP='C:\t\fulltmp' TMPDIR='C:\t\fulltmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/full/$1.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 4 --no-fail-fast --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor) > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
