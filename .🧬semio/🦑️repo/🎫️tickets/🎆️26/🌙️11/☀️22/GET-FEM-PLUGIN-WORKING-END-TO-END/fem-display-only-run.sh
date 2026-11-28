#!/usr/bin/env bash
# 🔏️ Runs the display-only no-resolve laws natively in private pf dirs. Usage: fem-display-only-run.sh <2d|3d>
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/pf" /c/t/pftmp
export CARGO_BUILD_BUILD_DIR='C:\z' CARGO_TARGET_DIR='C:\z' TMP='C:\t\pftmp' TEMP='C:\t\pftmp' TMPDIR='C:\t\pftmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_P_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
LOG="$T/🗑️generated/pf/run-$1.log"; : > "$LOG"
run() { (cd "$A/$1" && cargo test -j 3 --profile p --config 'profile.p.inherits="dev"' --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' "${@:2}") >> "$LOG" 2>&1; echo "rc[$*]=$?" >> "$LOG"; }
if [ "$1" = 2d ]; then
  run ◻️2d -p semio-s-artifact-fem-2d-model --test structure_laws -- physics display_only
  run ◻️2d -p semio-s-artifact-fem-2d-editor-session --lib -- display_only physics_edit revision_change
else
  run 🧊️3d -p semio-s-artifact-fem-3d-editor-session --lib
fi
echo done >> "$LOG"
