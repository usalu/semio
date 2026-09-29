#!/bin/zsh
# ⚖️ LW1: re-check every red reported so far under the canonical runner (cargo nextest = one process per test, RUST_MIN_STACK 128 MiB
# as `runCargoTestBudgeted`), including the retained_clone count fix (test-only).
export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true
cargo nextest run --profile long --no-fail-fast -p semio-framework-os-kernel --features sync,ureq --lib -E 'test(/retained_clone/)'; echo "LW1-STEP kernel-retained_clone-nextest rc=$?"
cargo nextest run --profile long --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -E 'test(/replay_refusal|unserved_guest_replay|space_artifact_creation_replay|agent_bridge_offer_scope|presence_pointer|one_mebibyte_thread|rebootstrap|reseed/)'; echo "LW1-STEP renderer-wg11-laws-nextest rc=$?"
cargo nextest run --profile long --no-fail-fast -p semio-framework-ui --features wgpu-engine --lib -E 'test(/table_row_grid|reconcile::tests|flex::tests|accessibility/)'; echo "LW1-STEP ui-wgpu-table-a11y-nextest rc=$?"
cargo nextest run --profile long --no-fail-fast -p semio-framework-ui-contract --all-features --lib; echo "LW1-STEP ui-contract-lib-nextest rc=$?"
