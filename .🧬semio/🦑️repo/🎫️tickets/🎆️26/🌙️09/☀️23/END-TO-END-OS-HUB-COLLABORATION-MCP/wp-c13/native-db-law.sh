#!/bin/zsh
# 🧪️ C13 (14c): the hub event log refuses a foreign Revert/Reinstate — kernel-db lib laws (the new law + the concurrent-write
# fixture law that also submits transitions) and a semio-hub check, under ONE native-lane hold (rule 3/25).
export NX_DAEMON=false CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-target"
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native c13 -- zsh -c '
  cd /Users/ueli/Documents/semio
  nice -n 15 cargo test -p semio-framework-os-kernel-db --lib --no-fail-fast -- a_history_transition_naming_another_actors_operation_is_refused_before_the_log a_write_is_graded_only_against_unseen_foreign_writes; echo "DB rc=$? $(date "+%F %T")"
  nice -n 15 cargo check -p semio-hub --lib --bins; echo "HUB rc=$? $(date "+%F %T")"
'
echo "EXIT rc=$? $(date '+%F %T')"
