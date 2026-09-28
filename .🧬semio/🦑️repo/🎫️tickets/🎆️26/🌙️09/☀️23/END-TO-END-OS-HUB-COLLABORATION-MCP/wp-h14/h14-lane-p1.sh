#!/bin/zsh
# ⚖️ H14 14b P1: native lane (build-fleet-b, private target) — check kernel-db lib+tests, run the db laws of the touched
# modules (security, wal, artifact, engine, sync, cluster), then check semio-hub lib+bins+tests and run the P1 + transient
# refusal bin laws; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short; echo "=== DB CHECK EXIT $? $(date +%T)"
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_security:: db_wal:: db_artifact:: db_engine:: db_sync:: db_cluster:: --test-threads 4 2>&1 | /usr/bin/grep -E "^test .* (ok|FAILED|ignored)$|^test result|panicked|^error|: error|FAILED|failures:"; echo "=== DB TEST EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short; echo "=== HUB CHECK EXIT $? $(date +%T)"
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch_names_the_declared_resend_code --test-threads 2 2>&1 | /usr/bin/grep -E "^test .* (ok|FAILED)$|^test result|panicked|^error|: error"; echo "=== HUB TEST EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
