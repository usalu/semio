#!/usr/bin/env bash
# 🪦️ Runs one native law group privately. Usage: fem3d-results-live-cleanup-run.sh <tag> <cargo test args...> (cwd given by $LC_CWD)
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/lc" /c/t/lctmp
export CARGO_BUILD_BUILD_DIR='C:\t\lc' CARGO_TARGET_DIR='C:\t\lc' TMP='C:\t\lctmp' TEMP='C:\t\lctmp' TMPDIR='C:\t\lctmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
TAG="$1"; shift; LOG="$T/🗑️generated/lc/run-$TAG.log"; : > "$LOG"
cd "$LC_CWD" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "$@" >> "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
