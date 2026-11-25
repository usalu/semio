#!/usr/bin/env bash
# 🧪️ Runs the FEM 3D editor app laws matching the filter in $1 natively in an own target dir (p3).
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/p3" /c/t/tmp
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
(cd "$A" && CARGO_TARGET_DIR='C:\t\p3' cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --test editor_app -- --nocapture $1) > "$T/🗑️generated/p3/${2:-law}.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/p3/${2:-law}.log"
grep -E "^test |test result|^error|rc=|\[DEBUG\]|panicked" "$T/🗑️generated/p3/${2:-law}.log" | head -60
