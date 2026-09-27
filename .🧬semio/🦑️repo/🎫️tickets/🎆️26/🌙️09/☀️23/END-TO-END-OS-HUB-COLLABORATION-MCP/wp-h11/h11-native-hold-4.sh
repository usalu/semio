#!/bin/zsh
# 🔐️ H11: one native-mutex hold — the PostgreSQL round-trip laws and the postgres fence lane, each under its own claim of the
# shared server. usage: h11-native-hold-4.sh <label>
cd /Users/ueli/Documents/semio
LABEL=$1
export H11_NICE=0
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h11
TS="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-h11-logs"
(cd "$TS" && bun ./📜️script.ts backend run postgres -- zsh $W/h11-cargo.sh "$LABEL-pg-round-trips" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- db_storage_postgres::round_trips:: --include-ignored) > "$L/$LABEL-claim-pg-round-trips.txt" 2>&1
(cd "$TS" && bun ./📜️script.ts backend run postgres -- zsh $W/h11-cargo.sh "$LABEL-pg-fence" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- --exact db_storage::writer::fence_conformance::postgres_wal_writer_fence_holds_every_shared_law --include-ignored) > "$L/$LABEL-claim-pg-fence.txt" 2>&1
