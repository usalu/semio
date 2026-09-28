#!/bin/zsh
# 🎚️ H13 hold 4 (session 14b, items 1, 3, 4, 7): the hub laws on today's tree — the P0 agent-ceiling / delegation / revocation
# bin laws + the transient-apply refusal bin law, the refusal unit laws (credential + transient schema message), the
# access-policy truth table and the cancellable-interpretation lib laws, the os-mcp role + catalog laws (lease corpus now at
# channel 19), then the all-driver os-hub binary (sqlite + postgres + neo4j) for the live gates and the full
# `os-hub:test-all-features` suite.
# usage: h13-hold-4.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-bin-laws" test -p semio-hub --all-features --bin os-hub --no-fail-fast -- an_agent_session_holds_at_most_its_delegations_audience_in_its_delegations_space an_agent_delegation_mints_a_session revoking_a_delegation_closes a_withdrawn_delegation_admits_no_agent_edit an_agent_can_never_widen only_an_author_of_the_space_can_delegate credential_optional_routes a_transiently_refused_batch_names_the_declared_resend_code
zsh $W/h13-cargo.sh "$1-lib-laws" test -p semio-hub --all-features --lib --no-fail-fast -- refusal:: access_policy trusted_catalog::tests::guest_codec_calls_run_off_the_async_worker_and_relay_their_fuel trusted_catalog::tests::an_aborted_catalog_verification_leaves_no_interpretation_for_the_runtime_to_wait_on
zsh $W/h13-cargo.sh "$1-mcp-laws" test -p semio-framework-os-mcp --lib --no-fail-fast -- an_agent_binds_at_its_capped_role authenticated_hub_workspace authenticated_hub_catalog
zsh $W/h13-cargo.sh "$1-os-hub" build -p semio-hub --bin os-hub --features postgres,neo4j
zsh $W/h13-cargo.sh "$1-all-features" test -p semio-hub --all-features --no-fail-fast
