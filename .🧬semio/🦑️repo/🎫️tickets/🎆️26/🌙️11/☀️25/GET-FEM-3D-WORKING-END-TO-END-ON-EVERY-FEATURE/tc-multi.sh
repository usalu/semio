#!/usr/bin/env bash
# 🧪️ Runs the --lib laws of several packages natively in the private tq dirs. Usage: tc-multi.sh <log tag> <package>...
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/tq" /c/t/tqtmp
export CARGO_BUILD_BUILD_DIR='C:/t/tq' TMP='C:/t/tqtmp' TEMP='C:/t/tqtmp' TMPDIR='C:/t/tqtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/tq/multi-$1.log"; shift
ARGS=(); for p in "$@"; do ARGS+=(-p "$p"); done
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && CARGO_TARGET_DIR='C:/t/tq' cargo test -j 3 --no-fail-fast --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "${ARGS[@]}" --lib) > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
