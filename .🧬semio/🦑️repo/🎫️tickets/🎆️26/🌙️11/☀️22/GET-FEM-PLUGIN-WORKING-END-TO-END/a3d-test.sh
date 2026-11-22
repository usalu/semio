#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
export TMP='G:\t\tmp' TEMP='G:\t\tmp' TMPDIR='G:\t\tmp' CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='G:\t\3d' CARGO_BUILD_BUILD_DIR='G:\t\3db' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
G="$T/🗑️generated"
cargo test --workspace --exclude semio-s-artifact-fem-3d-editor --no-fail-fast --message-format short > "$G/a3d-test-rest.log" 2>&1
echo "rc=$?" >> "$G/a3d-test-rest.log"
cargo test -p semio-s-artifact-fem-3d-editor --lib --test editor_app_2 --test statik_pane_smoke --no-fail-fast --message-format short > "$G/a3d-test-editor.log" 2>&1
echo "rc=$?" >> "$G/a3d-test-editor.log"
