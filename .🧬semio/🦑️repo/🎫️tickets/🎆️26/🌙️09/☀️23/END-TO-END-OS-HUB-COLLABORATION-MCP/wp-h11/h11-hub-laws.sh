#!/bin/zsh
# ⚖️ H11 item 1: H9's session-12 hub laws + H11's credential law, then the full all-features suite, on the H11 private
# target in build-fleet-b (the test binaries of `h11-cargo.sh <label> test -p semio-hub --all-features --no-run`).
# usage: h11-hub-laws.sh <label> [full]
cd /Users/ueli/Documents/semio
LABEL=$1; MODE=${2:-named}
BIN_LAWS=(
  a_member_of_many_spaces_reads_its_space_list_and_event_pages_within_the_page_budget
  a_withdrawn_delegation_admits_no_agent_edit_after_it_in_either_order
  a_committed_gis_map_approval_is_answered_with_its_receipt
  a_command_in_flight_never_stalls_the_spaces_directory_commands_or_deliveries
  every_command_is_answered_when_the_database_refuses_writes
  revoking_a_delegation_closes_the_agents_open_document_socket_and_roster_row
  an_agent_delegation_mints_a_session_that_works_until_it_is_revoked
  credential_optional_routes_refuse_a_revoked_or_forged_session_instead_of_answering_anonymously
  startup_catalog_progress_is_visible_at_the_launcher_level
  the_hostile_input_fixture_covers_every_registered_route
  every_route_answers_hostile_input_with_a_typed_signed_refusal
  gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write
  a_peer_beat_reaches_the_document_roster_and_the_directory_projection_only_when_it_changes
  document_open
  execution_target
)
LIB_LAWS=(refusal:: artifact_authority:: inference::)
if [ "$MODE" = named ]; then
  zsh .tmp-ticket/wp-h11/h11-cargo.sh "$LABEL-bin" test -p semio-hub --all-features --bin os-hub --no-fail-fast -- "${BIN_LAWS[@]}"
  zsh .tmp-ticket/wp-h11/h11-cargo.sh "$LABEL-lib" test -p semio-hub --all-features --lib --no-fail-fast -- "${LIB_LAWS[@]}"
else
  zsh .tmp-ticket/wp-h11/h11-cargo.sh "$LABEL-full" test -p semio-hub --all-features --no-fail-fast
fi
