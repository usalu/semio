# P8yp Typed Command Full Operation and Cancellation

Date: 2026-08-22  
Verdict: **PASS for the cancellation and Rust/WIT segmented-drain foundations; REJECT for the typed-command full-operation route. Phase 8 remains RED.**

## Scope and decision

This packet reattacked the shared plugin host after P8yj and the independent P8yk foundation-only PASS. It did not touch concurrent Layout or Diagram source files. It used no Cargo command, ticket lifecycle operation, or modifying Git command.

The requested full typed prepare/reducer/commit operation cannot honestly be activated on the current store surface. `refresh_cache`, draft/interaction snapshots, child-view construction, presence peer collection, transient capture, and document envelope/revision access materialize whole values or collections through APIs that expose no bounded cursor. `dispatch_emit` similarly owns multi-store mutation, child routing, effects, events, command logging, and task spawning as one async method; it exposes no bounded commit-candidate cursor plus O(1) atomic swap boundary.

Creating an enum around those same awaits would be phase-cursor theater. Therefore the nine owner-local reducer proofs remain inactive, bounded-first-step production activation remains zero, and the shared typed route now fails with `interactive-job.full-operation-pending` before `refresh_cache` or any snapshot/preparation work. This also closes exact app-owned typed command routes until a complete operation job exists. The old implementation remains in source as the explicit residual implementation target, but it is unreachable behind a non-constant fail-closed authority method.

## Implemented shared foundations

### Full-operation verifier and runtime fail-closure

`verify interactivity tool-jobs` now requires a concrete `TypedCommandFullOperationJob<A>` with all six governed stages:

1. `typed-command-prepare`
2. `typed-command-reducer`
3. `typed-command-output-validation`
4. `typed-command-ephemeral`
5. `typed-command-emit`
6. `typed-command-expose`

The static gate also requires cancellation polling, yield/watchdog control, decoded/output limits, previews, checkpoints, and revision/generation commit validation. It rejects preparation or post-job application outside the worker, monolithic effect/event serialization, whole child traversal, handler-only output validation, fake stage labels, missing per-stage watchdog control, and stale-result exposure.

When that structure is absent, `dispatch_typed_command_inner` must call `require_complete_tool_operation_pipeline(&admission)?` before its first preparation operation, and the authority method must be strictly fail-closed. This prevents exact factory metadata from making an incomplete operation reachable.

### O(1) hierarchical cancellation

`ToolCancellationHandle` no longer stores one flat operation map that is scanned or drained. It now owns:

- one lock-free app parent `CancelToken`;
- one atomic app scope generation;
- one indexed document scope per live document key;
- one operation child token beneath the current document scope.

The exact changes are:

- `begin` performs one document lookup, cancels the superseded document parent in O(1), installs one replacement scope, and returns the child token actually supplied to `BatchJobParams`;
- `cancel_document` performs one exact-key removal and parent cancellation;
- app `Drop` performs only `app_scope.cancel_now()` plus an atomic generation increment; it does not lock, scan, await, collect, or drain;
- lease success removes only its exact generation/operation row; lease drop cancels only its own child and removes at most that exact row;
- media-export supersession uses `media_export_documents` instead of scanning every live export;
- session cancellation, freshness checks, operation id, document id, base revision, and store generation remain exact.

Focused Rust source tests cover live-instance isolation, exact document close, same-document supersession, 1,024 simultaneous document scopes, app-generation close cancellation, first/last saturated descendant observation, and stale owner generations not removing the replacement scope. These tests were added but not run because Cargo was prohibited.

The verifier rejects `.iter().filter`, key scans, drains, vector collection, cancellation futures, media-export scans, and whole-map Drop cleanup inside the cancellation implementation. The previous cancellation and Drop failures are absent from the new ledger.

### Segmented drain terminal protocol and WIT bridge

The Rust download map previously removed an operation immediately after returning its last `Some(chunk)`. That made the required subsequent terminal read fail with `unknown-segmented-download`. It now removes the map entry only when `take_chunk()` itself returns `None`:

- last nonempty `Some`, maximum 4,096 bytes: authority remains addressable;
- next read: `Ok(None)` and authority is removed;
- later read or forged operation: exact `Fault`.

The actor jobs WIT now exports:

```wit
take-segmented-download-chunk: async func(instance-id: u32, operation-id: u64) -> result<option<list<u8>>, plugin-error>;
```

