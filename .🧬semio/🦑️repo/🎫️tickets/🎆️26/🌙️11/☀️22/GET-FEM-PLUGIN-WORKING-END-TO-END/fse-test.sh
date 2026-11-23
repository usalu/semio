#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"
mkdir -p "$G" /c/t/tmp
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_TARGET_DIR='C:\t\rc' CARGO_BUILD_BUILD_DIR='C:\t\rcb' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
step(){ local name=$1; shift; echo "[$(date +%T)] start $name" >> "$G/fse-progress.log"; "$@" > "$G/fse-$name.log" 2>&1; local rc=$?; echo "rc=$rc" >> "$G/fse-$name.log"; echo "[$(date +%T)] end $name rc=$rc" >> "$G/fse-progress.log"; }
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
step 3d-core cargo test -j 3 -p semio-s-artifact-fem-3d-engine -p semio-s-artifact-fem-3d-io -p semio-s-artifact-fem-3d-viewer -p semio-s-artifact-fem-3d-model --no-fail-fast --message-format short
step 3d-results cargo test -j 3 -p semio-s-artifact-fem-3d-editor --test editor_app_2 --no-fail-fast --message-format short -- results
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d" || exit 2
step 2d-core cargo test -j 3 -p semio-s-artifact-fem-2d-model -p semio-s-artifact-fem-2d-viewer --no-fail-fast --message-format short
echo "[$(date +%T)] all done" >> "$G/fse-progress.log"
