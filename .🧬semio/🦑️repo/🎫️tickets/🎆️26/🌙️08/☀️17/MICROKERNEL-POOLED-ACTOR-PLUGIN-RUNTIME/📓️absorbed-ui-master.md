# Semantic UI Contract and Renderer Family — Master Design

Anchor commit: `5e7b8046be44badd61d563b1fb0907b4b955851c` (2026-08-19 19:23 +0200).
Goal: `r2602/runningsketchpad`. GitHub issue: https://github.com/usalu/semio/issues/2570.
Coordinator model: Opus 5 High (`opus-4-7` recorded in ticket.json — the registry enum predates Claude 5).
Executors: Sonnet 5 High (`terra-*`). Auditors: Haiku 4.5 (`luna-*`, read-only).

`./compose` is out of scope. SwiftUI / Kotlin-Compose renderers are out of scope for this program.

## Why

The retained-mode `Ui` façade in `semio-framework-ui` is additive and never wired to pixels — the
immediate-mode `widgets.rs` path drives rendering. The OS renderer's `📦️glue.rs` (2,718 lines) is a
composition root that owns window state, plugin polling, theme sync, input, deadlines and tutorial
playback, and redraws continuously (`ControlFlow::Poll` + `request_redraw` every frame). `draw.rs`
fuses the display-list model with WGPU concerns. `UiNode` is consumed by 33 of 34 plugins and
mirrored by hand into TypeScript for a parallel React renderer.

## Locked decisions — do not relitigate

1. **Async at boundaries only — "await between frames, never halfway through a frame."**
   Frame construction (compose → reconcile → layout → prepaint → paint → commit) and input dispatch
   (hit-test → capture → target → bubble → intents) are synchronous run-to-completion transactions.
   Async only at: outer event loop, GPU init, transport, actors, assets. No mutable entity or frame
   reference crosses an await. Pending dependencies use `Measurement::{Ready,Pending,Failed}` plus
   invalidation — never a mid-frame await. Heavy CPU work runs as revisioned background jobs whose
   stale results are discarded. Handlers return `DispatchOutcome`, never `async fn`. Continuous
   redraw is replaced by invalidation + deadline scheduling; idle windows render zero frames.
2. **Big-bang migration.** No V1 freeze, no compat adapters, no deprecations (CLAUDE.md greenfield
   rules). The contract replaces `UiNode` outright. Fixtures are handcrafted; no migration scripts
   remain in the tree.
3. **Full native renderer matrix.** wgpu is restricted to browser WASM (target-gated dependency +
   `compile_error!` guard + `cargo tree` assertions). Windows → hand-written D3D12, macOS → Metal,
   Linux → Vulkan, all behind one `GraphicsBackend` trait consuming the same `RenderPacket`.
   Binding crates (windows-rs / objc2-metal / ash) stay confined to their backend crate.
4. **Coordinate with the concurrent MICROKERNEL session.** `🎠️runtime.rs` and the actor kernel are
   theirs. Read-mostly: define seams, never rewrite. Poll churn before waves touching shared files.

## Load-bearing findings (verified in-tree, these change the work)

- **The wire patch protocol already exists.** `🔌️plugin/🧬️schema/📜️component.wit` `interface ui`
  already defines `surface-ref`, path-addressed `patch-op {replace, insert-child, remove-child,
  set-props}`, revisioned `ui-patch {surface, kind, revision, base-revision, ops}`, and
  `patch-ack`/`patch-rejected`. Rust SSOT is `🔨️modules/🎠️kernel/🦀️component.rs` (`UiPatch` ~line
  865). We replace the **node schema and op addressing inside an existing revisioned envelope**.
- **Guest diffing is a stub.** `🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs` (`PatchTracker`) emits a
  full-body root `Replace` every dirty turn; its doc comment invites a real differ. Our keyed
  reconciler replaces it.
- **One SDK choke point.** Plugins import UI types via `semio_framework_plugin::{UiNode, …}`
  re-exported from `🔌️plugin/🦀️component.rs` (~line 159) — never `semio-framework-ui` directly.
- **ts-rs recursion blocker dies.** The hand-written TS `UiNode` union exists only because recursive
  `Vec<UiNode>` blocked ts-rs. The flat id-table snapshot is fully derivable; the mirror is deleted.
