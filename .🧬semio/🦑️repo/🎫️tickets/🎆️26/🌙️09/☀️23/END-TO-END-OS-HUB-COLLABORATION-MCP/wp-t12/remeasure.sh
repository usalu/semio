#!/bin/zsh
# 📏️ T12: re-measures every capture the report cites after the 12:19–12:35 low-disk sweep deleted `wp-t12/generated`.
# One cargo at a time, sequential; each step logs to `.🧬semio/🌐hub/s11-t12-captures/<step>.txt` and appends
# `EXIT=<code> SECONDS=<wall>`; the runner's own progress goes to `remeasure.txt` beside them.
# usage: zsh remeasure.sh [first-step]
cd /Users/ueli/Documents/semio || exit 1
OUT=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-t12-captures
T12=/Users/ueli/Documents/semio/.tmp-ticket/wp-t12
mkdir -p "$OUT"
export CARGO_INCREMENTAL=0 NX_DAEMON=false
PRIVATE=/Users/ueli/Documents/semio/.tmp-ticket/wp-t12/target
from=${1:-native-switch}; go=0

step() {
  local name=$1; shift
  [ "$name" = "$from" ] && go=1
  [ $go -eq 1 ] || return 0
  echo "START $name $(date '+%T')" >> "$OUT/remeasure.txt"
  local start=$(date +%s)
  "$@" > "$OUT/$name.txt" 2>&1
  local code=$?
  echo "EXIT=$code SECONDS=$(( $(date +%s) - start ))" >> "$OUT/$name.txt"
  echo "END $name EXIT=$code $(date '+%T')" >> "$OUT/remeasure.txt"
}

switch_packages=(); while IFS= read -r crate; do [ -n "$crate" ] && switch_packages+=(-p "$crate"); done < "$T12/check-crates.txt"
s15_plugins=(); for plugin in stdio sourcing demonstrator norm procedural raster process reasoning writer trinity gis vcs draw architect imperative remodel; do s15_plugins+=(-p "semio-s-plugin-$plugin"); done
stdio_kit=(); stdio_kit_features=(); for c in csv tsv txt md html json xml; do stdio_kit+=(-p "semio-s-artifact-stdio-$c"); stdio_kit_features+=("semio-s-artifact-stdio-$c/component-app-assembly"); done
oracle_manifest="✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/Cargo.toml"

step native-switch cargo check "${switch_packages[@]}" --lib --tests --keep-going --message-format=short
step native-s15 cargo check "${s15_plugins[@]}" --lib --keep-going --message-format=short
step wasm-guests zsh .tmp-ticket/📜️fleet-mutex.sh wasm T12 -- cargo check --target wasm32-wasip2 -p semio-framework-replication -p semio-s-plugin-cad "${s15_plugins[@]}" --lib --keep-going --message-format=short
step tsc ./node_modules/.bin/tsc -p "$T12/tsconfig.t12.json"
step oracle-crate-check cargo check --manifest-path "$oracle_manifest" --features oracles --lib --tests --message-format=short
step test-law-vector env CARGO_TARGET_DIR="$PRIVATE" cargo test --manifest-path "$oracle_manifest" --features oracles --lib --no-fail-fast -- law::vector
step test-stdio-kit env CARGO_TARGET_DIR="$PRIVATE" cargo test "${stdio_kit[@]}" --features "${(j:,:)stdio_kit_features}" --lib --no-fail-fast
step test-curation env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-sourcing-curation --lib --no-fail-fast
step test-norm env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-norm-contract -p semio-s-artifact-norm-din18599 -p semio-s-artifact-norm-en1990 --lib --no-fail-fast
step test-viewers env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-procedural-generation2d -p semio-s-artifact-raster-raster -p semio-s-artifact-process-process3d -p semio-s-artifact-reasoning-wires -p semio-s-artifact-writer-writer -p semio-s-artifact-trinity-jack -p semio-s-artifact-trinity-rewriting -p semio-s-artifact-draw-drawing --features semio-s-artifact-procedural-generation2d/component-app-assembly,semio-s-artifact-trinity-jack/component-app-assembly,semio-s-artifact-trinity-rewriting/component-app-assembly --lib --no-fail-fast -- viewer
step test-gis env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-gis-gismap --features component-app-assembly --lib --no-fail-fast
step test-vcs env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-vcs-vcs --lib --no-fail-fast
step test-trinity env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-trinity-jack -p semio-s-artifact-trinity-rewriting --features semio-s-artifact-trinity-jack/component-app-assembly,semio-s-artifact-trinity-rewriting/component-app-assembly --lib --no-fail-fast
step test-architect-imperative env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-architect-program -p semio-s-artifact-imperative-procedure --lib --no-fail-fast
step test-raster-1 env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-raster-raster --lib --no-fail-fast
step test-raster-2 env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-raster-raster --lib --no-fail-fast
step test-raster-3 env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-raster-raster --lib --no-fail-fast
step test-raster-4 env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-raster-raster --lib --no-fail-fast
step test-raster-5 env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-raster-raster --lib --no-fail-fast
step test-remodel env CARGO_TARGET_DIR="$PRIVATE" cargo test -p semio-s-artifact-remodel-remodeling --lib --no-fail-fast
step remodel-probe zsh -c "cd '$T12/remodel-probe' && CARGO_TARGET_DIR='$PRIVATE' cargo run --bin t12-remodel-probe"
step vector-harness-build zsh -c "cd '$T12/vector-harness' && CARGO_TARGET_DIR='$PRIVATE' cargo build --bin t12-vector-harness"
step editor-measure env T12_HARNESS="$PRIVATE/debug/t12-vector-harness" python3 "$T12/editor-measure.py"
step inventory bun nx run @semio-tech/repo-test-domain:test-inventory --outputStyle=stream
step contract bun nx run @semio-tech/repo-test-domain:test-contract --outputStyle=stream
step contract-rows cp .🧬semio/🦑️repo/⚡️cache/breaches/testing.json "$OUT/contract-rows.json"
for plugin in gis lowpoly mathematical wfc fem architect; do
  step "test-quick-$plugin" zsh -c "cd \"\$(ls -d ✏️s/🔌️plugins/*$plugin/📦️packages/🦀️rust | head -1)\" && CARGO_TARGET_DIR='$PRIVATE' bun ./📜️script.ts test quick"
done
echo "ALL_DONE $(date '+%T')" >> "$OUT/remeasure.txt"
