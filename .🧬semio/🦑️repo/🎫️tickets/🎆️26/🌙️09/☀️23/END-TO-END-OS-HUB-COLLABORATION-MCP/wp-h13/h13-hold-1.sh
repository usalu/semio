#!/bin/zsh
# 🎚️ H13 hold 1 (items 4, 1, 3): kernel + plugin type-check and the two populated-pair codec laws (Check In retire fix),
# semio-hub all-features lib/tests/bins and os-mcp lib/tests type-check on the current tree,
# the os-mcp agent-binding role law (🔗️remote) and its fixture neighbours,
# H11's P0 agent-ceiling / agent-role / revocation-fence / credential / presence bin laws, the cancellable-interpretation
# + access-policy + sqlite binding lib laws, the full `os-hub:test-all-features` suite, and the all-driver os-hub
# binary (build-dev-postgres features: sqlite + postgres + neo4j) + the semio-os-mcp gateway for the live probes.
# usage: h13-hold-1.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-check-codec" check -p semio-framework-os-kernel -p semio-framework-plugin --lib --tests
zsh $W/h13-cargo.sh "$1-kernel-codec-law" test -p semio-framework-os-kernel --lib --no-fail-fast -- document_codec_apply_ops_binary_reduces_a_nonempty_batch_and_closes_its_store replay_envelopes_onto_pair_equals_the_replica_that_folded_the_same_ledger
zsh $W/h13-cargo.sh "$1-plugin-codec-law" test -p semio-framework-plugin --lib --no-fail-fast -- the_codec_table_mirrors_and_passes_through_a_populated_pair_without_aborting
zsh $W/h13-cargo.sh "$1-check-hub" check -p semio-hub --all-features --lib --tests --bins
zsh $W/h13-cargo.sh "$1-check-mcp" check -p semio-framework-os-mcp --lib --tests
zsh $W/h13-cargo.sh "$1-mcp-laws" test -p semio-framework-os-mcp --lib --no-fail-fast -- an_agent_binds_at_its_capped_role authenticated_hub_workspace authenticated_hub_catalog
zsh $W/h13-cargo.sh "$1-bin-laws" test -p semio-hub --all-features --bin os-hub --no-fail-fast -- an_agent_session_holds_at_most_its_delegations_audience_in_its_delegations_space an_agent_delegation_mints_a_session revoking_a_delegation_closes a_withdrawn_delegation_admits_no_agent_edit an_agent_can_never_widen only_an_author_of_the_space_can_delegate credential_optional_routes a_peer_beat_reaches check_in
zsh $W/h13-cargo.sh "$1-lib-laws" test -p semio-hub --all-features --lib --no-fail-fast -- trusted_catalog::tests::guest_codec_calls_run_off_the_async_worker_and_relay_their_fuel trusted_catalog::tests::an_aborted_catalog_verification_leaves_no_interpretation_for_the_runtime_to_wait_on socket_binding_reads_are_exact_id_generation_selector_scope_and_status access_policy
zsh $W/h13-cargo.sh "$1-all-features" test -p semio-hub --all-features --no-fail-fast
zsh $W/h13-cargo.sh "$1-os-hub" build -p semio-hub --bin os-hub --features postgres,neo4j
zsh $W/h13-cargo.sh "$1-os-mcp" build -p semio-framework-os-mcp --bin semio-os-mcp
