#!/bin/zsh
# 🐘️ DB4 — boot an os-hub whose WHOLE durable state (documents/blobs AND directory) is the live
# PostgreSQL of 🔣️db4-compose.yaml on port 7691, with a trusted catalog published from TODAY's tree,
# and drive the document lane against it (create → two sessions → an edit crossing → SIGTERM →
# restart → both re-attach). Ticket 26/09/18 slice DB4.
#
#   zsh 📜️db4-postgres-hub-boot.sh [--fresh] [--directory-backend postgres|neo4j] [-- <probe args…>]
#
# `--fresh` drops and recreates the hub's schema in Postgres, wipes the Neo4j graph and deletes the
# data root, so the run starts from genuinely empty durable state and re-copies the catalog.
#
# 🔏️ The catalog comes from 🗑️generated/db4-fixture/checkpoint-publication-process-fixture/data,
# emitted by the `integration-fixtures` bin law (see 📓️db4-document-on-postgres-hub.md §1). It is
# COPIED into the data root, never shared: two hubs never address one data root.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
cd "$ROOT"
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
BIN="${DB4_HUB_BIN:-$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-db4/debug/os-hub}"
DATA="${DB4_HUB_DATA:-$ROOT/.🧬semio/🌐hub/db4-pg}"
PORT="${DB4_HUB_PORT:-7691}"
PGURL="postgres://db4:db4@127.0.0.1:5435/db4"
NEOURI="bolt://127.0.0.1:7690"
CATALOG="$ROOT/$TICKET/🗑️generated/db4-fixture/checkpoint-publication-process-fixture/data/trusted-catalog"

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

[[ -x "$BIN" ]] || { echo "missing $BIN — build it with: CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-db4 cargo build -p semio-hub --bin os-hub --features postgres,neo4j"; exit 1; }
[[ -d "$CATALOG" ]] || { echo "missing $CATALOG — emit it with: SEMIO_TEST_ARTIFACT_DIR=$TICKET/🗑️generated/db4-fixture cargo test -p semio-hub --bin os-hub --features integration-fixtures,postgres,neo4j tests::checkpoint_publication_process_fixture_emits_verified_gis_pair_and_catalog -- --exact"; exit 1; }
docker compose -f "$TICKET/🔣️db4-compose.yaml" up -d >/dev/null
for _ in $(seq 1 60); do docker exec semio-db4-postgres-1 pg_isready -U db4 -d db4 >/dev/null 2>&1 && break; sleep 2; done
docker exec semio-db4-postgres-1 pg_isready -U db4 -d db4

if (( FRESH )); then
  docker exec semio-db4-postgres-1 psql -U db4 -d db4 -c 'DROP SCHEMA public CASCADE; CREATE SCHEMA public;' >/dev/null
  if [[ "$DIRBACKEND" == neo4j ]]; then
    docker exec semio-db4-neo4j-1 cypher-shell -u neo4j -p db4passwd 'MATCH (n) DETACH DELETE n' >/dev/null
  fi
  rm -rf "$DATA"
fi
mkdir -p "$DATA"; chmod 700 "$DATA"
[[ -d "$DATA/trusted-catalog" ]] || cp -R "$CATALOG" "$DATA/trusted-catalog"

export OS_HUB_STORAGE_BACKEND=postgres
export OS_HUB_DATABASE_URL="$PGURL"
export OS_HUB_CREDENTIAL_SIGN_IN=true
if [[ "$DIRBACKEND" == neo4j ]]; then
  export OS_HUB_DIRECTORY_BACKEND=neo4j
  export OS_HUB_DIRECTORY_NEO4J_URI="$NEOURI"
  export OS_HUB_DIRECTORY_NEO4J_USER=neo4j
  export OS_HUB_DIRECTORY_NEO4J_PASSWORD=db4passwd
else
  export OS_HUB_DIRECTORY_BACKEND=postgres
  export OS_HUB_DIRECTORY_DATABASE_URL="$PGURL"
fi

exec bun "$TICKET/🐍️db4-document-lane.ts" --binary "$BIN" --port "$PORT" --data "$DATA" "${PROBE_ARGS[@]}"
