#!/bin/zsh
# ⚖️ LW1 → T6 round 2 part 2 (LB2 rows 5a/5b2/3, S19 row 3b): pptx fixture honesty, contract part21 + ifc committed-fixture law +
# IfcOpenShell oracle, manifest typegen law, hub trusted_catalog, demonstrator + describe libs, os TS surface-opens-kind.
R=/Users/ueli/Documents/semio; C="$R/.🧬semio/🌐hub/s14-lw1-logs"
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
cd "$R" || exit 2
mkdir -p "$C/t6-part21-oracle"
step pptx-fixture-honesty 600 cargo test --offline -p semio-s-artifact-stdio-pptx --features semio-s-artifact-stdio-pptx/component-app-assembly --lib fixture_honesty_law
step contract-part21 420 cargo test --offline -p semio-s-artifact-stdio-contract --lib part21
step ifc-part21-committed 600 env SEMIO_PART21_ORACLE_OUT="$C/t6-part21-oracle" cargo test --offline -p semio-s-artifact-stdio-ifc --features semio-s-artifact-stdio-ifc/component-app-assembly --lib committed_ifc2x3_fixtures_read_through_the_canonical_part21_codec
step ifcopenshell-oracle 300 python3 "$R/.tmp-ticket/wp-lb2/lb2-p16-part21-oracle.py" "$C/t6-part21-oracle"
step typegen-law 600 cargo test --offline -p semio-framework --features typegen --lib exports_typescript_bindings
export RUST_MIN_STACK=33554432
step hub-trusted-catalog 900 cargo test --offline --no-fail-fast -p semio-hub --lib trusted_catalog
step demonstrator-lib 900 cargo test --offline --no-fail-fast -p semio-s-plugin-demonstrator --lib
step describe-lib 600 cargo test --offline --no-fail-fast -p semio-framework-plugin-describe --lib
cd "$R/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" && step ts-surface-opens-kind 420 env NX_DAEMON=false bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "surface-opens-kind"
finish
