# Wave H — brush-mesh re-announce storm on window activation (2026-09-10)

Follow-up to `📓️2026-09-10-brush-mesh-upload-audit.md`. This file is written incrementally while the
fix lands; the closing sections carry the exact commands and counts.

## 1. Root cause (restated, with the architectural verdict)

Two authorities disagreed about one fact and neither could correct the other:

- Host authority: `registeredPuzzle3dBrushMeshes = new Map<string, string>()` — a **page-lifetime**
  module singleton (`🧰️framework/…/🛠️ShellHelpers/🟦️.tsx:2571`). Written at *enqueue* time
  (`🌐️World3dHost/🟦️.tsx:4367`, before a single byte was accepted), never invalidated by anything
  short of a full browser reload.
- Guest authority: `brush_mesh_store()` — a `static OnceLock<Mutex<…>>` scoped to **one wasm
  instantiation** (`✏️editor/⏳️precompute/🦀️.rs:970`). A restored actor (`shard N lost, restoring
  actors`) starts with an empty store.

The host's belief therefore outlives the fact it describes. On every later activation the seven
`BrushMeshRegistrar` mounts each took the id-only fast path, `adopt_shared_mesh` missed, and the arm
answered `Effect::Notify { "puzzle3d-register-mesh-digest: <url>" }` — a notice no host code has ever
read (grepping the whole TS tree for the code returns nothing), so nothing re-paged and the brush
utility stayed without collision geometry, forever, per tab.

The architectural fix is not "clear the map somewhere". It is that **the host may only cache a claim
about the guest for as long as the guest instance that made it lives, and a guest that refuses a claim
must say so in a form the host can act on.** Both halves are implemented below.

## 2. The two mechanisms

### 2.1 Lifetime scoping — the guest publishes a monotone residency counter

`Puzzle3dBrushMeshStore` gains `installs: u64`, incremented once per successful
`derive_brush_mesh`. It is process-wide and monotone **within one instantiation**, and a fresh
instantiation starts at zero. `world_interaction_json` publishes it as `meshResidency`.

The host folds it into the registry (`Puzzle3dBrushMeshRegistry.observeResidency`): a value **lower
than the highest one this page ever saw** proves the guest was re-instantiated, so every cached entry
is dropped and every mounted registrar is re-driven. Nothing else can make the counter fall.

This is a nonce-free signal: no clock, no random, no per-instance id plumbing, and it is correct across
a checkpoint/restore because the store is *not* part of the checkpoint — that is exactly the fact it
reports.

### 2.2 Refusal recovery — the refusal is a request for bytes, not a notice

The id-only arm's `adopt_shared_mesh` miss no longer emits `Effect::Notify`. It records the url in the
precompute session's bounded `mesh_reupload_requests` set and widens `*ctx.ui_scope` to
`puzzle3d_viewport_scope()` (the world body alone — the lane that carries `interactionJson`), so the
request reaches the host on the very next refresh instead of at some unrelated later paint. The url is
dropped from the set the moment its geometry installs.

`world_interaction_json` publishes the set as `meshReuploadUrls`. `World3dHost` claims each request
once (`Puzzle3dBrushMeshRegistry.claimReupload`, guarded by the residency the request was published
under, so a stale republish of the same scene cannot re-drive an upload that already ran), forgets the
url and bumps a per-url `revision` that `BrushMeshRegistrar` carries in its effect deps — which
re-invokes `onRegister` with the already-loaded GLB and takes the page path.

### 2.3 Confirm on the last page, not on enqueue

`registeredPuzzle3dBrushMeshes.set(url, digest)` used to run when a run was *queued*. It now runs when
the run's **last page** is dispatched, with an in-flight run map (`brushMeshRunsRef`) preventing a
second registrar mount from queueing the same run twice while the first drains.

## 3. Files changed

