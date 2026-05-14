#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-only
#region 🔖Neo4jMcpRun
# 🗄️Launches `mcp-neo4j-cypher` with Bolt URI, auth, telemetry, and the default Neo4j database for devcontainer and native MCP clients.
set -euo pipefail
if [ -f /etc/profile.d/99-semio-neo4j-mcp.sh ]; then
  # shellcheck source=/dev/null
  . /etc/profile.d/99-semio-neo4j-mcp.sh
fi
if [ -z "${NEO4J_URI:-}" ]; then
  if [ "${DEVCONTAINER:-}" = true ]; then
    export NEO4J_URI="bolt://neo4j:7687"
  else
    export NEO4J_URI="bolt://localhost:7687"
  fi
fi
export NEO4J_USERNAME="${NEO4J_USERNAME:-neo4j}"
export NEO4J_PASSWORD="${NEO4J_PASSWORD:-password}"
export NEO4J_TELEMETRY="${NEO4J_TELEMETRY:-false}"
export NEO4J_DATABASE="${1:-neo4j}"
#endregion 🔖Neo4jMcpRun
exec uvx mcp-neo4j-cypher
