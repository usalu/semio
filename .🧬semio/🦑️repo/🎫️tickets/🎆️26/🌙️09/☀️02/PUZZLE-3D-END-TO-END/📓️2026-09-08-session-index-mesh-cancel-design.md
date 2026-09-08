# Puzzle 3D — Session Registry, Indexed Brush Collision, Chunked Mesh Upload, Fill Cancel (2026-09-08)

Read-only design, no code changed. Scope: the four follow-up items from
`📓️2026-09-08-performance-architecture-audit.md` §1/§2(a)/§2(h)/§3 rows 1/3/5/§4. Every claim below
carries a `file:line` verified directly with `sed -n`/`grep -n` at read time (2026-09-09). Paths
abbreviated after first use exactly as the source audit does: `EDITOR`, `PRECOMPUTE`, `FILL`,
`GEOMETRY`, `RETAINED`, `PLUGIN` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`).

**Concurrency note**: `GEOMETRY` and `FILL` are named in the task as being edited concurrently by
another agent (lifting `FIXED_OWNER_SLOTS = 32`). At read time both files were git-clean (no
uncommitted diff) with stable mtimes from 2026-09-08 evening (well before this read on
2026-09-09 00:11) — no in-progress edit was visible. Re-diff both files before implementing item 2,
since its design touches the exact same `CollisionSpatialIndex`/`FixedOwnerMap` types the capacity
fix is changing.

## TL;DR

1. **Session registry.** Neither `EngineHandles` (a stateless, content-addressed *compute* cache,
   `🧰️framework/…/⚙️engine/🦀️.rs:63-70`, explicitly narrowing to "wasm guest↔host boundary only")
   nor the TRANSIENT lane (small serializable scratch, reconstructed fresh every call exactly like
   `Puzzle3dPlayApp` itself — `LowpolyPlayApp` is a **zero-sized unit struct**, `✏️editor/🦀️.rs:1553`,
   contradicting its own module's stale doc comment) is a ready-made per-instance session facility.
   The framework *does* already carry the missing identity — `app_instance_id: u32` /
   `parent_document_id: String` — into every place `with_puzzle3d_app_for` is called, completely
   unused today: `ArtifactView::operation()`/`render_operation()` (`PLUGIN:6873/6883`) for the
   immediate `handle()` path, and as plain fields on `ArtifactOwnedToolJobRequest`
   (`PLUGIN:11972-11973`) for the retained-job `build_tool_job` path (`EDITOR:6605`). The clean fix
   is a `fill_envelope_registry()`-shaped process-global registry (fixed slot array, monotonic
   per-slot generation for ABA safety — the exact pattern already proven in `PRECOMPUTE:341-356`),
   keyed by `app_instance_id`, threaded in via one new default no-op hook on the shared
   `PuzzleCommandWork` trait (`RETAINED:38-40`) mirroring the existing, currently-unused-by-puzzle3d
   `bind_operation` hook (called once per job construction at `RETAINED:334`, on **both** fresh
   admission and worker-hop resume). Blast radius: puzzle3d-local except two additive fields on
   `RetainedPuzzleCommandPayload` (`RETAINED:114`, shared with puzzle 2d/5d, default-empty/no-op for
   them). fem3d already runs a near-identical pattern (`🏗️fem/…/🧵️session/🦀️.rs:3358`,
   `app_instance_id % ACTIVE_CAPACITY`) but keyed to `thread_local!` — wrong for puzzle3d's
   correctness-sensitive fill state (per `EDITOR:2160-2163`'s own doc comment, worker hops are
   routine here), fine for fem3d's soft render cache.
2. **Indexed brush collision.** `brush_collision_free_until`/`preview_collides`
   (`PRECOMPUTE:1189`/`1152`) rebuild a `Vec<PlacedCollisionEntry>` from a full `O(N)` fixture scan
   and linear-scan it per candidate — confirmed still true. `GEOMETRY`'s `CollisionSpatialIndex`
   (`GEOMETRY:821`, `begin_query`/`step_query` at `1200`/`1217`) already gives `FILL` its resumable
   broad-phase (`FILL:3745-3778`); item 1's session registry is exactly where a second, persistent
   `CollisionSpatialIndex` for the interactive brush lane belongs, kept in sync incrementally from
   the same mutation stream `FILL` already drives it with, instead of a full scene rebuild per check.
3. **Mesh upload.** No chunking exists on the Rust side today: `register_brush_mesh`
   (`🎮️commands/📋️register-brush-mesh/🦀️.rs`, 21 lines total) takes one complete
   `positions`/`indices` pair per call with zero partial-accumulation state — confirmed by reading
   the entire file. Real Nakagin capsule GLBs are ~127-129 KB each
   (`🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/💊️capsules/**/*.glb`, sized
   directly), far over the 8,192-byte `PUZZLE_COMMAND_RAW_BYTES` wire cap once JSON-encoded. The
   puzzle plugin requests exactly three capabilities today (`documents.write`, `ui.dialog`,
   `shell.clipboard` — `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs`) — no assets/mesh-read capability exists
   anywhere in the codebase (`grep 'CapabilityId("'` repo-wide returns 13 distinct ids, none
   asset-shaped). The cleaner fix reuses `EngineHandles`/`EngineCache` for exactly what its own doc
   comment says it's for (content-addressed host↔wasm bytes-to-bytes compute) instead of dismissing
   it — a host-side `"puzzle3d.mesh-decode"` `Engine` turns the whole upload problem into a
   content-addressed *read*, shared across every open document, not a per-command *write*.
4. **Fill cancel.** `cancel_fill_job` (`PRECOMPUTE:1862`) is real and unreachable — still zero
   callers outside its own file. The `energy` plugin already ships the exact convention to copy:
   a dedicated action id (`CANCEL_ACTION_ID = "cancel-energy-simulation"`,
   `⚡️simulation/🦀️.rs:16`, registered `71`) carrying the job's own identity args so a stale cancel
   can't kill a superseded run, feeding `Effect::CancelJob` (`🎠️kernel/🦀️.rs:630-632`) — also used
   by `🏗️fem`'s session reconcile (`🧵️session/🦀️.rs:3580`). Puzzle3d's fill tool panel
   (`🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, 52 lines) has no cancel affordance at all today. A large,
   literal-string-matching self-test (`interactivityPuzzleFillEnvelopeSelfTests`,
   `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts`) mutation-tests ~55
   exact source substrings against `PRECOMPUTE`/`FILL`/`GEOMETRY` — any wave touching those files
   (items 1, 2, or 4) must re-run it and must not reformat/rename the lines it string-matches.