The component guest calls `plugin_take_segmented_download_chunk(runtime, instance_id, operation_id)` and maps `Fault` through the canonical `component::plugin_error`. It does not use `.ok().flatten()`, so an unknown operation or other fault cannot masquerade as terminal `None`. The scale guest and schema-parity export inventory were updated. Layout owns the TypeScript materializer/shard transport beyond this Rust/WIT boundary.

A focused Rust source test observes last `Some`, retained authority, terminal `None`, removal, and the subsequent exact unknown-operation fault. The static verifier requires the WIT result shape, guest bridge, lossless error mapping, terminal-removal ordering, and focused test marker.

## Adversarial coverage

The permanent verifier self-test suite increased from 38 to 46. New rejection fixtures cover:

- fake full-operation phase labels;
- preparation and commit outside the worker;
- an incomplete route without a pre-preparation fail-closed guard;
- monolithic huge effects and child-output validation;
- a purported full operation without per-stage cancellation/yield control;
- stale result exposure without revision/generation validation;
- collection-wide cancellation during begin/document/app close;
- whole-map Drop cancellation;
- premature segmented authority removal;
- a segmented guest bridge that could swallow errors.

Existing exact proof, copied-owner, alias, reserved-envelope, importer-pre-serialization, output-credit, and segmented-authority fixtures remain green.

## Canonical ledger

The two generated ledgers are byte-identical:

- `📊️p8yp-current-command-ledger.json`
- `📊️p8yp-canonical-diff-check.json`
- size: 310,794 bytes
- SHA-256: `1844ed06f3f4840f16b7cf33f79b35fa10a3a2ab0f02c8227014edc718a115a3`

| Inventory | Count |
| --- | ---: |
| Macro hosts / invocations | 50 / 50 |
| Macro rows / unique rows | 775 / 773 |
| Literal registrations | 656 |
| Owner-local reducer proofs | 9 |
| Admitted complete typed operations | 0 |
| Production factories / bounded activations | 11 / 0 |
| Typed dispatches / aliases | 3 / 4 |
| Remaining live command rows | 884 |
| Framework reserved routes | 8 |
| Pending app-owned importers | 35 |
| Global payload-store candidates | 34 |
| Verifier self-tests | 46 |
| Current fail-closed failure classes | 7 |

The former cancellation, Drop drain, segmented terminal, and WIT bridge failures are closed. The exact remaining failure classes are:

1. eight framework reserved routes still lack real route-owned jobs;
2. import submission still serializes/clones whole media before job construction;
3. typed preparation/commit still lacks the required full operation job;
4. 34 process-global payload stores remain;
5. the eight reserved routes remain fail closed;
6. 35 importer owners remain fail closed;
7. 884 command rows remain fail closed.

## Gates executed

| Gate | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | PASS: `self-tests=46 clean` |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean; one recorded allowlisted test-only blocking bridge |
| `bun ./📜️script.ts verify interactivity tool-jobs` | Expected fail-closed exit: 0 admitted, 884 remaining, seven failure classes |
| Two JSON generations plus `cmp -s` | PASS: byte-identical |
| `git diff --check` over this packet's script/Rust/WIT files | PASS |

No native compile, Rust test, Wasm bindgen/component compile, browser integration, runtime watchdog timing, or saturation-under-real-worker gate was run. None is claimed passing.

## Files changed by this packet

- `/Users/ueli/Documents/semio/📜️script.ts`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️component.wit`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️schema-parity/🦀️component.rs`
- the two P8yp ledger JSON files and this report

## Remaining full-route prerequisites

1. Add operation-owned, bounded snapshot cursors for artifact/config/draft/interaction/child/presence/transient data; their terminal product must be immutable `Arc` authority, not repeated whole clones.
2. Add a bounded `dispatch_emit` prepare cursor for artifact/config/draft/child/effect/event/task/command-log output with exact item and byte caps.
3. Add one generation/revision-validated O(1) multi-store commit swap, with stale candidates discarded before any exposure.
4. Implement the real `TypedCommandFullOperationJob`, including child reducer execution, previews/checkpoints, deterministic resume, cancellation after every bounded unit, and stage-specific watchdog tests.
5. Only then remove `require_complete_tool_operation_pipeline`, activate exact owner-local rows, and rerun native/Wasm/runtime/saturation/timing gates.

The cancellation and Rust/WIT segmented-drain foundations are ready for independent source audit. The typed-command full route is intentionally not accepted and not reachable; Phase 8 remains rejected.
