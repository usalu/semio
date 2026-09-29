#!/bin/zsh
# 🧪️ C13 (14c): the hub socket law for foreign history transitions (os-hub bin tests) + the kernel-db event-log law, ONE native-lane hold.
export NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-target"
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- zsh -c '
  cd /Users/ueli/Documents/semio
  nice -n 15 cargo test -p semio-hub --bin os-hub --no-fail-fast -- a_foreign_history_transition_is_refused_at_the_socket_and_never_relayed socket_grant_document_route_is_exact_replay_safe_actor_bound_and_revoke_live; echo "HUB rc=$? $(date "+%F %T")"
  nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- a_history_transition_naming_another_actors_operation_is_refused_before_the_log; echo "DB rc=$? $(date "+%F %T")"
'
echo "EXIT rc=$? $(date '+%F %T')"
