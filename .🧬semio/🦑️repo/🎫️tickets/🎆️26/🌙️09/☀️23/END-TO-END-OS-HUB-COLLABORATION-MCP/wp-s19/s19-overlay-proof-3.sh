#!/bin/zsh
# 🧪️ S19 overlay proof 3 in ONE `overlay`-lane hold: norm plugin + en1998 artifact `--lib` together (regenerated en1998 assets,
# roster route over 13 families, en1998 asset law; one invocation so features unify as in the census build), then the flow
# artifact `--lib` owed by set `flow-extensions`. Each cargo capped at 28 min. usage: zsh s19-overlay-proof-3.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target"
echo "START $(date '+%T')" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s19 -- zsh -c "
cd '$O' || exit 2
run() { perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo test \$@ --lib --no-fail-fast; echo \"STEP \$* rc=\$? \$(date +%T)\"; }
run -p semio-s-plugin-norm -p semio-s-artifact-norm-en1998
run -p semio-s-artifact-flow-flow
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
