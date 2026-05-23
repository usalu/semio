#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖Neo4jLocalCheck
# 🗄️Legacy helper that now verifies the single semio devcontainer's local Neo4j ports; post-start owns startup.
set -u

wait_for_neo4j_bolt() {
  local i=0
  while [ "$i" -lt 90 ]; do
    if command -v nc >/dev/null 2>&1 && nc -z localhost 7687 2>/dev/null; then
      return 0
    fi
    if timeout 1 bash -c "echo >/dev/tcp/localhost/7687" 2>/dev/null; then
      return 0
    fi
    sleep 1
    i=$((i + 1))
  done
  return 1
}

main() {
  if [ "${DEVCONTAINER:-}" != "true" ]; then
    exit 0
  fi
  if ! wait_for_neo4j_bolt; then
    echo "⚠️ Local Neo4j is not accepting Bolt connections on :7687 yet."
    exit 0
  fi
  echo "✅ Local Neo4j is accepting Bolt connections on :7687."
  exit 0
}

main "$@"
#endregion 🔖Neo4jLocalCheck
