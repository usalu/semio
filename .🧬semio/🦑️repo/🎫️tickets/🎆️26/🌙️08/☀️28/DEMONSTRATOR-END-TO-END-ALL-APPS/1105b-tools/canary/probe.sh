#!/bin/bash
# usage: probe.sh [url]  — headless energy pane probe; exit 0 only if every check passed. Output: 🗑️generated/1105b/canary-probe/{report.json,console-<lang>.txt,*.png,run.log,run.rc}
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$HERE"; while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
[ -f "$ROOT/AGENTS.md" ] || { echo "repo root not found" >&2; exit 2; }
export PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright}"
export CANARY_URL="${1:-http://127.0.0.1:6106/?plugin=energy}"
export CANARY_OUT="${CANARY_OUT:-$HERE/../../🗑️generated/1105b/canary-probe}"
mkdir -p "$CANARY_OUT"; cd "$ROOT" || exit 2
"${BUN:-bun}" "$HERE/probe.mjs" 2>&1 | tee "$CANARY_OUT/run.log"
rc=${PIPESTATUS[0]}; echo "$rc" > "$CANARY_OUT/run.rc"; exit "$rc"