Host (TypeScript):

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` —
  the page-lifetime `registeredPuzzle3dBrushMeshes = new Map<string, string>()` singleton is **gone**
  (a repo-wide grep for the name and for `registeredBrushMeshesRef` now returns nothing). In its place
  `Puzzle3dBrushMeshRegistry` (`:2585`) with `observeResidency` / `holds` / `confirm` / `forget` /
  `claimReupload` / `clear` / `residency` / `size`, and the page's one instance
  `puzzle3dBrushMeshRegistry` (`:2644`).
- `🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx` — `WorldInteractionRecord.meshResidency` /
  `.meshReuploadUrls` (`:268`, `:270`); `BrushMeshRegistrar` takes a `revision` prop and carries it in
  its effect deps (`:1721-1727`), which is the ONLY thing that can re-announce an already-`useLoader`-
  cached GLB; `handleRegisterBrushMesh` asks the registry instead of a raw map, keeps an in-flight
  `brushMeshRunsRef` so a second mount cannot queue the same run twice, and `confirm`s on the run's LAST
  page rather than at enqueue (`:4372-4402`); the residency/re-upload fold-in effect (`:4420-4432`); the
  registrar mount passes `revision={generation + urls[url]}` (`:5400`).
- `🧰️framework/…/📦️packages/🟦️typescript/🎯️targets/⚛️react/🟦️.tsx` — `Puzzle3dBrushMeshRegistry` and
  `puzzle3dBrushMeshRegistry` re-exported (`:755`, `:803`) so the contract suite can construct a private
  registry rather than mutating the page singleton.
- `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts` — the two new laws (`:8694`, `:8720`) plus the
  id/digest law rewritten against the registry (`:8674`).

Guest (Rust, `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`):

- `✏️editor/⏳️precompute/🦀️.rs` — `BRUSH_MESH_INSTALLS: AtomicU64` (`:1009`) and
  `shared_brush_mesh_installs()` (`:1021`), bumped in `derive_brush_mesh` (`:1033`) and deliberately
  kept *outside* the `Mutex` so a contended read can never answer `0` and look like a fresh
  instantiation; `Puzzle3dCollision::mesh_reupload_requests: Vec<String>` (`:1283`) with
  `request_mesh_reupload` (`:1696`, bounded by `FILL_WORKER_MAX_URL_BYTES` / `FILL_WORKER_MAX_MESHES`,
  idempotent, sorted so an unchanged set hashes to an unchanged lane) and `mesh_reupload_requests()`
  (`:1709`); the request retires inside `place_collision_mesh` the moment real (non-fallback) geometry
  lands (`:1729`); session delegation at `:2483`.
- `✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs` — the id-only `adopt_shared_mesh` miss no longer
  pushes `Effect::Notify`; it calls the new `request_reupload`, which records the identity and widens
  `*ctx.ui_scope` to `puzzle3d_viewport_scope()`. A refused *page* still faults with a notice — that one
  IS a message for a human.
- `✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` — the world body publishes `"meshResidency"` and
  `"meshReuploadUrls"` on `interactionJson` (`:389-390`) with the reasoning at `:376-378`.
- `✏️editor/🦀️.rs` — `registerBrushMesh` **removed** from `puzzle3d_action_uses_precompute` (`:3220`),
  with the cost account in the docstring above it (`:3208-3219`). This is §4's fix.
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs` — the native law
  `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes` (`:3951`).

## 4. The 0.65 → 2.85 s question, answered

Two separate facts were folded into one number by the browser observation, and only one of them is a
cost:

**(a) The rise is queueing, not growth.** The eight `handleAction`s measured at 181 → 2 579 ms on
2026-09-09 (`📓️2026-09-09-runtime-verification.md` §21:05 / "Shell quirks" (2)) are serialized on the
one actor. The seven announcements ramp by a near-constant ≈370 ms per position (650, ~1 020, …, 2 850),
which is the signature of *k × T* completion times on a FIFO queue, not of any single call getting more
expensive. So the thing to explain was never "2.85 s"; it was **one** call costing ~370 ms of guest time
for an arm the audit correctly measured as O(1) — two `HashMap` lookups and a `try_lock`.

