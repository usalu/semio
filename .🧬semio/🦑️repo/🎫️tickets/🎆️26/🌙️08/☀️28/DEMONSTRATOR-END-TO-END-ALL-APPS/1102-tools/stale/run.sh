#!/bin/bash
# stale lane W3 verification chain; markers stale-<n>.rc
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"; G="$C/🗑️generated/1102"; S="$C/1102-tools/slot.sh"
run(){ n=$1; shift; rm -f "$G/stale-$n.rc"; SEMIO_LANGUAGE=en "$S" "W-stale-$n" repo cargo --store=shared --configuration=prototype -- "$@" > "$G/stale-$n.txt" 2>&1; echo $? > "$G/stale-$n.rc"; }
run law test --manifest-path "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🌆️bin-02/📦️packages/🦀️rust/Cargo.toml" --no-fail-fast -- persisted_hydration_authority
run wasm check --manifest-path Cargo.toml --target wasm32-wasip2 --keep-going -p semio-framework-plugin --lib
run energy test --manifest-path "$PWD/✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.toml" --no-fail-fast -p semio-s-artifact-energy-model-editor-mounted-tests --lib
run verfolgen test --manifest-path "$PWD/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.toml" --no-fail-fast -p semio-s-artifact-gis-gismap-gis2d-unit-tests --lib -- verfolgen_pane
echo done > "$G/stale-all.rc"
