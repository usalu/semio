# P5b Renderer Retained Document Consumer Packet B Implementation

## Scope

Packet B replaces the renderer's nested legacy presentation boundary with generation-qualified fixed retained document owners. The implementation touches the six packet files and the authorized direct UI-engine expansion only. `frame_job`, `Layout`, `native_io`, `flex.rs`, `paint.rs`, and the UI component model are unchanged.

## Changed Regions

- Renderer wgpu `📦️glue.rs`
  - `KernelRuntime::ExchangeOutcome`
  - `KernelPoolState::RetainedDocumentExchange`
  - `KernelPoolState::apply_turn_result` / `apply_ui_patch` / `advance_retained_one`
  - fixed retained patch queue and mounted patch/document arena retirement
  - retryable fixed-arena document-builder admission; arena pressure retains the pending publication authority instead of falling back to the prior document
  - alternating semantic close lane, preventing patch and document pages from retiring in one opportunity
  - authorized `KernelRequest::AdvanceRetained { instance, surface }` maintenance seam, which performs one targeted retained unit without polling the guest
  - bounded instance teardown and smoke-report type fallout
- `ProgramBridge/🧊️component.rs`
  - native program exchange render result
  - one initial `SurfaceVisible` followed by bounded targeted `AdvanceRetained` requests until publication; no repeated visibility/Wake event
  - public `ProgramBridgeEntry::render` / `render_with_document`
  - wasm retained-document fail-closed boundary
  - loader scan/read regions are unchanged
- `Shell/🧊️component.rs`
  - plugin window/panel/spawned retained document stores
  - refresh ownership transfer and close lane
  - retained document render call sites
  - legacy framework panel storage/presentation removed; builders remain isolated testable producers until semantic publication owns them
- `Interpreter/🧊️component.rs`
  - `RetainedDocumentConsumer::render_ui_document`
  - existing clipboard regions are unchanged
- UI wgpu `🦀️engine.rs`
  - `DocumentIngress` begin/apply-one-page/finish/close state machine
  - atomic publication and last-valid retention
  - hostile retained-document fixtures
- UI wgpu `🦀️reconcile.rs`
  - `DocumentPageReconcile` direct `UiDocumentNodePage` admission
- UI wgpu `🦀️tree.rs`
  - fixed `UiDocumentTree`, stable `UiNodeId` lookup/parent/remove, iterative validation, one-record close
- Renderer `🦀️kernel_seam.rs` and `🦀️winit_app.rs`
  - caller census found no direct type fallout; unchanged

## Deleted Legacy Paths

- Recursive `present_snapshot` / `present_record` / tree-item / value / surface presentation.
- Whole nested `UiNode` and `Vec` reconstruction in renderer glue.
- `ExchangeOutcome` dynamic `HashMap<String, UiNode>` and clone-based surface extraction.
- Shell recursive external-slot tree resolution.
- wasm JSON-to-`UiNode` deserialization and dynamic render-function admission.
- Production whole `ui_contract::apply_patch(&mut state, &patch, ...)` reachability.
- Bulk retained-surface removal during instance close.

## Ownership and Close Inventory

| Owner | Admission | One-op progress | Rejection preservation | Close path | Terminal-empty witness |
| --- | --- | --- | --- | --- | --- |
| `UiPatchApplyProducer` | retained state + patch by value | one census/node/op/validation unit globally per turn | `UiPatchApplyRejected` | outcome/rejection `close_step`, then mounted `close_ui_patch_owner_one` | `take_state` only after close |
| queued `KernelUiPatch` | `UiFixedList<_, UI_DOCUMENT_LEASE_SLOTS>` by value | one patch enters the producer only when its predecessor is terminal | exact maximum/+1 `try_push` returns the patch owner | overflow moves into retained patch arena | fixed queue empty |
| `RetainedDocumentBuild` | retryable fixed arena builder | one admission, node record, or mounted arena retirement page per opportunity | arena admission pressure retains `build_pending`; builder retains/rejects exact record | pending scalar, builder release, then mounted document close | pending flag and builder absent |
| `UiDocumentLease` | fixed exchange list | one page read per render opportunity | fixed-list max/+1 returns document owner | each producer/consumer alias calls `close_step` once and leaves its lane; mounted arena close retires pages after the final alias | handle generation becomes terminal after final release |
| `UiDocumentTree` staging | header by value | one node page | `UiDocumentPageRejection` retains generation/revision/index/record | `close_step` removes one record | no records remain |
| Shell replaced lease alias | fixed close list | one consumer alias release per chrome opportunity | capacity rejection receives owner and releases its alias once | one `UiDocumentLease::close_step`, then the released alias leaves the list | close list empty; producer lease remains valid |
| Instance retained surface | fixed semantic owners | one owner/page per destroy step | queued patch remains owned | `RetainedSurface::close_step`, patch arena terminal latched before document arena close | map entry removed only after terminal |