**(b) The ~370 ms was the action prologue's precompute sync, and it was work the same command threw
away.** `git show 9b605a4550:✏️editor/🦀️.rs` proves `"registerBrushMesh"` was a member of
`puzzle3d_action_uses_precompute` before this wave (that list at `:3023-3041` of the old blob). Membership
means every dispatch owed `Puzzle3dActionPrologue::sync_step`'s three stages before the arm ran:

1. `Puzzle3dPrologueSyncStage::Meshes` — `seed_one_precompute_mesh_fallback` once per mesh identity the
   session holds no geometry for. Its own docstring carries the in-tree measurement: *"one registration
   costs a measured 0.6 ms and the 180-object Nakagin document carries twelve of them"*
   (`✏️editor/🦀️.rs:1349-1354`). Each of those twelve goes through `install_collision_mesh`
   (`⏳️precompute/🦀️.rs:1735`), which does `brush_queue.clear()`, `brush_cache.clear()`, then a whole
   `rebuild_queue()` (or `refresh_fill_job(true)`) and `re_enqueue_brush_targets()`.
2. `Build` — `scene_config`, the engine scene build for the whole document.
3. `Push` — `push_precompute_scene`, `SetScene` into the engine.

W-P3 measured this prologue at **17.6 ms native** on that document (`✏️editor/🦀️.rs:3035-3040`). The
guest runs it in a `wasm-dev` build, where a single-digit-multiple slowdown puts one dispatch squarely in
the hundreds of milliseconds — that scaling factor is *derived*, not measured, and it is the only step of
this account that is.

The decisive point is not the size of the number but that the work was **self-cancelling**: the sync
seeds a scaled-box fallback body for exactly the ids a `registerBrushMesh` run is about to supply, and
the arm's own `register_mesh` → `install_collision_mesh` then clears the queue and cache all over again
and replaces the fallback with the real geometry. Seven serialized announcements each paid twelve
fallback registrations plus a full engine scene build and push, to produce state the next line of the
same command discarded. `registerBrushMesh` only ever *writes* geometry into the session; it reads
nothing from the synced view. Removing it from the list is therefore not an optimisation with a
trade-off — it removes an inversion. The next scene-reading action re-syncs, so nothing downstream can
go stale.

The native law asserts the non-membership with that whole rationale attached
(`✏️editor/🧪️tests/🔬️unit/🦀️.rs`, last assertion of
`an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`), so a later wave cannot quietly
re-add it.

**Residual, deliberately left standing.** Half one, `Puzzle3dActionPrologue::scene_step`, still
materializes the transient scene for this action, because `dispatch_step` refuses to run without one.
For `registerBrushMesh` that scene is provably inert — the arm does not touch `ctx.scene`, `before` is
`None` (no document intent, so `operations` is empty), and `refreshed()` re-assigns
`scene.runtime = config.clone()` before every diff, so every config / window / transient comparison in
`dispatch_step` is an equality against its own input. Eliminating it wants a declared "reads nothing
from the scene" action class, symmetric with `puzzle3d_action_uses_precompute`; it is one bounded step
inside the interactive-step ceiling today, so it is a follow-up, not a defect. Recorded here rather than
done, because the wave's own laws do not cover it yet.

## 5. Verification — exact commands and counts

Every command run in the foreground with output flowing, on the private target
`…/scratchpad/target-p3d` with `RUSTC_WRAPPER=""` and `CARGO_INCREMENTAL=0`.

**Native.** `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 …`

