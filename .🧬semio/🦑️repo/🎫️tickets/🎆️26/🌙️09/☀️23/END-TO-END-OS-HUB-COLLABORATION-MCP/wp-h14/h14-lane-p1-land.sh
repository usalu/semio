#!/bin/zsh
# ⚖️ H14 14c-post-sweep hold 7 (native lane): prove the streamed hello tail (A, in the tree since hold 4 whose capture the 01:14 sweep
# deleted): kernel-db check (default + no vcs) + db_sync/db_artifact/db_index laws; then apply B2 (h14-hello-from-checkpoint.py) and
# B1 (h14-checkpoint-policy.py) — originals → .🧬semio/🌐hub/s14-h14-backup/hold7/ — check semio-hub lib+bins+tests (red → restore
# every file nobody changed since), bin laws + `integration-fixtures` Check In laws, hub TS observability/parity vitest.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
BK="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-backup/hold7"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target" W BK NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-logs"
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E -A5 "^error|: error|Finished|sync/.*warning" | head -40; echo "=== DB CHECK EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo check -p semio-framework-os-kernel-db --no-default-features --features fs,deflate --lib --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished"; echo "=== DB NO-VCS CHECK EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_sync:: db_artifact:: db_index:: db_engine:: --test-threads 4 > "$L/hold7-db-laws.txt" 2>&1; echo "=== DB LAWS EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/hold7-db-laws.txt" | head -30
/usr/bin/grep -E "^test .*(fifty_thousand|streams_exactly|derives_the_server_frontier|seventy_thousand|owned_appends|a_fold_beyond|index_runs_follow|applied_receipts_keep)" "$L/hold7-db-laws.txt"
mkdir -p "$BK"
H=/Users/ueli/Documents/semio/🌎️hub
files=("$H/🏗️bootstrap/🦀️.rs" "$H/🧪️tests/🔬️bin-unit/🦀️.rs" "$H/🚀️local-bootstrap/🧬️schema/🔣️.json" "$H/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json" "$H/README.md")
i=0; for f in $files; do i=$((i+1)); cp "$f" "$BK/$i.orig"; done
python3 $W/h14-hello-from-checkpoint.py && python3 $W/h14-checkpoint-policy.py || echo "=== APPLY FAILED $(date +%T)"
i=0; for f in $files; do i=$((i+1)); shasum "$f" | cut -c1-40 > "$BK/$i.applied"; done
python3 -c "import json,sys;[json.load(open(p)) for p in sys.argv[1:]];print(\"=== JSON OK\")" "$H/🚀️local-bootstrap/🧬️schema/🔣️.json" "$H/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json"
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E -A6 "^error|: error|Finished" | head -80; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$hub" != 0 ]; then
  i=0; for f in $files; do i=$((i+1)); if [ "$(shasum "$f" | cut -c1-40)" = "$(cat "$BK/$i.applied")" ]; then cp "$BK/$i.orig" "$f"; echo "restored $f"; else echo "NOT restored (changed since apply) $f"; fi; done
  echo "=== REVERTED B1+B2 $(date +%T)"; exit 1
fi
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_frontierless_hello_resumes_at_the_checkpoint_its_client_seeds_from the_checkpoint_policy a_batch_committed_past_the_frame_deadline_is_still_acknowledged socket_grant_revoke_before_command_admission_has_no_storage_effect a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch readiness liveness --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB BIN LAWS EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo test -p semio-hub --bin os-hub --features integration-fixtures --no-fail-fast -- check_in the_checkpoint_policy --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB CHECK-IN LAWS EXIT ${pipestatus[1]} $(date +%T)"
cd "$H/📦️packages/🟦️typescript" && bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/📊️observability/🟦️.ts ../../🧪️tests/🤝️integration/🟦️.ts 2>&1 | tail -8; echo "=== HUB TS EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
