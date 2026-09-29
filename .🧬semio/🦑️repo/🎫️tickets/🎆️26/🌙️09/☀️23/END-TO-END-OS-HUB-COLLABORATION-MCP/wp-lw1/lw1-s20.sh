#!/bin/zsh
# ⚖️ LW1 → S20 laws (T3 cad-solids, process-formats, silent-exports) under the canonical runner (nextest, RUST_MIN_STACK 128 MiB).
export RUST_MIN_STACK=134217728 CARGO_NET_OFFLINE=true
cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-cad-cad --lib -E 'test(/current_pane_exports_its_real_solids|export/)'; echo "LW1-STEP cad-export rc=$?"
cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-process-process3d --lib -E 'test(/export/)'; echo "LW1-STEP process3d-export rc=$?"
cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-remodel-remodeling --lib -E 'test(/qc|export/)'; echo "LW1-STEP remodeling-export rc=$?"
cargo nextest run --profile long --no-fail-fast -p semio-s-artifact-shooting-shooting --lib -E 'test(/export/)'; echo "LW1-STEP shooting-export rc=$?"
