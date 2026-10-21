#!/bin/sh
# runs the new laws and the existing suites of every touched area against an overlay (default /private/tmp/graph-mem/overlay), one test process at a time, niced
# usage: verify.sh [OVERLAY_DIR]      prints one line per suite: pass/fail counts and seconds
OVERLAY=${1:-/private/tmp/graph-mem/overlay}
LOGS=${GRAPH_MEM_LOGS:-/private/tmp/graph-mem/logs}; mkdir -p "$LOGS"
export SEMIO_TEST_ARTIFACT_DIR=${SEMIO_TEST_ARTIFACT_DIR:-/private/tmp/graph-mem/artifacts}; mkdir -p "$SEMIO_TEST_ARTIFACT_DIR"
L="$OVERLAY/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
B="$L/🏗️build"; G="$B/🕸️graph"
status=0
run() {
  name=$1; dir=$2; [ -d "$dir" ] || { echo "MISSING $name"; status=1; return; }
  start=$(date +%s)
  (cd "$dir" && nice -n 10 bun test --timeout 120000 ./🟦️.ts) > "$LOGS/$name.log" 2>&1
  code=$?
  pass=$(grep -E '^ *[0-9]+ pass' "$LOGS/$name.log" | awk '{print $1}'); fail=$(grep -E '^ *[0-9]+ fail' "$LOGS/$name.log" | awk '{print $1}')
  echo "$name exit=$code pass=${pass:-?} fail=${fail:-?} seconds=$(( $(date +%s) - start ))"
  [ "$code" = 0 ] || status=1
}
run memory-law "$B/🧠️memory/🧪️tests"
run owner-claims "$B/🧪️tests/🕸️graph/🧲️owner-claims"
run retention "$B/👁️observation/🧪️tests/🧠️retention"
run shared-strings "$G/🕰️generations/🧪️tests/🧶️shared-strings"
run governed-dispatch "$G/📥️inference/📦️project-nodes/🧪️tests/🚦️governed-dispatch"
run generations "$G/🕰️generations/🧪️tests"
run generations-load "$G/🕰️generations/🔌️load/🧪️tests"
run generations-environment "$G/🕰️generations/🌍️environment/🧪️tests"
run generation-protocol "$G/🧪️tests/🕰️generation-protocol"
run closure-facts "$G/📥️inference/🧪️tests/🧷️closure-facts"
run sealed-reuse "$G/🧪️tests/🔓️sealed-reuse"
run graph-phases "$G/⏱️phases/🧪️tests"
run observation "$B/👁️observation/🧪️tests"
run record-header "$B/👁️observation/🧪️tests/🧾️record-header"
exit $status
