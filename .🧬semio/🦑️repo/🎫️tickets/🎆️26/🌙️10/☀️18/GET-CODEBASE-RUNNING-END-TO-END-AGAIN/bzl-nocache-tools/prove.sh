#!/usr/bin/env bash
# Ticket input script: proves the converted targets of `staged/ops.json` hit Bazel's cache on the second identical run. Run ONLY after the coordinator's GO and after `apply.ts --write`; one heavy run machine-wide.
#   prove.sh light      go builds, print-viz typecheck, the ordering noops, canvas runtime assets          (minutes)
#   prove.sh bericht    @semio-tech/mit-bestand-bericht:build (needs demo-bzl applied; Tectonic report PDFs)
#   prove.sh course     @teaching/architecture-course:build (Vite release build, store receipt)
#   prove.sh wasm       the three receipted wasm producers (cargo wasm32; heavy, run alone)
#   prove.sh storybook  workspace:build-storybook
# Each phase runs the selection twice with `--tool=bazel`; the second run must report 0 local processes (INFO: N processes: X disk cache hit, Y internal, Z local).
# Logs: 🗑️generated/bzl-nocache/prove/<phase>-<run>-<stamp>.log under the ticket (delete when done).
set -u
set -o pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ticket="$(cd "$here/.." && pwd)"
root="$(cd "$ticket" && git rev-parse --show-toplevel)"
logs="$ticket/$(printf '\xf0\x9f\x97\x91\xef\xb8\x8fgenerated')/bzl-nocache/prove"; mkdir -p "$logs"
stamp="$(date +%Y%m%dT%H%M%S)"
log() { echo "[bzl-nocache $(date +%H:%M:%S)] $*"; }
repo() { (cd "$root" && bun repo --tool=bazel "$@"); }

targets_for() {
  case "$1" in
    light) echo "@semio-tech/repo-metrics-go:build @semio-tech/repo-test-runner-go:build @semio-tech/print-viz-inference:build @semio-tech/ui-styling:generate @semio-tech/ui-styling-py:generate @semio-tech/repo-vscode:build semio-framework-canvas:stage-runtime-assets" ;;
    bericht) echo "@semio-tech/mit-bestand-bericht:build" ;;
    course) echo "@teaching/architecture-course:build" ;;
    wasm) echo "@semio-tech/framework-os-node-graph-rs:wasm @semio-tech/s-spatial-kernel-semio-session-rs:wasm @semio-tech/s-cad-composition-rs:wasm" ;;
    storybook) echo "workspace:build-storybook" ;;
    *) return 1 ;;
  esac
}

phase="${1:-}"
targets="$(targets_for "$phase")" || { log "usage: prove.sh light|bericht|course|wasm|storybook"; exit 2; }
load="$(sysctl -n vm.loadavg | tr -d '{}' | awk '{print $1}')"
log "load $load"
awk "BEGIN{exit !($load < 8)}" || { log "FAIL load >= 8: refuse to start a heavy run"; exit 1; }
status=0
for run in first second; do
  out="$logs/$phase-$run-$stamp.log"; t0=$SECONDS
  log "$phase $run: bun repo --tool=bazel run $targets"
  # shellcheck disable=SC2086
  repo run $targets > "$out" 2>&1; code=$?
  log "$phase $run: exit $code in $((SECONDS - t0))s"
  command grep -E "INFO: [0-9]+ process|Build completed|Elapsed time|store-receipt" "$out" | tail -6
  [ "$code" = 0 ] || status=1
done
second="$logs/$phase-second-$stamp.log"
processes="$(command grep -E "INFO: [0-9]+ process" "$second" | tail -1)"
log "second run: ${processes:-no process summary line (only internal/elided actions)}"
if echo "$processes" | command grep -Eq "[1-9][0-9]* (local|linux-sandbox|darwin-sandbox|worker|processwrapper-sandbox)"; then log "FAIL second run executed actions: $processes"; status=1; else log "PASS second run executed nothing locally"; fi
exit "$status"
