#!/usr/bin/env bash
# 🧪️ Runs a visual-job lib law natively, reusing the fx7 build dir (disk is tight). Usage: hr-law.sh <tag> [test filter...]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/hr" /c/t/hrtmp
export CARGO_TARGET_DIR='C:\t\fx7' CARGO_BUILD_BUILD_DIR='C:\t\fx7b' TMP='C:\t\hrtmp' TEMP='C:\t\hrtmp' TMPDIR='C:\t\hrtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728 RUST_BACKTRACE=full
LOG="$M/🗑️generated/hr/$1.log"; shift
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 3 --no-fail-fast --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor-visual-job --lib -- "$@") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
