#!/usr/bin/env bash
# 📋️ Runs the FEM 3D clipboard laws natively in private dirs.
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; G="$M/🗑️generated/cp"; mkdir -p "$G" /c/t/cptmp
export TMP='C:\t\cptmp' TEMP='C:\t\cptmp' TMPDIR='C:\t\cptmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728 RUST_BACKTRACE=1
export CARGO_TARGET_DIR='C:\t\c'
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"; C=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""')
cd "$A/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust" && cargo test -j 3 "${C[@]}" -p semio-s-artifact-fem-3d-editor --lib -- clipboard > "$G/${1:-clip3d}.log" 2>&1; echo "rc=$?" >> "$G/${1:-clip3d}.log"
