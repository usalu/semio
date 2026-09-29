#!/bin/zsh
# 🎚️ H13 hold 17 (session 14c): the transport-refill kernel law with the native directory transport features (`sync`, `ureq`)
# (hold 16 died queued in the 00:4x panics; its engine declared-batch law passed in the land2 full kernel-db lib run).
# usage: h13-hold-17.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
zsh $W/h13-cargo.sh "$1-refill-law" test -p semio-framework-os-kernel --locked --lib --features sync,ureq --no-fail-fast -- an_exhausted_directory_byte_budget_names_itself_and_refills_on_the_pools_own_turn
