#!/usr/bin/env bash
# 👁️ Follow the next Demonstrator generation and probe its stable private browser copy.
set -euo pipefail
ticket="$(cd "$(dirname "$0")" && pwd)"
generated="$ticket/🗑️generated"
log="$generated/browser-demonstrator.log"
previous="${1:?previous generation}"
ready="$generated/demonstrator-probe-ready-$$"
while :; do
  run="$( { rg -o 'runs/w[A-Za-z0-9]+:' "$log" 2>/dev/null || true; } | tail -1 | sed -E 's@runs/([^:]+):@\1@')"
  if [[ -n "$run" && "$run" != "$previous" ]]; then
    printf '[DEBUG] Found Demonstrator generation %s\n' "$run"
    if PROBE_LINKED_READY_MARKER="$ready" bash "$ticket/probe-linked.sh" demonstrator "$run" "${PROBE_PORT:-8798}" both; then
      exit 0
    else
      status=$?
      if [[ -f "$ready" ]]; then exit "$status"; fi
      previous="$run"
    fi
  fi
  sleep 5
done
