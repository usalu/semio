# Architecture Operation Audit 35

Read-only source audit; no runtime tests, runners, production edits, Git mutations, or ticket lifecycle calls. Runtime acceptance remains unproven by this auditor.

## Actionable findings

1. **P1: Refused close changes parse phase.** `Pack JSON/📥️decode/🧵️operation/🦀️.rs:44` sets `close_started=true` before calculating admission or successfully transferring the cursor. An `into_retirement` admission error restores the cursor but leaves the flag set. `step` then returns `InvariantViolated` at line 36. This contradicts the native operation test that calls `operation.step(incoming).unwrap()` after denied capacity closure. Set the flag only after successful ownership handoff. Confirm refusal preserves position, source/control identity, candidate backing pointer, grammar phase, and receipts using an allocated partial candidate, in addition to the present allocation-free case.

2. **P2: Operation native coverage does not exercise allocated semantic completion or expensive boundaries.** Its two tests exhaust the normal item wallet after one UTF-8 validation step or cancel that first step; neither reaches a semantic result or produces parse-owned allocation. Add schema/fixture-backed allocated string/object success against serde_json, separate exhaustion of total copy/capacity/release wallets across turns, source validation work exhaustion with retained paged source identity, physical candidate depth denial, allocated post-effect cancellation, and close-total exhaustion/retry. Observe actual birth/release bytes and stable backing identity on admission refusal. Existing receipt tests cover cursor-level allocation boundaries, but do not prove conservation through the operation wrapper.

## Source-supported observations

- The operation retains its original `S: Copy` source and exclusive borrowed `NativeDecodeControl`; Rust lifetime bounds keep the control attached until the operation is dropped. Borrowed sources remain bound by `S` itself. The immutable-source contract is documented, though public custom `JsonReadSource` implementations can use interior mutation; this is a trait contract rather than an enforced source freeze.
- Per-turn parse and retirement grants are intersected with their independent original remaining totals. `normal_remaining` subtracts cumulative items/copy/capacity/release and retains depth as a bound; depth is intentionally not an additive quantity.
- Parse receipts are read and debited even on returned errors, and cleanup error receipts are likewise debited after restoring the original cursor. Cursor post-effect cancellation attaches cumulative turn progress before wrapper debit. Overflow or over-authority is reported as invariant failure, not a silent refill.
- Read limits constrain complete source bytes at construction, allocation through scoped control, per-collection declared extent, and depth. Total parse work is separately constrained by the operation normal item wallet, including UTF-8 validation. This audit found no source evidence of total wallet refill.
- Drop requires explicit terminal cleanup and uses ManuallyDrop to avoid unpaid destruction; during unwinding it intentionally retains owned backing. No claim that this behavior has passed runtime acceptance is made.

## Camera owning package boundary

Actual owner: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/⏱️camera/📦️packages/🦀️rust`.

Production dependencies are only first-party Job and Value; implementation uses standard allocation and generic `M: Copy` metadata. The public package loads and reexports the original storage module in place, preserving a single implementation. The WGPU renderer imports that package; the owner does not import the renderer. UI/WGPU is a dev-dependency for tests only. Public types use first-party grant/progress/error types and generic metadata, with no external renderer type in the public API. No actionable architecture-boundary violation found in the inspected owner/import direction. Camera physical native acceptance belongs to the separate active native interval, and is not asserted here.

## Expected open frontier

Legacy parse/raw grammar surfaces remain present and are not treated as completion of the new retained operation contract. The current operation tests alone cannot establish all expensive-source, allocated-result, cumulative cleanup, and candidate-pointer laws. Root full Source 6 and JSON native 6 results must be checked separately.
