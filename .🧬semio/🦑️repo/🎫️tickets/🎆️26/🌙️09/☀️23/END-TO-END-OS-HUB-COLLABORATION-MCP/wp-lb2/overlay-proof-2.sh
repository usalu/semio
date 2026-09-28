#!/bin/zsh
# 🎟️🔑️ LB2 session 14b overlay proofs after `overlay-sync-2.sh` (p1 + p2 + p2b applied), in TWO overlay-lane holds (the lane is
# 5 deep; one hold per cargo command would re-queue ten times). Hold A: p1's law + the details unit tests (the item-3 WAL reader
# is superseded: the conflict probe counts committed operations on a census socket). Hold B: p1's recalibrated law, p3's SDK table-kit laws, p2's plugin-level law on note (reference), stdio, block, puzzle, space, wfc, and block 2d's typed
# bridge law (which now also runs the declared-arguments check). Private build/target dirs, nice 15, incremental off.
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-lb2
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-overlay"
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-target"
step() { local capture="$W/generated/$1.txt"; shift; { echo "START $(date '+%H:%M:%S') cargo $*"; nice -n 15 cargo "$@"; echo "EXIT $? $(date '+%H:%M:%S')"; } > "$capture" 2>&1; }
hold_a() {
  step o2-p1-law test -p semio-s-artifact-stdio-contract --test details_arena_headroom --no-fail-fast -- --nocapture
  step o2-p1-unit test -p semio-s-artifact-stdio-contract --lib --no-fail-fast -- details
}
hold_b() {
  step o2-p4-shipped test -p semio-s-plugin-stdio --test shipped_fleet --no-fail-fast
  step o2-p1-law-2 test -p semio-s-artifact-stdio-contract --test details_arena_headroom --no-fail-fast -- --nocapture
  step o2-p3-sdk test -p semio-framework-plugin --lib --no-fail-fast -- window_kits_tests::
  step o2-p2-note test -p semio-s-plugin-note --lib --no-fail-fast -- every_registered_app_reads_only_its_declared_arguments
  step o2-p2-stdio test -p semio-s-plugin-stdio --test declared_arguments --no-fail-fast
  for crate in block puzzle space wfc; do
    step "o2-p2-$crate" test -p "semio-s-plugin-$crate" --lib --no-fail-fast -- every_registered_app_reads_only_its_declared_arguments
  done
  step o2-p2-block2d test -p semio-s-artifact-block-2d --features component-app-assembly --lib --no-fail-fast -- command_from_action_covers_every_declared_action
}
cd "$O" || exit 2
case "$1" in
  a) hold_a ;;
  b) hold_b ;;
  *)
    echo "QUEUED A $(date '+%H:%M:%S')"; FLEET_TICKET_STAMP="${FLEET_TICKET_STAMP:-}" zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay lb2 -- zsh "$W/overlay-proof-2.sh" a; echo "HOLD A rc=$? $(date '+%H:%M:%S')"
    unset FLEET_TICKET_STAMP
    echo "QUEUED B $(date '+%H:%M:%S')"; zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay lb2 -- zsh "$W/overlay-proof-2.sh" b; echo "HOLD B rc=$? $(date '+%H:%M:%S')"
    echo "ALL DONE $(date '+%H:%M:%S')" > "$W/generated/o2-done.txt"
    ;;
esac
