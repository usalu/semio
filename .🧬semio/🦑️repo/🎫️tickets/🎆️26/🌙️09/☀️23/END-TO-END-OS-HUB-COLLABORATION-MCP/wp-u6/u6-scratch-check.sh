#!/bin/zsh
# 🧾️ U6 set B scratch proof (native lane, PRIVATE build-dir + target inside the scratch mirror — never build-fleet-b: a mirror
# with the repo's relative member paths would collide with the shared units). Two runs, JSON diagnostics per unit:
#   lib   = `cargo check --lib` WITH `component-app-assembly` (every non-test use, editor/viewer included)
#   tests = `cargo check --lib --profile test` WITHOUT it (the lib-test unit alone: every test use that compiles on this tree)
# usage: zsh u6-scratch-check.sh <scratch-root> <capture-prefix>
setopt no_bg_nice
SC="${1:A}"; out="${2:A}"
T=/Users/ueli/Documents/semio/.tmp-ticket
export CARGO_INCREMENTAL=0 NX_DAEMON=false CARGO_BUILD_BUILD_DIR="$SC/.u6-build" CARGO_TARGET_DIR="$SC/.u6-target"
cd "$SC" || exit 2
echo "QUEUED scratch $(date '+%F %T')" > "$out.log"
zsh "$T/📜️fleet-mutex.sh" native u6 -- nice -n 15 zsh -c '
  out="$1"; rc=0
  p=(-p semio-s-artifact-stdio-docx -p semio-s-artifact-stdio-xlsx)
  echo "START lib $(date "+%F %T")"; s=$(date +%s)
  cargo check --locked --keep-going --message-format json-diagnostic-short --lib $p --features semio-s-artifact-stdio-docx/component-app-assembly,semio-s-artifact-stdio-xlsx/component-app-assembly > "$out.lib.json"; r=$?
  echo "END lib rc=$r wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; [ $r -ne 0 ] && rc=$r
  echo "START tests $(date "+%F %T")"; s=$(date +%s)
  cargo check --locked --keep-going --message-format json-diagnostic-short --lib --profile test $p > "$out.tests.json"; r=$?
  echo "END tests rc=$r wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; [ $r -ne 0 ] && rc=$r
  du -sh "$CARGO_BUILD_BUILD_DIR" "$CARGO_TARGET_DIR" 2>/dev/null
  exit $rc' u6 "$out" >> "$out.log" 2>&1
rc=$?
echo "LANE-EXIT rc=$rc $(date '+%F %T')" >> "$out.log"
exit $rc
