#!/usr/bin/env bash
# 🧪️ Runs one 3D law natively in the second private ri dirs, beside xi-law.sh. Usage: xi-law.sh <test filter> <log tag> [package] [test target or --lib]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/xi" /c/t/xitmp
export CARGO_BUILD_BUILD_DIR='C:\t\xi' TMP='C:\t\xitmp' TEMP='C:\t\xitmp' TMPDIR='C:\t\xitmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/xi/law-${2:-a}.log"
PKG="${3:-semio-s-artifact-fem-3d-editor-visual-job}"
if [ "${4:---lib}" = "--lib" ]; then TARGET="--lib"; else TARGET="--test $4"; fi
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && CARGO_TARGET_DIR='C:\t\xi' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" $TARGET -- --nocapture "${1:-probe}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
