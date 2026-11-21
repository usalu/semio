#!/usr/bin/env bash
# 👁️ Follow the next Demonstrator generation and probe its stable private browser copy.
set -euo pipefail
ticket="$(cd "$(dirname "$0")" && pwd)"
generated="$ticket/🗑️generated"
log="$generated/browser-demonstrator.log"
previous="${1:?previous generation}"
while :; do
  run="$( { rg -o 'runs/w[A-Za-z0-9]+:' "$log" 2>/dev/null || true; } | tail -1 | sed -E 's@runs/([^:]+):@\1@')"
  if [[ -n "$run" && "$run" != "$previous" ]]; then
    printf '[DEBUG] Found Demonstrator generation %s\n' "$run"
    bash "$ticket/probe-linked.sh" demonstrator "$run" 8798 both
    exit $?
  fi
  sleep 5
done
