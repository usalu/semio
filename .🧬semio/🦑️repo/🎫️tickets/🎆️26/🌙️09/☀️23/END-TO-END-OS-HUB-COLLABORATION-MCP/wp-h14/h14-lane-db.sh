#!/bin/zsh
# ⚖️ H14 14c hold 2 (native lane, keeps hold 1's queue stamp): inside ONE hold — apply the kernel-db index/Ack-path/receipt sets
# (h14-index-levels.py, h14-index-off-ack.py, h14-index-laws.py; originals copied to generated/pre-hold2/), check kernel-db as
# built by default (vcs) and as the hub builds it (no vcs), semio-hub lib+bins+tests (incl. the already applied Ack set); a red
# kernel-db check restores the three kernel-db files that nobody changed since; then db index/artifact laws + hub socket laws.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
DB="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=$W/target W DB
echo "=== queued $(date +%T)" > "$OUT"
FLEET_TICKET_STAMP=${FLEET_TICKET_STAMP:-} zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
mkdir -p $W/generated/pre-hold2
files=("$DB/🔢️index/🦀️.rs" "$DB/🗿️artifact/🦀️.rs" "$DB/⚙️engine/🦀️.rs" "$DB/🔢️index/🧪️tests/🔬️unit/🦀️.rs" "$DB/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs")
i=0; for f in $files; do i=$((i+1)); cp "$f" $W/generated/pre-hold2/$i.rs; done
python3 $W/h14-index-levels.py && python3 $W/h14-index-off-ack.py && python3 $W/h14-index-laws.py || { echo "=== APPLY FAILED $(date +%T)"; }
i=0; for f in $files; do i=$((i+1)); shasum "$f" | cut -c1-40 > $W/generated/pre-hold2/$i.applied; done
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|^warning: unused|: warning: unused|Finished|warning: .* generated"; db=${pipestatus[1]}; echo "=== DB CHECK EXIT $db $(date +%T)"
if [ "$db" = 0 ]; then nice -n 15 cargo check -p semio-framework-os-kernel-db --no-default-features --features fs,deflate --lib --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|warning: .* generated"; nv=${pipestatus[1]}; else nv=skipped; fi; echo "=== DB NO-VCS CHECK EXIT $nv $(date +%T)"
if [ "$db" != 0 ] || [ "$nv" != 0 ]; then
  i=0; for f in $files; do i=$((i+1)); if [ "$(shasum "$f" | cut -c1-40)" = "$(cat $W/generated/pre-hold2/$i.applied)" ]; then cp $W/generated/pre-hold2/$i.rs "$f"; echo "restored $f"; else echo "NOT restored (changed since apply) $f"; fi; done
  echo "=== REVERTED kernel-db $(date +%T)"
fi
nice -n 15 cargo check -p semio-hub --lib --bins --tests --message-format short 2>&1 | /usr/bin/grep -E "^error|: error|Finished|warning: .* generated"; hub=${pipestatus[1]}; echo "=== HUB CHECK EXIT $hub $(date +%T)"
if [ "$db" = 0 ] && [ "$nv" = 0 ]; then
  nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_index:: db_artifact:: --test-threads 4 2>&1 | /usr/bin/grep -E "^test .*(FAILED|ok)$|panicked|^test result|^error|: error" | /usr/bin/grep -vE "^test .* ok$" ; echo "=== DB LAWS EXIT ${pipestatus[1]} $(date +%T)"
  nice -n 15 cargo test -p semio-framework-os-kernel-db --lib -- seventy_thousand owned_appends a_fold_beyond index_runs_follow applied_receipts_keep --test-threads 4 2>&1 | /usr/bin/grep -E "^test |^test result"; echo "=== DB NEW LAWS EXIT ${pipestatus[1]} $(date +%T)"
fi
if [ "$hub" = 0 ]; then
  nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_batch_committed_past_the_frame_deadline_is_still_acknowledged socket_grant_revoke_before_command_admission_has_no_storage_effect a_document_socket_admits_a_declared_maximal_batch a_transiently_refused_batch --test-threads 2 2>&1 | /usr/bin/grep -E "^test |FAILED|panicked|^test result|^error|: error"; echo "=== HUB BIN LAWS EXIT ${pipestatus[1]} $(date +%T)"
fi
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
