#!/bin/zsh
# 🎚️ H13 hold 14 (session 14c, window 3): the transport-refill set landed by L1 in T1a (kernel byte-budget refill law, os-mcp
# hub-unavailable laws) and the engine declared-batch law in its own process (production profile, count-based over-declaration).
# usage: h13-hold-14.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
LAW='db_engine::tests::a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently'
SEMIO_DB_ISOLATED_LAW=$LAW zsh $W/h13-cargo.sh "$1-engine-law" test -p semio-framework-os-kernel-db --features sqlite --lib -- --exact $LAW --nocapture
zsh $W/h13-cargo.sh "$1-refill-law" test -p semio-framework-os-kernel --lib --no-fail-fast -- an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn
zsh $W/h13-cargo.sh "$1-mcp-laws" test -p semio-framework-os-mcp --lib --no-fail-fast -- a_hub_unavailable_refusal_names_its_typed_cause_in_english_and_german a_transport_fault_keeps_its_cause_and_the_next_refresh_recovers_the_binding authenticated_hub an_agent_binds_at_its_capped_role
