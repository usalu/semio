# Third Independent Typed-Command Full-Operation Audit

Date: 2026-08-26

## Verdict: RED

The current packet substantially improves the fail-closed shared foundation: production bare typed values are now test-only, the mounted wrapper has a fixed result-page/ACK state, and the store exposes per-item apply receipts. The requested runnable, universally pre-admitted route is nevertheless not present. In particular, the reachable manifest/action JSON ingress serializes and traverses input before raw-page admission; no concrete registered job implements retained raw-page decoding; publication removes items before a failed apply can return their owners; and the renderer-side exchange is declared but never installed.

No production source was modified. No Cargo, Nx, Wasm, browser, cache, or Git-mutating command was run.

## Verification Performed

- A focused `bun -e` evaluator ran all **40** declared fixture rows (7 output census, 4 grant, 3 freshness, 4 admission, 3 publication, 9 lane, 3 raw-page, 3 fault, and 4 close): `[typed-command fixture] rows=40 status=PASS`.
- `git diff --check` on the action bus, plugin, store, and WGPU glue completed with no whitespace diagnostic.
- `rustfmt --check` was attempted on those same sources, but returned formatting diffs in the shared, concurrently modified worktree. It made no edits and is not a format-green result.
- There is no separate Bun fixture evaluator in the repository; the focused evaluator above validates fixture declarations only and is explicitly not runtime evidence.

## Green Evidence

| Claim | Result | Evidence |
| --- | --- | --- |
| Bare typed values are not production entrypoints | GREEN | `VcsArtifactApp::dispatch_typed` is `#[cfg(test)]` at `plugin/component.rs:20390`; production `dispatch_command_frame` rejects an unqualified byte frame at `20100`. |
| App conversion follows the shared pipeline gate | GREEN, narrow | Action and manifest routes call `require_complete_tool_operation_pipeline` before `A::command_from_action` at `20057` and `20088`; generic bounded proofs fail closed at `20217-20223`. |
| Bounded fault representation | GREEN | `ArtifactBoundedToolFault::from_fault` copies scalar-aligned UTF-8 into a fixed `TYPED_OPERATION_FAULT_BYTES` array at `13170-13189`; fixture fault rows cover ASCII and multi-byte boundaries. |
| Fixed retained result-page/ACK state exists | GREEN, source-static | `MountedTypedCommandFullOperation` retains one page and an exact receiver/operation/generation/sequence/attempt token at `15285-15439`; duplicate retrieval increments attempt and then emits a fixed fault page. |
| One-item store APIs exist | GREEN, source-static | Artifact/config/draft call sites use `apply_one`, while presence/transient expose generation-checked `apply_one` at `store/component.rs:3546`, `3659`, and `13468`. |
| Document freshness is checked immediately before a publication attempt | GREEN, source-static | `publish_mounted_typed_operation_unit` reads current document revision and generation and calls `validate_commit` before choosing a page at `20132-20141`. |

## Blocking Findings

### RED-1 — Generic JSON serialization and traversal still happen before raw-page admission

`admit_command_json` first invokes `serde_json::to_writer` into `TypedCommandWireCounter` at `plugin/component.rs:17349-17351`, then only calls `ActionBus::begin_exact_wire` at `17362-17364`. It additionally calls `bounded_json_items(args, ...)` at `17365` before the retained input is filled/sealed. This is generic serialization/traversal before the raw-page owner exists.

`dispatch_command` also allocates `Value::Object` from every manifest argument at `20079` before it can enter `admit_command_json`. The action route accepts already materialized `Value` arguments for the same reason. The source regression test at `16320-16345` only proves that `A::command_from_action` follows the gate; it does not forbid the counter, `bounded_json_items`, or manifest `Value` construction before admission.

Required correction: make the externally supplied owner-qualified raw pages the production ingress. Admit declared length/key/schema first, retain/seal pages, and have the exact app factory decode them one page at a time. Do not synthesize an owned JSON wire from an already materialized `Value` as the production route.

### RED-2 — There is no concrete retained raw-page decoder/factory route

The action bus exposes `dispatch_wire_retained` and a `create_job_from_wire_pages` hook (`action-bus/component.rs:600+`), but `VcsArtifactApp` does not call that seam. It instead invokes `A::build_tool_job` only after generic command construction at `plugin/component.rs:20309-20337`, then dispatches an already-built `ToolOperationSpec`.

