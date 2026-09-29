#!/bin/zsh
# ⚖️ LW1 → T6 rows 7 (WG11 renderer sets), 8 (Marketplace) and 6 (U6 row-target, renderer half): the renderer-wgpu lib, one process
# per test (nextest `long`, RUST_MIN_STACK 128 MiB) — the one-process `cargo test` of this crate hangs under parallel neighbours
# (U6 r6-base, `shell::panel_anchor_model_tests::host_panel_action_is_claimed_before_guest…`). ui/contract/value/TS halves: block c2.
R=/Users/ueli/Documents/semio
source "$R/.tmp-ticket/wp-lw1/lw1-budget.sh"
export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true NX_DAEMON=false
cd "$R" || exit 2
step renderer-wgpu-lib-nextest 2520 cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-framework-os-renderer-wgpu --lib
finish
