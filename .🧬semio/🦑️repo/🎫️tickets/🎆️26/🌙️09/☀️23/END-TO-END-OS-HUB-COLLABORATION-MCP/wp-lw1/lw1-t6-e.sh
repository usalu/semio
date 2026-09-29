#!/bin/zsh
# ⚖️ LW1 → T6 rows 14 + 15 (CD1), CD1's own commands on the live tree: stdio-semio `--tests`, flow `flow_brep_invoke`, cad `typology`,
# repo-lib frozen/projection laws (bun), cad vitest (all suites; the semio suite needs the flow-core wasm rebuild — noted, not blocking).
R=/Users/ueli/Documents/semio
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export NX_DAEMON=false
cd "$R" || exit 2
step stdio-semio-tests 1200 cargo test --offline --no-fail-fast -p semio-s-artifact-stdio-semio --tests
step flow-brep-invoke 600 cargo test --offline --no-fail-fast -p semio-framework-os-flow --test flow_brep_invoke
step cad-typology 600 cargo test --offline --no-fail-fast -p semio-s-artifact-cad-cad --lib typology
cd "$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library" && step repo-lib-frozen-projection 420 bun test "./🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts" "./🧪️tests/☂️frozen-coordinate-wildcard-coverage/🟦️.ts" "./🧪️tests/🔬️workspace-contract/🟦️.ts" -t "frozen|catalog-projection|CAD frozen"
cd "$R/✏️s/🔌️plugins/📐️cad" && step cad-vitest 900 env SEMIO_TEST_LEVEL=long bunx vitest run --config "🧪️tests/🎚️config/🟦️.ts" --fileParallelism=false --reporter=dot
finish
