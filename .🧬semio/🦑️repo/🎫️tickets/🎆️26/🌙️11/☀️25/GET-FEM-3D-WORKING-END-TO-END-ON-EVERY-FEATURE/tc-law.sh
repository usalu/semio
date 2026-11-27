#!/usr/bin/env bash
# 🧪️ Runs one 3D law natively in the private tq dirs. Usage: tc-law.sh <test filter> <log tag> [package] [test target or --lib] [workspace dir]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/tq" /c/t/tqtmp
export CARGO_BUILD_BUILD_DIR='C:/t/tq' TMP='C:/t/tqtmp' TEMP='C:/t/tqtmp' TMPDIR='C:/t/tqtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/tq/law-${2:-a}.log"
PKG="${3:-semio-s-artifact-fem-3d-editor-visual-job}"
if [ "${4:---lib}" = "--lib" ]; then TARGET="--lib"; else TARGET="--test $4"; fi
WS="${5:-/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d}"
(cd "$WS" && CARGO_TARGET_DIR='C:/t/tq' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" $TARGET -- --nocapture "${1:-probe}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
