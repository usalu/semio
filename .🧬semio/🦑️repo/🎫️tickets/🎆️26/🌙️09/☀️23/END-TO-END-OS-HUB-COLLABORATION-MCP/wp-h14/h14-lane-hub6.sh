#!/bin/zsh
# ⚖️ H14 14c hold 5b (native lane): apply B2 (h14-hello-from-checkpoint.py) and B1 (h14-checkpoint-policy.py) — originals of
# every touched file → generated/pre-hold6/; check semio-hub lib+bins+tests (a red check restores each file nobody changed since);
# then the bin laws (default features) and the Check In laws incl. the policy e2e (`integration-fixtures`); schema/fixture JSON
# parse; os-hub-ts parity/observability vitest (README env rows).
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=$W/target W NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
FLEET_TICKET_STAMP=${FLEET_TICKET_STAMP:-} zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
mkdir -p $W/generated/pre-hold6
H=/Users/ueli/Documents/semio/🌎️hub
files=("$H/🏗️bootstrap/🦀️.rs" "$H/🧪️tests/🔬️bin-unit/🦀️.rs" "$H/🚀️local-bootstrap/🧬️schema/🔣️.json" "$H/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json" "$H/README.md")
i=0; for f in $files; do i=$((i+1)); cp "$f" $W/generated/pre-hold6/$i.orig; done
python3 $W/h14-hello-from-checkpoint.py && python3 $W/h14-checkpoint-policy.py || echo "=== APPLY FAILED $(date +%T)"
i=0; for f in $files; do i=$((i+1)); shasum "$f" | cut -c1-40 > $W/generated/pre-hold6/$i.applied; done
python3 -c "import json,sys;[json.load(open(p)) for p in sys.argv[1:]];print(\"=== JSON OK\")" "$H/🚀️local-bootstrap/🧬️schema/🔣️.json" "$H/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json"
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E -A6 "^error|: error|Finished" | head -80; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$hub" != 0 ]; then
  i=0; for f in $files; do i=$((i+1)); if [ "$(shasum "$f" | cut -c1-40)" = "$(cat $W/generated/pre-hold6/$i.applied)" ]; then cp $W/generated/pre-hold6/$i.orig "$f"; echo "restored $f"; else echo "NOT restored (changed since apply) $f"; fi; done
  echo "=== REVERTED B1+B2 $(date +%T)"; exit 1
fi
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_frontierless_hello_resumes_at_the_checkpoint_its_client_seeds_from the_checkpoint_policy a_batch_committed_past_the_frame_deadline_is_still_acknowledged socket_grant_revoke_before_command_admission_has_no_storage_effect a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch readiness liveness --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB BIN LAWS EXIT ${pipestatus[1]} $(date +%T)"
nice -n 15 cargo test -p semio-hub --bin os-hub --features integration-fixtures --no-fail-fast -- check_in the_checkpoint_policy --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB CHECK-IN LAWS EXIT ${pipestatus[1]} $(date +%T)"
cd "$H/📦️packages/🟦️typescript" && bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/📊️observability/🟦️.ts 2>&1 | tail -6; echo "=== HUB TS PARITY EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
