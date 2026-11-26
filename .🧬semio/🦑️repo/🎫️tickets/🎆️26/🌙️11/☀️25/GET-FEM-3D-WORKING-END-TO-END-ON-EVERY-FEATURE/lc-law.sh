#!/usr/bin/env bash
# 🧪️ Runs the FEM 3D live-cleanup zero-progress law natively in a private build dir. Usage: lc-law.sh <tag> [test filter...]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/lc" /c/t/lctmp
export CARGO_TARGET_DIR='C:\t\lc' CARGO_BUILD_BUILD_DIR='C:\t\lc' CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false TMP='C:\t\lctmp' TEMP='C:\t\lctmp' TMPDIR='C:\t\lctmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728 RUST_BACKTRACE=full
LOG="$M/🗑️generated/lc/$1.log"; shift
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 3 --no-fail-fast --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --test editor_live_cleanup -- --nocapture "$@") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
