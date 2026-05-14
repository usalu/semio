#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖Neo4jHostForward
# 🌉Listens on 0.0.0.0:7687 and :7474 in the dev container and forwards to Compose service `neo4j`, so Cursor/VS Code `forwardPorts` exposes Bolt/HTTP on the Windows/macOS/Linux host (Neo4j Desktop bonus).
set -u

wait_for_neo4j_bolt() {
  local i=0
  while [ "$i" -lt 90 ]; do
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

start_forwarders() {
  if ! command -v socat >/dev/null 2>&1; then
    echo "⚠️ socat not installed; cannot forward Neo4j to host-facing ports."
    return 1
  fi
  local bolt_pid="/tmp/semio-neo4j-bolt-socat.pid"
  local http_pid="/tmp/semio-neo4j-http-socat.pid"
  for f in "$bolt_pid" "$http_pid"; do
    if [ -f "$f" ]; then
      kill "$(cat "$f")" 2>/dev/null || true
      rm -f "$f"
    fi
  done
  socat TCP-LISTEN:7687,bind=0.0.0.0,fork,reuseaddr TCP:neo4j:7687 &
  echo $! >"$bolt_pid"
  socat TCP-LISTEN:7474,bind=0.0.0.0,fork,reuseaddr TCP:neo4j:7474 &
  echo $! >"$http_pid"
  sleep 0.4
  if command -v ss >/dev/null 2>&1; then
    ss -tln | grep -E ':7687|:7474' >/dev/null 2>&1 || echo "⚠️ Expected listeners on 7687/7474 not visible yet (may still come up)."
  fi
  echo "✅ Neo4j Bolt/HTTP forwarders on :7687 / :7474 → neo4j (use editor Ports → Windows localhost)."
}

main() {
  if [ "${DEVCONTAINER:-}" != "true" ]; then
    exit 0
  fi
  if ! getent hosts neo4j >/dev/null 2>&1; then
    echo "ℹ️ Host neo4j not in DNS; skipping Neo4j host forward (not Compose stack?)."
    exit 0
  fi
  if ! wait_for_neo4j_bolt; then
    echo "⚠️ neo4j:7687 not accepting connections; skipping Neo4j host forward."
    exit 0
  fi
  start_forwarders || true
  exit 0
}

main "$@"
#endregion 🔖Neo4jHostForward
