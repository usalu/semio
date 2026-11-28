#!/usr/bin/env bash
# 🧭️ Runs the FEM gumball lib laws natively in the private gb lane. Usage: gb-law.sh <2d|3d> <tag> <test filter>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/gb"; mkdir -p "$G" /c/t/gbtmp
export CARGO_TARGET_DIR='C:\t\gb' CARGO_BUILD_BUILD_DIR='C:\t\gb' TMP='C:\t\gbtmp' TEMP='C:\t\gbtmp' TMPDIR='C:\t\gbtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$G/law-$1-$2.log"
if [ "$1" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor-interaction; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor-interaction; fi
(cd "$DIR" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --lib -- --nocapture "$3") > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
