#!/bin/zsh
# 🚰️ H13 (session 14c, coordinator): lands the hub's execution-target asset stream rule in one hub-lane window — component
# and browser-actor bodies bounded by concurrent streams (hub-wide and per principal, first come first served, then the typed
# `execution-target-asset` 429) instead of any request bucket — plus the browser retry contract that rides that 429 out.
# Apply, semio-hub check (all features, lib+bins+tests), laws (auth unit, os-hub execution-target), TS oracles (hub
# hostile-input, os execution-target retry); any red reverses the patch.
# usage: zsh fleet-mutex.sh hub h13 -- zsh h13-land-hub-asset-streams.sh <label>
R=/Users/ueli/Documents/semio
W="$R/.🧬semio/🌐hub/s14-h13-work/hub-asset-streams"
H=$R/.tmp-ticket/wp-h13
L="$R/.🧬semio/🌐hub/s14-h13-logs"
P="$W/hub-asset-streams.patch"
LABEL=$1
cd $R || exit 1
echo "=== window $(date +%T) $LABEL (hub lane held)"

restore() {
  echo "=== RED $(date +%T): $1 — reversing"
  patch -p1 -R -s < "$P" || echo "RESTORE-MANUAL: reverse patch red"
  echo "=== RESTORED $(date +%T) $LABEL"
  exit 1
}

ts_oracle() {
  echo "=== start $(date +%T) $LABEL-$1" | tee "$L/$LABEL-$1.txt"
  (cd "$2" && NX_DAEMON=false nice -n 15 ${=3} >> "$L/$LABEL-$1.txt" 2>&1)
  local rc=$?
  echo "EXIT $rc $(date +%T)" | tee -a "$L/$LABEL-$1.txt"
  /usr/bin/grep -aE 'Tests |Test Files|FAIL|×' "$L/$LABEL-$1.txt" | head -12
  return $rc
}

patch -p1 -N --dry-run -s < "$P" || { echo "=== ABORT: patch dry-run red (nothing applied)"; exit 1; }
patch -p1 -N -s < "$P" || restore "patch apply"
echo "=== applied $(date +%T)"

zsh $H/h13-cargo.sh "$LABEL-hub-check" check -p semio-hub --locked --all-features --lib --bins --tests || restore "semio-hub check"
zsh $H/h13-cargo.sh "$LABEL-auth-laws" test -p semio-hub --locked --lib --no-fail-fast -- auth:: || restore "hub auth laws"
zsh $H/h13-cargo.sh "$LABEL-route-laws" test -p semio-hub --locked --bin os-hub --no-fail-fast -- execution_target || restore "os-hub execution-target laws"
ts_oracle hub-oracle "$R/🌎️hub/📦️packages/🟦️typescript" "bun ./📜️script.ts test quick hostile-input" || restore "hub hostile-input oracle"
ts_oracle retry-oracle "$R/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" "bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts ../../🧪️tests/🔁️execution-target-retry/🟦️.ts" || restore "os retry oracle"
echo "=== LANDED $(date +%T) $LABEL"
