#!/usr/bin/env bash
# 🧪️ Runs the viewer-fix laws natively (private C:\t\vw): viewer-fixes-law.sh -> 🗑️generated/viewer-fixes/law-<crate>.log
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/viewer-fixes"; mkdir -p "$G" /c/t/vwtmp
export CARGO_TARGET_DIR='C:\t\vw' CARGO_BUILD_BUILD_DIR='C:\t\vw' TMP='C:\t\vwtmp' TEMP='C:\t\vwtmp' TMPDIR='C:\t\vwtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem"
run() { (cd "$1" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$2" --lib -- "${@:3}") > "$G/law-$2.log" 2>&1; echo "rc=$?" >> "$G/law-$2.log"; }
run "$A" semio-s-fem-app-surface viewer_
run "$A/🗿️artifacts/◻️2d" semio-s-artifact-fem-2d-viewer
run "$A/🗿️artifacts/🧊️3d" semio-s-artifact-fem-3d-viewer
echo done > "$G/law-done.txt"
