#!/bin/zsh
# ⚖️ LW1 → T6 round 3a (+ round 1 kernel re-check) on the live tree: row 17 (H14 Rust link live socket) + rows 1/11 (kernel) = the
# os-kernel lib with the `sync` lane per process (nextest); row 6 (U6 row-target) + row 16 A (C13 puzzle 3d document schema) = U6's
# 38-crate lib build (`lw1-suites.sh`), ui-contract serial, the SDK row laws, and the row-named plugin suites (Home rows, set-verbs of
# puzzle 2d/3d/5d, cad, process3d; puzzle 3d retained-command catalog). Remaining suites + React + contract TS: block h2.
R=/Users/ueli/Documents/semio; C="$R/.🧬semio/🌐hub/s14-lw1-logs"; S="$R/.tmp-ticket/wp-lw1/lw1-suites.sh"; L="$C/t6-h-bins.txt"
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export CARGO_NET_OFFLINE=true NX_DAEMON=false
cd "$R" || exit 2
step kernel-lib-nextest 900 env RUST_MIN_STACK=134217728 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-framework-os-kernel --features sync,ureq --lib
export RUST_MIN_STACK=67108864
step u6-38-build 1800 zsh "$S" build "$L"
step contract-serial 420 zsh "$S" serial "$L" semio-framework-ui-contract
step sdk-row-laws 300 zsh "$S" serial "$L" semio-framework-plugin row_ table_kit home_shaped mounted_document_tree ui_history_panel
for c in semio-s-artifact-space-home semio-s-artifact-puzzle-3d semio-s-artifact-puzzle-2d semio-s-artifact-puzzle-5d semio-s-artifact-cad-cad semio-s-artifact-process-process3d semio-framework-ui-runtime; do
  step "suite-$c" 420 zsh "$S" suite "$L" "$c"
done
finish
