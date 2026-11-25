#!/usr/bin/env bash
# ⌨️ Runs the FEM 2D/3D editor manifest laws (history keys, initial utility, 3D action args) natively in their own target dirs.
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated" /c/t/tmp
export TMP='C:\t\tmp' TEMP='C:\t\tmp' TMPDIR='C:\t\tmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
run() { (cd "$A/$1" && CARGO_TARGET_DIR="$2" cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$3" --test editor_app -- $4) > "$T/🗑️generated/history-keys-$5.log" 2>&1; echo "rc=$?" >> "$T/🗑️generated/history-keys-$5.log"; grep -E "^test |test result|^error(\[|:)|rc=" "$T/🗑️generated/history-keys-$5.log" | head -20; }
run "◻️2d" 'C:\t\law' semio-s-artifact-fem-2d-editor "history_keys_bind canvas_windows_arm" 2d
run "🧊️3d" 'C:\t\3dargs' semio-s-artifact-fem-3d-editor "history_keys_bind every_mutation_action_declares_args" 3d
