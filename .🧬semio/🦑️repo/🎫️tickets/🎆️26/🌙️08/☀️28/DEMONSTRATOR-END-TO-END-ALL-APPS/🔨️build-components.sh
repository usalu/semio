#!/usr/bin/env bash
# 🧱️ Builds one wasm plugin component per pluginId, serially (cargo takes one global target-dir lock).
# Logs go to the session scratchpad, never to the ticket's 🗑️generated — peer sweeps delete that mid-run.
set -uo pipefail
cd /Users/ueli/Documents/semio || exit 1
export RUSTC_WRAPPER=""
export SEMIO_BUILD_BUDGET_MS="${SEMIO_BUILD_BUDGET_MS:-7200000}"
export SEMIO_CMD_BUDGET_MS="${SEMIO_CMD_BUDGET_MS:-7200000}"
# 🧭️ Resolved at run time: the dev module's directory name is being hand-repaired by a peer
# (variation-selector spelling in flux), so a hardcoded path breaks mid-refactor.
DEV=$(find 🧰️framework -maxdepth 8 -not -path "*/dist/*" -path "*dev/📦️packages/🟦️typescript/📜️script.ts" -print -quit 2>/dev/null)
[ -n "$DEV" ] || { echo "FATAL: dev script.ts not found"; exit 2; }
echo "DEV=$DEV"
LOG="${LOG_DIR:?LOG_DIR required}"
for pid in "$@"; do
  echo "=== BUILD $pid $(date '+%H:%M:%S') ==="
  SEMIO_PLUGIN_ONLY="$pid" bun "$DEV" plugin s > "$LOG/build-$pid.txt" 2>&1
  rc=$?
  echo "=== $pid rc=$rc $(date '+%H:%M:%S') ==="
  if [ $rc -ne 0 ]; then echo "--- tail ---"; tail -25 "$LOG/build-$pid.txt"; fi
done
