#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"
until grep -q "all done" "$G/fse-progress.log"; do sleep 10; done
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\rc' CARGO_BUILD_BUILD_DIR='C:\t\rcb' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
echo "[$(date +%T)] start 3d-core-2" >> "$G/fse-progress.log"
cargo test -j 3 -p semio-s-artifact-fem-3d-engine -p semio-s-artifact-fem-3d-io -p semio-s-artifact-fem-3d-viewer -p semio-s-artifact-fem-3d-model --no-fail-fast --message-format short > "$G/fse-3d-core-2.log" 2>&1
rc=$?; echo "rc=$rc" >> "$G/fse-3d-core-2.log"
echo "[$(date +%T)] end 3d-core-2 rc=$rc" >> "$G/fse-progress.log"
