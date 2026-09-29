#!/bin/zsh
# 🧯️ H13 (session 15, coordinator P0 21:4x, set B): lands "an ack is never given without a durable WAL record" in ONE native-lane
# window — the WAL reserves every page a submit needs before the engine applies it (typed bounded wait on DB I/O credit), a
# diverged engine fail-stops (`Closed`) instead of acknowledging a resend through its dedupe, submits beyond the process's
# slots wait first come first served (shared `db_policy::AdmissionLine`), refused applies never starve state retirement.
# Steps: Ajv schema oracles, TS source gates
# before/after, apply, kernel-db check (default + all features), semio-hub check, full kernel-db lib (sqlite; only the load-bound
# throughput wall-ratio laws tolerated), `os-hub:build-dev`; any red
# reverses the patch (peer edits made meanwhile survive).
# usage: zsh 📜️fleet-mutex.sh native h13 -- zsh h13-land-wal-writer-lease.sh <label>
R=/Users/ueli/Documents/semio
W="$R/.🧬semio/🌐hub/s14-h13-work/ack-durability"
H=$R/.tmp-ticket/wp-h13
L="$R/.🧬semio/🌐hub/s14-h13-logs"
P="$W/ack-durability.patch"
D="🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db"
LABEL=$1
TOLERATED='db_engine::throughput_tests::(fs|sqlite)_commits_and_reopen_storms_stay_within_their_throughput_bounds'
cd $R || exit 1
echo "=== window $(date +%T) $LABEL (native lane held)"

restore() {
  echo "=== RED $(date +%T): $1 — reversing"
  patch -p1 -R -s < "$P" || echo "RESTORE-MANUAL: reverse patch red"
  while IFS= read -r f; do rm -f "$R/$f.orig"; done < "$W/files.txt"
  echo "=== RESTORED $(date +%T) $LABEL"
  exit 1
}

patch -p1 -N --dry-run -s < "$P" || { echo "=== ABORT: patch dry-run red (nothing applied)"; exit 1; }
python3 $H/h13-ts-gates.py run "$L/$LABEL-gates-before.json" || exit 1

echo "=== apply $(date +%T)"
patch -p1 -N -s < "$P" || restore "patch apply"
while IFS= read -r f; do rm -f "$R/$f.orig"; [ -e "$R/$f.rej" ] && restore "patch reject $f.rej"; cmp -s "$R/$f" "$W/edit/$f" || echo "APPLIED-DIFFERS-FROM-EDIT (peer edit merged) $f"; done < "$W/files.txt"

bun $H/h13-ajv-def-check.ts "$R/$D/🗄️storage/🔐️writer/🧬️schema/🔣️.json" WriterAuthorityV1 "$R/$D/🗄️storage/🔐️writer/🧫️fixtures/🔣️.json" | tee "$L/$LABEL-ajv.txt" | /usr/bin/grep -q '^VALID' || restore "Ajv writer authority oracle"
bun $H/h13-ajv-def-check.ts "$R/$D/⚙️engine/🧬️schema/🔣️.json" ThroughputV1 "$R/$D/⚙️engine/🧫️fixtures/⏱️throughput/🔣️.json" | tee -a "$L/$LABEL-ajv.txt" | /usr/bin/grep -q '^VALID' || restore "Ajv throughput oracle"
zsh $H/h13-cargo.sh "$LABEL-db-default" check -p semio-framework-os-kernel-db --locked --lib --tests || restore "kernel-db default check"
zsh $H/h13-cargo.sh "$LABEL-db-all" check -p semio-framework-os-kernel-db --locked --all-features --lib --tests || restore "kernel-db all-features check"
zsh $H/h13-cargo.sh "$LABEL-hub-check" check -p semio-hub --locked --all-features --lib --bins --tests || restore "semio-hub check"
for law in db_engine::tests::submits_beyond_the_process_slots_wait_first_come_first_served_and_all_commit db_artifact::tests::refused_applies_never_starve_state_retirement db_wal::tests::a_prepared_submit_waits_for_db_io_credit_and_then_commits db_artifact::tests::a_filesystem_edit_is_acknowledged_only_once_durable db_artifact::tests::a_sqlite_edit_is_acknowledged_only_once_durable db_engine::tests::open_filesystem_documents_beyond_the_writer_slots_commit_concurrently_and_replay; do
  SEMIO_DB_ISOLATED_LAW=$law zsh $H/h13-cargo.sh "$LABEL-law-${law##*::}" test -p semio-framework-os-kernel-db --locked --features sqlite --lib -- --exact $law --nocapture || red_laws="$red_laws $law"
done
[ -n "$red_laws" ] && restore "laws$red_laws"
zsh $H/h13-cargo.sh "$LABEL-db-lib" test -p semio-framework-os-kernel-db --locked --features sqlite --lib --no-fail-fast
if [ $? -ne 0 ]; then
  /usr/bin/grep -aq '^test result: ' "$L/$LABEL-db-lib.txt" || restore "kernel-db lib test did not run"
  failed=$(/usr/bin/grep -aE '^test .* FAILED$' "$L/$LABEL-db-lib.txt" | sed 's/^test //; s/ \.\.\. FAILED$//' | sort -u | /usr/bin/grep -vE "^($TOLERATED)$")
  [ -n "$failed" ] && restore "kernel-db lib laws: $failed"
  echo "=== kernel-db lib: only the tolerated load-bound throughput laws failed"
fi
python3 $H/h13-ts-gates.py run "$L/$LABEL-gates-after.json" || restore "ts gates run"
python3 $H/h13-ts-gates.py compare "$L/$LABEL-gates-before.json" "$L/$LABEL-gates-after.json" || restore "new ts gate failures"

echo "=== start $(date +%T) os-hub:build-dev" | tee "$L/$LABEL-build-dev.txt"
(cd $R && NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" SEMIO_TEST_ARTIFACT_DIR="$R/.🧬semio/🌐hub/s14-h13-test-artifacts/$LABEL-build-dev" nice -n 15 bun nx run os-hub:build-dev >> "$L/$LABEL-build-dev.txt" 2>&1)
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$L/$LABEL-build-dev.txt"
/usr/bin/grep -aE '^error(\[|:)|Successfully ran|existing outputs match the cache|Nx read the output from the cache|failed' "$L/$LABEL-build-dev.txt" | head -8
[ $rc -eq 0 ] || restore "os-hub:build-dev"

echo "=== LANDED $(date +%T) $LABEL"
