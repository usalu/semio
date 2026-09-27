#!/bin/zsh
# 🛬️ EN2 window-3 landing of the prepared sets, compile-atomic: every set's dry run must be clean on the live tree, then
# all sets are written, then the native lane checks/tests every touched crate (private target dir, fleet build-dir).
# usage: zsh en2-land.sh dry|write|native
#   dry     — dry-run every set on the live tree (no writes)
#   write   — apply every set (refuses on the first set with problems; nothing of that set is written)
#   native  — the native proof, meant to run inside `📜️fleet-mutex.sh native en2 -- zsh en2-land.sh native`
set -u
ROOT="/Users/ueli/Documents/semio"
HERE="$ROOT/.tmp-ticket/wp-en2"
SETS=(test-raw-routing energy-epjson draw-verb-descriptions gltf-production-inverse gltf-create-material-fixture gltf-create-material-manifest gltf-probe-glb)
case "${1:-dry}" in
  dry)
    for s in $SETS; do python3 "$HERE/en2-patch.py" apply "$s" || exit 1; done ;;
  write)
    for s in $SETS; do python3 "$HERE/en2-patch.py" apply "$s" --write || exit 1; done ;;
  native)
    export CARGO_BUILD_BUILD_DIR="$ROOT/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR="$HERE/target" CARGO_INCREMENTAL=0
    cd "$ROOT" || exit 90
    rc=0
    step() { echo "[en2] $(date '+%H:%M:%S') $*"; nice -n 15 "$@"; local r=$?; echo "[en2] $(date '+%H:%M:%S') rc=$r"; rc=$(( rc + r )); }
    step cargo test --manifest-path "$ROOT/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust/Cargo.toml" --no-fail-fast
    step cargo check -p semio-s-artifact-energy-model --lib --tests
    step cargo test -p semio-s-artifact-energy-model --lib --no-fail-fast -- epjson
    step cargo check -p semio-s-plugin-draw --lib --tests
    step cargo check -p semio-s-artifact-stdio-gltf --lib --tests
    exit $rc ;;
esac
