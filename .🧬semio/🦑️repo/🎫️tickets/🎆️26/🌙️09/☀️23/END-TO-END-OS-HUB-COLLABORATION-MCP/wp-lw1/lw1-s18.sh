#!/bin/zsh
# ⚖️ LW1 → S18 laws (T3 s18-twin): renderer-wgpu lib tests filtered to the preference / named-layout laws (S18's overlay filter),
# at the DEFAULT test-thread stack (WG11's shell-turn set is in T3). os-config lib 169/169 ran in the SH2 block (sh2-laws-1).
cargo test --offline --no-fail-fast -p semio-framework-os-renderer-wgpu --lib -- --test-threads 4 named_layout preference_log seeded_snapshot prefs; echo "LW1-STEP renderer-prefs rc=$?"
