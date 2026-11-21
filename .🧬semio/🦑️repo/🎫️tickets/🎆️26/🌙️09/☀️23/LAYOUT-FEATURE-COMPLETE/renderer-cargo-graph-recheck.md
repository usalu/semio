# Renderer Cargo Planning Drift Before Source Recheck

The failed renderer invocation ran in lane `p5706b2` and recorded its full diagnostics in `semio-cargo-logs/cargo-artifacts-aViwuF.log`. Cargo began at 08:53:12.867 UTC. The timing receipt records `web-sys` starting 215.64 seconds later with 75 enabled features, without `CanvasWindingRule`. The live CanvasScene manifest changed at 08:56:54.221 UTC, about six seconds after that old feature selection began compiling. The current Canvas2D consumer file changed at 09:00:22.387 UTC. The same timing receipt puts CanvasScene at 926.2 seconds, and the raw log records E0432 for that exact import.

The renderer log shows a fresh Cargo metadata and unit graph capture after the failed compiler exited (lines 63375–63400). CanvasScene source admission fell back to physical reads and completed with 15 files, up from 14 (63680–63685). Source admission then stopped during `semio-plugin-instance-app` at unit 594 (64760–64764). The phase ledger records three graph and source rechecks and only two facts rechecks (64789–64793). The replacement graph therefore reached source admission before its planning identity was compared.

On the fresh invocation, unit 594 reached the same per-crate read and byte checkpoints. The surfaced refusal was `process.source.boundary-exceeded` at 17 files and 263,482 bytes against the 262,144-byte cap (66211–66228). This is the matching source failure that the failed-compiler path did not surface. The existing recovery records only typed concurrent edits from its recheck, so the first invocation kept its original Cargo failure and exited 1.

The artifact runner now compares the Cargo workspace, graph roots, and canonical unit identities and digests as soon as it recaptures the graph. A feature change enters the existing concurrent-edit retry before a later source refusal can mask it. Stable planning still proceeds through the original source admission and facts checks. The replacement plan must still satisfy the source cap.

## Verification

- The neutral fixture reproduced the masked Cargo feature change before the change. Both fixture outcomes now pass against an independent SQLite oracle. A changed dependency emits one typed graph-drift diagnostic before reading source, and the stable graph reaches the existing source refusal.
- All 52 scoped tests passed across artifact planning, issued control, source memo, failure churn, and the owned artifact runner. The two owned fake-compiler tests confirmed that an unchanged compiler error stays terminal and a compiler failure after a source edit still retries and publishes the corrected artifact. Scoped whitespace checking also passed.
