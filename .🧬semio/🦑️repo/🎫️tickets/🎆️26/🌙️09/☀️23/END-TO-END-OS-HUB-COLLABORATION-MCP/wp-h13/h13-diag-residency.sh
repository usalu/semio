#!/bin/zsh
# 🔬️ H13 (session 15): diagnostic native-lane window for the writer-lease set — applies it (compile-checked), runs the two
# residency laws in-process with visible output, then ALWAYS reverses it (never lands).
# usage: zsh 📜️fleet-mutex.sh native h13 -- zsh h13-diag-residency.sh <label>
R=/Users/ueli/Documents/semio
W="$R/.🧬semio/🌐hub/s14-h13-work/wal-writer-lease"
H=$R/.tmp-ticket/wp-h13
P="$W/wal-writer-lease.patch"
LABEL=$1
cd $R || exit 1
echo "=== diagnostic window $(date +%T) $LABEL (native lane held)"
patch -p1 -N --dry-run -s < "$P" || { echo "=== ABORT: patch dry-run red (nothing applied)"; exit 1; }
patch -p1 -N -s < "$P" || { patch -p1 -R -s < "$P"; echo "=== ABORT: apply red, reversed"; exit 1; }
while IFS= read -r f; do rm -f "$R/$f.orig"; done < "$W/files.txt"
echo "=== applied $(date +%T)"
if zsh $H/h13-cargo.sh "$LABEL-db-check" check -p semio-framework-os-kernel-db --locked --features sqlite --lib --tests; then
  for law in db_engine::tests::open_filesystem_documents_beyond_the_writer_slots_commit_concurrently_and_replay db_engine::tests::a_thousand_open_documents_commit_concurrently_through_the_declared_writer_slots; do
    SEMIO_DB_ISOLATED_LAW=$law zsh $H/h13-cargo.sh "$LABEL-law-${law##*::}" test -p semio-framework-os-kernel-db --locked --features sqlite --lib -- --exact $law --nocapture
  done
fi
patch -p1 -R -s < "$P" || echo "RESTORE-MANUAL: reverse patch red"
while IFS= read -r f; do rm -f "$R/$f.orig"; cmp -s "$R/$f" "$W/base/$f" || echo "RESTORED-DIFFERS-FROM-BASE (peer edit?) $f"; done < "$W/files.txt"
echo "=== REVERSED $(date +%T) $LABEL (diagnostic only)"
