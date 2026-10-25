#!/bin/sh
# usage: smoke.sh [mode]   Headless smoke of the INSTALLED dashboard (isolated instance daemon, stopped at the end).
# Records every pid it starts in generated/dash-b/smoke-pids.txt; always stops the instance daemon.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
G="$C/🗑️generated/dash-b"; mode=${1:-prototype}
rec=".🧬semio/🦑️repo/⚡️cache/🎛️dashboard/$mode/installed.json"
bin=$(python3 -c "import json,sys;print(json.load(open('$rec'))['path'])") || exit 2
bin="$PWD/$bin"; [ -x "$bin" ] || { echo "no executable $bin"; exit 2; }
export SEMIO_DASHBOARD_INSTANCE=dash-b-smoke SEMIO_BUILD_MODE=$mode
out="$G/smoke.out"; : > "$out"
step() { echo "### $*" | tee -a "$out"; "$@" >> "$out" 2>&1; rc=$?; echo "rc=$rc" | tee -a "$out"; return $rc; }
trap '"$bin" daemon stop >> "$out" 2>&1; echo "stopped" >> "$out"' EXIT
for lang in en de; do
  export SEMIO_LOCALE=$lang
  step "$bin" --help
  step "$bin" commands --check
  step "$bin" commands --json
done
export SEMIO_LOCALE=en
step "$bin" daemon start
step "$bin" daemon status
echo "daemon pid: $("$bin" daemon status 2>&1 | sed -n 's/.*daemon pid \([0-9]*\).*/\1/p')" | tee -a "$G/smoke-pids.txt"
step "$bin" tasks --json
step "$bin" run @semio-tech/repo-dashboard-rs:test-language --dry-run
step "$bin" daemon status
