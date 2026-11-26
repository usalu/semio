#!/usr/bin/env bash
# 🔺️ Runs the LDLT crate's unit laws natively in the private ld dirs of the ✏️s workspace.
export PATH="/usr/bin:/bin:$PATH"
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/mb" /c/t/ldtmp
export CARGO_TARGET_DIR='C:\t\ld' CARGO_BUILD_BUILD_DIR='C:\t\ld' TMP='C:\t\ldtmp' TEMP='C:\t\ldtmp' TMPDIR='C:\t\ldtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false
LOG="$M/🗑️generated/mb/law-ldlt.log"
(cd /c/semio/✏️s && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-fem-sparse-ldlt --lib -- --nocapture ${1:-}) > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
