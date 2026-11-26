#!/usr/bin/env bash
# 🧪️ Runs one 3D law natively in the second private mc dirs, beside mc-law.sh. Usage: mc-law.sh <test filter> <log tag> [package] [test target or --lib]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/mc" /c/t/mctmp
export CARGO_BUILD_BUILD_DIR='C:\t\mc' TMP='C:\t\mctmp' TEMP='C:\t\mctmp' TMPDIR='C:\t\mctmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/mc/law-${2:-a}.log"
PKG="${3:-semio-s-artifact-fem-3d-editor-visual-job}"
if [ "${4:---lib}" = "--lib" ]; then TARGET="--lib"; else TARGET="--test $4"; fi
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && CARGO_TARGET_DIR='C:\t\mc' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" $TARGET -- --nocapture "${1:-probe}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