- **The committed wgpu-engine render path cannot create pipelines.** The repo-wide asyncify pass
  corrupted the WGSL string constants in `🎯️targets/🧊️wgpu/🦀️shaders.rs` — every entry point reads
  `async fn vs_main` / `async fn fs_main`, passed verbatim to `create_shader_module`
  (draw.rs:1266/1562). `async` is not WGSL; naga cannot parse it. Additionally several async fns are
  called without `.await` and their futures dropped (`push_dashed_line`→`push_line`,
  `boot_runtime`→`gpu.resize`/`upload_font_atlas`, `frame`→`render_frame`). The sync-frame decision
  is therefore a correctness repair, not only an architecture choice. **Goldens are generated from
  the fixed port, never from the broken committed state.**
- The UI "crate" is one package `#[path]`-mounting its wgpu target; the OS renderer target *is* a
  crate (`semio-framework-os-renderer-wgpu`) and also depends on vello 0.7 / resvg / tiny-skia
  (vello pulls wgpu on native — resolved in G12).
- 7 WGSL constants / 5 shader families: UI SDF quad megashader (9 kinds, animated borders via
  `globals._pad.x`), vector triangles, world3d mesh/lines/textured, blur mip chain (5 levels) +
  scene blit, glass backdrop.

## Target architecture

```
CQRS projections
      │
      ▼
Headless UI runtime      Entity<T> · effects · actual-read tracking · Present
      │                  keyed reconciliation → transactional revisions
      ▼
UiSnapshot / UiPatch  ← the ONLY universal renderer boundary
      │
      ├──────────────┬─────────────────────────────┐
      ▼              ▼                             ▼
React DOM      Custom GPU renderer            (future renderers)
               elements → taffy → prepaint → paint
               → FrameSnapshot → RenderPacket
                    │
      ┌─────────────┼──────────┬──────────┐
      ▼             ▼          ▼          ▼
  webgpu(wasm)   D3D12      Metal      Vulkan
```

