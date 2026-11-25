#!/usr/bin/env bash
# 📋️ Checks the framework clipboard work and the FEM 3D clipboard natively in the vc3 target dir.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/vc3" /c/t/tmp
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🪟️surface/📦️packages/🦀️rust" && CARGO_TARGET_DIR='C:\t\vc3' cargo check -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-plugin-decl-surface > "$T/🗑️generated/vc3/surface-check.log" 2>&1
echo "rc=$?" >> "$T/🗑️generated/vc3/surface-check.log"
grep -E "^error|rc=" -A12 "$T/🗑️generated/vc3/surface-check.log" | head -80
