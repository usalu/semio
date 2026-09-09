# Brush-mesh upload audit — seven refused `registerBrushMesh` on window activation (2026-09-10)

Read-only audit. Observed 2026-09-09 23:55 in the browser (puzzle 3d, React release target): activating
the Perspective window dispatches seven `registerBrushMesh` actions, each answered with
`Effect::Notify { message: "<code>: <url>" }`, ~12 s of serialized guest time total, the activating click
swallowed (no `interactionSelect`). No cargo build, no browser repro was run for this audit — findings
are from static reading of the current tree plus the two prior waves' own measurements.

## 1. Who dispatches them, and why activation (not scene load)

`handleRegisterBrushMesh` — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:4351-4372`.
Its only caller is `BrushMeshRegistrar` (`🟦️.tsx:1710-1718`), mounted once per distinct mesh URL:

```
🟦️.tsx:5346-5350
{brushMeshUrls.map((url) => (
  <Suspense key={url} fallback={null}>
    <BrushMeshRegistrar url={url} onRegister={handleRegisterBrushMesh} />
```

`brushMeshUrls` (`🟦️.tsx:4384`) is `new Set([...meshes.map(m => m.url), brushPreview?.meshUrl])` — one
entry per distinct mesh URL among the window's placed object kinds (plus the live brush-ghost preview),
not one per object instance. `BrushMeshRegistrar` uses `useLoader(GLTFLoader, …)` (react-three-fiber
Suspense) and fires its registration effect on mount (`🟦️.tsx:1712-1716`). This subtree only exists once
this window's `world-3d` surface is rendered at all: `SurfaceView`/`PagedSurfaceView`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1262-1279`,
doc comment at `:10`: "lazily mounts `canvas-2d`/`world-3d`/etc. surface [hosts]") depends on a
`UiNodeRecord` for that surface being present in the tree. If body/panel refresh is scoped to the
active window, an inactive window's `world-3d` record — and therefore its `World3dHost`/`BrushMeshRegistrar`
subtree — never mounts, so first mount (and first `useLoader`/registration effect) coincides with first
activation rather than with document/scene load. This chain is read from source, not confirmed with a
DOM/React-profiler trace in this audit; treat "activation triggers first mount" as the best-supported
static explanation, not a measured fact.

## 2. Which refusal, and why

`handleRegisterBrushMesh` takes one of two paths (`🟦️.tsx:4351-4372`):

- Fast/digest path (`:4354-4357`): if `registeredBrushMeshesRef.current.get(url) === digest`, dispatch
  `registerBrushMesh { url, digest }` only — no page bytes.
- Page path (`:4358-4369`): otherwise queue `puzzle3dBrushMeshPages(...)` and drain one page per macrotask.

`registeredBrushMeshesRef` aliases the **module-level singleton** `registeredPuzzle3dBrushMeshes = new
Map<string, string>()` — `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx:2572`.
It lives for the life of the browser tab's JS module graph and is never reset by a component
mount/unmount, a window activation, or a plugin-guest restart.

On the Rust side, `register_brush_mesh`
(`✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs:24-52`)
branches on the presence of `page`/`pageCount`: absent → the id-only path, `:38`
`ctx.app.precompute.borrow_mut().adopt_shared_mesh(url, digest)`; on `false`, `fault(ctx, url,
Puzzle3dMeshUploadFault::Digest)` (`:39`, `fault()` at `:66-68`). `adopt_shared_mesh`
(`⏳️precompute/🦀️.rs:1607-1616`) first checks this document's own `mesh_sources`/`mesh_is_fallback`
(per-session state), then falls back to `shared_brush_mesh(url)` (`:993-997`), which reads the
**process-wide** `brush_mesh_store()` (`:966-978`, a `static OnceLock<Mutex<Puzzle3dBrushMeshStore>>`).
"Process-wide" here means one wasm-guest instantiation's linear memory — a fresh instantiation starts
with an empty `OnceLock`, empty `handles` map, no derived geometry for any URL.

So the refusal is the **digest form**: the host believes (via the eternal `registeredPuzzle3dBrushMeshes`
Map) that this exact `(url, digest)` was already paged and derived, and re-announces id-only; the guest's
own store — scoped to its current instantiation — has no record of it, `shared_brush_mesh` misses, and
`adopt_shared_mesh` returns `false`. The wire code is `Puzzle3dMeshUploadFault::code()` →
`"puzzle3d-register-mesh-digest"` (`⏳️precompute/🦀️.rs:1030-1057`, `Digest` arm at `:1046-1057` →
`"puzzle3d-register-mesh-digest"`), matching the observed `"<code>: <url>"` shape exactly
(`register-brush-mesh/🦀️.rs:66-68`: `format!("{}: {url}", rejection.code())`).

Page-size vs. wire bound, for reference (unaffected by this bug since the digest path carries no page
bytes): `PUZZLE3D_MESH_PAGE_VALUES = 1_024` values/page (`⏳️precompute/🦀️.rs:1009`),
`PUZZLE3D_MESH_PAGE_BASE64_CHARS = 5_464` (`:1015`), against the retained command's
`PUZZLE_COMMAND_RAW_BYTES = 8_192` (`✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs:10`) — a full
page's JSON envelope measures ≈5 725 bytes per wave-M2 (§2), comfortably inside the 8 192 bound.

## 3. Why guest time is 0.65-2.85 s and rising, and whether the scope is really quiet

The scope IS declared quiet, and the mechanism is real, not a dead declaration:
`puzzle3d_command_scope_class("registerBrushMesh") → Puzzle3dScopeClass::Quiet`
(`✏️editor/🦀️.rs:2054`), `puzzle3d_scope(Quiet) = UiDirtyScope::None` (`:2026`), and this is computed
and installed as `ctx.ui_scope`'s starting value *before* the command arm runs, at dispatch time
(`:3025`: `let mut ui_scope = puzzle3d_scope(puzzle3d_command_scope_class(action));`), with `ui_scope`
handed to the arm as `&mut` (`:3028`). `register_brush_mesh` never touches `ctx.ui_scope`, so `None`
survives to the terminal `Emit` (`:3077`) on every exit — the shared-mesh fast path and the refusal
included. This supersedes wave-L's earlier, cruder fix (an inline `*ctx.ui_scope =
UiDirtyScope::None` the wave-L report describes adding at `register-brush-mesh/🦀️.rs:32`; the file at
that path today carries no such line — the later scope-class table replaced it). On the host side,
`applyHostEffects`'s `notify` branch (`🏛️ShellHost/🟦️.tsx:4535-4540`) is a cheap
`showTransientNoticeRef.current(...)` call, and the `SET_SESSION`/`readHistory` churn that wave-L
documented as costing "7 `readHistory` calls for the 7 boot mesh pages" (`🏛️ShellHost/🟦️.tsx:4843-4853`)
is fixed by comparing `nextViewState` against `baseSession.viewState` rather than a freshly-built
per-call projection — a `Notify`-only effect never rewrites `nextViewState`, so no new session, no
history re-fetch. **Both defects this audit could find in this exact path are already closed.**

That said, the direct Rust work for a `Digest`-refusal is O(1) — a couple of `HashMap` lookups plus one
`Mutex::try_lock` (`⏳️precompute/🦀️.rs:1607-1616`, `:971-978`) — nothing in it scales with call count,
so it cannot by itself explain 650 ms rising to 2 850 ms across seven sequential calls. This audit could
not identify, by static reading alone, what else runs on the same reactor turn or actor queue as each
`registerBrushMesh` dispatch and would explain the rise (candidates: `InteractiveJobClassification::Migrated`
continuation/step-budget machinery this action is registered under, `✏️editor/🦀️.rs:7899`; or queuing
delay from the "every other action queues" serialization the observation itself names, which would
inflate host-measured wall time without any single command's own compute growing). **This is the one
open question in this audit; it needs guest-time instrumentation around `register_brush_mesh` itself
versus total reactor-turn time, not more static reading.**

## 4. Once per activation, or every activation

Every activation, not just the first. Nothing on either side ever corrects the false "already
registered" entry: the JS `registeredPuzzle3dBrushMeshes` Map (`🛠️ShellHelpers/🟦️.tsx:2572`) is written
once (`🟦️.tsx:4360`, `registeredBrushMeshesRef.current.set(url, digest)`) when the page-form upload is
first *queued* — not when it is confirmed installed — and is never deleted in response to a fault.
Grepping the whole TS/TSX tree for the fault's wire code or `Puzzle3dMeshUploadFault` returns nothing:
the host never inspects a `Notify` message's content, so a `puzzle3d-register-mesh-digest` refusal is
indistinguishable, to the host, from any other plugin toast. Every later activation of this same window
(or any other window sharing the tab's module singleton) that mounts `BrushMeshRegistrar` for one of
these URLs takes the fast/digest path again and is refused again, forever, until a full page reload
clears the JS module state.

## 5. Existing native laws and the missing one

Coverage (`⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`, region `PagedBrushMeshUploads`):
`a_document_scale_mesh_uploads_in_pages_and_registers` (`:1379`),
`a_gapped_or_mismatched_page_run_is_refused` (`:1419`),
`an_uploaded_mesh_is_adopted_by_url_and_digest` (`:1442`),
`the_paged_upload_contract_matches_the_language_neutral_fixture` (`:1462`),
`an_abandoned_page_run_is_retired_and_a_live_one_survives` (`:1490`); plus
`✏️editor/🧪️tests/🔬️unit/🦀️.rs:3951` `one_brush_mesh_page_validates_inside_one_command_work_budget`.

`an_uploaded_mesh_is_adopted_by_url_and_digest` (`⏳️precompute/🧪️tests/🔬️unit/🦀️.rs:1442-1456`) proves a
*fresh* `Puzzle3dCollision`/session (i.e. a second open document) adopts by id+digest fine — because the
process-wide `brush_mesh_store()` it reads from is still populated **in the same process**. No law
anywhere — Rust or TypeScript — exercises the scenario this audit found: the client believes a mesh is
registered (its map says so) while the *store the guest would need to serve it from* is empty (simulating
a guest re-instantiation, or more directly, unit-testing that a `Digest` refusal on the host causes
`handleRegisterBrushMesh` to evict the stale map entry and re-drive a full page upload). That TS-side
recovery code does not exist (§4), so there is nothing yet to write a law against; the missing law is
this recovery contract itself, on both ends.

## Recommended fix

1. On the host, make the digest fast path self-healing: when a `registerBrushMesh` dispatch answers with
   a `Notify` whose message starts with `puzzle3d-register-mesh-digest:` (or, better, thread a typed
   fault code through instead of parsing text), delete that URL from `registeredPuzzle3dBrushMeshes`
   (`🛠️ShellHelpers/🟦️.tsx:2572`) and re-queue the full page run. Add the TS law this currently lacks.
2. Only mark a URL "registered" in the map once the corresponding page run's *last page* dispatch
   resolves without a fault (`🟦️.tsx:4360` currently marks it at enqueue time, before any bytes are
   confirmed accepted).
3. Instrument `register_brush_mesh` with a guest-side timing span independent of the reactor-turn/queue
   wait, to settle §3's open question before treating "0.65-2.85 s rising" as a command-handler cost.
