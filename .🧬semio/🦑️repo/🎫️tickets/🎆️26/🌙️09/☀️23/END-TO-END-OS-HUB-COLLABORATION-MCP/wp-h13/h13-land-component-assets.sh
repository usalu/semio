#!/bin/zsh
# 🧩️ H13 (session 14c, coordinator): lands execution-target components as a static-asset class in one native-lane window —
# the per-user execution-target store bounded least-recently-used, the kernel's component stream on its own transport, and
# the semio MCP gateway fetching a missing component lazily per plugin over an unmetered asset pool with typed progress
# (a store copy costs no network). Apply, compile (kernel sync+ureq, os-mcp, renderer-wgpu), laws (kernel execution-target,
# os-mcp remote), TS oracles (os resolution + eviction, os-mcp component corpus), then the os-mcp dev build staged for G12;
# any red before the build reverses the patch.
# usage: zsh fleet-mutex.sh native h13 -- zsh h13-land-component-assets.sh <label>
R=/Users/ueli/Documents/semio
W="$R/.🧬semio/🌐hub/s14-h13-work/component-assets"
H=$R/.tmp-ticket/wp-h13
L="$R/.🧬semio/🌐hub/s14-h13-logs"
P="$W/component-assets.patch"
CREATED="$R/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🧫️fixtures/🔣️hub-component-asset.json"
LABEL=$1
cd $R || exit 1
echo "=== window $(date +%T) $LABEL (native lane held)"

restore() {
  echo "=== RED $(date +%T): $1 — reversing"
  patch -p1 -R -s < "$P" || echo "RESTORE-MANUAL: reverse patch red"
  [ -f "$CREATED" ] && [ ! -s "$CREATED" ] && rm -f "$CREATED"
  echo "=== RESTORED $(date +%T) $LABEL"
  exit 1
}

ts_oracle() {
  echo "=== start $(date +%T) $LABEL-$1" | tee "$L/$LABEL-$1.txt"
  (cd "$2" && NX_DAEMON=false nice -n 15 bunx vitest run ${=3} >> "$L/$LABEL-$1.txt" 2>&1)
  local rc=$?
  echo "EXIT $rc $(date +%T)" | tee -a "$L/$LABEL-$1.txt"
  /usr/bin/grep -aE 'Tests |Test Files|FAIL|×' "$L/$LABEL-$1.txt" | head -12
  return $rc
}

patch -p1 -N --dry-run -s < "$P" || { echo "=== ABORT: patch dry-run red (nothing applied)"; exit 1; }
patch -p1 -N -s < "$P" || restore "patch apply"
echo "=== applied $(date +%T)"

zsh $H/h13-cargo.sh "$LABEL-kernel-check" check -p semio-framework-os-kernel --locked --lib --tests --features sync,ureq || restore "kernel check"
zsh $H/h13-cargo.sh "$LABEL-mcp-check" check -p semio-framework-os-mcp --locked --lib --tests || restore "os-mcp check"
zsh $H/h13-cargo.sh "$LABEL-renderer-check" check -p semio-framework-os-renderer-wgpu --locked --lib || restore "renderer-wgpu check"
zsh $H/h13-cargo.sh "$LABEL-kernel-laws" test -p semio-framework-os-kernel --locked --lib --features sync,ureq --no-fail-fast -- execution_target || restore "kernel execution-target laws"
zsh $H/h13-cargo.sh "$LABEL-mcp-laws" test -p semio-framework-os-mcp --locked --lib --no-fail-fast -- workspace::remote:: || restore "os-mcp remote laws"
ts_oracle os-oracle "$R/🧰️framework/🛍️products/💻️os" "--root . --config $H/h13-vitest-resolution.config.ts" || restore "os resolution oracle"
ts_oracle mcp-oracle "$R/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript" "--config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🔐️authenticated-hub-workspace/🟦️.ts" || restore "os-mcp oracle"
echo "=== LANDED $(date +%T) $LABEL"

echo "=== start $(date +%T) $LABEL-mcp-build" | tee "$L/$LABEL-mcp-build.txt"
(cd $R && NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="$R/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" SEMIO_TEST_ARTIFACT_DIR="$R/.🧬semio/🌐hub/s14-h13-test-artifacts/$LABEL-mcp-build" nice -n 15 bun nx run @semio-tech/framework-os-mcp-rs:build >> "$L/$LABEL-mcp-build.txt" 2>&1)
rc=$?
echo "EXIT $rc $(date +%T)" | tee -a "$L/$LABEL-mcp-build.txt"
/usr/bin/grep -aE '^error(\[|:)|Successfully ran|failed' "$L/$LABEL-mcp-build.txt" | head -6
if [ $rc -eq 0 ]; then
  mkdir -p "$R/.🧬semio/🌐hub/s14-h13-bin"
  rm -f "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-assets"
  cp "$R/🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build/semio-os-mcp" "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-assets"
  codesign -f -s - "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-assets" 2> /dev/null
  shasum -a 256 "$R/.🧬semio/🌐hub/s14-h13-bin/semio-os-mcp-assets"
fi
echo "=== DONE $(date +%T) $LABEL"