The only concrete app `build_tool_job` located is Layout's export builder (`plugins/layout/.../editor/component.rs:272-291`). It pattern-matches the already-decoded `LayoutCommand`; its `LayoutExportJobFactory` implements ordinary `create_job` (`.../export/component.rs:2497-2523`), not retained-page decoding. Puzzle registers reserved clipboard/import jobs but no `build_tool_job`. Thus production registration is app-owned only in a narrow factory-identity sense, not in the requested raw-input decoder/reducer sense.

### RED-3 — Publication is not lossless/retry-safe, and host lanes do not execute their effects

The mounted publisher removes a completion cell with `completion.take()` at `20127-20132`, then removes each durable/ephemeral mutation with `.pop()` before calling `apply_one` (`20159-20181`). Any apply error is converted with `map_err`, discarding the returned mutation owner. A stale or malformed item cannot be retained for a bounded retry, explicit fault handback, or close proof.

Child/effect/event/UI lanes are only serialized into a `TypedOperationResultPage` (`20182-20192`). ACK merely pops the source vectors or clears `ui_pending` (`15412-15437`); it does not call a child coordinator, effect/event outbox, or UI invalidation path. This is a result-page proposal, not demonstrated application-visible publication.

### RED-4 — The renderer exchange seam is inert

The WGPU glue declares an object-safe `TypedOperationResultExchange` and `install_typed_operation_result_exchange` at `wgpu/glue.rs:4839-4850`. A whole-repository Rust census found only that declaration and readers of its `OnceLock`; there is no implementation of the trait and no call to the installer. Therefore `ExchangeOutcome::take_typed_operation_result` and `KernelClient` methods simply return `None` in every actual renderer session.

The plugin runtime can expose an optional `typed_operation_result` in `PluginExchangeOutput` (`plugin/component.rs:28307-28340`), but no renderer-owned receiver/ACK installation establishes the requested end-to-end renderer seam.

### RED-5 — Fixture laws remain a model, not hostile execution of the mounted operation

`every_language_neutral_hostile_row_executes_the_owned_state_machine_and_serde_oracle` at `16020-16312` drives `OwnedFixtureMachine`, a test-only state model. It never constructs `TypedCommandFullOperationJob`, `MountedTypedCommandFullOperation`, an app factory, `ActionBus::dispatch_wire_retained`, store receipt, or renderer exchange.

Consequently the green 40-row Bun evaluator and the Rust fixture test can validate arithmetic and declared transition names, but not actual max/+1 admission, raw-page ownership, store saturation, stale/cancel/close behavior, result retry, or terminal parsed fields through the live route.

### RED-6 — Roots are captured on the dispatch continuation, not inside worker-owned bounded preparation

After admission and before `MountedWorkerJobSession::try_new`, `dispatch_typed_command_inner` awaits cache refreshes and reads snapshot/config/history/draft/interaction/presence/transient/document roots (`20242-20272`). The worker starts only later at `20343-20350`. This is the stated “roots captured during worker construction” limitation: capture is outside the worker's resumable budget and cancellation/freshness step protocol. The later document freshness check is useful, but it does not turn that async root collection into bounded worker preparation.

## Required Completion Boundary

1. Replace JSON/`Value` synthesis ingress with an owner-qualified retained-page ingress, including declared byte admission before serializer/traversal.
2. Require a concrete domain factory to implement retained-page decode plus one-unit reducer output, and route it through `ActionBus::dispatch_wire_retained`; do not build a generic typed command first.
3. Preserve a mutation/page owner until its store/host receipt is accepted. On stale/error retain it for exact retry/fault/close rather than dropping it from `.pop()`.
4. Connect child/effect/event/UI pages to actual one-unit receivers with receipts, then ACK only after the receiver accepts them.
5. Implement and install a renderer `TypedOperationResultExchange`, marshal the exact fixed token/page in both directions, and add an end-to-end hostile test.
6. Move root acquisition into explicit bounded worker preparation stages or supply fixed retained snapshot handles at admission, with fresh witness validation for every publication lane.
7. Replace the `OwnedFixtureMachine` proof with live state-machine traces whose terminal fields are parsed from actual operation outcomes.

Until those corrections are present and compiled/executed in the appropriate native and Wasm matrices, the honest state is a promising fail-closed scaffold, not a complete typed-command full-operation foundation.
