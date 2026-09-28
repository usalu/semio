#!/bin/zsh
# 🧮️ S19 norm census + surface proof + emitter build in ONE `overlay`-lane hold (shares the overlay build-dir): the overlay-only
# `[[test]] s19_example_census` (roster documents vs production decode, code-built constructors vs assets; mismatches written
# to $S19_CENSUS_OUT), the norm plugin `--lib` (roster law over all 15 families) and the overlay-only `harness = false`
# `[[test]] s19_emitter` (N1's production fixture emitter, same units). Each cargo capped at 28 min.
# usage: zsh s19-norm-census.sh <capture>
capture="$1"
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-overlay"; H=/Users/ueli/Documents/semio/.tmp-ticket/wp-s19
export CARGO_INCREMENTAL=0 RUST_BACKTRACE=1 RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$O/.s19-build" CARGO_TARGET_DIR="$O/.s19-target"
export S19_CENSUS_OUT="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s19-census"
M="$O/✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml"
/usr/bin/grep -q 's19_example_census' "$M" || printf '\n[[test]]\nname = "s19_example_census"\npath = "%s/s19-example-census.rs"\n' "$H" >> "$M"
/usr/bin/grep -q 's19_emitter' "$M" || printf '\n[[test]]\nname = "s19_emitter"\npath = "%s/s19-emitter.rs"\nharness = false\n' "$H" >> "$M"
rm -rf "$S19_CENSUS_OUT"; mkdir -p "$S19_CENSUS_OUT"
echo "START $(date '+%T')" > "$capture"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay s19 -- zsh -c "
cd '$O' || exit 2
run() { perl -e 'alarm shift; exec @ARGV' 1680 nice -n 15 cargo test -p semio-s-plugin-norm \$@; echo \"STEP \$* rc=\$? \$(date +%T)\"; }
run --test s19_example_census --no-fail-fast -- --nocapture
run --lib --no-fail-fast -- --nocapture
run --test s19_emitter --no-run
" >> "$capture" 2>&1
echo "END rc=$? $(date '+%T')" >> "$capture"
