#!/usr/bin/env bash
# Runs semio-plugin-reactor-jobs --lib natively in private dirs. Usage: reactor-jobs-run.sh <log tag> [test filter...]
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/rj"; mkdir -p "$G" /c/t/rjtmp
export TMP='C:\t\rjtmp' TEMP='C:\t\rjtmp' TMPDIR='C:\t\rjtmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_TARGET_DIR='C:\t\rj' CARGO_BUILD_BUILD_DIR='C:\t\rj'
TAG="$1"; shift
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/📦️packages/🦀️rust" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-plugin-reactor-jobs --lib --no-fail-fast -- "$@" > "$G/$TAG.log" 2>&1
echo "rc=$?" >> "$G/$TAG.log"
