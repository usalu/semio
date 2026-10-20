#!/bin/sh
# Architecture course on Bazel: preflight | cold | warm | edit | e2e | dev | all, foreground only, hard per-command timeouts are the caller's (Bash tool timeout).
# usage: COURSE_BZL_GO=1 sh course-run.sh <phase> [outdir]      (refuses without COURSE_BZL_GO=1: heavy; the coordinator must have sent GO)
# Output: <outdir>/<phase>-<n>-<label>.log, <phase>.tsv (label, rc, seconds, bazel process line, slot/wait lines), cache-status-<phase>-*.txt, capacity-<phase>-*.txt
# Env: TOOL (bazel), MODE (release for publish/e2e), EDIT_FILE (probe file, default a leaf course module), PROCTOR_EDIT (set to a framework Rust file to also probe the proctor miss),
#      SEMIO_CARGO_LANE (default course: this driver's private cargo lane; conc-cargo), GRAPH_POLICY (current: only for graph/show; task commands always plan fresh),
#      SEMIO_HOST_CAPACITY (left to the governor; mem-gov queues heavy commands and prints progress; =off only as the documented escape hatch)
# Phases: preflight cold warm edit parallel lanes e2e dev all
set -u
phase=${1:-}
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../../../../../../../.." && pwd)
out=${2:-"$here/../🗑️generated/course-bzl/run"}
TOOL=${TOOL:-bazel}
MODE=${MODE:-release}
COURSE=@teaching/architecture-course
PROCTOR=@teaching/proctor
GRAPH_POLICY=${GRAPH_POLICY:-current}
SEMIO_CARGO_LANE=${SEMIO_CARGO_LANE:-course}; export SEMIO_CARGO_LANE
EDIT_FILE=${EDIT_FILE:-"🎓️teaching/🏛️architecture/📚️course/🏗️builder/⏱️budget/🟦️.ts"}
[ "${COURSE_BZL_GO:-}" = 1 ] || { echo "refused: set COURSE_BZL_GO=1 after the coordinator's GO" >&2; exit 2; }
mkdir -p "$out"; cd "$root" || exit 2
n=0
r() {  # r <label> <repo args...>   runs `bun repo --tool=$TOOL <args>`, records rc/seconds/bazel summary
  label=$1; shift; n=$((n+1)); log="$out/$phase-$n-$label.log"; start=$(date +%s)
  bun repo --tool="$TOOL" "$@" > "$log" 2>&1; rc=$?
  secs=$(( $(date +%s) - start ))
  summary=$(grep -E "[0-9]+ processes|disk cache hit|remote cache hit|cache hit|replayed" "$log" | tail -2 | tr '\n' ' ' | cut -c1-200)
  waits=$(grep -E "session slot|server slots? (are|is) in use|Waiting for access|capacity|queued" "$log" | head -3 | tr '\n' ' ' | cut -c1-240)
  printf '%s\t%s\t%s\t%s\t%s\n' "$label" "$rc" "$secs" "$summary" "$waits" | tee -a "$out/$phase.tsv"
  return $rc
}
status() {
  bun repo --tool="$TOOL" cache status > "$out/cache-status-$phase-$1.txt" 2>&1
  bun repo capacity status > "$out/capacity-$phase-$1.txt" 2>&1
  bun repo leases status > "$out/leases-$phase-$1.txt" 2>&1
}
list() {  # the course closure in dependency order; publish/test-e2e need the release axis
  r proctor-build run "$PROCTOR:build" --configuration="$MODE" || return 1
  r typecheck run "$COURSE:typecheck" || return 1
  r test run "$COURSE:test" -- quick || return 1
  r build run "$COURSE:build" --configuration="$MODE" || return 1
  r check run "$COURSE:check" || return 1
  r publish run "$COURSE:publish" --configuration=release || return 1
  r proctor-test run "$PROCTOR:test" || return 1
}
case "$phase" in
  preflight)
    r show-current show projects --graph-policy="$GRAPH_POLICY"
    for t in "$PROCTOR:build" "$PROCTOR:test" "$PROCTOR:check" "$COURSE:typecheck" "$COURSE:test" "$COURSE:build" "$COURSE:check" "$COURSE:publish" "$COURSE:test-e2e" "$COURSE:deploy-check" "$COURSE:dev-site" "$COURSE:dev"; do
      r "dry-$(echo "$t" | tr '/:@' '___')" run "$t" --configuration="$MODE" --dry-run
    done ;;
  cold) status before; list; status after ;;
  warm) status before; list; status after
    echo "--- warm verdict: every cacheable target must show no executed action"; cat "$out/warm.tsv" ;;
  edit)
    [ -f "$EDIT_FILE" ] || { echo "no such file $EDIT_FILE" >&2; exit 2; }
    cp "$EDIT_FILE" "$out/edit-original.bak"
    restore() { cat "$out/edit-original.bak" > "$EDIT_FILE"; }
    trap restore EXIT INT TERM
    printf '\n// [DEBUG] course-bzl edit probe\n' >> "$EDIT_FILE"
    list; restore; trap - EXIT INT TERM
    echo "--- restored $EDIT_FILE; expect misses for typecheck/test/build/check/publish, a hit for proctor-build and proctor-test"
    if [ -n "${PROCTOR_EDIT:-}" ]; then
      cp "$PROCTOR_EDIT" "$out/proctor-edit.bak"; restore2() { cat "$out/proctor-edit.bak" > "$PROCTOR_EDIT"; }; trap restore2 EXIT INT TERM
      printf '\n// [DEBUG] course-bzl proctor probe\n' >> "$PROCTOR_EDIT"; r proctor-build-after-rust-edit run "$PROCTOR:build" --configuration="$MODE"; restore2; trap - EXIT INT TERM
    fi ;;
  parallel)
    status before
    for spec in "ta $COURSE:typecheck" "tb $PROCTOR:test" "tc $COURSE:check"; do
      set -- $spec; label=$1; shift
      ( n=$((n+1)); log="$out/parallel-$label.log"; start=$(date +%s); bun repo --tool="$TOOL" run "$1" > "$log" 2>&1; rc=$?
        printf '%s\t%s\t%s\t%s\n' "$label:$1" "$rc" "$(( $(date +%s) - start ))" "$(grep -E 'session slot|processes' "$log" | tail -2 | tr '\n' ' ' | cut -c1-200)" >> "$out/parallel.tsv" ) &
    done
    wait; cat "$out/parallel.tsv"; status after
    echo "--- expect: three sessions in slots s0/s1/s2 (no waiting line unless the host budget is below 3), distinct SEMIO_CARGO_LANE overflow lanes for cargo, no 'Waiting for access to repo:compiler-writer'" ;;
  lanes)
    status before
    for lane in lane-a lane-b; do
      ( start=$(date +%s); SEMIO_CARGO_LANE=$lane bun repo --tool="$TOOL" run "$PROCTOR:build" --configuration="$MODE" --skip-cache > "$out/lanes-$lane.log" 2>&1; rc=$?
        printf '%s\t%s\t%s\n' "$lane" "$rc" "$(( $(date +%s) - start ))" >> "$out/lanes.tsv" ) &
    done
    wait; cat "$out/lanes.tsv"; status after
    echo "--- expect: both lanes progress at once; the second lane compiles from the shared unit cache (hits), no repo:compiler-writer wait" ;;
  e2e)
    r test-e2e run "$COURSE:test-e2e" --configuration=release -- --topology both -- ;;
  dev)
    for spec in "$PROCTOR:dev 8791" "$COURSE:dev-site 6061" "$COURSE:dev 6061,8791"; do
      set -- $spec
      bun "$here/../../../☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-dev-tools/📜️drive.ts" "$1" --ports="$2" --tool="$TOOL" --deadline=900 --log="$out/dev-$(echo "$1" | tr '/:@' '___').log"
    done ;;
  all) for p in preflight cold warm edit parallel lanes e2e dev; do sh "$0" "$p" "$out" || exit $?; done ;;
  *) echo "usage: COURSE_BZL_GO=1 sh course-run.sh preflight|cold|warm|edit|parallel|lanes|e2e|dev|all [outdir]" >&2; exit 2 ;;
esac
