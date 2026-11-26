# EN 1990 Warm Cargo Lane Audit

Both completed retries and the receiving-quote retry used the same repository-managed shared-store lane, `pebbc24`, and Cargo target `t-ce555540`. The launcher holds exclusive use of a pooled lane for the compilation and prefers the same crate set on subsequent calls. The CLI's isolated store instead allocates a fresh nonce and retires its target and build directories when the subprocess settles. For warm reruns with existing admission and exclusive lane ownership, use the existing `repo cargo --store=shared --configuration=prototype --parallelism=1 -- test --offline --locked -p semio-s-artifact-norm-en1990-mutations --test replay_preparation_physical` invocation.

| Retry | Cargo `Compiling` lines | Wrapper compile misses | Wrapper local hits |
| --- | ---: | ---: | ---: |
| Geometry | 556 | 694 | 0 |
| Native warm | 531 | 664 | 17 |
| Receiving quote, in progress at 00:45 | 78 | 8 | 87 |

Cargo reports the units it invokes. The wrapper can restore those units without compiling, and a Cargo package line can cover multiple Rust units. The first two runs primarily involved real compilation. The native-warm run finished compilation in 25m 34s, with 1,278.7 seconds summed across wrapper compile misses. Its final physical test ran in 0.97 seconds and failed at the receiving-scope assertion.

The lookup ledger in `.cache/b/r/m/u/y/` documents the reason for the earlier misses. Between geometry and native-warm, 640 crate-and-slot pairs could be matched. Of those, 576 missed with only extern identity changes, 44 changed extern and source keys, 2 changed only source keys, 17 hit identical keys, and 1 missed with an identical key. The `semio_framework_value` extern changed for 432 matched units. The source digest changed for 44 distinct crate names, including framework value factory and retirement contract, multiple plugin crates and Norm units. A source key includes the reached and recorded source inputs, so the ledger alone does not identify the files or authors responsible. Cargo code-producing units key against dependency bytes or a dynamic extern identity. The source interface cutoff is confined to metadata-only invocations, so dependency changes can invalidate unchanged linking units across this graph.

The receiving-quote retry started with substantial reuse. A later overlapping ledger sample matched 140 crate-and-slot pairs against native-warm, with 103 exact-key local hits, 32 extern-only misses, 4 source-only misses, and 1 source-plus-extern miss. The four names with source changes were framework deflate, DSL record model, replication history and value factory. The cache collector freed 9.63 GiB during geometry and 4.71 GiB during native-warm, but changing key components accounted for all except one of the matched misses. The two earlier 25-minute-scale runs do not establish that every retry rebuilds all dependencies.
