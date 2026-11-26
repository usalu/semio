#!/usr/bin/env bash
# Runs a FEM 3D results law natively. Usage: f3-law.sh <filter> <tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/f3" /c/t/f3btmp
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_BUILD_BUILD_DIR='C:\t\f3b' TMP='C:\t\f3btmp' TEMP='C:\t\f3btmp' TMPDIR='C:\t\f3btmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/f3/law-$2.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && CARGO_TARGET_DIR='C:\t\f3b' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --test editor_app_2 -- --nocapture "$1") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
