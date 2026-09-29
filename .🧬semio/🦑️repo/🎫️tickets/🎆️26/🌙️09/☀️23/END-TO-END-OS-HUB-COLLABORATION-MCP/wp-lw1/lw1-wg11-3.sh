#!/bin/zsh
# ⚖️ LW1 → WG11 third pass: WG11's own renderer laws by name (default stack), the AgentBridge offer-scope vitest (env-gated suite),
# ui-contract accessibility laws, then the full renderer lib with both aborting laws skipped (failure messages for triage).
R=/Users/ueli/Documents/semio
unset RUST_MIN_STACK
cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- replay_refusal unserved_guest_replay space_artifact_creation_replay agent_bridge_offer_scope presence_pointer one_mebibyte_thread rebootstrap reseed; echo "LW1-STEP renderer-wg11-laws rc=$?"
cargo test --offline --no-fail-fast -p semio-framework-ui-contract --all-features --lib -- accessibility; echo "LW1-STEP ui-contract-a11y rc=$?"
cd "$R/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript" && SEMIO_INCLUDE_AGENT_BRIDGE=1 SEMIO_TEST_LEVEL=long bun ./📜️script.ts test --reporter=verbose; echo "LW1-STEP agent-bridge-vitest rc=$?"
cd "$R" && cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- --skip async_boundary_tests::an_independent_decoder_job_recovers_its_response_from_an_exact_rejected_session --skip kernel_runtime::semantic_document_tests::product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder; echo "LW1-STEP renderer-wgpu-lib-skip2 rc=$?"
