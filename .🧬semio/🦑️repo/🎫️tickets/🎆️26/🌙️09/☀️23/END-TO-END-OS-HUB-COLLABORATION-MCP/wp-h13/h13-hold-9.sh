#!/bin/zsh
# 🎚️ H13 hold 9 (session 14c): the close-ring cleanup-fault routing (kernel-db) — compile-atomic check, the routing law, the
# full kernel-db lib suite with sqlite, the semio-hub all-feature check; queued after hold 8 released its lane slot.
# usage: h13-hold-9.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-check-db" check -p semio-framework-os-kernel-db --features sqlite --lib --tests || exit 1
zsh $W/h13-cargo.sh "$1-routing-law" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast -- a_backend_cleanup_fault_reaches_only_its_own_waiters_and_close
zsh $W/h13-cargo.sh "$1-db-lib" test -p semio-framework-os-kernel-db --features sqlite --lib --no-fail-fast
zsh $W/h13-cargo.sh "$1-check-hub" check -p semio-hub --all-features --lib --bins --tests
