#!/usr/bin/env bash
# 🧠️ Runs the remove-selection refusal laws natively (3D then 2D) in the private rf target. Detached launcher: /c/t/rf-run.sh
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; mkdir -p "$T/🗑️generated/rf"
export CARGO_TARGET_DIR='C:\t\rf' CARGO_BUILD_BUILD_DIR='C:\t\rf' TMP='C:\t\rftmp' TEMP='C:\t\rftmp' TMPDIR='C:\t\rftmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
for d in 3d 2d; do
  if [ "$d" = 2d ]; then DIR="$A/◻️2d"; PKG=semio-s-artifact-fem-2d-editor; else DIR="$A/🧊️3d"; PKG=semio-s-artifact-fem-3d-editor; fi
  LOG="$T/🗑️generated/rf/law-$d.log"
  (cd "$DIR" && cargo test -j 3 --config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""' -p "$PKG" --test editor_app_2 -- --nocapture "${1:-remove_selection}") > "$LOG" 2>&1
  echo "rc=$?" >> "$LOG"
done
