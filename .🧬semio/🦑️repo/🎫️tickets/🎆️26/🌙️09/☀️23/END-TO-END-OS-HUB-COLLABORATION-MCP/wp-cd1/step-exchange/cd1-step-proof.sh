#!/bin/zsh
# ⚖️ CD1 overlay proof — ONE native-lane ticket, stages in order: regenerate the step-exchange fixture's golden exchange text
# (ignored printer) → write it into the overlay → stdio-semio lib laws (all) → the exchange laws again (determinism across
# processes) → cad laws of both sets (exports, typology/construction, interactions, meshes) → cad vitest (all suites) → tsc of
# the cad TS package and of the touched suites. Captures `.🧬semio/🌐hub/s14-cd1-work/s15/<round>-<stage>.txt`. Usage: cd1-step-proof.sh <round>
H="/Users/ueli/Documents/semio/.🧬semio/🌐hub"; O="$H/s14-cd1-overlay"; W="$H/s14-cd1-work/s15"; T=/Users/ueli/Documents/semio/.tmp-ticket/wp-cd1/step-exchange
r="$1"
export CARGO_INCREMENTAL=0 NX_DAEMON=false RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$H/s14-cd1-build" CARGO_TARGET_DIR="$H/s14-cd1-target"
stage() { local name="$1"; shift; echo "STAGE $name START $(date '+%H:%M:%S') load=$(sysctl -n vm.loadavg)"; nice -n 15 "$@" > "$W/$r-$name.txt" 2>&1; local rc=$?; echo "STAGE $name rc=$rc $(date '+%H:%M:%S')"; return $rc; }
cd "$O" || exit 2
if stage print cargo test --offline -p semio-s-artifact-stdio-semio --lib -- --ignored print_step_exchange_fixture --nocapture; then
  python3 "$T/cd1-fill-fixture.py" "$W/$r-print.txt" "$O"
  stage stdio cargo test --offline --no-fail-fast -p semio-s-artifact-stdio-semio --lib
  stage exchange-again cargo test --offline -p semio-s-artifact-stdio-semio --lib -- step_exchange
fi
stage stdio-io cargo test --offline --no-fail-fast -p semio-s-artifact-stdio-semio --lib --features conversion-brep -- brep::io
stage cad cargo test --offline --no-fail-fast -p semio-s-artifact-cad-cad --lib -- export_solids_as modelspace_brep_out current_pane_exports typology interaction world_meshes typology_brep_mesh object_mesh
cd "$O/✏️s/🔌️plugins/📐️cad" && stage vitest bunx vitest run --config "🧪️tests/🎚️config/🟦️.ts" --fileParallelism=false --reporter=verbose
cd "$O" && stage tsc-cad bunx tsc --noEmit -p "✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/tsconfig.json" --tsBuildInfoFile "$W/tsc-cad.tsbuildinfo"
stage tsc-suites bunx tsc --noEmit -p "$T/tsconfig-cd1-suite.json"
echo "CHAIN DONE $(date '+%H:%M:%S')" > "$W/$r-done.txt"
