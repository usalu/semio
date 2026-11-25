#!/usr/bin/env bash
# 🧪️ Runs one cargo test target natively. Usage: vp-law.sh <package> <test target> <filter> <tag> [features]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/vp" /c/t/vptmp
export CARGO_BUILD_BUILD_DIR='C:\t\vpb' TMP='C:\t\vptmp' TEMP='C:\t\vptmp' TMPDIR='C:\t\vptmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
LOG="$M/🗑️generated/vp/law-$4.log"
(cd /c/semio && CARGO_TARGET_DIR='C:\t\vp' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$1" ${5:+--features "$5"} --test "$2" -- --nocapture "$3") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=|panicked|\[DEBUG\]" "$LOG" | head -40
