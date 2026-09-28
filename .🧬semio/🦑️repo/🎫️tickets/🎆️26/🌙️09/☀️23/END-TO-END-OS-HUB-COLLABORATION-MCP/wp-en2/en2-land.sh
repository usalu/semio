#!/bin/zsh
# 🛬️ EN2 window-3 landing of the prepared sets, compile-atomic: the sets are chained in order (a later set builds on an
# earlier one's result), the whole chain must dry-run clean on the live tree, then all sets are written in one go, then the
# native lane checks/tests every touched crate (private target dir, fleet build-dir).
# usage: zsh en2-land.sh dry|write|native
#   dry     — chained dry run of every set on the live tree (no writes)
#   write   — chained apply of every set (refuses if any set has a problem; then nothing is written)
#   native  — the native proof, meant to run inside `📜️fleet-mutex.sh native en2 -- zsh en2-land.sh native`
# After `write`: the draw descriptor (`✏️s/🔌️plugins/🖍️draw/🔣️.json`) must be re-described before any live MCP check —
# the three verb descriptions live in the editor source until then.
set -u
ROOT="/Users/ueli/Documents/semio"
HERE="$ROOT/.tmp-ticket/wp-en2"
SETS=(test-raw-routing energy-epjson energy-epjson-fixtures draw-verb-descriptions gltf-production-inverse gltf-create-material-fixture gltf-create-material-manifest gltf-probe-glb gltf-any-reader energy-oracle-translator bcf-reader docx-reader)
case "${1:-dry}" in
  dry)
    python3 "$HERE/en2-patch.py" apply $SETS ;;
  write)
    python3 "$HERE/en2-patch.py" apply $SETS --write ;;
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
