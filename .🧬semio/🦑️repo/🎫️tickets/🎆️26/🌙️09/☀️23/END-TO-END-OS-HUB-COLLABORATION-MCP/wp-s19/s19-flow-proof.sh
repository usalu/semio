#!/bin/zsh
# 🧪️ S19 set `flow-extensions` re-proof in ONE `overlay`-lane hold: the flow artifact lib suite (baseline 258/258 on the live
# tree) plus the contributions law in its own process. Capped at 28 min. usage: zsh s19-flow-proof.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target"
echo "START $(date '+%T')" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s19 -- zsh -c "
cd '$O' || exit 2
perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo test -p semio-s-artifact-flow-flow --lib --test set-contributions-registry --no-fail-fast; echo \"STEP flow rc=\$? \$(date +%T)\"
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
