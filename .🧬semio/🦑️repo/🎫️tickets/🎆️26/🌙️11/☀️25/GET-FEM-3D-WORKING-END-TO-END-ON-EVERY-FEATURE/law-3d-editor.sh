#!/usr/bin/env bash
# 🧪️ Runs FEM 3D editor lib laws natively with a private target/build dir. Usage: law-3d-editor.sh <filter> <tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/co" /c/t/cotmp
export CARGO_TARGET_DIR='C:\t\co' CARGO_BUILD_BUILD_DIR='C:\t\co' TMP='C:\t\cotmp' TEMP='C:\t\cotmp' TMPDIR='C:\t\cotmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/co/law-$2.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --lib -- --nocapture "$1") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
