# Lock Ownership Frontier Performance

The selected lock operation intentionally retains all current authored workspace ownership. Its preservation envelope and cross-owner generation relationships require that global Source custody. Narrowing it to the requested owner alone would remove observations needed to reject changes elsewhere, so this repair preserves the global selection and its complete authenticated topology.

Published ownership rechecks previously replayed the complete retained frontier twice: once in validate, then again in currentFence after the membership and manifest continuations. The first replay supplied no effect authority: every path to publication or successful return immediately awaited currentFence again. Removing the earlier replay preserves the complete final fence after every caller continuation while eliminating duplicated filesystem work.

The physical ownership interface now accepts an explicitly bounded frontier continuation batch. Its default remains one path, preserving the existing detailed observer contract. Lock production opts into 64 paths. Every path still checks cancellation and its original state; the second complete synchronous state/listing verification still runs after the final awaited continuation. The last retained path always emits progress, and the cumulative work counter still counts every actual observed path. Batch values outside 1 through 64 are refused before ownership computation.

Lock production now publishes caller progress after each 4096 completed ownership operations rather than after each 64. This reduces output volume and the associated retained output without changing the global Source fence, preservation records, staged data verification or conditional lock publication.

The change was test driven: the new actual-source regression first failed with 63 callbacks instead of one bounded batch and with two duplicated frontier authentication events. It then passed with schema-authored vectors and an independent SQLite batch-count oracle. Early and final continuation mutations remain refused. The existing complete refresh suite runs its unchanged assertions twice, at batch one and batch 64, including topology changes, manifest changes, cancellation, undefined primary failure, and semantic refresh adoption.

Final validation: the registered physical ownership test entry passed 96 tests and 1707 assertions in 8.05 seconds. No prior assertions were weakened. The parent task started the end-to-end `repo dependencies lock --all` regeneration after this result; this audit does not claim its integration outcome.

Files changed by this performance repair are the Bun physical implementation and public control type, its neutral schema, registered test entry, existing refresh test parameterization, new frontier test and JSON corpus, and dependency storage implementation/install schema progress policy. The repair introduces no runtime dependency.
