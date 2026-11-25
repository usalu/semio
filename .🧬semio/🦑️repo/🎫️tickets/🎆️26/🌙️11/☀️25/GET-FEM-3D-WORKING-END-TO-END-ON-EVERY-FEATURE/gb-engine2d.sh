#!/usr/bin/env bash
# 🧪️ Runs the FEM 3D engine unit laws natively in own gc target. Usage: gb-engine.sh <filter> <tag> [package]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/gb" /c/t/gct
export CARGO_BUILD_BUILD_DIR='C:\t\gd' TMP='C:\t\gct' TEMP='C:\t\gct' TMPDIR='C:\t\gct' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
LOG="$M/🗑️generated/gb/${2:-eng2d}.log"
(cd "$A" && CARGO_TARGET_DIR='C:\t\gd' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-2d-engine --lib -- --nocapture $1) > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error|rc=|\[DEBUG\]|panicked" "$LOG" | head -80
