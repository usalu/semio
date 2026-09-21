#!/bin/zsh
# 🐘️ DB3 — boot an os-hub whose WHOLE durable state (document/blob storage AND directory) is the
# live PostgreSQL of 🔣️db3-compose.yaml, on port 7671, and run the H1b runtime probe against it
# (including its kill + restart persistence step). Ticket 26/09/18 slice DB3.
#
#   zsh 📜️db3-postgres-hub-boot.sh [--fresh] [--directory-backend postgres|neo4j] [-- <probe args…>]
#
# `--fresh` drops and recreates the hub's schema in Postgres, wipes the Neo4j graph and deletes the
# data root, so the run starts from genuinely empty durable state. Without it the run reuses
# whatever is there, which is what proves persistence across a SECOND invocation.
#
# `--directory-backend neo4j` keeps documents on PostgreSQL but moves the identity/tenancy
# directory onto the live Neo4j of the same compose stack.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
cd "$ROOT"
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
BIN="${DB3_HUB_BIN:-$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-db3/debug/os-hub}"
DATA="${DB3_HUB_DATA:-$ROOT/.🧬semio/🌐hub/db3-pg}"
PORT="${DB3_HUB_PORT:-7671}"
PGURL="postgres://db3:db3@127.0.0.1:5434/db3"
NEOURI="bolt://127.0.0.1:7689"

FRESH=0
DIRBACKEND=postgres
PROBE_ARGS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --fresh) FRESH=1; shift;;
    --directory-backend) DIRBACKEND="$2"; shift 2;;
    --) shift; PROBE_ARGS=("$@"); break;;
    *) PROBE_ARGS=("$@"); break;;
  esac
done

[[ -x "$BIN" ]] || { echo "missing $BIN — build it with: CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-db3 cargo build -p semio-hub --bin os-hub --no-default-features --features postgres,neo4j"; exit 1; }
docker compose -f "$TICKET/🔣️db3-compose.yaml" up -d >/dev/null
for _ in $(seq 1 60); do docker exec semio-db3-postgres-1 pg_isready -U db3 -d db3 >/dev/null 2>&1 && break; sleep 2; done
docker exec semio-db3-postgres-1 pg_isready -U db3 -d db3

if (( FRESH )); then
  docker exec semio-db3-postgres-1 psql -U db3 -d db3 -c 'DROP SCHEMA public CASCADE; CREATE SCHEMA public;' >/dev/null
  if [[ "$DIRBACKEND" == neo4j ]]; then
    docker exec semio-db3-neo4j-1 cypher-shell -u neo4j -p db3passwd 'MATCH (n) DETACH DELETE n' >/dev/null
  fi
  rm -rf "$DATA"
fi
mkdir -p "$DATA"; chmod 700 "$DATA"

export OS_HUB_STORAGE_BACKEND=postgres
export OS_HUB_DATABASE_URL="$PGURL"
export OS_HUB_CREDENTIAL_SIGN_IN=true
if [[ "$DIRBACKEND" == neo4j ]]; then
  export OS_HUB_DIRECTORY_BACKEND=neo4j
  export OS_HUB_DIRECTORY_NEO4J_URI="$NEOURI"
  export OS_HUB_DIRECTORY_NEO4J_USER=neo4j
  export OS_HUB_DIRECTORY_NEO4J_PASSWORD=db3passwd
else
  export OS_HUB_DIRECTORY_BACKEND=postgres
  export OS_HUB_DIRECTORY_DATABASE_URL="$PGURL"
fi

exec bun "$TICKET/🐍️h1b-hub-runtime-probe.ts" --binary "$BIN" --port "$PORT" --data "$DATA" "${PROBE_ARGS[@]}"
