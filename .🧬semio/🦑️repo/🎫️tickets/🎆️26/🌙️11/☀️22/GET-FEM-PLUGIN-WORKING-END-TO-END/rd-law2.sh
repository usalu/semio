#!/usr/bin/env bash
# 🔁️ Runs the FEM redo-stall laws natively in the private rd lane. Usage: rd-law.sh <2d|3d> <tag> <test filter> [test target] [package suffix]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/rd"; mkdir -p "$G" /c/t/rdtmp
export CARGO_TARGET_DIR='C:\t\rd' CARGO_BUILD_BUILD_DIR='C:\t\rd' TMP='C:\t\rdtmp' TEMP='C:\t\rdtmp' TMPDIR='C:\t\rdtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$G/law-$1-$2.log"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
(cd "$DIR" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG${5:-}" ${4:---test editor_app_2} -- --nocapture --test-threads 1 "$3") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
