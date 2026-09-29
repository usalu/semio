#!/bin/zsh
# ⚖️ H14 hold 8 (native lane): (1) apply the one-reader streamed tail + stall-bounded hello deadline (h14-hello-tail-reader.py,
# kernel-db sync; restore-on-red), kernel-db check default + no vcs, db_sync/db_artifact/db_index/db_engine laws; (2) apply B2 + B1
# (h14-hello-from-checkpoint.py, h14-checkpoint-policy.py; restore-on-red), semio-hub check, bin laws, integration-fixtures Check In
# laws, hub TS vitest. Backups + captures under .🧬semio/🌐hub/s14-h14-*.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
BK="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-backup/hold8"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target" W BK NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-logs"
mkdir -p "$BK"
restore() { tag=$1; shift; i=0; for f in "$@"; do i=$((i+1)); if [ "$(shasum "$f" | cut -c1-40)" = "$(cat "$BK/$tag-$i.applied")" ]; then cp "$BK/$tag-$i.orig" "$f"; echo "restored $f"; else echo "NOT restored (changed since apply) $f"; fi; done; }
S="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs"
cp "$S" "$BK/sync-1.orig"; python3 $W/h14-hello-tail-reader.py || echo "=== SYNC APPLY FAILED"; shasum "$S" | cut -c1-40 > "$BK/sync-1.applied"
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E -A5 "^error|: error|Finished|sync/🦀️.rs.*warning" | head -60; db=${pipestatus[1]}; echo "=== DB CHECK EXIT $db $(date +%T)"
if [ "$db" = 0 ]; then nice -n 15 cargo check -p semio-framework-os-kernel-db --no-default-features --features fs,deflate --lib --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished"; nv=${pipestatus[1]}; else nv=skipped; fi; echo "=== DB NO-VCS CHECK EXIT $nv $(date +%T)"
if [ "$db" != 0 ] || [ "$nv" != 0 ]; then restore sync "$S"; echo "=== REVERTED SYNC $(date +%T)"; fi
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_sync:: db_artifact:: db_index:: db_engine:: --test-threads 4 > "$L/hold8-db-laws.txt" 2>&1; echo "=== DB LAWS EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/hold8-db-laws.txt" | head -30
/usr/bin/grep -E "^test .*(fifty_thousand|streams_exactly|derives_the_server_frontier|seventy_thousand|owned_appends|a_fold_beyond|index_runs_follow|applied_receipts_keep)" "$L/hold8-db-laws.txt"
H=/Users/ueli/Documents/semio/🌎️hub
files=("$H/🏗️bootstrap/🦀️.rs" "$H/🧪️tests/🔬️bin-unit/🦀️.rs" "$H/🚀️local-bootstrap/🧬️schema/🔣️.json" "$H/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json" "$H/README.md")
i=0; for f in $files; do i=$((i+1)); cp "$f" "$BK/hub-$i.orig"; done
python3 $W/h14-hello-from-checkpoint.py && python3 $W/h14-checkpoint-policy.py || echo "=== HUB APPLY FAILED $(date +%T)"
i=0; for f in $files; do i=$((i+1)); shasum "$f" | cut -c1-40 > "$BK/hub-$i.applied"; done
python3 -c "import json,sys;[json.load(open(p)) for p in sys.argv[1:]];print(\"=== JSON OK\")" "$H/🚀️local-bootstrap/🧬️schema/🔣️.json" "$H/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json"
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E -A6 "^error|: error|Finished" | head -80; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$hub" != 0 ]; then restore hub $files; echo "=== REVERTED B1+B2 $(date +%T)"; exit 1; fi
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_frontierless_hello_resumes_at_the_checkpoint_its_client_seeds_from the_checkpoint_policy a_batch_committed_past_the_frame_deadline_is_still_acknowledged socket_grant_revoke_before_command_admission_has_no_storage_effect a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch readiness liveness --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB BIN LAWS EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo test -p semio-hub --bin os-hub --features integration-fixtures --no-fail-fast -- check_in the_checkpoint_policy --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB CHECK-IN LAWS EXIT ${pipestatus[1]} $(date +%T)"
cd "$H/📦️packages/🟦️typescript" && bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/📊️observability/🟦️.ts ../../🧪️tests/🤝️integration/🟦️.ts 2>&1 | tail -8; echo "=== HUB TS EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
