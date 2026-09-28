#!/bin/zsh
# ⚖️ H14 14c: one native-lane hold — kernel-db as the hub builds it (no `vcs`), semio-hub lib+bins+tests, then the frame-deadline
# Ack law + neighbouring socket laws; capture → <capture>.
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
nice -n 15 cargo check -p semio-framework-os-kernel-db --no-default-features --features fs,deflate --lib --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|: warning: unused|Finished|warning: .* generated"; echo "=== DB NO-VCS CHECK EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|warning: .* generated"; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
[ "$hub" = 0 ] || exit 1
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_batch_committed_past_the_frame_deadline_is_still_acknowledged socket_grant_revoke_before_command_admission_has_no_storage_effect a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch --test-threads 2 2>&1 | /usr/bin/grep -E "^test |FAILED|panicked|^test result|^error|: error"; echo "=== BIN LAWS EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
