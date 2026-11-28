#!/usr/bin/env bash
# 🧪 Runs the FEM laws that never ran, one cargo test target at a time, in private dirs C:\t\ts and C:\t\tstmp. Log: 🗑️generated/unrun/rust.log
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"; G="$T/🗑️generated/unrun"; mkdir -p "$G" /c/t/tstmp
export CARGO_TARGET_DIR='C:\t\ts' CARGO_BUILD_BUILD_DIR='C:\t\ts' TMP='C:\t\tstmp' TEMP='C:\t\tstmp' TMPDIR='C:\t\tstmp' CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0 RUSTC_WRAPPER= RUST_MIN_STACK=134217728
A="/c/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts"
C=(--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""')
run() { local dir="$1" log="$2"; shift 2
  for try in 1 2 3; do (cd "$A/$dir" && cargo test -j 3 "${C[@]}" "$@") > "$G/$log.log" 2>&1; rc=$?
    if [ $rc -ne 0 ] && grep -q "STATUS_ACCESS_VIOLATION" "$G/$log.log"; then continue; fi; break; done
  echo "rc=$rc" >> "$G/$log.log"; echo "[$(date +%T)] $log rc=$rc" >> "$G/progress.log"; }
case "$1" in
 wc) run ◻️2d wc2d -p semio-s-artifact-fem-2d-editor-model-window --lib
     run 🧊️3d wc3d -p semio-s-artifact-fem-3d-editor-model-window --lib ;;
 e3) run 🧊️3d e3 -p semio-s-artifact-fem-3d-editor --test editor_app -- focus_entity_from_a_panel a_selected_combination_renders the_analysis_row_selection ;;
esac
echo "all done $1" >> "$G/progress.log"
