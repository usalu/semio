#!/usr/bin/env bash
# 🏗️ FEM 2D smoke with this ticket's change: wasm | materialize | serve | smoke. Reuses the FEM ticket's native route (profile, materializer) and the 2D smoke; logs land in this ticket's generated folder.
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; R=/c/semio
F="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
S="$R/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/GET-DEV-FEM-2D-WORKING-END-TO-END/🧪️browser-smoke.mjs"
WASM='C:/t/wd/wasm32-wasip2/prototype/semio_hub_fem_component.wasm'
mkdir -p "$G" /c/t/wd /c/t/wdb /c/t/wdtmp
case "$1" in
wasm)
  cd "$R/🌎️hub" || exit 2
  export TMP='C:\t\wdtmp' TEMP='C:\t\wdtmp' TMPDIR='C:\t\wdtmp' CARGO_INCREMENTAL=0 CARGO_TARGET_DIR='C:\t\wd' CARGO_BUILD_BUILD_DIR='C:\t\wdb'
  export CARGO_BUILD_RUSTC=rustc CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= SEMIO_BUILD_MODE=prototype RUST_MIN_STACK=134217728
  export CARGO_UNSTABLE_FEATURE_UNIFICATION=true CARGO_RESOLVER_FEATURE_UNIFICATION=selected
  for attempt in 1 2 3; do
    cargo build --offline -j 3 --manifest-path "$R/🌎️hub/Cargo.toml" --profile prototype --target wasm32-wasip2 -p semio-hub-fem-component --lib --config "$(cygpath -m "$F/native-fem-profile.toml")" > "$G/fem-wasm.log" 2>&1
    rc=$?; echo "attempt=$attempt rc=$rc" >> "$G/fem-wasm.log"; [ $rc = 0 ] && break
  done
  ;;
materialize)
  cd "$F" || exit 2
  bun "$F/native-fem-materialize.ts" "$WASM" > "$G/fem-materialize.log" 2>&1; echo "rc=$?" >> "$G/fem-materialize.log"
  ;;
serve)
  bash "$F/native-fem-dev.sh" serve2d
  ;;
smoke)
  cd "$R" || exit 2
  node "$S" "http://127.0.0.1:6086/?plugin=fem2d" > "$G/fem-smoke.log" 2>&1; echo "rc=$?" >> "$G/fem-smoke.log"
  ;;
*) echo "usage: $0 wasm|materialize|serve|smoke"; exit 2;;
esac
