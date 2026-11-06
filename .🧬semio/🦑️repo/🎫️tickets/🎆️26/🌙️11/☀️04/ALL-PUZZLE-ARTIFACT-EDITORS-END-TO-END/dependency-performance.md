# Dependency producer replay performance

## Findings

The publication loop's full fence is deliberately periodic, and removing it in favor of endpoint checks and a final fence changes observable failure detection. No safe equivalent reduction has been proven. Do not change the live producer based on this analysis.

In dependency storage `runJavascriptDependencyStoreProducerV1`, the fence at line 737 checks the lease, Store, published ownership custody, every plan's source inputs, the complete preservation snapshot, every captured input receipt, and retained preservation descriptors. Line 760 invokes it on each 64th published binding; other bindings receive a local endpoint/ancestry fence. After the loop, `verifyReady` includes another full fence and another full fence follows the final requested-scope reads. There are other full fences before and during build, outside this count.

The exclusive bindings lease coordinates cooperating repo operations. It does not prove arbitrary filesystem writers cannot change Original files, ancestors, or Store data during publication. A source changed across an intermediate full fence then restored before the final fence is detected by the present schedule and missed by endpoint checks plus final fences. The model test below demonstrates this schedule difference; it does not claim current periodic fences detect every shorter transient mutation.

## Injected-I/O count model

Run `repo tools exec --light node -- <ticket>/dependency-fence-count.mjs`. The input is beside this report. It injects counting source reads and local endpoint checks into a small model of the documented line 760 schedule, and asserts the mutation counterexample. It does not import the actual producer, measure wall-clock time, execute filesystem calls, or reproduce receipt and preservation traversal; its numbers are a lower-bound schedule model, not a live producer profile. In particular the 64 interval is taken from the source/test contract and must be checked against current schema if changed.

For N bindings and S plan source entries, the publication loop plus the two following fences reads S * (floor(N / 64) + 2) modeled source entries. Independent plans may contain overlapping source entries, each checked again. For the illustrative S=N case:

| N | Current source checks | Final-only source checks | Current local checks |
| --- | --- | --- | --- |
| 64 | 192 | 128 | 63 |
| 256 | 1536 | 512 | 252 |
| 1024 | 18432 | 2048 | 1008 |
| 4096 | 270336 | 8192 | 4032 |

The resulting modeled count is quadratic when S scales with N; source files require additional bounded descriptor reads and content comparisons. The actual fence also traverses preservation state and receipts, so exact total work depends on real inputs and their sizes. Separately, the producer-binding observation at line 125 rechecks accumulated parents and endpoint rows at each advance, plus checks around progress yields; its growth can also be superlinear. This model does not quantify that path.

## Tests, cancellation and retry

The existing storage fence tests include local versus full comparison, independently checked Python lstat and SQLite JSON oracles, changed nonlocal ancestry, and a source-level assertion requiring the interval fence and both later checks. The ancestry tests exercise hostile identity and cancellation; installed-path scanning checks cancellation. Incremental tests compare mutation classification with a Python oracle. The producer catch records unresolved Source transition state and retained preservation capsules; retry is gated by cancellation, unresolved transition and attempt policy (lines 763-764). An optimization must preserve these rejection, cancellation, retry and custody behaviors, not only successful final data.

The active progress label `dependencies:store-input-replay` comes from descriptor verification lines 468-470. That routine checks directory identity, named and opened file identity, bounded chunk bytes, hash, final identity and cooperative checkpoints. The same label occurs whenever the verifier is called, including calls outside the publication loop; it does not establish which fence or repeated pass the live producer is executing. No active input count, operation count, or timing was obtained, so the model does not establish the cause of the observed long run.

A safe next investigation is passive, per-phase instrumentation through the existing progress interface that counts full fences, source bytes/files, receipt files/chunks, preservation entries and elapsed time without logging paths or changing lease/cancellation semantics. Reproduce with owned injected-I/O and adversarial tests before proposing caching or batching. Any cache would need a trustworthy invalidation mechanism covering external writes and the same detection boundaries, plus tests for nonlocal ancestor changes, transient source edits, receipt/retained capsule changes, cancellation and interrupted Source transitions. No such mechanism has been demonstrated here.
