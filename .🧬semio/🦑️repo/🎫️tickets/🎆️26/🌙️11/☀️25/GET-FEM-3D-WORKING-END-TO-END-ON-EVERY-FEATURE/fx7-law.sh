#!/usr/bin/env bash
# 🧪️ Runs the seven FEM 3D editor_app laws natively. Usage: fx7-law.sh <tag>
M="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️25/GET-FEM-3D-WORKING-END-TO-END-ON-EVERY-FEATURE"; mkdir -p "$M/🗑️generated/fx7" /c/t/fx7tmp
export CARGO_TARGET_DIR='C:\t\fx7' CARGO_BUILD_BUILD_DIR='C:\t\fx7b' TMP='C:\t\fx7tmp' TEMP='C:\t\fx7tmp' TMPDIR='C:\t\fx7tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728 RUST_BACKTRACE=1
LOG="$M/🗑️generated/fx7/$1.log"
(cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" && cargo test -j 3 --no-fail-fast --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p semio-s-artifact-fem-3d-editor --test editor_app -- add_combination:: add_nodal_load::add_combination add_solid:: artifact_panel::every_tree context::history_edits_end_to_end model_window::) > "$LOG" 2>&1
echo "rc=$?" >> "$LOG"
