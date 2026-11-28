#!/usr/bin/env bash
# 🧠️ Runs one FEM editor law natively in the private vs dirs. Usage: vs-law.sh <3d|2d> <test filter> <log tag> [test target]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/vs" /c/t/vstmp
export CARGO_BUILD_BUILD_DIR='C:\t\vs' TMP='C:\t\vstmp' TEMP='C:\t\vstmp' TMPDIR='C:\t\vstmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$T/🗑️generated/vs/law-${3:-a}.log"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\vs' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test "${4:-editor_app_2}" -- --nocapture "${2:-superseded_solve}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
