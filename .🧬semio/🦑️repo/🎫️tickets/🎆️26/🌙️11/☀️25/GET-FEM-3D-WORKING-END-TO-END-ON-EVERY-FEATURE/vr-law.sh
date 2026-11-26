#!/usr/bin/env bash
# 🧪️ Runs the window-authority and history-move laws natively in private dirs, then compiles the instance crate.
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; G="$M/🗑️generated/vr"; mkdir -p "$G" /c/t/vt
export TMP='C:\t\vt' TEMP='C:\t\vt' TMPDIR='C:\t\vt' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
export CARGO_TARGET_DIR='C:\t\v' CARGO_BUILD_BUILD_DIR='C:\t\v'
C=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""')
cd /c/semio
cargo test -j 3 "${C[@]}" -p semio-plugin-window-config --test commit_policy > "$G/law-job.log" 2>&1; echo "rc=$?" >> "$G/law-job.log"
cargo test -j 3 "${C[@]}" -p semio-plugin-window-config --lib -- stale_authority_law > "$G/law.log" 2>&1; echo "rc=$?" >> "$G/law.log"
cargo check -j 3 "${C[@]}" -p semio-plugin-instance-app --lib > "$G/check.log" 2>&1; echo "rc=$?" >> "$G/check.log"
