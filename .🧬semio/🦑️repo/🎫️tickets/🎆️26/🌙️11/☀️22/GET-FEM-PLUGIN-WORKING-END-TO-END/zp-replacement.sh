#!/usr/bin/env bash
# Runs the 3D/2D replacement laws and the reactor-jobs selection crossing law in private C:\t\zp. Detached, -j 3.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/zp"; mkdir -p "$G" /c/t/zptmp
export TMP='C:\t\zptmp' TEMP='C:\t\zptmp' TMPDIR='C:\t\zptmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
export CARGO_UNSTABLE_BUILD_DIR_NEW_LAYOUT=false CARGO_UNSTABLE_FINE_GRAIN_LOCKING=false CARGO_TARGET_DIR='C:\t\zp' CARGO_BUILD_BUILD_DIR='C:\t\zp'
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"; CFG=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""')
(cd "/c/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/📦️packages/🦀️rust" && cargo test -j 3 "${CFG[@]}" -p semio-plugin-reactor-jobs --lib -- a_selection_or_tool_switch > "$G/sel.log" 2>&1; echo "rc=$?" >> "$G/sel.log")
(cd "$A/🧊️3d" && cargo test -j 3 "${CFG[@]}" -p semio-s-artifact-fem-3d-editor --test editor_live_cleanup -- --nocapture replacement_revokes > "$G/r3d.log" 2>&1; echo "rc=$?" >> "$G/r3d.log")
(cd "$A/◻️2d" && cargo test -j 3 "${CFG[@]}" -p semio-s-artifact-fem-2d-editor --test editor_live_cleanup -- --nocapture replacement_revokes > "$G/r2d.log" 2>&1; echo "rc=$?" >> "$G/r2d.log")
