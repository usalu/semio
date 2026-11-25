#!/usr/bin/env bash
# 🧪️ Runs the FEM 2D editor lib unit tests natively. Usage: vp-2d-law.sh <filter> <tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/vp" /c/t/vptmp
export CARGO_BUILD_BUILD_DIR='C:\t\vp' TMP='C:\t\vptmp' TEMP='C:\t\vptmp' TMPDIR='C:\t\vptmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/vp/law-$2.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" && CARGO_TARGET_DIR='C:\t\vp' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-2d-editor --lib -- --nocapture "$1") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
