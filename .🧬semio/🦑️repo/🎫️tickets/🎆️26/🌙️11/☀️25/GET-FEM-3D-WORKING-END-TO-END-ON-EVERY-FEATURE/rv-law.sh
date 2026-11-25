#!/usr/bin/env bash
# 🧠️ Runs one 3D FEM law natively in the private rv dirs. Usage: rv-law.sh <test filter> <log tag> [test target]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/rv" /c/t/rvtmp
export CARGO_BUILD_BUILD_DIR='C:\t\rv' TMP='C:\t\rvtmp' TEMP='C:\t\rvtmp' TMPDIR='C:\t\rvtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/rv/law-${2:-a}.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && CARGO_TARGET_DIR='C:\t\rv' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --test "${3:-editor_app_2}" -- --nocapture "${1:-superseded_solve}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=|\[DEBUG\]|panicked" "$LOG" | head -40
