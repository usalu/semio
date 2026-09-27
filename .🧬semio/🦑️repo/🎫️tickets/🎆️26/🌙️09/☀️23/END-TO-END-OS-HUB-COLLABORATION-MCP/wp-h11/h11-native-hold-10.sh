#!/bin/zsh
# 🎚️ H11: the P0 agent-audience laws (read delegation never edits, edit delegation edits, no other space, no owner
# authority) with the neighbouring agent/credential laws, then the cancellable-interpretation trusted-catalog laws and the
# sqlite binding law, an os-hub binary for the live checks, and a plugin-host test-target check.
# usage: h11-native-hold-10.sh <label>
cd /Users/ueli/Documents/semio
export H11_NICE=0
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h11
zsh $W/h11-cargo.sh "$1-bin-laws" test -p semio-hub --all-features --bin os-hub --no-fail-fast -- an_agent_session_holds_at_most_its_delegations_audience_in_its_delegations_space an_agent_delegation_mints_a_session revoking_a_delegation_closes a_withdrawn_delegation_admits_no_agent_edit an_agent_can_never_widen only_an_author_of_the_space_can_delegate credential_optional_routes a_peer_beat_reaches
zsh $W/h11-cargo.sh "$1-laws" test -p semio-hub --all-features --lib --no-fail-fast -- trusted_catalog::tests::guest_codec_calls_run_off_the_async_worker_and_relay_their_fuel trusted_catalog::tests::an_aborted_catalog_verification_leaves_no_interpretation_for_the_runtime_to_wait_on trusted_catalog::tests::no_codec_call_is_served_from_a_row_its_component_has_not_answered trusted_catalog::tests::all_trust_failures_precede_activation_and_have_bounded_diagnostics trusted_catalog::tests::concurrent_operations_share_one_compile trusted_catalog::tests::a_guest_codec_verification socket_binding_reads_are_exact_id_generation_selector_scope_and_status access_policy
zsh $W/h11-cargo.sh "$1-bin" build -p semio-hub --bin os-hub
zsh $W/h11-cargo.sh "$1-plugin-host" check -p semio-framework-plugin-host --lib --tests
