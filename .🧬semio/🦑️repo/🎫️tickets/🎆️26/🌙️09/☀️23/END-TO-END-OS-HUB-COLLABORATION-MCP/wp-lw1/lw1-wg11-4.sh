#!/bin/zsh
# ⚖️ LW1 → WG11 fourth pass: name the stack-overflowing law (one test thread prints each name before it runs), the same laws with
# RUST_MIN_STACK 8 MiB (WG11's measurement condition), then the full renderer lib under 8 MiB with the 2 aborting laws skipped.
unset RUST_MIN_STACK
cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- --test-threads 1 replay_refusal unserved_guest_replay space_artifact_creation_replay agent_bridge_offer_scope presence_pointer one_mebibyte_thread rebootstrap reseed; echo "LW1-STEP renderer-wg11-laws-serial rc=$?"
RUST_MIN_STACK=8388608 cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- replay_refusal unserved_guest_replay space_artifact_creation_replay agent_bridge_offer_scope presence_pointer one_mebibyte_thread rebootstrap reseed; echo "LW1-STEP renderer-wg11-laws-8mib rc=$?"
RUST_MIN_STACK=8388608 cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- --skip async_boundary_tests::an_independent_decoder_job_recovers_its_response_from_an_exact_rejected_session --skip kernel_runtime::semantic_document_tests::product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder; echo "LW1-STEP renderer-wgpu-lib-8mib-skip2 rc=$?"
