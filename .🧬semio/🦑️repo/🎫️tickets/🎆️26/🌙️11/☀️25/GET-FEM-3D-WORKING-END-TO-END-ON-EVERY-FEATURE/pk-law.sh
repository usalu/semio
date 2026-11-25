#!/usr/bin/env bash
# 🧪️ Runs FEM playback laws natively. Usage: pk-law.sh <3d|2d> <filter> <tag> [test target]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/pk" /c/t/pktmp
export CARGO_BUILD_BUILD_DIR='C:\t\pk' TMP='C:\t\pktmp' TEMP='C:\t\pktmp' TMPDIR='C:\t\pktmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$M/🗑️generated/pk/law-$3.log"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\pk' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test "${4:-editor_app_2}" -- --nocapture --test-threads=1 "$2") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=|panicked|\[DEBUG\]" "$LOG" | head -40
