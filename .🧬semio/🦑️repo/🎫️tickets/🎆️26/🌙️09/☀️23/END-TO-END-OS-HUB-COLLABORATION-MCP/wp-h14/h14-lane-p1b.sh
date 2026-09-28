#!/bin/zsh
# ⚖️ H14 14b P1 (2nd hold): native lane — check kernel-db lib+tests after the WAL transaction-gate sizing, then run the
# P1 db laws unfiltered-output (maximal batch, resend, WAL maximal transaction, security) and the throughput law whose
# process-isolated child failed in hold 1; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|warning: unused|🛢️db/.*(📝️wal/🦀️|🔒️security/🦀️|🗿️artifact/🦀️).*warning|Finished"; echo "=== DB CHECK EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- a_declared_legal_byte_maximal_batch an_envelope_of_a_refused_batch a_declared_maximal_command_batch db_security:: wal_transaction_gate wal_recovery --test-threads 4; echo "=== DB LAWS EXIT $? $(date +%T)"
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- fs_commits_and_reopen_storms_stay_within_their_throughput_bounds --test-threads 1; echo "=== THROUGHPUT EXIT $? $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