---

## 1. Document-keyed session registry

### 1.1 What `fill_envelope_registry()` actually does (the pattern to copy)

`PRECOMPUTE:341-356`:

```rust
struct FillEnvelopeRegistry {
    slots: [Option<FillEnvelopeAuthority>; FILL_ENVELOPE_MAX_OPERATIONS],  // fixed array, N=4
    generations: [u64; FILL_ENVELOPE_MAX_OPERATIONS],                      // per-slot ABA counter
    next_slot: usize,
    aggregate_bytes: usize,                                                // process-wide memory census
}
fn fill_envelope_registry() -> &'static Mutex<FillEnvelopeRegistry> {
    static REGISTRY: OnceLock<Mutex<FillEnvelopeRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(FillEnvelopeRegistry::default()))
}
```

- **Slot allocation**: `begin_measurement` (`PRECOMPUTE:388-417`) round-robins over 4 candidate
  slots starting at `next_slot`, picks the first empty one whose generation hasn't saturated
  (`u64::MAX` retires a slot permanently — exhaustion is a real, handled terminal state, not a
  panic), bumps that slot's `generations[slot]`.
- **Token**: a 56-byte (`FILL_ENVELOPE_TOKEN_BYTES`, `PRECOMPUTE:48`) capability —
  `magic(8) + slot(1)+pad(7) + registry_generation(8) + job(8) + operation(8) + generation(8) +
  base_revision(8)` (`fill_envelope_token`, `PRECOMPUTE:359-368`) — round-trips through
  `Puzzle3dConfig.fill_checkpoint: Vec<u8>` and is the *only* state a resumed command carries
  forward. `decode_fill_envelope_request`/`fill_envelope_raw_request` (`PRECOMPUTE:127-143`)
  validate every field is non-zero before trusting it.
- **Ownership across workers**: `checked_out: Arc<AtomicBool>` per slot (`PRECOMPUTE:~415`,
  compare-exchanged in `take_closed`/`cancel_fill_job`) — a worker "signs out" a slot with a CAS,
  so two workers racing to resume the same fill job can't both drive it; a stale token (wrong
  `registry_generation`, `PRECOMPUTE:490` `authority_mut`) is rejected outright, which is exactly
  the ABA protection a `u32` reused `app_instance_id` would also need once documents can close and a
  future one reuse the same id.
- **Thread-safety**: everything is `try_lock()`, never blocking `lock()` — a contended registry
  fails the current step with `Blocked`/`None` rather than stalling a worker (`drive_fill_envelope`,
  `PRECOMPUTE:583-586`).
- **Memory census / retirement**: `finish_measurement` (`PRECOMPUTE:398-424`) bounds
  `aggregate_bytes` against `FILL_ENVELOPE_PROCESS_BYTES` (16 MB, `= 16KiB page × 256 pages × 4
  slots`) before admitting a request; `FillBuilderRetirementCursor`/`close_step` release items
  incrementally (`maximum_items`/`maximum_bytes` budgets) rather than all at once — this is almost
  certainly why `FIXED_OWNER_SLOTS = 32` exists at all (a bookkeeping-retirement batch size), which
  is the audit's own diagnosis of bottleneck (a)'s root cause and *not* this design's concern.

### 1.2 `EngineHandles` is not the answer — verified, not assumed

`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs:41-70`:

```rust
/// ⚙️ Host-registered pure compute kernel. Plugins never own registries — only handles.
pub trait Engine: Send + Sync + 'static {
    const ENGINE_ID: &'static str;
    fn compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault>;
}
pub struct EngineHandles { pub handles: Vec<EngineHandle> }
```

with the `EngineCache`'s own doc comment (`🦀️.rs:79-84`): *"Scope is narrowing to the wasm
guest↔host boundary only… It is no longer a general 'kernel cache': a kernel that caches derived
values outside an artifact facet is state living outside the store."* This is a **pure,
content-addressed `(engine_id, input) → output` LRU cache**, not a stateful per-instance session:
`derive`/`read` (`🦀️.rs:112-129`) take/return byte blobs keyed by `blake3(engine_id, input)`, with
no notion of "this document" at all — two different documents computing the same input hit the same
cache entry, which is the *opposite* of what a per-instance `geometry_cache`/registered-mesh session
needs. **However** — see §3 below — this exact shape is the right tool for the mesh-decode problem,
just not for item 1.

### 1.3 The identity is already threaded to every call site, unused

