#!/bin/bash
# 🚦️ One command: per-pane browser verdict table for a running demonstrator.
# Usage: run.sh <base-url> [--panes a,b] [--languages en,de] [--no-landing]   |   run.sh --check
# Env: E2E_ACTIONS_ALL_LANGUAGES=1 (act in every language), DEMONSTRATOR_E2E_GPU=swiftshader, E2E_OUT=<dir>, BUN=<bun>
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$HERE"
while [ ! -f "$ROOT/AGENTS.md" ] && [ "$ROOT" != "/" ]; do ROOT="$(dirname "$ROOT")"; done
[ -f "$ROOT/AGENTS.md" ] || { echo "repo root not found" >&2; exit 2; }
BUN="${BUN:-bun}"
export SEMIO_ROOT="$ROOT"
export PLAYWRIGHT_BROWSERS_PATH="${PLAYWRIGHT_BROWSERS_PATH:-$ROOT/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright}"
cd "$ROOT" || exit 2
if [ "${1:-}" = "--check" ]; then exec "$BUN" "$HERE/probe.ts" --check; fi
[ -n "${1:-}" ] || { echo "usage: run.sh <base-url> [--panes a,b] [--languages en,de] [--no-landing] | --check" >&2; exit 2; }
OUT="${E2E_OUT:-$HERE/../../🗑️generated/1105b/e2eready}"
mkdir -p "$OUT"
"$BUN" "$HERE/probe.ts" "$@" --out "$OUT" 2>&1 | tee "$OUT/run.log"
rc=${PIPESTATUS[0]}
echo "$rc" > "$OUT/run.rc"
exit "$rc"
