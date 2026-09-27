#!/bin/zsh
# 🔐️ H11: one native-mutex hold — hub lib laws after the channel-18 fixture moves, the db in-process `--lib` gate (item 3), and
# the pg/neo4j live laws on the shared claimed servers. usage: h11-native-hold-3.sh <label>
cd /Users/ueli/Documents/semio
LABEL=$1
export H11_NICE=0
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h11
TS="/Users/ueli/Documents/semio/🌎️hub/📦️packages/🟦️typescript"
zsh $W/h11-cargo.sh "$LABEL-hub-lib" test -p semio-hub --all-features --lib --no-fail-fast -- refusal:: artifact_authority:: inference::
zsh $W/h11-cargo.sh "$LABEL-db-inprocess" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- --skip long:: --skip throughput_tests::
(cd "$TS" && bun ./📜️script.ts backend run postgres -- zsh $W/h11-cargo.sh "$LABEL-db-postgres" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- --include-ignored db_storage_postgres::round_trips db_storage::writer::fence_conformance::postgres_wal_writer_fence_holds_every_shared_law) > "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-h11-logs/$LABEL-claim-postgres.txt" 2>&1
(cd "$TS" && bun ./📜️script.ts backend run neo4j -- zsh $W/h11-cargo.sh "$LABEL-db-neo4j" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- --include-ignored db_storage::writer::fence_conformance::neo4j_wal_writer_fence_holds_every_shared_law) > "/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-h11-logs/$LABEL-claim-neo4j.txt" 2>&1
