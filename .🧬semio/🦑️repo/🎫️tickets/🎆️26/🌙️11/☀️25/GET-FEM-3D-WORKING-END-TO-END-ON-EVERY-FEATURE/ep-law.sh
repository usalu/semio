#!/usr/bin/env bash
# 🧪️ Editor-parity laws, native, private dirs C:\t\ep*. Usage: ep-law.sh <vcs|fem3d|fem2d> <tag> [test filter] [test target]
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/ep" /c/t/eptmp
export CARGO_BUILD_BUILD_DIR='C:\t\epb' TMP='C:\t\eptmp' TEMP='C:\t\eptmp' TMPDIR='C:\t\eptmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728 CARGO_TARGET_DIR='C:\t\ep'
LOG="$M/🗑️generated/ep/$2.log"
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
case "$1" in
  fem3d|fem2d) export CARGO_TARGET_DIR='C:\t\epf' CARGO_BUILD_BUILD_DIR='C:\t\epf' ;;
esac
case "$1" in
  vcs) (cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌅️bin-05/📦️packages/🦀️rust" && cargo test -j 3 --config 'unstable.build-dir-new-layout=false' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-os-store-tests-05 --lib -- ${3:-alternative}) > "$LOG" 2>&1 ;;
  fem3d) (cd "$A/🧊️3d" && cargo test -j 3 --config 'unstable.build-dir-new-layout=false' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --test "${4:-editor_app}" -- ${3:-}) > "$LOG" 2>&1 ;;
  fem2d) (cd "$A/◻️2d" && cargo test -j 3 --config 'unstable.build-dir-new-layout=false' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-2d-editor --test "${4:-editor_app}" -- ${3:-}) > "$LOG" 2>&1 ;;
esac
echo "rc=$?" >> "$LOG"
grep -E "^test |test result|^error(\[|:)|rc=" "$LOG" | cut -c1-240 | head -80
