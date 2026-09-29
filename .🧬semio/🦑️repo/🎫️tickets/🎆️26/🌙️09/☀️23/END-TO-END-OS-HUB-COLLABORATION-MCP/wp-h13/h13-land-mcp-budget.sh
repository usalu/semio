#!/bin/zsh
# 💰️ H13 (session 14c, coordinator P1): lands the os-mcp remote budget/settle fix in one native-lane window — a hub catalog
# refresh fetches each package descriptor once (content-addressed), a spent network byte budget is a typed wait for the refill
# (binding actor and tool threads, with progress), and a canonical pair attempt that raced an authority refresh waits for it.
# Apply, compile (os-mcp, kernel sync+ureq, renderer-wgpu), laws (os-mcp remote + pair, kernel refill law, TS AJV oracle),
# then the os-mcp dev build staged for G12; any red before the build reverses the patch.
# usage: zsh fleet-mutex.sh native h13 -- zsh h13-land-mcp-budget.sh <label>
R=/Users/ueli/Documents/semio
W="$R/.🧬semio/🌐hub/s14-h13-work/mcp-budget"
H=$R/.tmp-ticket/wp-h13
L="$R/.🧬semio/🌐hub/s14-h13-logs"
P="$W/mcp-budget.patch"
LABEL=$1
cd $R || exit 1
echo "=== window $(date +%T) $LABEL (native lane held)"

restore() {
  echo "=== RED $(date +%T): $1 — reversing"
  patch -p1 -R -s < "$P" || echo "RESTORE-MANUAL: reverse patch red"
  echo "=== RESTORED $(date +%T) $LABEL"
  exit 1
}

patch -p1 -N --dry-run -s < "$P" || { echo "=== ABORT: patch dry-run red (nothing applied)"; exit 1; }
patch -p1 -N -s < "$P" || restore "patch apply"
echo "=== applied $(date +%T)"

zsh $H/h13-cargo.sh "$LABEL-mcp-check" check -p semio-framework-os-mcp --locked --lib --tests || restore "os-mcp check"
zsh $H/h13-cargo.sh "$LABEL-kernel-check" check -p semio-framework-os-kernel --locked --lib --tests --features sync,ureq || restore "kernel check"
zsh $H/h13-cargo.sh "$LABEL-renderer-check" check -p semio-framework-os-renderer-wgpu --locked --lib || restore "renderer-wgpu check"
zsh $H/h13-cargo.sh "$LABEL-mcp-laws" test -p semio-framework-os-mcp --locked --lib --no-fail-fast -- workspace::remote:: || restore "os-mcp remote laws"
zsh $H/h13-cargo.sh "$LABEL-kernel-law" test -p semio-framework-os-kernel --locked --lib --features sync,ureq --no-fail-fast -- an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn || restore "kernel refill law"
echo "=== start $(date +%T) $LABEL-ts-oracle" | tee "$L/$LABEL-ts-oracle.txt"
(cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript" && NX_DAEMON=false nice -n 15 bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts >> "$L/$LABEL-ts-oracle.txt" 2>&1)
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$L/$LABEL-ts-oracle.txt"
/usr/bin/grep -aE 'Tests |Test Files|FAIL|✓|×' "$L/$LABEL-ts-oracle.txt" | head -12
[ $rc -eq 0 ] || restore "TS oracle"
echo "=== LANDED $(date +%T) $LABEL"

echo "=== start $(date +%T) $LABEL-mcp-build" | tee "$L/$LABEL-mcp-build.txt"
(cd $R && NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" SEMIO_TEST_ARTIFACT_DIR="$R/.🧬semio/🌐hub/s14-h13-test-artifacts/$LABEL-mcp-build" nice -n 15 bun nx run @semio-tech/framework-os-mcp-rs:build >> "$L/$LABEL-mcp-build.txt" 2>&1)
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$L/$LABEL-mcp-build.txt"
/usr/bin/grep -aE '^error(\[|:)|Successfully ran|failed' "$L/$LABEL-mcp-build.txt" | head -6
if [ $rc -eq 0 ]; then
  mkdir -p "$R/.🧬semio/🌐hub/s14-h13-bin"
  rm -f "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-budget"
  cp "$R/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp" "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-budget"
  codesign -f -s - "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-budget" 2> /dev/null
  shasum -a 256 "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-budget"
fi
echo "=== DONE $(date +%T) $LABEL"
