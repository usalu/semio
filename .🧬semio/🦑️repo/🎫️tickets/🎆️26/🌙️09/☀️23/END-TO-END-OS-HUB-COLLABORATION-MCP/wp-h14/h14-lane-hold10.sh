#!/bin/zsh
# ⚖️ H14 hold 10 (native lane): apply h14-law-isolation.py (test-only, restore-on-red), kernel-db `--lib --tests` check, the whole
# kernel-db lib suite under the default runner, db_engine:: three more times, then 8-way isolated stress of the two shut-down laws.
OUT=$1
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14
BK="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-backup/hold10"
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-target" W BK NX_DAEMON=false
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c '
echo "=== start $(date +%T)"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h14-logs"
T="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests"
mkdir -p "$BK"
cp "$T/🔬️vcs-integration-retained/🦀️.rs" "$BK/vcs.orig"; cp "$T/🔬️unit/🦀️.rs" "$BK/unit.orig"
python3 $W/h14-law-isolation.py; shasum "$T/🔬️vcs-integration-retained/🦀️.rs" "$T/🔬️unit/🦀️.rs" | cut -c1-40 > "$BK/applied"
nice -n 15 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short 2>&1 | /usr/bin/grep -E -A5 "^error|: error|Finished|🧪️tests/🔬️(unit|vcs-integration-retained)/🦀️.rs.*warning" | head -40; db=${pipestatus[1]}; echo "=== DB CHECK EXIT $db $(date +%T)"
if [ "$db" != 0 ]; then if [ "$(shasum "$T/🔬️vcs-integration-retained/🦀️.rs" "$T/🔬️unit/🦀️.rs" | cut -c1-40)" = "$(cat "$BK/applied")" ]; then cp "$BK/vcs.orig" "$T/🔬️vcs-integration-retained/🦀️.rs"; cp "$BK/unit.orig" "$T/🔬️unit/🦀️.rs"; echo "=== REVERTED law isolation"; fi; exit 1; fi
nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast > "$L/hold10-all-laws.txt" 2>&1; echo "=== ALL LIB LAWS (default runner) EXIT $? $(date +%T)"
/usr/bin/grep -E "^test result|FAILED|panicked" "$L/hold10-all-laws.txt" | head -20
for r in 1 2 3; do nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- db_engine:: > "$L/hold10-engine-$r.txt" 2>&1; echo "=== ENGINE LAWS round $r EXIT $? $(date +%T)"; /usr/bin/grep -E "^test result|FAILED" "$L/hold10-engine-$r.txt" | head -8; done
B=$(/usr/bin/grep -o "Running unittests [^(]*(\([^)]*\))" "$L/hold10-engine-1.txt" | head -1 | sed -E "s/.*\((.*)\)/\1/")
echo "=== binary $B"
for n in db_engine::tests::artifact_history_empty_and_two_batch_replay_are_deterministic db_engine::tests::full_submit_durable_query_round_trip_over_a_real_document_authority db_engine::vcs_integration::retained_tests::vcs_derived_owner_process_aggregate_plus_one_rejects_without_consuming_input; do
  f=0; for r in 1 2 3 4 5; do for j in 1 2 3 4 5 6 7 8; do ( RUST_MIN_STACK=67108864 SEMIO_DB_ISOLATED_LAW=$n nice -n 10 "$B" --exact $n --test-threads=1 > "$L/h10-stress-$r-$j.txt" 2>&1 && rm "$L/h10-stress-$r-$j.txt" ) & done; wait; done
  echo "=== STRESS $n fails=$(ls "$L" | /usr/bin/grep -c "^h10-stress-") of 40 $(date +%T)"; ls "$L" | /usr/bin/grep "^h10-stress-" | head -1 | while read x; do /usr/bin/grep -E "panicked|witness" "$L/$x" | head -2; done; rm -f "$L"/h10-stress-*.txt
done
' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
