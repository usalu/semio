#!/bin/zsh
# 🎚️ H13 hold 16 (session 14c): the engine declared-batch law in its own process (the over-declared submit's refusal read at
# either result level), and the transport-refill kernel law with the native directory transport features (`sync`, `ureq`).
# usage: h13-hold-16.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
LAW='db_engine::tests::a_declared_maximal_batch_commits_through_the_database_and_an_over_declared_one_is_refused_permanently'
SEMIO_DB_ISOLATED_LAW=$LAW zsh $W/h13-cargo.sh "$1-engine-law" test -p semio-framework-os-kernel-db --features sqlite --lib -- --exact $LAW --nocapture
zsh $W/h13-cargo.sh "$1-refill-law" test -p semio-framework-os-kernel --lib --features sync,ureq --no-fail-fast -- an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn
