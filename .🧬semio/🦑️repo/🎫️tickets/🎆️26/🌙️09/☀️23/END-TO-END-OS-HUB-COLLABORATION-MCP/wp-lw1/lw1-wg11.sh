#!/bin/zsh
# ⚖️ LW1 → WG11 laws (T3 wg11-reseed, board, a11y, replay-routes, offer-scope, shell-turn + lb2-p3-wg11 painter), default test stack.
R=/Users/ueli/Documents/semio
unset RUST_MIN_STACK
cargo test --offline --no-fail-fast -p semio-framework-os-kernel --lib; echo "LW1-STEP kernel-lib rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib; echo "LW1-STEP renderer-wgpu-lib-all rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-ui --features wgpu-engine --lib -- table_row_grid reconcile::tests flex::tests accessibility; echo "LW1-STEP ui-wgpu-table-a11y rc=$?"
cd "$R/🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust" && bun ./📜️script.ts test; echo "LW1-STEP ui-contract-test rc=$?"
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && SEMIO_TEST_LEVEL=long bun ./📜️script.ts test "../../../../🧪️tests/🔬️engine-contract/🟦️.ts" "../../../../🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts" "../../../../🧱️elements/🤖️AgentDelegations/🧪️tests/🧩️component/🟦️.tsx"; echo "LW1-STEP renderer-react-vitest rc=$?"
cd "$R/🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript" && bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "canonical-checkpoint-pair"; echo "LW1-STEP os-vitest-checkpoint-pair rc=$?"