- **Retained-job admission** (the path for ~50 of puzzle3d's ~60 retained actions, including all
  fill/brush actions): `build_tool_job` (`EDITOR:6605`) receives
  `request: ArtifactOwnedToolJobRequest<EditorApp<Self>>`, whose plain fields include
  `pub app_instance_id: u32` and `pub parent_document_id: String` (`PLUGIN:11972-11973`). `EDITOR`
  reads `request.command`/`snapshot`/`config`/`interaction_state`/`interaction_hover`/`completion`/
  `controller_id`/`tool_id`/`payload_schema_id`/`operation` (confirmed by direct read,
  `EDITOR:6730-6746`) and **never reads `request.app_instance_id`/`request.parent_document_id`**.
  This request is constructed at `PLUGIN:20233` (`start_typed_command_operation`, the framework's
  sole call site for the "AppOwned" tool-proof kind puzzle3d uses via
  `bounded_first_step_tool_proofs!`), which is the *only* place `A::build_tool_job` is invoked
  (`grep -rn "build_tool_job("` finds no other framework call site) — meaning `build_tool_job` runs
  on every admission **and** is the sole place puzzle3d constructs its
  `Box<dyn PuzzleCommandWork<A>>` "work" objects (`EDITOR:6739-6786`), which per the audit's own
  finding are rebuilt fresh every time (no persistent job-object identity survives a worker hop —
  only the 112-byte `PuzzleCommandCheckpointState`, `RETAINED:132-138`, does, and it carries no
  `app_instance_id` field at all). So `request.app_instance_id` is available on **every** step of a
  multi-step retained command, not just the first.
- **Immediate (non-retained) dispatch**: `ArtifactEditor::handle` (`EDITOR:6713`,
  `impl ArtifactEditor for Puzzle3dPlayApp`) receives
  `doc: &ArtifactView<'_, Puzzle3dPlaySnapshot>`; `doc.operation()` (`PLUGIN:6873`) returns
  `Result<&AppOperationContext, Fault>` with `app_instance_id: u32`/`parent_document_id: String`
  (`PLUGIN:6819-6820`). Unused today (`grep -n "app_instance_id\|\.operation("` in `EDITOR` returns
  zero hits outside two unrelated `.operation()` calls on a different type at lines 6464/6556).
- **Render**: `fn render(body_key, doc, cfg, view_state)` (`EDITOR:6956` call site,
  trait signature at `PLUGIN:23976`, confirmed genuinely `&self`-free — see §1.5) —
  `doc.render_operation()` (`PLUGIN:6883`) returns `Option<AppRenderOperationContext>` with
  `app_instance_id: u32` (`PLUGIN:6831`), `base_revision`, `generation`, `canonical_base_revision`.
  Also unused. This is the *easy* case: no framework change needed, just read it.
