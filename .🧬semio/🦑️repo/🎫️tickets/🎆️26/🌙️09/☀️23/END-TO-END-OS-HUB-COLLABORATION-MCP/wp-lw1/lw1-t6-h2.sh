#!/bin/zsh
# ⚖️ LW1 → T6 row 6 (U6 row-target) part 2 on the live tree: the other lib suites of U6's 38-crate set (renderer-wgpu runs per process in
# block c; the SDK per process in block d2), the React renderer vitest (U6: whole package) and the ui-contract TS self-tests.
R=/Users/ueli/Documents/semio; C="$R/.🧬semio/🌐hub/s14-lw1-logs"; S="$R/.tmp-ticket/wp-lw1/lw1-suites.sh"; L="$C/t6-h2-bins.txt"
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export CARGO_NET_OFFLINE=true NX_DAEMON=false RUST_MIN_STACK=67108864
cd "$R" || exit 2
step u6-38-build 1200 zsh "$S" build "$L"
for c in semio-s-artifact-space-space semio-s-plugin-space semio-s-artifact-stdio-contract semio-s-artifact-stdio-csv semio-s-artifact-stdio-tsv semio-s-artifact-stdio-wav semio-s-artifact-stdio-bcf semio-s-artifact-stdio-xlsx semio-s-plugin-procedural semio-s-artifact-procedural-generation3d semio-s-artifact-procedural-generation2d semio-s-plugin-playbook-procedural semio-s-artifact-forms-forms semio-s-artifact-lowpoly-lowpoly semio-s-artifact-energy-model semio-s-artifact-raster-raster semio-s-artifact-flow-flow semio-s-artifact-sourcing-curation semio-s-plugin-puzzle semio-s-plugin-cad semio-s-plugin-process semio-s-plugin-flow semio-s-plugin-forms semio-s-plugin-lowpoly semio-s-plugin-energy semio-s-plugin-raster semio-s-plugin-sourcing semio-s-plugin-stdio; do
  step "suite-$c" 420 zsh "$S" suite "$L" "$c"
done
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && step react-vitest 900 env SEMIO_TEST_LEVEL=standard bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" --reporter=dot
cd "$R/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust" && step ui-contract-ts-test 600 bun ./📜️script.ts test
finish
