#!/bin/zsh
# ⚖️ H14 14c hold 4 (native lane): apply the streamed hello tail (h14-hello-tail-stream.py; originals → generated/pre-hold4/, a red
# kernel-db check restores the two sync files nobody changed since), check kernel-db (default + no vcs) and semio-hub, then the
# fixed index/artifact laws, every db sync law and the new tail laws, full output of failures kept.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
DB="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=$W/target W DB
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
mkdir -p $W/generated/pre-hold4
files=("$DB/🔄️sync/🦀️.rs" "$DB/🔄️sync/🧪️tests/🔬️unit/🦀️.rs")
i=0; for f in $files; do i=$((i+1)); cp "$f" $W/generated/pre-hold4/$i.rs; done
python3 $W/h14-hello-tail-stream.py && python3 $W/h14-hello-tail-fix.py || echo "=== APPLY FAILED $(date +%T)"
i=0; for f in $files; do i=$((i+1)); shasum "$f" | cut -c1-40 > $W/generated/pre-hold4/$i.applied; done
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|warning: .* generated|sync/.*warning"; db=${pipestatus[1]}; echo "=== DB CHECK EXIT $db $(date +%T)"
if [ "$db" = 0 ]; then nice -n 15 cargo check -p semio-framework-os-kernel-db --no-default-features --features fs,deflate --lib --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished"; nv=${pipestatus[1]}; else nv=skipped; fi; echo "=== DB NO-VCS CHECK EXIT $nv $(date +%T)"
if [ "$db" != 0 ] || [ "$nv" != 0 ]; then
  nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E -A6 "^error|: error" | head -120
  i=0; for f in $files; do i=$((i+1)); if [ "$(shasum "$f" | cut -c1-40)" = "$(cat $W/generated/pre-hold4/$i.applied)" ]; then cp $W/generated/pre-hold4/$i.rs "$f"; echo "restored $f"; else echo "NOT restored (changed since apply) $f"; fi; done
  echo "=== REVERTED sync $(date +%T)"
fi
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_index:: db_artifact:: db_sync:: db_engine:: --test-threads 4 > $W/generated/hold4-db-laws.txt 2>&1; echo "=== DB LAWS EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" $W/generated/hold4-db-laws.txt | head -40
/usr/bin/grep -E "^test .*(seventy_thousand|owned_appends|a_fold_beyond|index_runs_follow|applied_receipts_keep|streams_exactly|fifty_thousand|derives_the_server_frontier)" $W/generated/hold4-db-laws.txt
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E -A4 "^error|: error|Finished" | head -40; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$hub" = 0 ]; then nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_batch_committed_past_the_frame_deadline_is_still_acknowledged socket_grant_revoke_before_command_admission_has_no_storage_effect a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB BIN LAWS EXIT ${pipestatus[1]} $(date +%T)"; fi
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
