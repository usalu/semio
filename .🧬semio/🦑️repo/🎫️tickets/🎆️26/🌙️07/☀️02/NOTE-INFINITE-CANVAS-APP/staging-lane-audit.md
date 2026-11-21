# Staging Lane Audit

Attempt 14 selected pooled Cargo build lane `.cache/b/r/m/u/l/pe037ed` and target `t-ce555540`; attempts 11–13 used `p986733`. The current run `winOvPi` Cargo JSONL contains 700 compiler artifacts marked `fresh: false` and none marked `fresh: true` at the time of inspection. The driver counts only Cargo fresh artifacts as reused; wrapper unit-cache restores are a separate metric. Zero therefore indicates no Cargo fingerprint reuse in this lane, not a failure.

The lane pool prefers an idle lane with the same crate-set key, then may choose an idle or new lane, and may trim idle lanes. A future authorized retry of the same normal selection should prefer the current lane if it remains idle and retained, but this is not guaranteed. No supported pin option was found in the inspected driver. No compiler, graph operation or cache mutation was run for this audit.

## Note prototype closure

The graph for warm run w3tNHX4 contains 893 wasm32-wasip2 units. Hub Note directly lists five format crates without using their names in its Rust modules. Removing the five graph edges in memory leaves 873 reachable units. PNG and its ZIP-related workspace units account for 18 and DWG for two. DXF, PDF and SVG stay reachable through Note's typed IO routes. All third-party and build-script units stay reachable. The smaller graph has no wall-clock measurement. The launch work keeps the current manifest and lockfile to preserve reuse of the admitted guest build.
