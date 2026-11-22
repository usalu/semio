#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
cd "/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d" || exit 2
export TMP='G:\t\tmp' TEMP='G:\t\tmp' TMPDIR='G:\t\tmp' CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='G:\t\3d' CARGO_BUILD_BUILD_DIR='G:\t\3db' CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
OUT="$T/🗑️generated/a3d-editor-$1.jsonl"
shift
cargo check -p semio-s-artifact-fem-3d-editor --tests --keep-going --message-format json-diagnostic-short "$@" > "$OUT" 2> "$OUT.err"
echo "rc=$?" >> "$OUT.err"
python "$T/a3d-editor-errors.py" "$OUT" > "$OUT.txt"