- **`ViewModel.window_id`** (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4420`) is a *window-pane*
  instance id, not a document instance id — two windows can show the same document, so it is the
  wrong key for this session (per its own doc comment at `🦀️.rs:4404-4406`: "so that two window
  instances of the same kind… never share options" — a *window*-scoped concept, orthogonal to
  document identity).

### 1.4 Design: `puzzle3d_session_registry()`

Mirror `FillEnvelopeRegistry` exactly, in `PRECOMPUTE` (or a new sibling module next to it — same
crate, same blast radius either way):

```rust
const PUZZLE3D_SESSION_SLOTS: usize = 64;   // size against realistic concurrently-open documents,
                                             // not FIXED_OWNER_SLOTS's bookkeeping-batch meaning
struct Puzzle3dSessionEntry {
    app_instance_id: u32,
    generation: u64,                         // ABA guard — a closed-then-reopened instance id must not
                                              // adopt a stale entry
    geometry_cache: Option<(u64, String, String)>,   // fingerprint, instances_json, meshes_json
    document_tree_cache: Option<(u64, BuiltNode)>,
    fill_display_memo: Option<FillDisplayMemo>,
    brush_spatial_index: Option<CollisionSpatialIndex>,   // item 2
    registered_meshes: HashMap<String, CollisionBody>,    // supersedes per-call engine.meshes if item 3
                                                           // isn't adopted; redundant if it is (§3)
}
fn puzzle3d_session_registry() -> &'static Mutex<[Option<Puzzle3dSessionEntry>; PUZZLE3D_SESSION_SLOTS]> { ... }
```

Keyed by `app_instance_id % PUZZLE3D_SESSION_SLOTS` with linear-probe collision handling exactly
like `FillEnvelopeRegistry::begin_measurement`'s `candidates` array (`PRECOMPUTE:388`), same
`try_lock()`-never-blocks discipline, same per-slot generation guard against a stale/reused
`app_instance_id`.

**Identity capture, minimal blast radius**:
- `EDITOR:6605` `build_tool_job` — after constructing each `Box<dyn PuzzleCommandWork<...>>`, call
  a **new** default no-op hook mirroring the existing `bind_operation` pattern:
  ```rust
  // RETAINED.rs, alongside the existing default at line 40:
  fn bind_operation(&mut self, _operation: Operation) {}
  fn bind_instance(&mut self, _app_instance_id: u32, _parent_document_id: &str) {}   // NEW, default no-op
  ```
  and two new fields on `RetainedPuzzleCommandPayload<A>` (`RETAINED:114`):
  `pub app_instance_id: u32, pub parent_document_id: String` — populated at `EDITOR:6745` from
  `request.app_instance_id`/`request.parent_document_id`, which are already in scope there.
  `RetainedPuzzleCommandJob::from_payload` (`RETAINED:329-334`, the single constructor used by
  **both** `new`/fresh-admission and `from_wire`/`from_validated_wire_checkpoint`/resume) already
  calls `payload.work.bind_operation(operation)` at line 334 — add
  `payload.work.bind_instance(payload.app_instance_id, &payload.parent_document_id)` right beside
  it. This is additive, default-no-op for puzzle 2d/5d unless they opt in, and reaches every
  puzzle3d Work struct on both admission and every resume.
- `EDITOR:6713` `handle()` (immediate path) — read `doc.operation()?.app_instance_id` directly, no
  framework change.
- `EDITOR:6956` `render()` — read `doc.render_operation()?.app_instance_id`, no framework change,
  falling back to "no session" (current behavior) when `None` (render/test-only views per
  `PLUGIN:6828`'s own doc comment on why it's optional).
- `with_puzzle3d_app_for` (`EDITOR:2164-2169`) becomes `with_puzzle3d_app_for(app_instance_id:
  Option<u32>, config, f)`: looks up/creates the session slot when `Some`, falls back to today's
  fresh-`default()` behavior when `None` (never breaks a call site that genuinely has no identity,
  e.g. `#[cfg(test)] with_puzzle3d_app`).

**Invalidation**: reuse the existing `fixture_geometry_fingerprint`
(`main::fixture_geometry_fingerprint`, referenced `EDITOR:2211`) as the cache-validity key inside
the session entry exactly as `geometry_cache` already does — this part of the mechanism is *already
correct*, it just never gets to run twice on the same struct. No new invalidation logic needed
beyond what's already written; the bug is entirely "runs on a struct that dies every call," not "the
fingerprint check is wrong."

**Retirement / memory bound**: `build_document_store_disposer`/`build_config_store_disposer`
(`EDITOR:~6706`, implementing `ArtifactOwnedDisposer<T>` — `PLUGIN:12031-12034`, `close_step`/
`terminal_is_empty`, incremental/budgeted exactly like `FillBuilderRetirementCursor`) are puzzle3d's
existing disposal hooks for the document/config stores; whether `close_step`'s `owner: &mut T`
exposes `app_instance_id` needs one follow-up read before wiring retirement through it. Failing
that, the fixed 64-slot array with generation-guarded reuse (same shape as
`FillEnvelopeRegistry.generations`) bounds memory without needing an explicit close hook at all —
an evicted/overwritten slot just costs one cold cache-miss for whichever *other* instance
collided into it, which is a performance regression, not a correctness bug (same class of tradeoff
`FillEnvelopeRegistry`'s own 4-slot round-robin already accepts).

**Cross-worker resume correctness**: unlike `fill_checkpoint`, this session cache is *not* itself
part of any correctness-relevant checkpoint — a cache miss (wrong/evicted slot, or a resume that
lands with no session, i.e. `with_puzzle3d_app_for(None, …)`) just costs a full rebuild, identical
to today's behavior on every single call. This session registry can never make correctness *worse*
than the current always-cold baseline; it only ever helps.

### 1.5 `LowpolyScratch`/`ArtifactEditor::handle` — verified, not assumed

- `pub struct LowpolyPlayApp;` (`✏️s/🔌️plugins/💠️lowpoly/…/✏️editor/🦀️.rs:1553`) — a **zero-sized
  unit struct**. `LowpolyScratch` is rebuilt every call from
  `LowpolyScratch::from_transient(&LowpolyTransient::default(), selection)` (`🦀️.rs:1748`, inside
  `handle`) or `LowpolyScratch::default()` (`🦀️.rs:1753`, inside `render`) — **not** held behind a
  `RefCell` on a long-lived app instance. The `🖌️session/🦀️.rs:1-6` module doc comment claiming
  `"render(&self, ..)"`/`"handle(&self, ..)"` is stale/aspirational, contradicted by the actual unit
  struct and by the trait itself.
- `ArtifactEditor::handle`/`render` are confirmed **still** associated functions with no `&self`
  today: `async fn handle(command: &Self::Command, doc: &ArtifactView<'_, …>, …, engines:
  &EngineHandles) -> …` (`PLUGIN:9832-9844`, the async host-facing trait) and the sync mirror at
  `PLUGIN:23959`/`23976` (`ArtifactEditor`) — neither takes `self`. So the doc comment at
  `EDITOR:2160-2163` ("`ArtifactApp` methods are associated fns (no `&self`)") is **still accurate**
  today, and lowpoly does not contradict it — lowpoly's actual survival mechanism for
  `LowpolyScratch` is reconstruction from the framework's **TRANSIENT lane**
  (`TransientView<'_, T>`, `PLUGIN:7568-7571`, "the typed replacement for plugin `thread_local!`
  scratch state"), which is small, serializable, mutation-driven state — a fundamentally different,
  *lighter* mechanism than puzzle3d's megabyte-scale JSON geometry caches / raw mesh
  `HashMap<String, CollisionBody>`, and one that (like everything else) is unavailable inside the
  retained-job `step()` signature (`RETAINED:22-28`, fixed to `Command`/`Snapshot`/`Config`/
  `InteractionState`/`InteractionHoverState`) — so it would not solve the retained-command majority
  of puzzle3d's calls anyway.
- Closest **real, running precedent** for a process-wide per-instance slot array:
  `🏗️fem/…/✏️editor/🧵️session/🦀️.rs:3358`, `thread_local! { static MOUNTED: RefCell<Registry> = … }`,
  `const ACTIVE_CAPACITY: usize = 16` (`🦀️.rs:26`), slot = `app_instance_id % ACTIVE_CAPACITY`
  (`🦀️.rs:3345` etc.). This validates the "bounded array keyed by `app_instance_id % N`" idiom as
  established in this codebase — but fem3d backs it with `thread_local!`, which is *wrong* for
  puzzle3d specifically because `EDITOR:2160-2163`'s own doc comment says worker hops are routine
  here (a thread-local session would almost never hit); fem3d's use case is a soft, best-effort
  render cache where a thread-local miss just costs one rebuild on whichever worker happens to
  render next, which is an acceptable tradeoff fem3d apparently made deliberately. Puzzle3d's
  registry must be **process-global** (`OnceLock<Mutex<[...; N]>>`, not `thread_local!`) for the
  same reason `fill_envelope_registry()` already is.

---

## 2. Indexed brush-suggestion collision

### 2.1 Confirmed still O(N × C) per target, O(N²×C) per full requeue

- `re_enqueue_brush_targets` (`PRECOMPUTE:925-930`) resets cursors to 0;
  `prepare_one_brush_target` (`PRECOMPUTE:933-951`) walks `scene.fixture.objects[cursor]` one
  object/vortex at a time, queuing any vortex whose full id isn't already cached — this part is
  bounded and incremental (one object per call), not itself the bottleneck.
- `precompute_step_lane(PrecomputeLane::Brush, budget)` (`PRECOMPUTE:1310`) pops one queued vortex
  id per tick and calls `compute_brush_cache_entry_partial` → `brush_collision_free_until`
  (`PRECOMPUTE:1189-1246`), which on **every single popped entry**:
  1. rebuilds `placed: Vec<PlacedCollisionEntry>` via
     `scene.fixture.objects.iter().filter(...).filter_map(...)` — a full `O(N)` scan
     (`PRECOMPUTE:1200-1210`);
  2. for each of `candidates` (`C`), calls `Self::preview_collides(&self.meshes, &preview, &placed,
     …)` (`PRECOMPUTE:1152-1187`), which does `for entry in placed` — a **plain linear scan**, only
     AABB-pruned inline, no spatial acceleration at all (`PRECOMPUTE:1169`).
  So one popped vortex costs `O(N × C)`; the queue holds up to `V ∝ N` vortices, so a full
  requeue-and-drain costs `O(N² × C)`, exactly as the audit found — re-verified directly, unchanged.
- `FILL` already solves the identical broad-phase problem with `CollisionSpatialIndex`:
  `query_broad_phase` (`FILL:3745-3778`) calls `self.spatial_index.begin_query(owner, bounds)` /
  `.step_query(query, owner)` (`GEOMETRY:1200`/`1217`) — a resumable, cursor-based 8-unit-cell
  spatial hash — then `test_collision` (`FILL:3780+`) narrow-phases only the query's own candidate
  page with `CollisionOverlapState::new(512, 8, self.overlap_budget)` (`FILL:3813` area,
  `GEOMETRY:1503`/`1524`).

### 2.2 Design

Give the session entry from item 1 an owned, persistent `CollisionSpatialIndex` for the *brush*
lane (separate instance from `FILL`'s own `spatial_index: CollisionSpatialIndex` at `FILL:1069` —
different lifetime, different owner, same type). Concretely:

- **Which functions change**: `brush_collision_free_until` (`PRECOMPUTE:1189`) stops building
  `placed: Vec<PlacedCollisionEntry>` from a full fixture scan; instead it calls
  `session.brush_spatial_index.begin_query(owner, preview_bounds)` /`.step_query(...)` to get a
  bounded candidate page, then narrow-phases only that page with the existing
  `CollisionOverlapState` — unchanged narrow-phase code, only the broad-phase source changes.
  `preview_collides` (`PRECOMPUTE:1152`) becomes "narrow-phase against one supplied candidate page"
  instead of "narrow-phase against every placed object."
- **Keeping the index in sync — event-driven, not full-rebuild**: today `set_scene`
  (`PRECOMPUTE:1050`) diffs against `self.scene_json` (byte-equality) and calls `rebuild_queue()`
  (`PRECOMPUTE:963`) on any real change, which wipes and re-derives *everything* from the new
  fixture. Instead: since every mutation reaching `PRECOMPUTE` arrives through the already
  event-sourced `Puzzle3dMutation` stream (CQRS, per CLAUDE.md's own "event-driven over
  state-driven"), apply each emitted object add/move/remove mutation directly to the persistent
  index via `CollisionSpatialIndex::begin_replacement`/`step_replacement` (`GEOMETRY`, same API
  `FILL`'s own `AcceptPhase::InstallLookup`/`CollisionSpatialIndex::step_replacement` already
  drives, per the audit's citation of `FILL:3975-3981`/`GEOMETRY:1013`) instead of
  `re_enqueue_brush_targets`'s full re-scan. `re_enqueue_brush_targets` remains as-is for populating
  the *candidate-check queue* (which vortices need a fresh suggestion) — that part is genuinely
  cheap and incremental already; only the *collision* broad-phase inside each popped check needs
  the index.
- **Progress/cancellation surface**: `precompute_step_lane` already time-boxes each tick to
  `PUZZLE3D_PRECOMPUTE_STEP_BUDGET_US = 2_000` (`PRECOMPUTE:858-859`) and the existing
  `BrushCollisionFreeResult{ unknown_pending, resume_candidate_index }` resumption fields
  (`PRECOMPUTE:1189` return type) already carry a natural progress cursor — no new progress field
  needed. Cancellation: `Puzzle3dCollision` has no `CancelToken` at all today (only `Puzzle3dCollision.fill_cancel`
  exists, fill-specific) — the audit's §4 finding that "brush candidate preparation… expose[s] no
  cancellation surface" is confirmed independently; if a user-facing "stop suggesting" affordance is
  wanted later, `re_enqueue_brush_targets`/the queue itself is the natural cancel point (drop the
  queue, same shape as `fill_cancel.cancel_now()` clearing `self.fill_steps_remaining`), but this is
  out of the current task's scope (item 4 only asks for *fill* cancel).

---

## 3. Mesh upload

### 3.1 Real wire shape today

- `register_brush_mesh` (`✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs`, full 21-line file
  read): takes `url`/`positions`/`indices` from one command's `args`, checks
  `MAX_LEAF_BYTES = 4*1024` (url), `MAX_POSITIONS`/`MAX_INDICES = 196_608` (element counts), then
  calls `ctx.app.precompute.borrow_mut().register_mesh(url, &positions, &indices)` directly. **No
  partial-accumulation state, no chunk cursor, no multi-call assembly logic anywhere in this file or
  in `install_collision_mesh`/`register_mesh` (`PRECOMPUTE:~1098-1116`)** — confirmed by reading both
  in full. Today's protocol is genuinely one-shot: whatever arrives in one call *is* the mesh.
- Real GLB sizes, measured directly (`ls -la`) against the source asset tree the Nakagin fixture's
  mesh URLs resolve to (`🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/…`, matched
  against `NAKAGIN_DSL`'s own `"/mesh/…"` string literals):
  - Visual capsule meshes (`🧊️capsule_{p,q,s,z,J,L,slash,backslash}.glb`): **127,248 – 128,760
    bytes** each (8 distinct meshes referenced by Nakagin's fixture).
  - Collider variants (`💥️capsule_*_collider.glb`): **2,136 bytes** each — three orders of
    magnitude smaller, i.e. a coarse collision proxy already exists as a *separate* asset family,
    not something puzzle3d derives itself.
  - (Storybook-built copies of the same "metabolism" set at
    `storybook-static/asset/🌱️metabolism/🎨️representation/` show ellipsoid/trapezoid capsule
    variants at a uniform 771,728 bytes each — a different, higher-poly export of the same family;
    not what Nakagin's DSL references, but useful as an upper bound for "a bigger real mesh.")
  - A 128 KB binary GLB, once decoded to `positions: Vec<f32>`/`indices: Vec<u32>` and re-encoded as
    JSON numbers (the wire format `register_brush_mesh` actually parses,
    `args.get("positions").as_array()`), inflates well past the 8,192-byte
    `PUZZLE_COMMAND_RAW_BYTES` cap for any mesh with more than a few hundred vertices — confirms the
    audit's finding that real GLB geometry cannot reach the plugin as a single `registerBrushMesh`
    call under the current cap, for **any** of Nakagin's 8 real capsule meshes.
- Mesh catalog: `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json` is a *thin pointer table*
  (url → collection-relative path) referencing the "metabolism" collection's own
  `📇️catalog.json` — mesh URLs are resolved to on-disk paths entirely host-side; the plugin never
  sees a filesystem path, only the `"/mesh/…"` URL string embedded in the fixture DSL
  (`grep -o '"/mesh/[^"]*"' NAKAGIN_DSL` → 12 distinct URLs).

### 3.2 Capability check — verified, not assumed

`✏️s/🔌️plugins/🧩️puzzle/🦀️.rs` (`Plugin::builder("puzzle")…`, full file read): exactly three
`.requests(CapabilityRequest{...})` calls — `documents.write`, `ui.dialog`, `shell.clipboard`. A
repo-wide `grep -rn 'CapabilityId("'` finds 13 distinct capability ids in use anywhere
(`documents.read`, `documents.write`, two `fs.read:`/`fs.write:` scoped-path examples,
`http:example.com`, `jobs.spawn`, one intentionally-invalid test id, `shell.clipboard`,
`shell.navigate`, `shell.observe`, `shell.raw`, `storage.read`, `storage.write`, `ui.dialog`) —
**no `assets.*`/`mesh.*` capability exists anywhere in the codebase today.** The closest analogs are
`storage.read`/`fs.read:<path>`. A "plugin reads `/mesh/*.glb` directly" scheme would need a
genuinely new capability (or a new WIT import class alongside `engine-derive`/`engine-read`, not a
filesystem grant, since the plugin runs sandboxed wasm under `ExecutionMode::Isolated`
(`✏️s/🔌️plugins/🧩️puzzle/🦀️.rs`, `.execution(ExecutionMode::Isolated)`) and mesh URLs resolve to
paths only the host's catalog knows).

Examples load `.glb`/`.pack.semio` today entirely at build time, not at runtime: the Nakagin example
(`📚️examples/🏗️nakagin-capsule-tower/🦀️.rs`) uses `include_str!`/`include_bytes!` to embed
`DSL_TEXT`/`OP_TEXT`/`PACK_BYTES`/`SPR_BYTES` directly into the compiled plugin — the fixture's mesh
URL *strings* are embedded this way, but no GLB bytes are — confirming there is genuinely no
existing runtime asset-fetch path for this plugin to model item 3(B) on; it would be new.

### 3.3 Two designs

**(A) Chunked, resumable, progress-reporting client push** — extends the existing wire without new
host capabilities:
- Turn `registerBrushMesh` into a multi-step retained command (it already IS one of
  `PUZZLE3D_RETAINED_TOOL_IDS`, dispatched via `Puzzle3dPrecomputeCommandWork`, `EDITOR:6759-6765`
  area — currently single-shot inside that work type) carrying
  `{url, chunk_index: u32, chunk_count: u32, byte_offset: u32, positions_chunk: Vec<f32>,
  indices_chunk: Vec<u32>}`, each chunk sized to fit `PUZZLE_COMMAND_RAW_BYTES = 8_192` once
  JSON-encoded (a 128 KB capsule mesh needs on the order of dozens of chunks at this cap).
- Partial-accumulation state (`Vec<f32>`/`Vec<u32>` being assembled) is exactly the kind of
  per-instance, cross-worker-resumable state item 1's session registry exists for — key by
  `(app_instance_id, mesh_url, upload_generation)`, assemble into a real `CollisionBody` only once
  `chunk_index + 1 == chunk_count`, discard the partial buffer on any generation mismatch (same ABA
  pattern as `fill_envelope_registry`).
- Progress: `{received_chunks, total_chunks}` returned in the command's `Emit` output, mirroring
  `fill_progress_summary()`'s `{count, applied_count, max_count, done}` shape
  (`FillProgressSummary`, `PRECOMPUTE:1420`/`1609`).
- Cancellation: a per-upload `CancelToken` in the same session slot, released the same way
  `fill_cancel` is.

**(B) Host-resolved mesh-by-URL (recommended)** — reuses `EngineHandles`/`EngineCache` for exactly
its documented purpose instead of inventing a parallel upload protocol:
- The retained command shrinks to `{url}` — comfortably under 8,192 bytes for any mesh, forever,
  regardless of geometry complexity.
- Host registers a `"puzzle3d.mesh-decode"` `Engine` (`EngineCache::register`, `⚙️engine/🦀️.rs:100`)
  whose `compute(&self, input: &[u8]) -> Result<Vec<u8>, EngineFault>` takes the mesh URL bytes,
  resolves it through the *same* catalog mechanism the React host already uses today
  (`🥽️mesh/📇️catalog.json` → collection catalog → on-disk `.glb` path), decodes the GLB, and
  returns encoded `positions`/`indices`.
- The plugin calls `engines.derive("puzzle3d.mesh-decode", url_bytes)` → `EngineHandle`, then
  `engines.read(handle)` (both already-existing `EngineCache` methods, `⚙️engine/🦀️.rs:112-129`) to
  get the decoded geometry — content-addressed by `blake3(engine_id, url)`, so the **same mesh URL
  decodes once, host-wide, across every open document/instance**, not once per document the way
  item 1's session registry or design (A) would still require. This directly and more thoroughly
  resolves audit bottleneck (h) ("registered meshes not persisted") — no re-upload ever needed
  again, for any document.
- This is squarely inside what `EngineCache`'s own doc comment says it's for ("Scope is narrowing to
  the wasm guest↔host boundary only… where byte serialization is unavoidable regardless of
  doctrine") — a real GLB decode is exactly a pure, deterministic, potentially-expensive
  bytes-to-bytes compute, the textbook `Engine` use case.
- Cost: needs the WIT `engine-derive`/`engine-read` imports "threaded through exchange," per
  `PLUGIN:9835`'s own doc comment ("`engines` is the host-owned `EngineHandles` bag (empty until
  WIT engine-derive/read is threaded through exchange)") — i.e. this plumbing is documented as
  **not yet live** end-to-end; needs a framework-side check on how close `engine-derive`/`engine-read`
  are to real wasm-boundary wiring before committing to (B) over (A). If that plumbing isn't ready
  this wave, (A) is the fallback that ships entirely within puzzle3d's existing retained-command
  machinery.

---

## 4. User-facing fill cancel

### 4.1 `cancel_fill_job` — real, unreachable

`PRECOMPUTE:1862`, `pub fn cancel_fill_job(&mut self) -> bool`: looks up `self.fill_job`, locks
`fill_envelope_registry()`, calls `authority.cancel.cancel_now()` on the real `CancelToken`. A
repo-wide `grep -rn "cancel_fill_job"` returns exactly its own definition — still zero callers,
confirmed independently of the audit.

### 4.2 The established convention — `energy`'s simulation cancel

`✏️s/🔌️plugins/🔋️energy/…/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`:
- `pub const CANCEL_ACTION_ID: &str = "cancel-energy-simulation";` (line 16), alongside
  `START_ACTION_ID`/`RETRY_ACTION_ID`/`DISCARD_ACTION_ID`/`ADOPT_ACTION_ID` — a full lifecycle
  action family, not a single ad hoc button.
- Registered as `action(CANCEL_ACTION_ID, "Cancel simulation", "Simulation abbrechen",
  request_identity_args())` (line 71).
- `request_identity_args()` (lines 47-54) — **every non-start action carries
  `request`/`operation`/`generation`/`configDigest` identity args**, "so a cancel/retry/discard/
  adopt can never be applied to a run other than the one the user is looking at" (doc comment,
  line 46) — directly analogous to `fill_envelope_registry`'s own `registry_generation` ABA guard;
  puzzle3d's cancel action should carry the fill job's own `(job, operation, generation)` triple
  (already present in `FillJobRequest`, `PRECOMPUTE:61-65`) for the same reason.
- A real keyboard shortcut is wired too: `"mod+period"` → `"energy-keyboard-cancel"`,
  labeled "cancel the running simulation"/"laufende Simulation abbrechen" (line 243).
- Feeds `Effect::CancelJob { job }` (`🎠️kernel/🦀️.rs:630-632`) — the same effect `🏗️fem`'s own
  session reconcile already emits on supersession (`🧵️session/🦀️.rs:3580`,
  `effects.push(Effect::CancelJob { job: previous.identity.job })`) and `🔋️energy`'s own
  `🧵️simulation-session/🦀️.rs:2163` does too — three independent, live precedents for the same
  effect, none of them puzzle-specific, confirming this is the framework's real convention rather
  than a one-off.

### 4.3 Design for puzzle3d

- New action id following puzzle3d's own camelCase convention (matching `fillBuildTick`,
  `setFillCount`, `engagementAbort` — not energy's kebab-case): `"cancelFillBuild"`, added to
  `PUZZLE3D_RETAINED_TOOL_IDS` (`EDITOR:2526-2532`) and dispatched through
  `Puzzle3dPrecomputeCommandWork` (the same work type `fillBuildTick`/`registerBrushMesh` already
  use, `EDITOR:6759-6765` area) since it needs the same precompute-session access.
- Args: the fill job's own `job`/`operation`/`generation` (from `FillJobRequest`,
  `PRECOMPUTE:61-65`) — reject (no-op) a cancel whose triple doesn't match the session's current
  `fill_job`, exactly mirroring energy's `request_identity_args()` guard.
- Reducer: call `precompute.cancel_fill_job()` (`PRECOMPUTE:1862`) directly — no new engine-side
  code needed, only the dispatch wiring.
- New terminology label: `✏️editor/🗣️terminology/🦀️.rs` has no `cancel`/`abort`-shaped term today
  (`grep -n "cancel\|abort"` in that file returns nothing); add one following the file's exact macro
  shape (line 25 as the template):
  ```rust
  cancel_fill: native_en "Cancel fill", native_de "Füllen abbrechen", reuse_en "Cancel fill", reuse_de "Füllen abbrechen";
  ```
- UI: the fill tool panel (`🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`, 52 lines, read in full — currently
  only `count_measure`/`measures` driving a slider off `fill_progress_summary()`) needs a button
  node bound to `"cancelFillBuild"`, shown only while `!progress.done` — same conditional
  `energy`'s own window uses for its cancel action's visibility (gated on
  `EnergySimulationStatus`/`EnergyJobStage`, `⚡️simulation/🦀️.rs:6-7` imports).

### 4.4 Literal-preservation constraint — verified, not assumed

`✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts` (13,136 bytes, read in
full) runs `interactivityPuzzleFillEnvelopeFailures` from the root `📜️script.ts` against **~55**
deliberately-mutated copies of `PRECOMPUTE`/`FILL`/`GEOMETRY`/an "action" file's *exact source text*
(`.replace("<exact substring>", "<mutated substring>")`), asserting every mutation is caught and the
unmutated baseline passes clean. Exact substrings this self-test string-matches against
`PRECOMPUTE` alone include (non-exhaustive, the ones nearest this design's blast radius):
- `"FILL_ENVELOPE_PAGE_BYTES: usize = 16 * 1024"`, `"slots: [Option<FillEnvelopeAuthority>;
  FILL_ENVELOPE_MAX_OPERATIONS]"` (registry shape — item 1 must not reformat these lines even
  though it isn't touching `FillEnvelopeRegistry` itself).
- `"let fill = self.engine.fill.take()?"`, `"self.engine.fill = None;"`,
  `"request.job != self.context_job"`, `return Err("fill worker envelope owner is stale");`,
  `"*current != request && live(current)"`, `"self.fill_job = Some(request.clone());"` — all inside
  `restore_persisted_fill`/`bind`/`authority_mut`, which item 4's cancel wiring reads adjacent to
  but does not itself need to modify (`cancel_fill_job` itself does not appear in this particular
  mutation list, but the file it lives in is scanned wholesale — any incidental reformatting of
  these lines while adding cancel wiring nearby would break the self-test).
- `"self.fill_generation.checked_add(1)?"`, `"self.fill_revision.checked_add(1)?"`,
  `"self.request.generation == 0"` — identity/exhaustion guards item 4's cancel-identity-args design
  (§4.3) deliberately mirrors; do not weaken these to `wrapping_add`/`saturating_*` equivalents.
- **Action item for whoever implements**: re-run `interactivityPuzzleFillEnvelopeSelfTests` (and the
  sibling `interactivityPuzzleFillP4eSelfTests`/`interactivityPuzzleFillPreviewJsonSelfTests`,
  same directory tree, not read line-by-line in this pass) after *any* edit to `PRECOMPUTE`/`FILL`/
  `GEOMETRY` from items 1, 2, or 4 — before assuming the change is safe to land, since a passing
  `cargo test` does not exercise this TypeScript-side literal audit.

---

## Files referenced

- `EDITOR` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `PRECOMPUTE` = `…/✏️editor/⏳️precompute/🦀️.rs`
- `FILL` = `…/✏️editor/⏳️precompute/🪣️fill/🦀️.rs`
- `GEOMETRY` = `…/✏️editor/⏳️precompute/📐️geometry/🦀️.rs`
- `RETAINED` = `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs`
- `REG_MESH` = `…/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs`
- `FILLTOOL` = `…/✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs`
- `TERM` = `…/✏️editor/🗣️terminology/🦀️.rs`
- `PLUGIN` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `ENGINE` = `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️.rs`
- `MANIFEST` = `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` (`ViewModel`)
- `KERNEL` = `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` (`Effect::CancelJob`)
- `LOWPOLY` = `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  and `…/✏️editor/🖌️session/🦀️.rs`
- `FEM3D_SESSION` = `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs`
- `ENERGY_SIM_WINDOW` = `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️标准/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/⚡️simulation/🦀️.rs`
  (canonical path uses `🏅️standards`, corrected here)
- `MESH_CATALOG` = `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/📇️catalog.json`,
  `🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🎨️representation/📇️catalog.json`
- `NAKAGIN_DSL` = `…/📚️examples/🏗️nakagin-capsule-tower/🖼️assets/🏢️tower/🗣️.dsl.semio`
- `INTERACTIVITY_TEST` = `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/🔬️interactivity-puzzle-fill-envelope/🟦️.ts`
