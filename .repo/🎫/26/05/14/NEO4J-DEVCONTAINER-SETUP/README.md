# Neo4j Devcontainer Setup

## Work

- Opened manually because repo MCP resources/tools were not exposed in this Codex session and the local Go CLI is currently blocked by an unrelated workspace module mismatch.
- Corrected the setup to a single `semio` devcontainer. Neo4j is installed in the `semio` image, starts from `post-start.sh`, and publishes `127.0.0.1:7687` and `127.0.0.1:7474` from that same container.
- Kept the dev setup on Neo4j Community and the default `neo4j` database because Community supports one user database per DBMS.
- Added explicit `7474` and `7687` devcontainer forwarding for Codespaces/local devcontainers.
- Collapsed Neo4j MCP registration to one `neo4j` server using the default database.

## Validation

- `docker compose -f .devcontainer/docker-compose.yml config --quiet`
- `bunx prettier --check .devcontainer/devcontainer.json .mcp.json`
- `docker compose -f .devcontainer/docker-compose.yml up -d`
- `docker compose -f .devcontainer/docker-compose.yml config --services`
- `docker exec devcontainer-semio-1 bash -lc 'cypher-shell -a bolt://localhost:7687 -u neo4j -p password "RETURN 1 AS ok;"'`
- `Test-NetConnection -ComputerName 127.0.0.1 -Port 7687`
- `Test-NetConnection -ComputerName 127.0.0.1 -Port 7474`
- `docker exec devcontainer-semio-1 bash -lc 'test "$NEO4J_URI" = bolt://localhost:7687 && timeout 3 bash -c "echo >/dev/tcp/localhost/7687" && echo single-container-neo4j-ok'`
- `docker exec devcontainer-semio-1 bash -lc 'cd /workspaces/semio && NEO4J_DATABASE=neo4j timeout 30 uvx mcp-neo4j-cypher --help >/tmp/neo4j-mcp-help.txt && head -20 /tmp/neo4j-mcp-help.txt'`
