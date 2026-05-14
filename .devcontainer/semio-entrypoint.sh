#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖SemioEntrypoint
# 🚀Waits for Compose `neo4j:7687`, runs `neo4j-host-forward.sh` once, then execs the devcontainer command so host `127.0.0.1:7687` maps to a live listener (not only after post-start).
set -eu
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORWARD="${SCRIPT_DIR}/neo4j-host-forward.sh"

wait_neo4j_bolt() {
  local i=0
  while [ "$i" -lt 120 ]; do
    if command -v getent >/dev/null 2>&1 && ! getent hosts neo4j >/dev/null 2>&1; then
      sleep 1
      i=$((i + 1))
      continue
    fi
    if command -v nc >/dev/null 2>&1 && nc -z neo4j 7687 2>/dev/null; then
      return 0
    fi
    if timeout 1 bash -c "echo >/dev/tcp/neo4j/7687" 2>/dev/null; then
      return 0
    fi
    sleep 1
    i=$((i + 1))
  done
  return 1
}

if [ -f "$FORWARD" ]; then
  if wait_neo4j_bolt; then
    bash "$FORWARD" || true
  else
    echo "⚠️ semio-entrypoint: neo4j:7687 not ready in time; post-start may still start socat later."
  fi
fi
exec "$@"
#endregion 🔖SemioEntrypoint
