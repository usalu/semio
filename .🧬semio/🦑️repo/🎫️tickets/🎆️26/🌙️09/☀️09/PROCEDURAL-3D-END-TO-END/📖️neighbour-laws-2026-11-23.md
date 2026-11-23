# Neighbour interactivity laws, 2026-11-23

## Surface lifetime

Bun: `1 pass`, `0 fail` (`P3mn interactivity mounted engine surface lifetime`, about 2.8s).

The law was reading facades and stale slice markers. Production behavior lives in the module trees below, joined with `policyModuleTreeFiles(..., { crates: true })`, with the engine test corpus after `POLICY_RUST_TEST_EVIDENCE_BOUNDARY`.

Real sources:

- Engine canvas and prepared canvas: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
- Element contracts sibling of the renderer: `try_borrow_mut` / `try_lock`
- Surface registry and retirement ladder under the engine-registry crate, including `slot.generation.checked_add(1)`
- Host, winit, browser worker, renderer glue, NodeGraph under elements, flow `host-close`, tiled map, editor, dag model/graph/host

Needles retargeted to the current text of the same close, packet, generation, and retirement behavior (`OriginalFixedSlotPages` rejection, `candidate.begin_close` / `close_step`, `EngineSurfaceClosePhase::Raster`, flow `store.close_owned_store_step`, dag `artifact_retire_struct!`, host `try_into_retirement`). No Rust edit. `wrapping_add` stays forbidden.

## Live reconcile

Bun: `0 pass`, `1 fail` (`live reconcile interactivity`). The failure is thrown before the law loops: dozens of mutation needles still do not occur in the re-anchored text. A direct call of `interactivityLiveReconcileFailures` on the clean re-anchored sources still returns **30** failures, so the test would stay red even after every mutation bound.

What was stale:

- Credit caps are aliases. `SURFACE_RECONCILE_SURFACE_BYTES` is `ui_contract::UI_RESIDENT_SURFACE_BYTES`, defined as `8 * 1024 * 1024` in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏠️resident/🦀️.rs` (no digit separators). Aggregate bytes are `UI_RESIDENT_SLOTS * UI_RESIDENT_DOCUMENT_BYTES`, not `SURFACE_BYTES * 4`. Admission slots alias `UI_RESIDENT_SLOTS`.
- `policyReadRustPolicyFamily` on the renderer glue does not follow `extern crate`. Kernel runtime is `semio_renderer_wgpu_kernel_runtime`, physically under `🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/` (`kernel-core`, `kernel-retained-state`, `kernel-pool`).
- `cursor.step(current)` and `pub fn drive_one(&mut self, cx:` moved to `cursor.step(&current.shadow)` and `pub fn drive_one(&mut self, generation: u64, cx:` in `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🏃️reconcile-job/🦀️.rs`. Generation is `generation != state.generation`.
- Fixed slot tables are `slots: Box<[Option<T>; N]>`.
- `UiFixedList` stores `storage: fixed_list_storage::PagedList<T, N>` (`🌱️values/🔢️fixed-list/🦀️.rs`). `pub struct UiText` is `🧬️contract/🔤️text/🦀️.rs`. Grid tracks are `🧬️contract/🖼️presentation/📐️layout/🦀️.rs` (`UI_GRID_TRACKS = 32`).
- Hostile fixtures live in `🧪️tests` trees that are not `#[path] = "🦀️.rs"` mounts, so the evidence reader skipped them. The test now appends `listSourceFamily(..., { tests: true })` for the UI contract, kernel, engine, and reactor patches after the test boundary.

Readers now used by `interactivityLiveReconcileBoundSources`: reconcile family plus resident production; values module tree plus the text crate; presentation, conformance, kernel `TurnResult`, and window-kits in the schema string; renderer glue with `crates: true` plus kernel-retained-state and kernel-pool; reactor turn via `policyReadModuleTreeSource` (not `policyReadRustPolicyFamily`, because `fixture.rs` is not a canonical test path).

Law edits (TypeScript only):

- `🖥️mounted/🟠️reserve/🟦️.ts`: credit aliases plus the contract literals; drive grant uses the current signature, `cursor.step(&current.shadow)`, and `generation != state.generation`.
- `🖥️mounted/🔶️reconcile/🟦️.ts`: fixed-list needle is the paged storage field; the value scan rejects `wrapping_` (generation wrap) and no longer treats every `saturating_` page-byte clamp as that failure.
- `🖥️mounted/🟢️part4/🟦️.ts`: retained surface, pending rejection, shell document, and command-document slot needles include the `Box<...>`.

Rust edits: none. The remaining 30 baseline failures were not cleared by retargeting a spelling. They still name missing or rewritten owners (UiValue arena construction, table kit, command JSON cursor, document lease `= 8`, reconcile preflight, patch tracker, shell refresh, window-measure rail, one command-batch fixture name). Those were left in place rather than deleted.

## Bun counts

| test | result |
| --- | --- |
| `🔬️interactivity-mounted-engine-surface-lifetime/🟦️.ts` | 1 pass, 0 fail |
| `🔬️interactivity-live-reconcile/🟦️.ts` | 0 pass, 1 fail |
