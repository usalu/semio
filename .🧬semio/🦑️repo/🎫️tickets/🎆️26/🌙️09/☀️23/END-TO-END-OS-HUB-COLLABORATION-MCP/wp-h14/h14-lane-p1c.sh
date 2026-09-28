#!/bin/zsh
# ⚖️ H14 14b hold 3: native lane — kernel-db check, the WAL laws over the neutral committed-transactions fixture (now at the
# declared transaction bound, `repeat` frames), the cli boundary law, the P1 db laws; then the hub lib trusted_catalog laws
# (channel-19 fixtures) and the hub check; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished"; echo "=== DB CHECK EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_wal:: db_cli:: db_security:: a_declared_legal_byte_maximal_batch an_envelope_of_a_refused_batch --test-threads 4 2>&1 | /usr/bin/grep -E "^test .* (ok|FAILED)$|^test result|panicked|^error|: error|left:|right:" ; echo "=== DB LAWS EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo test -p semio-hub --lib --no-fail-fast -- trusted_catalog --test-threads 4 2>&1 | /usr/bin/grep -E "^test .* (ok|FAILED)$|^test result|panicked|^error|: error|left:|right:"; echo "=== HUB LIB LAWS EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
