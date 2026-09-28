#!/bin/zsh
# ⚖️ H14 14c hold 5 (native lane): apply B2 (h14-hello-from-checkpoint.py; originals → generated/pre-hold5/), check semio-hub
# lib+bins+tests (a red check restores both files if nobody changed them since), then the new law + the socket/Ack laws.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=$W/target W
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
mkdir -p $W/generated/pre-hold5
files=("/Users/ueli/Documents/semio/🌎️hub/🏗️bootstrap/🦀️.rs" "/Users/ueli/Documents/semio/🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs")
i=0; for f in $files; do i=$((i+1)); cp "$f" $W/generated/pre-hold5/$i.rs; done
python3 $W/h14-hello-from-checkpoint.py || echo "=== APPLY FAILED $(date +%T)"
i=0; for f in $files; do i=$((i+1)); shasum "$f" | cut -c1-40 > $W/generated/pre-hold5/$i.applied; done
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E -A6 "^error|: error|Finished" | head -60; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$hub" != 0 ]; then
  i=0; for f in $files; do i=$((i+1)); if [ "$(shasum "$f" | cut -c1-40)" = "$(cat $W/generated/pre-hold5/$i.applied)" ]; then cp $W/generated/pre-hold5/$i.rs "$f"; echo "restored $f"; else echo "NOT restored (changed since apply) $f"; fi; done
  echo "=== REVERTED B2 $(date +%T)"; exit 1
fi
nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_frontierless_hello_resumes_at_the_checkpoint_its_client_seeds_from a_batch_committed_past_the_frame_deadline_is_still_acknowledged socket_grant_revoke_before_command_admission_has_no_storage_effect a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch document_open_plan --test-threads 2 2>&1 | /usr/bin/grep -E "^test |panicked|^test result"; echo "=== HUB BIN LAWS EXIT ${pipestatus[1]} $(date +%T)"
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
