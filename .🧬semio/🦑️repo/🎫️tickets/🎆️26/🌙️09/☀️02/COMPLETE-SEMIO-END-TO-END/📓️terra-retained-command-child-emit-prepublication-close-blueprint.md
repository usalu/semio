# Retained Command Pre-Publication `ChildEmit` Close Blueprint

## Verdict

**RED — two generic retained-command exits destroy a pre-publication `Emit` that can contain a `ChildEmit`.**  No native gate was run for this audit.  This is independent of the later `PendingChildGroupPublication` ownership, which is already child-first once a completion value has been mounted.

## Current ownership map

| Boundary | Current behaviour | Result |
| --- | --- | --- |
| Work -> generic job | `ArtifactCommandWorkStep::Complete*` stores the output in `ArtifactRetainedCommandJob.emit` | Correct transfer, at [retained-command:490](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:490). |
| Generic job cancellation/fault close | `retire_one!(emit)` directly drops `self.emit.take()` | **Leak of bounded retirement protocol** at [retained-command:586](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:586) and [retained-command:597](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:597). |
| Generic job -> completion cell | It removes `emit` and `ephemeral` before calling a fallible, consuming `completion.complete` | On `Busy` or duplicate-cell rejection, both local values are directly dropped at [retained-command:511](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:511)-[514](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs:514). |
| Completion cell -> mounted operation | `ArtifactToolCompletion::complete` stores the cell on success; the mounted operation transfers a child result to `PendingChildGroupPublication` | Correct child-owner handoff on success at [plugin:13312](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13312), [plugin:22925](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:22925). |
| Mounted cancellation/retirement | Retires last `ChildEmit` before all other `Emit` lanes, one bounded `close_one` step at a time | Existing correct downstream pattern at [plugin:16460](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16460)-[16468](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16468), and committed child publication at [plugin:16144](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16144)-[16150](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16150). |

`ChildEmit::close_one` already defines the required byte/item granularity (op bytes, empty op/vector entry, label UTF-8 scalar, schema, child id, slot) at [plugin:10415](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10415)-[10451](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:10451), but it is private to `app`; the sibling retained-command module cannot currently invoke it.

The generic bounded `TypedCommandFullOperationJob` has the same hostile-output defect: it rejects a nonempty child lane at [plugin:16785](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16785), then its close directly drops `self.emit` at [plugin:16884](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16884). It must use the same closer, but this must **not** grant the bounded factory the `Child` publication lane.

The framework’s keyed native fixture is the smallest real child producer: it constructs `ChildEmit::of` and calls the same completion API at [plugin:33953](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33953)-[33965](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:33965), with an explicit `Child` publication contract at [plugin:34012](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34012)-[34018](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34012). Existing retained-command coverage only cancels before the worker has produced output at [plugin:35016](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35016)-[35037](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:35037); it cannot prove either RED.

## Smallest sound P0

1. Make `ChildEmit::close_one` **`pub(crate)` only**, or expose an equally crate-private `close_child_emits_one(&mut Vec<ChildEmit>, grant)` next to `ChildEmit`. It must retain the current last-child ordering, only pop after `Complete`, and return `PluginCloseStep`; no public SDK API or erased disposer is warranted.
2. In `ArtifactRetainedCommandJob::close_step`, before `retire_one!(emit)`, drive the last child in `self.emit.as_mut().child_emits`. Map the `PluginCloseStep` exactly into `InteractiveJobCloseStep`; on a zero/insufficient grant leave the entire `Emit` unchanged. Only after child list terminal may the existing outer `emit` retirement occur.
3. Make completion rejection ownership-preserving. The present `ArtifactToolCompletion::complete` takes both owners and returns only `Fault` ([plugin:13312](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13312)-[13318](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13312)). Replace it source-first with a rejected-handoff result that returns the original `Result<Emit, Fault>`, `EphemeralEmit`, and bounded rejection fault. The generic job restores both fields before entering its fault/close route. A preflight `is_empty` check is unsound: the `Mutex` state can change between it and `complete`.
4. Use the same child-first close helper in `TypedCommandFullOperationJob`, before description/coalesce and `drop(self.emit.take())`. This is a hostile-retirement repair only; keep its validation denial and its lack of `ArtifactToolPublicationLane::Child` unchanged.
5. Migrate every direct `ArtifactToolCompletion::complete` caller to consume the owner-returning rejection. Current production callers include writer, draw, puzzle, Flow, the generic retained command, and framework tests; merely fixing the generic caller leaves an API that can still destroy a future child-bearing output. The immediate proven child producer is the keyed framework fixture, but the API must be closed globally rather than relying on current caller shape.

The P0 does not claim panic-safe abandonment. A scheduler that violates the `begin_close`/`close_step` protocol can still directly drop the job. It also does not redefine `ChildEmit` allocation-capacity accounting: the existing closer is reused as the established logical byte/item authority; do not claim allocator-capacity bytes were incrementally retired unless that separate `ChildEmit` contract is strengthened.

## Non-vacuous law packet

Add a language-neutral `retained-command-child-prepublication-close-v1` fixture beside the existing framework plugin neutral fixtures, then bind it in the existing framework plugin native gate rather than a generated launch file.

Required rows:

1. A UTF-8 slot/id/schema/label plus multi-byte op payload; cancellation after `Work::Complete` and before `Publish`; grants `0`, insufficient byte, `1`, and exact multi-byte scalar. The owner identity and child bytes remain unchanged on each rejection; terminal is reached only through child-first steps.
2. Two children prove LIFO ownership, each child’s op bytes drain before its label/schema/id/slot, and no parent mutation/ephemeral lane is released first.
3. A completion cell prefilled by another output, then a generic-job publish attempt. Rejection returns the original child emit to the job; the prefilled cell is untouched; the returned owner closes to terminal and is never published.
4. A deliberately busy completion lock, if exposed through an existing test-only cell contention seam; it has the same owner-return behaviour as duplicate. Do not simulate busy with a non-atomic prior observation.
5. The bounded generic factory receives a malicious nonempty child lane: validation faults, it never creates a pending child publication, and close drains the child before the outer emit.
6. Existing success handoff remains: completion -> mounted operation -> `PendingChildGroupPublication`, cancellation and retirement are bounded, and no duplicate group is dispatched.

Native entry points: extend the retained-command fixture at [plugin:34921](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:34921) with an output-producing work case, and add dedicated framework tests for generic bounded hostile child output and completion duplicate/busy handback. Retain the keyed `CompositeEdit` factory as the real `ChildEmit::of` source; avoid a hand-built child struct.

## Acceptance and nonclaims

Green means every pre-publication `ChildEmit` owned by these two generic jobs survives zero/failed grants, is retired in bounded child-first order, and a failed completion handoff returns rather than destroys the owner. It does **not** make Flow `addWidget` available, grant `Child` to the bounded factory, prove atomic composition publication, or make any direct external `Drop` safe.
