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

(filled in as the edits land — see §6)

## 4. The 0.65 → 2.85 s question

(see §5)
