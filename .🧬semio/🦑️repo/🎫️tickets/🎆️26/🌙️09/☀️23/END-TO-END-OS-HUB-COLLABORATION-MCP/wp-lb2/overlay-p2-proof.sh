#!/bin/zsh
# 🔑️ LB2 overlay proof of p2 + p2b in ONE overlay-lane hold: apply the declared-arguments law (SDK) + the per-plugin calls to the overlay, then run the plugin-level law for note (reference: its own verb-argument law is green) and the heuristic's heaviest candidates, plus the typed bridge laws of block 2d / puzzle 3d.
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-overlay"
cd "$O" || exit 2
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-lb2-target"
echo "APPLY $(date '+%H:%M:%S')"; python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/lb2-p2-declared-arguments.py --write --root "$O"; echo "APPLY p2 rc=$?"
python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-lb2/lb2-p2-plugin-calls.py --write --root "$O" | tail -2; echo "APPLY p2b rc=$?"
for crate in semio-s-plugin-note semio-s-plugin-block semio-s-plugin-puzzle semio-s-plugin-space semio-s-plugin-wfc; do
  echo "LAW $crate $(date '+%H:%M:%S')"; nice -n 15 cargo test -p "$crate" --lib --no-fail-fast -- every_registered_app_reads_only_its_declared_arguments; echo "LAW $crate rc=$? $(date '+%H:%M:%S')"
done
echo "BLOCK2D $(date '+%H:%M:%S')"; nice -n 15 cargo test -p semio-s-artifact-block-2d --features component-app-assembly --lib --no-fail-fast -- command_from_action_covers_every_declared_action; echo "BLOCK2D rc=$? $(date '+%H:%M:%S')"
