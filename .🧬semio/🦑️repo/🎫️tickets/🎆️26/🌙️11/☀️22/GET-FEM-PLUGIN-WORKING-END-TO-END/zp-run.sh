#!/usr/bin/env bash
# Runs the wall-clock zero-progress laws natively in private dirs C:\t\zp. Usage: zp-run.sh <tag> <crate-dir-relative-to-plugin> <package> [test filter...]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/zp"; mkdir -p "$G" /c/t/zptmp
export TMP='C:\t\zptmp' TEMP='C:\t\zptmp' TMPDIR='C:\t\zptmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_TARGET_DIR='C:\t\zp' CARGO_BUILD_BUILD_DIR='C:\t\zp'
TAG="$1"; DIR="$2"; PKG="$3"; shift 3
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/$DIR" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --lib --no-fail-fast -- "$@" > "$G/$TAG.log" 2>&1
echo "rc=$?" >> "$G/$TAG.log"
