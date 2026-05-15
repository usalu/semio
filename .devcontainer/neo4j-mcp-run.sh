#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖Neo4jMcpRun
# 🗄️Delegates to root `dev.mcp.neo4j.script.ts` (same as IDE MCP configs; keeps devcontainer PATH/profile hooks).
set -euo pipefail
if [ -f /etc/profile.d/99-semio-neo4j-mcp.sh ]; then
  # shellcheck source=/dev/null
  . /etc/profile.d/99-semio-neo4j-mcp.sh
fi
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
#endregion 🔖Neo4jMcpRun
exec bun ./dev.mcp.neo4j.script.ts "$@"
