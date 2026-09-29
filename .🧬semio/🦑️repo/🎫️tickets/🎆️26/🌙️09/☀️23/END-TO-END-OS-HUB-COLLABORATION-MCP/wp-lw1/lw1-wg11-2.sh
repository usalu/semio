#!/bin/zsh
# ⚖️ LW1 → WG11 focused re-run: kernel with the sync lane (reseed + delegation laws are `sync,ureq`-gated), renderer lib at the
# default stack without the one aborting async-boundary law, ui-contract Rust + a11y TS twin, renderer-react verbose, pair TS law.
R=/Users/ueli/Documents/semio
unset RUST_MIN_STACK
cargo test --offline --no-fail-fast -p semio-framework-os-kernel --features sync,ureq --lib; echo "LW1-STEP kernel-sync-lib rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- --skip async_boundary_tests::an_independent_decoder_job_recovers_its_response_from_an_exact_rejected_session; echo "LW1-STEP renderer-wgpu-lib-skip1 rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-ui-contract --all-features; echo "LW1-STEP ui-contract-cargo rc=$?"
cd "$R/🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️accessibility-projection" && bun -e 'import { accessibilityProjectionSelfTests } from "./🟦️.ts"; console.log("a11y-twin checks=" + accessibilityProjectionSelfTests());'; echo "LW1-STEP a11y-ts-twin rc=$?"
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && SEMIO_TEST_LEVEL=long bun ./📜️script.ts test "../../../../🧪️tests/🔬️engine-contract/🟦️.ts" "../../../../🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts" "../../../../🧱️elements/🤖️AgentDelegations/🧪️tests/🧩️component/🟦️.tsx" --reporter=verbose; echo "LW1-STEP renderer-react-vitest-verbose rc=$?"
cd "$R" && WG10_VITEST_FILE="🧪️tests/🪢️canonical-checkpoint-pair/🟦️.ts" bunx vitest run --config "$R/.tmp-ticket/wp-wg10/vitest-one.config.mts" --reporter=verbose; echo "LW1-STEP pair-ts-law rc=$?"
