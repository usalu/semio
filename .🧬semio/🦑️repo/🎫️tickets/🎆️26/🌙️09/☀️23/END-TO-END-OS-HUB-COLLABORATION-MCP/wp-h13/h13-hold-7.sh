#!/bin/zsh
# 🎚️ H13 hold 7 (session 14c, item 6): SQLite WAL reader connections landed in kernel-db (hub-closure-only, coordinator GO
# 17:3x): compile-atomic check, the sqlite storage laws incl. `file_reads_run_on_wal_readers_beside_a_held_write_lock_and_never_write`,
# the semio-hub all-feature check, the sqlite storm law, then the full kernel-db lib suite with sqlite.
# usage: h13-hold-7.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
OS="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust"
zsh $W/h13-cargo.sh "$1-check-db" check -p semio-framework-os-kernel-db --features sqlite --lib --tests || exit 1
zsh $W/h13-cargo.sh "$1-sqlite-laws" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast -- db_storage_sqlite
zsh $W/h13-cargo.sh "$1-check-hub" check -p semio-hub --all-features --lib --bins --tests
zsh $W/h13-verb.sh "$1-reopen-storm-sqlite" "$OS" reopen-storm-check sqlite
zsh $W/h13-cargo.sh "$1-db-lib" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast
