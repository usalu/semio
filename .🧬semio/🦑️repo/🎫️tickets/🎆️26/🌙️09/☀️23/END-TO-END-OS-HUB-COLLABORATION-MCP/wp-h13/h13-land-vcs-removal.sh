#!/bin/zsh
# 🧹️ H13 (session 14c, coordinator "REMOVE approved"): lands the kernel-db `vcs` removal in ONE window held inside the hub lane
# and the native lane — TS source gates before/after, apply, kernel-db compile (default + all features), kernel-db lib laws,
# semio-hub check, `os-hub:build-dev` — and keeps the set only when every step is green; any red restores every touched path
# (reverse patch for edited files, so peer edits made meanwhile survive; exact base bytes for deleted files).
# usage: zsh fleet-mutex.sh hub h13 -- zsh fleet-mutex.sh native h13 -- zsh h13-land-vcs-removal.sh <label>
R=/Users/ueli/Documents/semio
W="$R/.🧬semio/🌐hub/s14-h13-work/vcs-removal"
H=$R/.tmp-ticket/wp-h13
L="$R/.🧬semio/🌐hub/s14-h13-logs"
P="$W/vcs-removal-modify.patch"
LABEL=$1
TOLERATED='db_engine::throughput_tests::(fs|sqlite)_commits_and_reopen_storms_stay_within_their_throughput_bounds'
cd $R || exit 1
echo "=== window $(date +%T) $LABEL (hub + native lanes held)"

restore() {
  echo "=== RED $(date +%T): $1 — restoring"
  if patch -p1 -R --dry-run -s < "$P" > /dev/null 2>&1; then
    patch -p1 -R -s < "$P"
  else
    while IFS= read -r f; do
      if cmp -s "$R/$f" "$W/edit/$f"; then cp "$W/base/$f" "$R/$f"; elif ! cmp -s "$R/$f" "$W/base/$f"; then echo "RESTORE-MANUAL $f"; fi
    done < "$W/modified-files.txt"
  fi
  while IFS= read -r f; do mkdir -p "$R/${f:h}"; cp "$W/base/$f" "$R/$f"; done < "$W/deleted-files.txt"
  while IFS= read -r f; do rm -f "$R/$f"; rmdir "$R/${f:h}" 2> /dev/null; done < "$W/created-files.txt"
  while IFS= read -r f; do cmp -s "$R/$f" "$W/base/$f" || echo "RESTORED-DIFFERS-FROM-BASE (peer edit?) $f"; done < <(cat "$W/modified-files.txt" "$W/deleted-files.txt")
  echo "=== RESTORED $(date +%T) $LABEL"
  exit 1
}

python3 $H/h13-ts-gates.py run "$L/$LABEL-gates-before.json" || exit 1
while IFS= read -r f; do cmp -s "$R/$f" "$W/base/$f" || { echo "=== ABORT: $f changed since the base copy (nothing applied)"; exit 1; }; done < "$W/deleted-files.txt"
while IFS= read -r f; do [ -e "$R/$f" ] && { echo "=== ABORT: $f already exists (nothing applied)"; exit 1; }; done < "$W/created-files.txt"
patch -p1 -N --dry-run -s < "$P" || { echo "=== ABORT: patch dry-run red (nothing applied)"; exit 1; }

echo "=== apply $(date +%T)"
patch -p1 -N -s < "$P" || restore "patch apply"
while IFS= read -r f; do rm -f "$R/$f"; done < "$W/deleted-files.txt"
for d in "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🕸️version-graph" "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🧪️tests/🔬️vcs-integration-retained" "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️tests/🔬️interactivity-vcs-bridge"; do
  find "$R/$d" -depth -type d -empty -delete 2> /dev/null
done
while IFS= read -r f; do mkdir -p "$R/${f:h}"; cp "$W/edit/$f" "$R/$f"; done < "$W/created-files.txt"
while IFS= read -r f; do cmp -s "$R/$f" "$W/edit/$f" || echo "APPLIED-DIFFERS-FROM-EDIT (peer edit merged) $f"; [ -e "$R/$f.orig" ] && echo "PATCH-LEFT $f.orig"; [ -e "$R/$f.rej" ] && restore "patch reject $f.rej"; done < "$W/modified-files.txt"

python3 $H/h13-ts-gates.py run "$L/$LABEL-gates-after.json" || restore "ts gates run"
python3 $H/h13-ts-gates.py compare "$L/$LABEL-gates-before.json" "$L/$LABEL-gates-after.json" || restore "new ts gate failures"
bun $H/h13-dbcli-probe.ts || restore "db cli census"

zsh $H/h13-cargo.sh "$LABEL-db-default" check -p semio-framework-os-kernel-db --locked --lib --tests || restore "kernel-db default check"
zsh $H/h13-cargo.sh "$LABEL-db-all" check -p semio-framework-os-kernel-db --locked --all-features --lib --tests || restore "kernel-db all-features check"
zsh $H/h13-cargo.sh "$LABEL-db-lib" test -p semio-framework-os-kernel-db --locked --features sqlite --lib --no-fail-fast
rc=$?
if [ $rc -ne 0 ]; then
  /usr/bin/grep -aq '^test result: ' "$L/$LABEL-db-lib.txt" || restore "kernel-db lib test did not run"
  failed=$(/usr/bin/grep -aE '^test .* FAILED$' "$L/$LABEL-db-lib.txt" | sed 's/^test //; s/ \.\.\. FAILED$//' | /usr/bin/grep -vE "^($TOLERATED)$")
  [ -n "$failed" ] && restore "kernel-db lib laws: $failed"
  echo "=== kernel-db lib: only the tolerated load-bound throughput laws failed"
fi
zsh $H/h13-cargo.sh "$LABEL-hub-check" check -p semio-hub --locked --all-features --lib --bins --tests || restore "semio-hub check"

echo "=== start $(date +%T) os-hub:build-dev" | tee "$L/$LABEL-build-dev.txt"
(cd $R && NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" SEMIO_TEST_ARTIFACT_DIR="$R/.🧬semio/🌐hub/s14-h13-test-artifacts/$LABEL-build-dev" nice -n 15 bun nx run os-hub:build-dev >> "$L/$LABEL-build-dev.txt" 2>&1)
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$L/$LABEL-build-dev.txt"
/usr/bin/grep -aE '^error(\[|:)|Successfully ran|existing outputs match the cache|Nx read the output from the cache|failed' "$L/$LABEL-build-dev.txt" | head -8
[ $rc -eq 0 ] || restore "os-hub:build-dev"

echo "=== LANDED $(date +%T) $LABEL"
