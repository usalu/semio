#!/bin/zsh
# 🔐️ H11: one native-mutex hold — hub all-features test build + H9/H11 named hub laws, then the db test build + the item-3
# and claimed-backend laws that need no server. usage: h11-native-hold.sh <label>
cd /Users/ueli/Documents/semio
LABEL=$1
export H11_NICE=0
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h11
zsh $W/h11-cargo.sh "$LABEL-hub-build" test -p semio-hub --all-features --no-run --message-format=short || exit 1
zsh $W/h11-hub-laws.sh "$LABEL" named
zsh $W/h11-cargo.sh "$LABEL-db-build" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-run --message-format=short || exit 1
zsh $W/h11-cargo.sh "$LABEL-db-laws" test -p semio-framework-os-kernel-db --features sqlite,postgres,neo4j --lib --no-fail-fast -- db_storage::tests:: db_storage::writer::fence_conformance::sqlite_wal_writer_fence_holds_every_shared_law
