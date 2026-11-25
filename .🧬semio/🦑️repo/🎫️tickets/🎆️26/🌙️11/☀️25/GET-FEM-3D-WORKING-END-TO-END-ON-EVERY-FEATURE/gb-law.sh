#!/usr/bin/env bash
# 🧪️ Runs FEM 3D editor app laws natively in own gb target. Usage: gb-law.sh <filter> <tag> [test target]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/gb" /c/t/gbt
export CARGO_BUILD_BUILD_DIR='C:\t\g' TMP='C:\t\gbt' TEMP='C:\t\gbt' TMPDIR='C:\t\gbt' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
LOG="$M/🗑️generated/gb/${2:-law}.log"
(cd "$A" && CARGO_TARGET_DIR='C:\t\g' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "${4:-semio-s-artifact-fem-3d-editor}" --test "${3:-editor_app}" -- --nocapture $1) > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error|rc=|\[DEBUG\]|panicked" "$LOG" | head -80
