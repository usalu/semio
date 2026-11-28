#!/usr/bin/env bash
# 🕰️ Runs the FEM history laws natively. Usage: hu-law.sh <2d|3d> <tag> [test filter] [test target]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/hu"; mkdir -p "$G" "/c/t/hu$1tmp"
export CARGO_TARGET_DIR="C:\t\hu$1" CARGO_BUILD_BUILD_DIR="C:\t\hu$1" TMP="C:\t\hu$1tmp" TEMP="C:\t\hu$1tmp" TMPDIR="C:\t\hu$1tmp" CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$G/law-$1-$2.log"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
(cd "$DIR" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test "${4:-editor_app_2}" -- --nocapture "${3:-history_undo_and_alternatives}") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
