#!/bin/zsh
# ⚖️ LW1 → T6 row 7 + T7a/T7b (WG11) + row 6 (U6, ui wgpu half): ui(wgpu-engine) lib per process (nextest), the contract/value
# JSON-number laws, then the TS halves on the LIVE tree with WG11's own commands (`wg11-overlay-ts.sh` minus the overlay).
R=/Users/ueli/Documents/semio
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true NX_DAEMON=false
cd "$R" || exit 2
step ui-wgpu-lib-nextest 1500 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-framework-ui --features wgpu-engine --lib
step contract-ui-number 420 cargo nextest run --profile long --no-fail-fast -p semio-framework-ui-contract --all-features --lib -E 'test(/a_ui_number_serializes/)'
step value-json-number 420 cargo nextest run --profile long --no-fail-fast -p semio-framework-replication --lib -E 'test(/a_json_number_reads_back/)'
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" || exit 2
C="../../🧪️tests/🎚️config/🟦️.ts"
step ts-display-ids 420 env SEMIO_TEST_LEVEL=standard bunx vitest run --config "$C" --reporter=dot "📌️ChromePanels/🧪️tests/🧩️component" "🪟️window-lifecycle-template-drag"
step ts-engine-contract-display 420 env SEMIO_TEST_LEVEL=standard bunx vitest run --config "$C" --reporter=dot "🔬️engine-contract" -t "window kind|world-3d window kind|pre-reverses|drag payload decodes|delivery|lighting fixture"
step ts-interpreter-tree-window 420 env SEMIO_TEST_LEVEL=long bunx vitest run --config "$C" --reporter=dot "🗣️Interpreter/🟦️.tsx" -t "neutral viewport vectors|a window the guest answers short"
step ts-text-corpus-navbar-graphtimeline 600 env SEMIO_TEST_LEVEL=standard bunx vitest run --config "$C" --reporter=verbose "🔤️text-advances" "🔝️navbar-centered-band" "🌳️GraphTimelineHost/🧪️tests/🎨️layout"
finish
