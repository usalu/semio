#!/usr/bin/env bash
# ⏱️ Runs a FEM 3D visual-job law natively in C:\t\st. Usage: settle-law.sh <filter> <tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/st" /c/t/sttmp
export PATH="/usr/bin:/bin:$PATH" CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_TARGET_DIR='C:\t\st' CARGO_BUILD_BUILD_DIR='C:\t\st' TMP='C:\t\sttmp' TEMP='C:\t\sttmp' TMPDIR='C:\t\sttmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/st/law-$2.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 4 --release --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor-visual-job --lib -- --nocapture "$1") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
