#!/usr/bin/env bash
# 🧪 Runs the unknown-reference refusal laws natively in the private pa lane. Usage: pa-law.sh <3d|2d> <test filter> <log tag>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/pa" /c/t/patmp
export CARGO_BUILD_BUILD_DIR='C:\t\pa' TMP='C:\t\patmp' TEMP='C:\t\patmp' TMPDIR='C:\t\patmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="/c/t/pa-law-${3:-a}.log"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
(cd "$DIR" && CARGO_TARGET_DIR='C:\t\pa' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test "${4:-editor_app}" -- --nocapture ${2:-unknown_entity}) > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