| filter | result |
| --- | --- |
| `--no-run` | `Finished \`test\` profile … in 2m 13s` — compiles; warning output present (unused imports, `with_puzzle3d_app_mut` dead in this feature set), zero errors. Both compile breaks this wave inherited (the duplicated `use std::collections::HashMap` in `✏️editor/🦀️.rs`, the stale `?` on `ui_value_number` in `📌️panels/🗿️artifact/🦀️.rs`) were already repaired in the tree when W-H3 started. |
| `an_id_only_announcement -- --nocapture --test-threads=2` | **1 passed; 0 failed**; 628 filtered out |
| `mesh -- --nocapture --test-threads=2` | **21 passed; 1 failed** — the one failure is `precompute::component::tests::fill_worker_admitted_fixed_pages_survive_replan_and_mesh_supersession_until_retained_close`, see below |
| (no filter) `-- --test-threads=2` | **607 passed; 23 failed**; 0 filtered, 79.6 s |

All 23 failures are one lane and none of them is this wave's: `fill_worker_*` (13),
`fill_build_tick_*` (4), `set_fill_count_clamps_to_available_and_no_longer_dispatches_catch_up`,
`fill_count_is_shared_across_split_panes_reveal_cutoffs_and_instances`,
`fill_render_reveals_the_full_available_plan_tagged_with_reveal_index`,
`two_instances_converge_disjoint_object_edits_via_backbone`,
`window_options_are_local_to_the_window_instance_not_shared_across_split_panes`, and
`a_nakagin_lane_that_did_not_change_does_not_republish_on_a_partial_refresh`. That last one names the
interaction lane and therefore had to be ruled in or out by evidence, not by topic: a temporary
`[DEBUG]` dump of both censuses' `interaction_json` (added, read, removed) shows the two payloads differ
in exactly one field —

```
before … "fillBuild":{"appliedCount":0,"count":0,"done":false,"maxCount":1000} … "meshResidency":0,"meshReuploadUrls":[] …
moved  … "fillBuild":{"appliedCount":0,"count":0,"done":true ,"maxCount":1000} … "meshResidency":0,"meshReuploadUrls":[] …
```

`fillBuild.done` flips `false → true` across a camera move; `meshResidency` and `meshReuploadUrls` are
byte-identical, so the two fields this wave added to that lane are not what republishes it. The whole
group belongs to W-F3's in-flight fill-job work in `⏳️precompute/🦀️.rs`.

One environment note worth keeping: under the harness's default thread stack,
`the_world_scene_names_built_in_meshes_by_reference_and_fits_its_fixed_capacity` aborts the whole test
binary with `has overflowed its stack` / `SIGABRT` (which also truncates the run's summary line, so the
counts above are only readable with the bigger stack). `RUST_MIN_STACK=67108864` makes it pass; it is a
stack-size artifact of the debug build, not a defect.

**Host typecheck.**
`cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react && bun ./📜️script.ts typecheck`
→ **820 `error TS…`**, all pre-existing and all in peer files (190 in `📜️script.ts`, 152 in
`🖱️ui/🧪️tests/🧪️owned-locale-detector-retirement`, 84 in `🧪️tests/🧪️docklayoutstore`, …). **Zero** in
`🌐️World3dHost/🟦️.tsx`, `🛠️ShellHelpers/🟦️.tsx`, `🔬️engine-contract/🟦️.ts` or the react target's own
`🟦️.tsx`.

**Host laws.** `bun ./📜️script.ts test long --run 'engine-contract'`
→ **1 file passed, 469 tests passed (469)**, 11.6 s. Baseline before this wave was 467
(`📋️master-plan-2026-09-08.md`), so the two new laws are in and nothing regressed.

## 6. What is NOT claimed

The browser was not re-driven in this wave: no `serve-puzzle3d-react-dev` boot, no wasm
`component-release`, so the "seven refusals on activation, ~12 s" observation is not re-measured against
the fix. What is proven is the contract on both sides, natively and in the engine-contract suite, plus
the static account in §4 with the in-tree measurements it rests on. Re-driving the Perspective-window
activation in the browser after a guest restart — and confirming zero `puzzle3d-register-mesh-digest`
notices and a page run instead — is the one remaining item.