### 1. Contract crate — `semio-framework-ui-contract`

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/` (nx `@semio-tech/ui-contract-rs`).
Region files: `🦀️document.rs`, `🦀️component.rs`, `🦀️layout.rs`, `🦀️style.rs`, `🦀️action.rs`,
`🦀️accessibility.rs`, `🦀️presence.rs`, `🦀️surface.rs`, `🦀️limits.rs`, `🦀️builder.rs`.

Schema source of truth: **Rust structs + ts-rs**. The WIT file stays the neutral ABI envelope
schema; payload SSOT is Rust with mechanically generated TS. Deps: serde, `ui_styling`, optional
`ts-rs` behind `typegen`. No dsl/os-kernel/engine deps — compiles on wasip2 and
wasm32-unknown-unknown by construction. Wire bytes stay `pack::encode_record_body` (never
`encode_json_value`, which wraps a full `.spk` container).

- `UiSnapshot { surface, revision, root, nodes: Vec<UiNodeRecord>, layout_epoch }` — flat, no recursion.
- `UiNodeRecord { id: UiNodeId(u64), key, component, layout, style, activity, disabled, transition,
  accessibility, bindings, menu, children: Vec<UiNodeId> }`.
- `UiPatchOp { Upsert | SetComponent | SetLayout | SetActivity | SetChildren | Remove | SetRoot }`.
  Application is transactional: base-revision check → shadow map → validate → commit, else reject
  the whole patch (existing `patch-rejected` event).
- `Component`: Container(role Plain|Section|Group|Field|Form|Toolbar), Text, Button, Separator,
  Input(closed kind enum), Select, Toggle(explicit `on: bool`), KeyValueList, Slider, NumberStepper,
  Ring, IconSelect, Tree/TreeSection/TreeItem (**rows are first-class nodes**; inline
  `UiControlNode` becomes ordinary child nodes and the enum dies), Image, Surface, Extension.
- `UiPresence` decomposes: `Disabled`→`record.disabled`; `Hidden`→node not emitted (reconciler emits
  `Remove`); `Introducing`/`Celebrating`→`transition`; `status`→`activity`; hover/selected/color/
  peers→**separate PresenceUpdate channel** keyed `(surface, node_key)` with TTL + coalescing
  (`ui_tree_stamp_presence` deleted).
- `ActionId { scope, name, version }`; `Trigger { Activate, Change, Commit, Delta, Drop, Submit,
  Abort, RepeatLast, HoverPreview }`; neutral `UiValue` (DslValue conversions live in os-kernel).
- `UiIntent { surface, revision, node, node_key, trigger, action, args, input, seq }`. Stale intents
  (revision < current − 1) are dropped as `Stale`.
- The 15 product scene structs move to the existing `🖱️ui/🎬️scene/🦀️component.rs`; the sparse
  15-`Option` ComponentScene struct dies — exactly one pack-encoded payload in `SurfaceProps.doc`
  with a `doc_schema` id (e.g. `"world3d@1"`).
- Field-by-field disposition for all 19 old variants: see `📓️recipe-plugin.md`.

### 2. Headless runtime — `semio-framework-ui-runtime`

`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/` (nx `@semio-tech/ui-runtime-rs`).
`🦀️entity.rs` (generational store, `Entity<T>`/`WeakEntity<T>`, lease guard — mutation only through
sync `FnOnce` closures, so no await can cross a lease *by construction*), `🦀️context.rs`,
`🦀️tracking.rs`, `🦀️present.rs`, `🦀️reconcile.rs`, `🦀️gateway.rs`, `🦀️inbox.rs`, `🦀️presence.rs`,
`🦀️dispatch.rs`, `🦀️transaction.rs`.

`transact()` is **not** async: drain inbox → dispatch intents → effect-flush to fixpoint (64-cycle
EffectStorm budget) → present dirty surfaces → reconcile → collect gateway output and next wake.
Used by both guests (plugins, via the reactor) and hosts. No `Send` bounds.

**Kernel seam without touching `🎠️runtime.rs`:** a new sibling file
`🔌️plugin/🖥️host/🧠️ui-runtime-bridge.rs` implements `CommandSink` over the kernel's existing mailbox
handle; only a one-line `mod` registration touches existing host code. Backpressure surfaces as
`activity: Waiting` on the initiating control, retried next transaction.

### 3. Protocol flip

WIT `interface ui`: node-id-addressed ops replace path-addressed ones; `ui-patch` loses
`kind: string`; new `ui-intent-event` (UI intents stop masquerading as `app-command`); `turn-result`
gains `presence: list<presence-update>`; `patch-ack`/`patch-rejected` unchanged. `🎠️kernel` deletes
its own UiPatch/PatchOp copies, re-exports the contract's, and adds `Event::UiIntent`.

### 4. GPU renderer family

New crates: `semio-framework-ui-render` (gpu-core, **no wgpu, no winit**) at
`🖱️ui/🖼️render/📦️packages/🦀️rust/`; backends at its `🎯️targets/{🧊️webgpu,🍎️metal,🪟️d3d12,🌋️vulkan}/`;
platform layer `semio-framework-ui-host` at `🖱️ui/🖥️host/📦️packages/🦀️rust/`.

- `ElementId` = fxhash(parent, key), stable across frames; retained element state in a generational
  map; protocol-node-id ↔ ElementId bimap in `GpuSemanticAdapter`.
- Sync `Element { request_layout → prepaint → paint }` over a per-frame bump arena.
- `FrameSnapshot { generation, packet, hitboxes, dispatch, focus, ime, access, next_deadline }`;
  presented/building atomic swap; **input always dispatched against the presented generation**.
- `InvalidationReason` bitflags; `FrameScheduler` (dirty set + deadline heap + visibility);
  `should_render` returning `None` ⇒ zero idle frames.
- Dispatch preserves verbatim the verified `events.rs` semantics: overlay-first reverse-order hit
  test, `CLIPS_CHILDREN` pruning, `HIT_TRANSPARENT` and plain-`Stack` pass-through (including the
  activate / drop_action / DRAG_SOURCE exceptions), pointer capture, capture→target→bubble, focus
  ring, overlay dismiss policies, drag ghost. The seven `events.rs` test regions port as replay tests.
- `RenderPacket`: quad instances (byte-identical `UiInstance` layout), vector vertices, glass
  instances, ordered `DrawBatch` (stencil silhouette masks precomputed CPU-side), surface passes,
  resource ops, `has_animated_primitives`, 64-bit content hash (equal hash ⇒ skip submission).
  String-keyed raster/mesh tables become interned typed generational ids.
- `Scene::finish` = validate → snap to physical pixels → order → batch → hash.
- `GraphicsBackend`: `resize` (0-size parks the surface), `apply_resources`, `render → RenderReport`,
  `device_status`, `recover() → lost ResourceIds`, plus `backend-testing` hooks
  (`debug_force_device_loss`, `read_back`). Only construction is async.
- **Shader strategy:** canonical corruption-fixed WGSL in gpu-core + build-time naga
  cross-compilation (build.rs only, never linked at runtime, so the wgpu guard stays green): webgpu
  consumes WGSL; vulkan embeds SPIR-V; metal generates MSL compiled at device init; d3d12 generates
  HLSL compiled via D3DCompile. Per-backend hand-written override slots exist for when naga output
  is inadequate. A naga dev-dep test permanently guards the WGSL-corruption class.

### 5. OsHost decomposition

New files in the existing OS renderer crate dir (**no directory rename** while the peer session owns
`🎠️runtime.rs`): `🦀️os_host.rs` (renderer no longer owns the actor kernel), `🦀️kernel_seam.rs`
(`KernelSeam { submit_intents, drain_outcomes, set_waker }` over the existing kernel_runtime
statics; `poll_tasks()` only on wake), `🦀️deadlines.rs` (camera settle 350 ms, wheel zoom 120 ms,
caret blink 500 ms, tutorial keyframes, asset fetches, native plugin hot-swap 1 s coarse poll — all
become scheduler deadlines or wake events), `🦀️winit_app.rs` (WaitUntil control flow; both
`request_redraw` re-arms at glue.rs:2406/2424 and `start_frame_loop` deleted).

### 6. React renderer + manifest TS

Generate `🛂️manifest/🤖️generated/🟦️ui-contract.ts`; delete the hand-written UiNode block and
`PluginUiNode`. New `🧱️elements/UiDocumentStore/🟦️component.tsx`: per-surface `{revision, root,
nodes: Map}` with transactional `applyUiPatch` (base-revision reject, draft validate, atomic swap,
`useSyncExternalStore` per-node subscription so a `SetComponent` re-renders exactly one component).
`Interpreter` switches on `component.type` with LayoutSpec-driven wrappers and `UiNodeId` keys; one
`emitIntent()` helper; presence read from a `PresenceOverlay` context, never the document store.

## Test strategy

Contract: serde + pack round-trips per variant/op; property tests; an invalid-patch fuzz corpus
shared byte-for-byte between Rust `apply_patch` and the TS store (rejection must be total — state
hash unchanged). Runtime: nested-lease violation, read-during-lease, actual-read precision, revision
coherence under ack/reject interleavings, effect-storm budget, stale-intent drop, gateway-full
backpressure, byte-identical determinism. Renderer: GPU-free RenderPacket structural goldens +
content-hash determinism across dpr 1.0/1.5/2.0; naga WGSL validation; input replay conformance;
backend pixel conformance behind `backend-testing` (tolerance |Δ| ≤ 3/255 for ≥ 99.9 % of pixels,
none > 12); device-loss and zero-size scenarios. Shared conformance corpus lives at
`🧬️contract/📚️examples/🧪️conformance/` and is consumed by **both** React vitest and the GPU harness.

This macOS machine validates gpu-core + Metal fully (including pixel goldens), webgpu compile plus
browser runtime via the existing Trunk/vitest harness, and D3D12/Vulkan **compile-only** cross
checks. Pixel jobs for D3D12 (WARP) and Vulkan (lavapipe) are defined for their platforms and
skipped — never faked — here.

## Definition of done

- The contract is the only renderer boundary; React and the GPU family pass the same corpus.
- Frame construction and input dispatch are run-to-completion sync transactions; the naga test guards
  shader integrity; idle windows submit zero frames (5 s idle → 0 frames).
- wgpu appears in exactly one crate, enforced by `compile_error!` + `verify ui-boundaries`.
- Metal passes pixel conformance here; D3D12/Vulkan compile-check green on cross triples.
- The renderer no longer owns the actor kernel; `🎠️runtime.rs` untouched.
- All 33 plugins present via `Present`/`ComponentTree`; wasip2 fleet sweep zero-warnings; grep-zero
  for `UiNode` / `ActionDescriptor` / `UiPresence` outside ticket folders.
- Old immediate-mode path, fused DrawList, `engine.rs` façade, continuous redraw and the hand-written
  TS mirror are deleted.
