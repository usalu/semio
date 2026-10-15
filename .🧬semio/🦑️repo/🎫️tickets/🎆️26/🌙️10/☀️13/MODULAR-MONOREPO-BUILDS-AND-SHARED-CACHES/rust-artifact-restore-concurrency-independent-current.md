# Rust Artifact Restore Concurrency Independent Audit

Source-only review, 2026-10-15. No tests, runtime, image reads, or Source edits.

## Actionable Cache Promotion Race

The authored Nx patch checks Source identity before acquiring output custody, then caches the current stable output slot without validating that its receipt belongs to the task being cached. `library/⚡️caching/🩹️patches/nx@23.2.0.patch:1266-1268` checks selected-graph/Source receipts. Lines 1271-1272 subsequently await fixed-cache/output custody and invoke `this.cache.put(task.hash, ...)` without checking the unit receipt's semantic digest under that custody.

Two Source revisions of the same crate share a slot: `units/🟦️.ts:31` omits semantic digest from the slot identity, and underlying Cargo unit identity (`framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🕸️units/🟦️.ts:22`) omits Source content. The compiler's output EX ends when its command callback ends (`units/📜️script.ts:31-42`), before Nx's later cache.put callback.

A possible supported interleaving is: A publishes revision A and exits its unit command; A's promotion Source precheck passes; before A acquires the output reader, B edits and compiles revision B into the same slot under its output EX; A eventually acquires its output reader and caches B's receipt/artifact under A's task hash. The mutex prevents simultaneous byte mutation but does not bind the version observed after waiting to A's cache key. The precheck-to-custody awaits provide the interleaving window. This is a Source-established missing under-custody identity check; no runtime interleaving was executed.

Validate the actual output receipt semantic digest against task Rust metadata after acquiring output custody and before cache.put, or retain an immutable digest-specific output version. A Source recheck performed only before waiting is insufficient. Rechecking Source inside custody also narrows the demonstrated interleaving but does not substitute for authenticating output-to-task identity.

## Existing Restore and Consumer Custody

`storage/🔒️enforcement/🔌️nx/📦️outputs/🟦️.ts:8` maps Rust output paths to the exact `rust-unit:<absolute-output>` resource used by the compiler. `storage/🔒️enforcement/🔌️nx/🟦️.ts:36-46` retains output SH while cache.put reads and EX while local cache restore replaces outputs. The local restore patch at lines 1278-1283 invokes that scope for the complete restore callback. Different revisions cannot overwrite an output while another holder retains its reader.

`units/📜️script.ts:33` defines predecessor readers on that same resource. `units/🏗️compiler/🦀️rustc/🟦️.ts:27` acquires all predecessor SH scopes recursively, revalidates actual dependency receipts/digests under those readers, and retains them through the awaited rustc invocation and consumed-input evidence read. Thus the inspected compiler consumer's linking interval is protected; initial pre-reader receipt races can refuse rather than silently consume a changed artifact. There is no demonstrated overwrite-during-link defect in this path.

The actionable gap is the producer-command-to-Nx-promotion handoff, not absence of resource locks. Other Nx cache backend paths, remote application details, and arbitrary downstream consumers are not certified by this review.

## Smallest Runtime Regression Design

Extend existing `storage/🔒️enforcement/🔌️nx/📦️outputs/🧪️tests/🟦️.ts`, using its actual-hook test beginning at line 29 and existing Node/TypeScript-AST patch-function execution pattern later in that file. The registered commands already are `bun repo run repo-nx-output-custody:test --configuration=prototype --skip-cache --exclude-dependencies` and its release counterpart (`.vscode/launch.json:4421,4432`). Do not add a new executable script.

1. Create a small owned synthetic Rust output slot in the already-authenticated test storage, with an actual rlib-sized byte fixture and valid revision-A receipt. Model both revisions with the same unit/mode/slot and distinct Source semantic digests. Have an independent SQLite comparison classify A/A as accepted and A/B as refused; do not use the production helper to compute that expectation.
2. Execute the actual patched cache.put method, extracted with TypeScript AST using the existing test infrastructure. Its task metadata must select revision A. Return true from the external Source precheck solely to isolate the postcheck window; record that A's precheck occurred. Route the immutable-authorizer port to the real new production helper and actual `withOriginalNxFixedCacheV1`, rather than reproducing its condition in the test.
3. Before the output reader can enter, hold the genuine `rust-unit:<absolute-slot>` EX from a peer operation. The existing actual-hook test already uses an actual blocker lease plus an observed wait-publication callback. Use that callback/gate rather than wall-clock sleep to establish A is waiting after its precheck. While holding EX, replace the synthetic slot with valid revision-B artifact and receipt, then release EX.
4. Await A and assert the under-custody production helper refuses the receipt/task mismatch before the intercepted native cache.put counter increments. Assert actual on-disk receipt remains B and no cache-key-A publication side effect occurred. Also run the matching A/A control through the same hook and real helper and assert exactly one cache.put. A second refused case with mismatched artifact digest ensures the test does not accept merely relabeled receipt content if the fix includes actual artifact validation.
5. Prove output custody covers the production helper and native put callback by attempting an actual peer EX during their accepted callback; it must refuse/wait until callback settlement. Clean only synthetic owned fixture files and release gates/leases in finally.

This exercises the changed production hook/helper and actual cross-process-compatible SQLite lease protocol without recompiling a crate or spawning a new compiler Original. A port-based native cache.put counter is appropriate because the assertion is refusal before that effect; string-presence or mirrored predicate tests alone would not prove the handoff. If extracting a class method, use the installed cache class parsed with TypeScript and apply the exact authored patch method body as the existing constructor tests do, then invoke it on a narrow receiver; do not invent a parallel cache.put implementation.

No regression test was authored or run in this independent audit. Await Root's implementation for Source re-review.
