#!/bin/zsh
# 🐘️ DB2 — boot an os-hub whose BOTH durable halves are the live PostgreSQL of 🔣️db2-compose.yaml,
# on port 7671, and run the H1b runtime probe against it (including its kill + restart persistence
# step). Ticket 26/09/18 slice DB2.
#
#   zsh 📜️db2-postgres-hub-boot.sh [--fresh]
#
# `--fresh` drops and recreates the hub's schema in Postgres and deletes the data root, so the run
# starts from genuinely empty durable state. Without it the run reuses whatever is there, which is
# what proves persistence across a SECOND invocation.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../../../../../.." && pwd)"
cd "$ROOT"
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
BIN="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/target-db2/debug/os-hub"
DATA="$ROOT/.🧬semio/🌐hub/db2-pg"
PGURL="postgres://semio:semio-db2@127.0.0.1:5433/semio_hub_db2"

[[ -x "$BIN" ]] || { echo "missing $BIN — build it with: CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-db2 cargo build -p semio-hub --bin os-hub --no-default-features --features postgres"; exit 1; }
docker compose -f "$TICKET/🔣️db2-compose.yaml" up -d --wait --wait-timeout 200 >/dev/null
docker exec semio-hub-db2-postgres-1 pg_isready -U semio -d semio_hub_db2

if [[ "${1:-}" == "--fresh" ]]; then
  docker exec semio-hub-db2-postgres-1 psql -U semio -d semio_hub_db2 -c 'DROP SCHEMA public CASCADE; CREATE SCHEMA public;' >/dev/null
  rm -rf "$DATA"
fi
mkdir -p "$DATA"; chmod 700 "$DATA"

export OS_HUB_STORAGE_BACKEND=postgres
export OS_HUB_DATABASE_URL="$PGURL"
export OS_HUB_DIRECTORY_BACKEND=postgres
export OS_HUB_DIRECTORY_DATABASE_URL="$PGURL"
export OS_HUB_CREDENTIAL_SIGN_IN=true

exec bun "$TICKET/🐍️h1b-hub-runtime-probe.ts" --binary "$BIN" --port 7671 --data "$DATA"
