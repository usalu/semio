#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖SemioEntrypoint
# 🚀Waits for Compose `neo4j:7687`, then execs the devcontainer command for legacy Compose callers.
set -eu

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

if command -v getent >/dev/null 2>&1 && getent hosts neo4j >/dev/null 2>&1; then
  if wait_neo4j_bolt; then
    echo "✅ semio-entrypoint: neo4j:7687 is reachable."
  else
    echo "⚠️ semio-entrypoint: neo4j:7687 not ready in time."
  fi
fi
exec "$@"
#endregion 🔖SemioEntrypoint
