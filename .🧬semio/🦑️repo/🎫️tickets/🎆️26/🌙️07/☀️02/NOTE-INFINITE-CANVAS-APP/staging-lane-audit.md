# Staging Lane Audit

Attempt 14 selected pooled Cargo build lane `.cache/b/r/m/u/l/pe037ed` and target `t-ce555540`; attempts 11–13 used `p986733`. The current run `winOvPi` Cargo JSONL contains 700 compiler artifacts marked `fresh: false` and none marked `fresh: true` at the time of inspection. The driver counts only Cargo fresh artifacts as reused; wrapper unit-cache restores are a separate metric. Zero therefore indicates no Cargo fingerprint reuse in this lane, not a failure.

The lane pool prefers an idle lane with the same crate-set key, then may choose an idle or new lane, and may trim idle lanes. A future authorized retry of the same normal selection should prefer the current lane if it remains idle and retained, but this is not guaranteed. No supported pin option was found in the inspected driver. No compiler, graph operation or cache mutation was run for this audit.
