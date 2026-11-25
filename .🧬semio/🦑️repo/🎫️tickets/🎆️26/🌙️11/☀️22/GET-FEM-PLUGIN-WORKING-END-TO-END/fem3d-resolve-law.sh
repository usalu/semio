#!/usr/bin/env bash
# 🧠️ Runs FEM results laws natively in their own rs3 target and build dirs. Usage: fem3d-resolve-law.sh <3d|2d> <test filter> <log tag>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/rs3" /c/t/tmp3
export CARGO_BUILD_BUILD_DIR='C:\t\rs3b' TMP='C:\t\tmp3' TEMP='C:\t\tmp3' TMPDIR='C:\t\tmp3' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$T/🗑️generated/rs3/law-${3:-a}.log"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\rs3' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test "${4:-editor_app_2}" -- --nocapture "${2:-superseded_solve}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=|\[DEBUG\]" "$LOG" | head -40