## Caller Census

- `ProgramBridgeEntry::render_with_document` is the only program-to-shell surface render boundary and now returns `UiDocumentLease`.
- `KernelClient::advance_retained` is the only no-guest maintenance caller; ProgramBridge invokes it after the initial visibility exchange until the requested fixed surface publishes or the exact opportunity ceiling is reached.
- Shell plugin callers are window refresh, declared panel-tab refresh, and spawned-app refresh; each stores the lease by move.
- Shell plugin render sites call `Interpreter::render_ui_document`; framework-local panel storage no longer enters the renderer and production fails closed until that chrome is emitted by the semantic producer.
- `Interpreter::render_ui_document` is the sole production document-page reader and engine ingress caller.
- The former `Ui::apply_tree` ingress and `Interpreter::render_ui_node` are `cfg(test)` oracles only and have zero production callers.
- `kernel_seam.rs` and `winit_app.rs` do not name `ExchangeOutcome`, `UiNode`, or program render return types and require no fallout patch.
- The upstream kernel authority is the live `UiTurnPatches` fixed single-patch owner at `🧰️framework/🔨️modules/🎠️kernel/🦀️component.rs:1069-1153`; it exposes `try_push_ui_patch`, borrowed/owned iteration, bounded serde admission, and incremental close. Reactor production is `plugin/⚛️reactor/🦀️component.rs:1583,2131`, shard encoding is `plugin/🖥️host/🧵️shard/🦀️component.rs:158`, and renderer decoding is `📦️glue.rs::decode_actor_turn_result`. Glue consumes the owned fixed authority directly and advances one global retained patch/document unit after admission.

## Hostile Fixtures

The permanent `RetainedDocumentHostileFixtures` source fixture covers fixed arena maximum/+1 owner return, an actual cancelled context, stale/ABA generation rejection, interrupted staged close, iterative nested depth, lost lease handles, consumer/device-style owner drop, and preservation of the last fully published document. Glue's hostile first-render fixture proves publication needs multiple fixed opportunities, a stale patch preserves the last-valid lease, and incremental close reaches terminal. Production source additionally exposes explicit deadline checks and exact page-order/revision/duplicate/capacity rejection.

## Verifier Mutation Handoff

Required mutations:

1. Replace `>=` with `>` in published-generation stale admission; stale/ABA fixture must fail.
2. Remove the `page.index() != expected_index` rejection; page-order fixture/source gate must fail.
3. Move `publish_document` before the record/graph validation cursors reach terminal; last-valid/interrupted fixture must fail.
4. Change one `close_step` record removal into a loop; one-owner close source gate must fail.
5. Change fixed exchange or Shell close storage to `Vec`/`HashMap`; bounded-owner source gate must fail.
6. Reintroduce `present_snapshot`, recursive `present_record`, JSON `UiNode` deserialization, or `.clone()` surface extraction; prohibited-source census must fail.
7. Remove cancellation, deadline, or generation comparison in `begin_document`/`apply_document_page`; qualification source gate must fail.
8. Replace `UiPatchApplyProducer` with production `apply_patch`; by-value producer source gate must fail.
9. Replace `TurnResult::ui_patches: UiTurnPatches` with dynamic admission, bypass its bounded serde visitor, or overwrite a full retained patch queue; maximum/+1 owner-return mutation must fail.
10. Replace `AdvanceRetained` with a repeated `SurfaceVisible`/`Wake`, advance more than one retained unit per request, or drop its fixed `SurfaceId` owner during queue shutdown; first-render and request-ownership mutations must fail.
11. Clear `build_pending` when fixed document-arena admission rejects, or combine the terminal patch-owner close with the first document-node push; arena-pressure and one-opportunity mutations must fail.

## Scoped Verification

- `rustfmt --edition 2021` and `rustfmt --check` completed for the seven changed Rust sources.
- Exact prohibited-source census completed for recursive presentation, external-slot recursion, dynamic `UiNode` staging, `HashMap<String, UiNode>` exchange, clone extraction, and production `apply_patch`.
- Direct caller/type census completed across renderer engine, `kernel_seam.rs`, and `winit_app.rs`.
- Integrated compile/audit is deferred until the core lane publishes the retained TurnResult page codec that replaces the frozen shard JSON encode and renderer JSON decode boundary.
- Cargo, Nx, Wasm, browser, network, and broad runtime commands were intentionally not run under Packet B restrictions.
