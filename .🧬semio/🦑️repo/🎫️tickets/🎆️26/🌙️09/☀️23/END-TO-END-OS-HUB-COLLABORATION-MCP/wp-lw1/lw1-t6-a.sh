#!/bin/zsh
# ⚖️ LW1 → T6 round 1, H14 row 1 (retire-pages) + H13 row 2 (taxonomy probe) + C12 row 11's kernel half: the whole os-kernel lib
# (sync lane included) under the canonical runner (nextest per process, RUST_MIN_STACK 128 MiB), then the taxonomy-load probe.
R=/Users/ueli/Documents/semio
export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true
cargo nextest run --profile long --no-fail-fast --test-threads 4 -p semio-framework-os-kernel --features sync,ureq --lib; echo "LW1-STEP kernel-lib-nextest rc=$?"
cd "$R" && bun "$R/.tmp-ticket/wp-coord/taxonomy-load-probe.ts"; echo "LW1-STEP taxonomy-load-probe rc=$?"
