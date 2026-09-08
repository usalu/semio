# Graph Discovery Memory and CPU

The native input discovery CPU profile identified repeated Rust tokenization, source reads, and retained token streams as the main graph-refresh costs. The Nx plugin now parses one file at a time into compact include expressions and module mounts. A bounded 16 MiB content-addressed cache retains those facts, including conservative fallback results. Every graph pass still reads and hashes current source bytes; modification timestamps never stand in for content identity. Repeated Cargo owners share one read snapshot within that graph pass.

The language-neutral fixture changes same-length source bytes while preserving timestamps. The repository contract test compares each revision against rustc dep-info, checks a repeated-source cache hit, and exercises eviction under a 4 KiB budget. The red run failed on the absent cache interface; the green public `bun nx run repo:test` completed successfully.

The same 304-project profiling input measured 42.41 / 51.06 seconds before and 17.38 / 23.50 seconds after. Retained heap after two passes decreased from 968.4 MiB to 220.1 MiB. These are observations under concurrent native compilation and source changes, not an unloaded performance guarantee. The second pass is still dominated by current-byte reads and filesystem traversal.

The authored reproducible probe is `🔬️graph-profile/📜️script.ts`; raw CPU profiles and timing JSON remain temporary ticket output.

The current 449-project helper probe exited 0 on 2026-09-08: 44.33s first graph pass and 6.19s second pass. Nested aggregate timings across both passes were cargoSourceInputs 38.39s (1,140 calls), projectInputs 39.62s (898), relativeScriptInputs 2.83s (164), and nativeDependencyRoots 1.94s (4,388). Nested totals overlap and must not be added. This confirms cold source discovery remains material while the in-process warm cache helps. Measurements occurred during concurrent Rust compilation and changed sources, so they do not isolate unloaded host performance.
