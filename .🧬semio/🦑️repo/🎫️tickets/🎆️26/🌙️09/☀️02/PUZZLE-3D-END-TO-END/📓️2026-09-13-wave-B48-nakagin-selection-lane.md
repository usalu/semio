# Wave B48 — the guest selection lane and the `registerBrushMesh` re-announce loop on Nakagin

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave B48, 2026-09-12. Written incrementally.

Predecessor: `📓️2026-09-13-wave-B46-selection-on-nakagin.md` §8 items 1 and 2.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, shared live tree. No state-modifying git command,
  no worktree, no `CARGO_TARGET_DIR`/`RUSTC_WRAPPER`, everything foreground. The ticket is NOT closed.
  Nothing under `🗑️generated` was deleted.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- `RUST_MIN_STACK=134217728` on every law run.

## 1 Hop maps

Both defects are root-caused, fixed and gated. In one line each:

- **Defect 1** — `MountedReconcileGrant`'s `Drop` released less than its `cancel` did, so any refused
  `commit_source` left `slot.output_index` set and its `ready` output open, which made that surface
  **permanently un-reservable**: every later dirty render was refused, deferred, re-dirtied and refused
  again, and the surface stayed frozen at its last published revision while the actor answered
  `more-work` and published nothing (§4). Live on `:6013` at #58 that is all 13 surfaces of the puzzle3d
  instance deferred across 62 288 consecutive turns with `effects=0` (§6.4).
- **Defect 2** — the guest's standing re-upload request had ONE retirement site, unreachable for an
  identity that is already resident, while the host's only brake on re-claiming it was a counter the
  guest increments from inside the very work the brake exists to suppress (§5).

### 1.1 Defect 1 — the refresh hop, end to end (read off the tree, 2026-09-12)

| # | hop | file:line | what it does |
| --- | --- | --- | --- |
| 1 | `interactionSelect` reaches the framework's interaction route, persists, and returns the DECLARED scope | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:23389` | `A::interaction_scope(verb, &declared)` → `puzzle3d_selection_scope()` |
| 2 | the app's declaration | `✏️editor/🦀️.rs:2269` | `Partial { window_bodies: [puzzle3d.play.composite], panel_bodies: [inspection, document, framework history], measures: true }` |
| 3 | host unions guest scope with host-effect scope and refreshes | `🏛️ShellHost/🟦️.tsx:5684`–`5692` | `hostEffectRefreshScopeV1(effects, uiScope ?? full, …)` → `refreshUi(nextSession, refreshScope, …)`; traces `[DEBUG] applyHostEffects refresh` |
| 4 | scope → request | `🛠️ShellHelpers/🟦️.tsx:4889` `buildUiRefreshRequest` | window instances are filtered by `instance.bodyKey`, so a KIND-keyed `window_bodies` DOES match both `puzzle3d-main-top`/`-perspective` instances; each entry carries the host's cached hash. Traces `[DEBUG] refreshUi sections` with `asked`/`changed` |
| 5 | request → guest events | `🔌️PluginRuntime/🟦️.tsx:2577` `refreshUi` → `:1730` `uiRefreshSurfaceEvents` | one `Event::SurfaceVisible` per requested body; `missingSurfaceIds` = only the bodies with NO retained surface or no root, and that set is what `settlePluginTurn` waits for |
| 6 | guest: event → dirty surface | `⚛️reactor/🔄️turn/🦀️.rs:537` | `plugin_mount_surface` then `dirty.try_surface(instance, surface)` |
| 7 | guest: dirty → render | `⚛️reactor/🔄️turn/🦀️.rs:1093`–`1104` | `PATCHES.reserve_mounted(...)` then `plugin_render_surface`; **a surface whose previous reconcile is still in flight is REFUSED and only `defer`red** (`🩹️patches/🦀️.rs:389`,`:348`), and a `defer` that finds no free slot is dropped (`let _ = patches.defer(surface)`) |
| 8 | guest: render carries the live selection | `🔌️plugin/🦀️.rs:33563` `plugin_render_surface` → `instance.app.render` → `:27828` `VcsArtifactApp::render` → `A::render_with_request_context` | the live `InteractionView` is materialized here, so the puzzle3d body always renders against the persisted selection (`✏️editor/🦀️.rs:8156`; plain `render` with `Puzzle3dInteractionSnapshot::default()` is unreachable) |
| 9 | guest: tree → patch | `⚛️reactor/🔄️turn/🦀️.rs:1106`+ | `drive_one`/`take_ready_patch_into` page the diff at `SURFACE_RECONCILE_PAGE_BYTES` (32 KiB); only ONE deferred surface is re-dirtied per turn (`:484`) |
| 10 | host: retained surface → response | `🔌️PluginRuntime/🟦️.tsx:1949` `ownedUiRefreshResponse` | projects the RETAINED tree; `uiRefreshSectionUnchanged` skips a body whose retained view hash equals the host's cached hash |
| 11 | the DOM lane the probe reads | `🌐️World3dHost/🟦️.tsx:6544` | `data-guest-selection-json` = `worldSurfaceGuestSelectionDomV1(scene.selectionJson)`, i.e. the world3d scene's `n` field |

Hops 2, 4 and 8 are the three candidates the brief named, and all three are **already correct**:
`buildUiRefreshRequest` matches window KINDS against instance body keys (hop 4), and every guest render
of the world body carries the live interaction (hop 8) — which B46's own native law already proved
(`✏️editor/🧪️tests/🔬️selection-scale/🦀️.rs`,
`a_pick_by_id_on_the_flagship_document_persists_and_reaches_inspection`). The remaining candidates are
hops 5, 7 and 9: the host settles the refresh turn without waiting for a body that ALREADY has a root,
and the guest refuses-and-defers a re-render while the previous reconcile of that surface is in flight.

### 1.2 Defect 2 — the `registerBrushMesh` announce, read off the tree

| # | hop | file:line | what it does |
| --- | --- | --- | --- |
| 1 | host pager announces one mesh identity, or one PAGE of it | `🌐️World3dHost/🟦️.tsx` (`BrushMeshRegistrar`) | driven by the world body's `interactionJson.meshReuploadUrls` + `meshResidency` |
| 2 | guest accepts an identity-only announce | `✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs:44` | `precompute.adopt_shared_mesh(url, digest)`; on `false` → `request_reupload` |
| 3 | residency by url, then by DIGEST | `⏳️precompute/🦀️.rs:1986` `adopt_shared_mesh` → `:1320` `shared_brush_mesh` → `:1332` `adopt_brush_mesh_by_digest` | both take the process-wide store through **`try_lock().ok()?`**, so a miss and a contended lock are the same answer: `false` |
| 4 | a refused identity is recorded, idempotently | `⏳️precompute/🦀️.rs:2008` `request_mesh_reupload` | bounded at `FILL_WORKER_MAX_MESHES = 64`, sorted, `true` only for a NEW id — the only exit that widens the scope (`📋️register-brush-mesh/🦀️.rs:98`) |
| 5 | a completed page run installs and **clears** the request | `⏳️precompute/🦀️.rs:2042` inside `place_collision_mesh` | `if !is_fallback { self.mesh_reupload_requests.retain(|pending| *pending != url) }` — reached ONLY if every earlier guard in `place_collision_mesh` passed |
| 6 | page size | `⏳️precompute/🦀️.rs:1351` `PUZZLE3D_MESH_PAGE_VALUES = 1_024` | 1 024 values per command, because the shared retained wire admits 8 192 raw bytes per command |

The scale this sets, from the codebase's own numbers (`⏳️precompute/🦀️.rs:1327`-`1338`): every
`dist/mesh/*.glb` in this repo is the same 771 728-byte capsule, 294 912 payload bytes, **72 commands
per identity**. So a cold Nakagin catalogue is one 72-page run plus one identity-only announce per
remaining id — and B46's census (seq 22 → 124, still arriving 8 minutes later) is the same order of
magnitude as ONE such run, not obviously a loop. The two readings that census cannot separate are
therefore:

- **(a) a re-announce loop** — residency is never recognised, so the same id is paged again and again,
  and `place_collision_mesh` never reaches its `retain`; or
- **(b) an unbounded cold run** — residency works, the run is simply ~72 commands × ~3 s, serialized
  per actor behind every other command (`serializeCommandIngressForActor`).

They are distinguished by whether the SAME `url` appears in more than one page-1 command, which is what
`🔍️b48-selection-refresh.ts`'s per-10-s bucket census plus the page/pageCount fields answer. (b) would
also explain defect 1 on its own, because the pick's `refreshUi` queues behind the run in the same
per-actor ingress lane.

## 2 Blockers encountered (recorded, not worked around)

- `2026-09-12 18:20`–`18:30`: a peer is mid-refactor in `semio-framework-os-kernel` /
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`, moving
  symbols out of `os_store`. Every cargo invocation in this wave's verification set fails there
  (`error: let chains are only allowed in Rust 2024 or later`, `E0425 cannot find type Edit in module
  crate::os_store`, `E0433 ArtifactStoreDecodedEditRetirement`, …). The error count fell 12 → 9 while
  polling, so it is converging; nothing in this wave touches that crate.
- The probe slot (`pgrep -f 'bun .*browser-probe'`) was held by two live processes for the whole
  investigation window, so no browser run was taken yet.

## 3 Live measurement of the refresh hop — `🔍️b48-selection-refresh.ts`, 2026-09-12 16:55, `:6013`, wasm #58

The probe arms the shell's own runtime diagnostics in `localStorage` BEFORE the first module evaluates
(`runtimeDiagnosticsEnabled` resolves once per page, `🏛️ShellHost/🟦️.tsx:1653`), which makes the three
permanent taps on the refresh lane print. Log: `🗑️generated/wave-B48-probe-2.txt`, report
`🗑️generated/b48-2026-09-12T16-55-54-host-fix.md`.

**This run stayed on the 1-object Concrete Forest document** — the navbar example switch did not take
(`example-switch instances=1 bytes=266 waitedMs=203557`, see §5). That makes the reading STRONGER, not
weaker: the defect below is not Nakagin-specific at all.

The scope is declared correctly and the host asks for exactly the right bodies:

```
[DEBUG] applyHostEffects refresh {"declared":{"kind":"partial","measures":true,
  "panelBodies":["puzzle.3d.play.inspector","puzzle.3d.play.document","framework.body.history"],
  "windowBodies":["puzzle3d.play.composite"],…},"scope":{…identical…},"viewStateSame":true}
[DEBUG] refreshUi lane {"decision":"owed","scope":{…windowBodies:["puzzle3d.play.composite"]…},"passes":5}
[DEBUG] refreshUi lane {"decision":"pass",…,"passes":6}
```

and then the answer:

```
[DEBUG] refreshUi sections {"scope":{…"windowBodies":["puzzle3d.play.composite"]},
  "asked":["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"],
  "changed":[],
  "hashes":{"puzzle3d-main":"94ebfe0d:1","puzzle3d-main-top":"5877dc1d:1","puzzle3d-main-perspective":"aba169d7:1"}}
[DEBUG] refreshUi sections {"scope":{"kind":"full"},
  "asked":["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"],"changed":[],
  "hashes":{…identical…}}
```

with, in the same 51 530-line window after the click:

```
pick 0.50,0.50 census interactionSelectIngress=6 interactionSelectSettled=2 registerBrushMesh=0
               applyHostEffectsRefresh=6 refreshUiLane=9 refreshUiSections=3
pick 0.50,0.50 dropped=[]
pick 0.50,0.50 guest-selection-json=[{"surface":"framework.window.puzzle3dMainTop",
  "raw":"{\"selectedIds\":[],\"activeObjectId\":null,\"hoveredId\":null,\"gumballActive\":false}"}, …]
```

### 3.1 What this settles

| question | answer | evidence |
| --- | --- | --- |
| does the app declare the right scope for `interactionSelect`? | **yes** | `declared` names `puzzle3d.play.composite` + the three selection panels + `measures` |
| does the host lose or narrow it? | **no** | `scope` is byte-identical to `declared`; the coalescer runs it as its own pass |
| does the host ask the guest for the world body? | **yes, for all three window instances** | `asked:["puzzle3d-main","puzzle3d-main-top","puzzle3d-main-perspective"]`, twice (partial, then a full) |
| does the host DROP a requested body? | **no** | `dropped=[]` — no `refreshUi dropped requested body`, no `did not publish its requested UI surfaces`, no `stopped without publishing` |
| what does the guest answer? | **`unchanged` for every world body** | `changed:[]`, and every response hash ends `:1` — the retained surfaces are still at **revision 1** after 6 `interactionSelect` ingresses and 2 settles |
| is this Nakagin-specific? | **no** | measured on the 1-object document |

So the defect is one hop earlier than even B46 placed it: **the guest's retained world surface never
reaches revision 2 after a pick.** `uiRefreshSectionUnchanged` is then behaving exactly as specified —
the retained tree genuinely has not moved — and every host-side candidate (scope width, kind-vs-instance
addressing, the cached-hash skip, `missingSurfaceIds`) is now excluded by measurement.

Two readings remain, and they are distinguished by whether the guest RENDERED:

- **(a) the guest re-rendered and produced a byte-identical tree** — i.e. the render that the refresh
  drives does not carry the persisted selection, even though `Puzzle3dPlayApp::render_body` does when a
  test calls it (B46's `🔬️selection-scale` law is green); or
- **(b) the guest never re-rendered that surface** — the dirty render was refused and deferred
  (§1.1 hops 7/9).

(a) is answerable natively and is tested in §4. (b) needs a guest-side trace and therefore a wasm build.

## 4 Defect 1 — root cause, fix, laws

### 4.1 The two readings, decided

Reading **(a)** ("the guest re-rendered a byte-identical tree") is **closed by a native law**, at every
window instance id the host addresses, on both documents:

`✏️editor/🧪️tests/🔬️selection-scale/🦀️.rs` →
`a_pick_reaches_the_world_body_of_every_window_instance_the_host_refreshes`

```
[DEBUG] b48.worldLane example=concrete-forest objects=1 window=puzzle3d-main picked=seed-left-001 ids=["seed-left-001"]
[DEBUG] b48.worldLane example=concrete-forest objects=1 window=puzzle3d-main-top picked=seed-left-001 ids=["seed-left-001"]
[DEBUG] b48.worldLane example=concrete-forest objects=1 window=puzzle3d-main-perspective picked=seed-left-001 ids=["seed-left-001"]
[DEBUG] b48.worldLane example=nakagin-capsule-tower objects=180 window=puzzle3d-main picked=01890804-… ids=["01890804-…"]
[DEBUG] b48.worldLane example=nakagin-capsule-tower objects=180 window=puzzle3d-main-top picked=01890804-… ids=["01890804-…"]
[DEBUG] b48.worldLane example=nakagin-capsule-tower objects=180 window=puzzle3d-main-perspective picked=01890804-… ids=["01890804-…"]
test result: ok. 1 passed
```

So the render carries the pick. The tree that the refresh produces is therefore never committed.

### 4.2 Root cause — `MountedReconcileGrant`'s `Drop` is not `cancel`

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs`

| what | where | what it did |
| --- | --- | --- |
| a reservation ended **without** `cancel` | `Drop for MountedReconcileGrant`, `🩹️patches/🦀️.rs:252` (pre-fix) | released ONLY the unadmitted marker, the rejected reservation, the reconciler and the reservation |
| `cancel`, the other exit | `🩹️patches/🦀️.rs:223` (pre-fix) | released all of that **plus** `slot.output_index = None` and `ready[output_index].closing = true` — the two lines `Drop` lacked (and it carried them twice over, a botched earlier edit) |
| who takes the `Drop` exit | `commit_source`, `🩹️patches/🦀️.rs:138` | **seven** `return Err(root)` exits — a closing instance, a generation mismatch, a taken slot, a slot-state mismatch, a missing marker, a transferred owner, a refused `ComponentTreeProducer` — and none of them disarms `active`, so every one of them is a plain drop |
| who refuses a slot with a live output | `reserve_mounted_owned`, `🩹️patches/🦀️.rs:404` | `slot.output_index.is_some()` ⇒ `Err(surface)` |
| the caller of `commit_source` | `⚛️reactor/🔄️turn/🦀️.rs:1130` | `let _ = grant.commit_source(tree.root);` — the failure was **not even recorded** |

So ONE refused commit made that surface **permanently un-reservable**: every later dirty render was
refused (`:404`), the turn only `defer`red it, `deferred_surface_ready` answered `true` (no producer, no
job, reconciler restored by `Drop`, revision acknowledged), the turn re-dirtied it, and the refusal
repeated — an unbounded refuse/defer cycle in which the surface never renders again and the actor
answers `more-work` while publishing nothing. Each such exit also leaked one of `READY_PATCH_CAPACITY`
(= `UI_RESIDENT_SLOTS` = 64) output slots, so after 64 of them `reserve_mounted` refuses EVERY surface
in the shell.

That is exactly the live reading of §3 (world bodies asked for, nothing dropped, `changed:[]`, every
retained surface frozen at **revision 1**) and it is the same shape the runtime's own
`PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT` doc comment already records from this ticket:
*"`acks=0` for over a thousand consecutive continuations"* (`🔌️PluginRuntime/🟦️.tsx:1432`).

### 4.3 The fix

1. **`🩹️patches/🦀️.rs`** — one `MountedReconcileGrant::release(&mut PatchTrackerState)` is now the ONE
   release path, and both `cancel` and `Drop` call it (it sets `active = false` first, so `cancel`'s own
   drop is a no-op). The duplicated pair of blocks in `cancel` is gone, and the `#[cfg(test)] commit`
   helper's two early returns release through it instead of leaking the same way.
2. **`⚛️reactor/🔄️turn/🦀️.rs`** — a refused `commit_source` is a `shell_fault_effect`
   (`ui.surface-render-uncommitted`) naming the surface, not `let _ =`. Deliberately NOT a turn-fatal
   `?`: a commit refused because the instance is closing is an ordinary teardown race and faulting the
   turn for it would break the close ladder.
3. **`⚛️reactor/🔄️turn/🦀️.rs`** — a `defer` that finds no room in the fixed ring is a typed fault
   (`ui.dirty-surface-deferred-capacity`), not `let _ =`: a dropped deferral is a render the host asked
   for that no later turn performs.
4. **`⚛️reactor/🔄️turn/🦀️.rs`** — `redirty_acknowledged_deferred_surfaces` re-dirties EVERY acknowledged
   deferred surface in the same turn instead of one. One `puzzle3d_selection_scope` refresh dirties three
   world bodies (`puzzle3d-main`, `-top`, `-perspective`, the last two plus the leftover `window` alias)
   and three panel bodies, so the old pacing charged five extra host round trips before the last of them
   re-rendered.

### 4.4 Laws

| law | file | what it gates |
| --- | --- | --- |
| `a_dropped_render_reservation_releases_its_surface_slot_and_its_output` | `⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | `cancel` and `Drop` leave the slot in the SAME reservable state, and a released output returns to the fixed `ready` pool (the loop runs past `READY_PATCH_CAPACITY`) |
| `a_retired_surface_publication_leaves_its_slot_reservable_for_the_next_dirty_render` | `⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs` | a Nakagin-scale world publication that the host retired becomes reservable again, and names the slot family in its failure message |
| `one_turn_redirties_every_acknowledged_deferred_surface`, `a_drain_over_an_empty_deferred_ring_dirties_nothing` | `⚛️reactor/🔄️turn/🧪️tests/🕹️deferred-render-drain/🦀️.rs` (new module) | the drain takes ALL ready deferred surfaces, terminates, and neither loses nor invents one |
| `a_pick_reaches_the_world_body_of_every_window_instance_the_host_refreshes` | `✏️editor/🧪️tests/🔬️selection-scale/🦀️.rs` | the guest render carries the pick on the refresh's own route, at every instance id, on both documents |

**Proved red without the fix** (temporary revert, restored immediately): dropping the two release lines
from `release` fails the first law with the slot state quoted —

```
`cancel` and `Drop` must leave the slot in the same reservable state:
slots=[5:window#g1:--R:ack0/rev0:outSome(0)] ready=[g67:--r-] …
```

`outSome(0)` is the leaked output that made the surface un-reservable.

## 5 Defect 2 — root cause, fix, laws

### 5.1 Root cause — the request set never retires, and the host's brake is moved by the traffic it suppresses

Two halves, and each alone is enough to keep the announce running.

**Guest half.** `mesh_reupload_requests`' own field doc (`⏳️precompute/🦀️.rs:1583`) asserts *"an entry
retires the instant that identity's geometry installs, so the set is empty in every steady state and a
refusal can never become a standing request"* — and the single retirement site was the `retain` inside
`place_collision_mesh` (`:2042`), **behind that function's own already-resident bail** (`:2036`). The
three exits that satisfy an announcement without writing geometry all left the request standing:

| exit | file:line | why it left the request |
| --- | --- | --- |
| `adopt_shared_mesh`'s resident fast return | `⏳️precompute/🦀️.rs:1987` | answers `true` with no install at all |
| `stage_mesh_page`'s page-0 short circuit | `⏳️precompute/🦀️.rs:2762` | delegates to that same fast return, returns `Ok(None)` |
| `place_collision_mesh`'s already-resident bail | `⏳️precompute/🦀️.rs:2036` | returns `false` BEFORE the `retain` at `:2042` |

On the 12-mesh Nakagin catalogue that is guaranteed to fire: every `dist/mesh/*.glb` in this repo is the
same capsule, so eleven ids become resident by **digest alias** (`adopt_brush_mesh_by_digest`, `:1332`)
rather than by their own page run — i.e. through exactly the exits that do not retire.

**Host half.** `Puzzle3dBrushMeshRegistry.claimReupload(url, residency)`
(`🛠️ShellHelpers/🟦️.tsx:3079`, pre-fix) gated re-claims on `residency <= #repaged[url]` — one claim per
published `meshResidency` VALUE. `meshResidency` is `BRUSH_MESH_INSTALLS`, which **every accepted
announcement increments** (`derive_brush_mesh:1313`, unconditionally even on an `EngineCache` hit, and
`adopt_brush_mesh_by_digest:1337`). So the brake was moved by the traffic it existed to suppress: one
standing request re-opened the gate on every unit of progress anywhere in the tab, each claim deleted
the paged entry and permanently set `#refusedAlias` (so `mayAlias` is false thereafter), and the next
announcement therefore took the **full 72-command page path** instead of a one-command adopt — whose
pages 1..71 the guest then refuses as `Gap`, because its own page-0 short circuit had already closed the
run. At B22's measured 0.64 s–7 s per command on the serialized per-actor ingress lane that is 45 s to
8+ minutes per url, with eleven urls eligible — B46's census (123 of 285 lines, seq 22 → 124 over 306 s,
still arriving 8 minutes in) is one such cycle in progress, against the `72 + 11 = 83` commands ONCE
that a cold 12-identity/one-digest catalogue should cost.

### 5.2 The fix

1. **Guest** — `⏳️precompute/🦀️.rs`: one `Puzzle3dCollision::retire_mesh_reupload(url)`, called from
   `adopt_shared_mesh`'s BOTH `true` exits and from `place_collision_mesh`'s already-resident bail, with
   the install path's `retain` folded into it. Residency is the authority; the request set is a cache of
   "announced and could not be served", so every path that answers an announcement prunes it.
2. **Host** — `🛠️ShellHelpers/🟦️.tsx`: `claimReupload(url)` (the `residency` parameter is gone) is
   bounded by a per-url claim COUNT against the new `PUZZLE3D_MESH_REUPLOAD_CLAIMS = 2` — the run the
   request asks for, plus one retry for a run that failed part-way. `#repaged` is deleted. Only a guest
   RESTART (`observeResidency` seeing the counter fall) or a deliberate `clear` releases the claims, so
   the gate is a fact the guest's accepted work cannot move. Call site updated at
   `🌐️World3dHost/🟦️.tsx:5439`.

**The bounded-announce invariant this buys**: for a fixed document and a live guest instantiation, a
surface emits at most `distinct digests × pages(digest) + distinct urls` `registerBrushMesh` commands,
plus at most one retry per url — and **zero** while `meshResidency` is unchanged and `meshReuploadUrls`
is byte-identical. Equivalently, on the guest side:
`∀url. has_mesh(url) ∧ ¬mesh_is_fallback[url] ⇒ url ∉ mesh_reupload_requests`, at every publication of
the world body.

### 5.3 Laws

| law | file | what it gates |
| --- | --- | --- |
| `a_resident_identity_never_stays_in_the_re_upload_request_set` | `⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` | all three answering paths retire the request: the resident fast return, the page-0 short circuit, the already-resident bail |
| `claims a guest re-upload request a bounded number of times per guest instantiation` | `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` (rewritten from W-H's "once per residency") | a CLIMBING residency is progress, never a new claim; a completed run buys no further claim; a restart releases every claim |

The TS law is the cross-language twin of the Rust one, as the rest of this wire contract already is
(`🥽️brush-mesh-upload/🔣️.json` drives both ends).

**Proved red without the fix** (temporary revert, restored immediately):

```
thread '…a_resident_identity_never_stays_in_the_re_upload_request_set' panicked at …⏳️precompute/🧪️tests/🔬️unit/🦀️.rs:1597:
so the request it answered must be gone, not left for the world body to republish
test result: FAILED. 0 passed; 1 failed
```

## 6 Live evidence and what rides #60

### 6.1 What was live for this wave

| half | build | measured |
| --- | --- | --- |
| host — `🛠️ShellHelpers/🟦️.tsx` (`claimReupload`, `PUZZLE3D_MESH_REUPLOAD_CLAIMS`), `🌐️World3dHost/🟦️.tsx` call site | **live off the repo** (Vite dev server on `:6013`) | §3 and §6.2 |
| guest — `⏳️precompute/🦀️.rs` retirement, `⚛️reactor/🩹️patches/🦀️.rs` release, `⚛️reactor/🔄️turn/🦀️.rs` drain + the two faults | **not in any materialized wasm** — the diagnostic runs rode **#58** | native only (§4, §5) |

So every guest-side claim in this report is native. **All four guest fixes ride #60.** The host half is
already measured live.

### 6.2 The mesh census with the host fix live, guest still #58

`bun 🔍️b48-selection-refresh.ts --port=6013 --label=host-fix --settle=20 --census=50` →
`🗑️generated/wave-B48-probe-2.txt`:

```
mesh census seconds=50 registerBrushMesh=0 buckets=["0s:4","10s:22","20s:34"]
pick 0.50,0.50 census … registerBrushMesh=0 …
```

60 `registerBrushMesh` commands in the first 30 s after boot and then **zero** for the rest of the run
(the census window opens at ~230 s and the four picks run to 470 s) — against B46's 123 in a 150 s
window still climbing at 306 s and arriving 8 minutes in. This is the 1-object document, so it is a
convergence reading rather than the flagship number; the flagship number needs §7's Nakagin run.

### 6.3 A probe defect found and fixed on the way

`example-switch instances=1 bytes=266 waitedMs=203557` — the example switch **silently did nothing**, so
that run measured the 1-object document while its label said otherwise. `--inspect-navbar` (added to
this wave's probe) says why: `playground.navbar.fixture` is a `<button role="combobox">` over a
PORTALLED listbox, `selects=[]`, and the three options (`No example`, `Concrete Forest`,
`Nakagin Capsule Tower`) exist only once `aria-expanded="true"`. The inherited switch block (B44/B46's)
probed for a `<select>`, then clicked blind after a fixed 800 ms sleep and swallowed every failure. It
now opens the combobox, WAITS for the option to be visible, clicks it un-forced first, and verifies the
picker's own label changed before measuring anything — with up to four attempts and a quoted tail when
it still fails. Any wave reusing `🔍️b44-mutation-latency.ts` or `🔍️b46-selection-nakagin.ts` inherits
the blind version and should assume its "flagship" numbers may be Concrete Forest numbers.

### 6.4 The guest's own verdict — the Nakagin switch on wasm #58 is a permanent refuse/defer spin

With the fixed switch (§6.3) the picker DOES take — and the document still never arrives:

```
example-switch attempt=0 expanded=true optionVisible=true
example-switch attempt=0 pickerLabel="Nakagin Capsule Tower" ok=true
example-switch switched=true instances=1 bytes=266 waitedMs=180264
```

The guest says why, in its own `reactor more-work streak` trace (`🗑️generated/wave-B48-probe-3.txt`,
one line of 57 000 identical ones):

```
[actor] [DEBUG] reactor more-work streak=57451 seen=57502 sources=["reconcile"] contended=false effects=0
 patches=[slots=[
   1:puzzle3d-main#g1:--R:ack1/rev1:outNone,
   1:puzzle3d-main-top#g2:--R:ack1/rev1:outNone,
   1:puzzle3d-main-perspective#g3:--R:ack1/rev1:outNone,
   1:window#g14:-J-:ack1/rev0:outSome(0),
   1:framework.panel.artifact#g5:--R:ack1/rev1:outNone,
   1:framework.panel.catalogue#g6:--R:ack1/rev1:outNone,
   1:framework.panel.inspection#g7:--R:ack1/rev1:outNone,
   1:puzzle3d.panel.settings#g8:--R:ack1/rev1:outNone,
   1:framework.panel.history#g9:--R:ack1/rev1:outNone,
   1:framework.section.engagements#g10:--R:ack1/rev1:outNone,
   1:framework.section.measures#g11:--R:ack1/rev1:outNone,
   1:framework.section.tools#g12:--R:ack1/rev1:outNone,
   1:framework.section.catalogue#g13:--R:ack1/rev1:outNone]
  ready=[g14:--r-] terminals=[] producer_terminals=[]
  deferred=[1:puzzle3d-main,1:puzzle3d-main-top,1:puzzle3d-main-perspective,1:window,
            1:framework.panel.artifact,1:framework.panel.catalogue,1:framework.panel.inspection,
            1:puzzle3d.panel.settings,1:framework.panel.history,1:framework.section.engagements,
            1:framework.section.measures,1:framework.section.tools,1:framework.section.catalogue]
  rejected=0 unadmitted=0 closing=0 output_fault=none generation_exhausted=false close_cursor=1]
 pending=[slots=[] handback_empty=true handback_sequence=None exhausted=false closing=0]
```

The slot triple is `(producer?, job?, reconciler?)` (`🩹️patches/🦀️.rs` `debug_state`). So:

- **Every one of the thirteen surfaces of instance 1 is in `deferred`** — every world body, every panel
  body, every reserved section. Nothing is rendering.
- Twelve of them have a **clean, reservable-looking slot**: `--R` (no producer, no job, reconciler
  restored), `ack1/rev1` (the host acknowledged the revision they hold), `outNone` (no output held).
  `deferred_surface_ready` therefore answers `true` for them, the turn re-dirties one per turn, and the
  reservation is refused again — so this is a **refuse/defer cycle**, not a wait.
- The one surface that is NOT clean is `1:window#g14:-J-:ack1/rev0:outSome(0)` — the **leftover `window`
  alias** that `windowHostContextBindings` appends to every refresh request
  (`🔌️PluginRuntime/🟦️.tsx:1622`). It holds a JOB that never publishes (`rev0`), owns the only reserved
  output, and is therefore what keeps `has_publishable_work` true for **57 451 consecutive turns with
  `effects=0`** — the exact streak shape `PLUGIN_UI_ZERO_PROGRESS_CONTINUATION_LIMIT`'s doc records
  from this ticket.
- `output_fault=none`, `rejected=0`, `unadmitted=0`, `closing=0`, `generation_exhausted=false`: none of
  the refusal predicates that leave evidence fired.

This is the same defect §3 measured from the host side, now seen from the guest, and it is worse than
"the selection lane does not republish": **on the flagship document the whole guest UI freezes after its
first publication.** `data-guest-selection-json` stuck at `selectedIds:[]` is one symptom of it; the
example switch never materialising is another.

Reproduced deterministically. A second run (`🗑️generated/wave-B48-probe-5.txt`, after `touch`ing a
peer's relocated module — §9) lands on the identical state: `switched=true`, `instances=1` after 180 s,
`streak=62288 seen=62338 sources=["reconcile"] contended=false effects=0`, the same thirteen surfaces
deferred, and the same `1:window#g14:-J-:ack1/rev0:outSome(0)`. The mesh announce converges in both runs
with the host fix live: `registerBrushMesh` buckets `["0s:4","10s:32","20s:24"]` then **zero** for the
remaining 40-second census window and the picks after it.

### 6.5 A refusal that leaves no evidence — made self-diagnosing

Eight of the eleven refusal predicates in `reserve_mounted_owned` returned a bare `Err(surface)`, so a
guest in this cycle was indistinguishable from a guest with nothing to do. `PatchTrackerState` now
carries `reserve_refusal: Option<(u8, &'static str)>` — the refused slot index plus one of
`instance-mismatch`, `instance-closing`, `unadmitted-full`, `slots-full`, `slot-key-stale`,
`slot-output-held`, `slot-producer-live`, `slot-job-live`, `slot-reconciler-checked-out`,
`generations-exhausted`, `rejected-full`, `registry-reservation-unavailable`, `ready-full`,
`output-reservation-unavailable`, `output-reservation-fault` — and `debug_state` prints it as
`reserve_refusal=<surface>:<reason>`, so the reactor's own more-work streak trace (already quoted above)
names the predicate. Stored as a `u8` slot index rather than a `SurfaceId` so the fixed tracker state
keeps its 256-byte stack budget (`tracker_initialization_fits_the_component_stack_budget`: 232 → 248 of
256 bytes).

**#60 will therefore answer, in one line, which predicate holds the twelve clean slots.** The two
candidates the trace above cannot separate are `slot-key-stale` (the instance's live `NativeCloseKey`
generation no longer matches the key the slots were reserved under, which would refuse every surface of
that instance for ever) and `output-reservation-unavailable` (the process-wide reconcile registry has no
reservation left — which is exactly what the `Drop` leak of §4.2 drains).

## 7 Verification — every command run in the foreground, tails quoted

```
$ RUST_MIN_STACK=134217728 cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
warning: `semio-s-artifact-puzzle-3d` (lib) generated 92 warnings (run `cargo fix --lib …`)
    Finished `dev` profile [unoptimized] target(s) in 26.81s          → 0 errors, 114 warnings
```

```
$ RUST_MIN_STACK=134217728 cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Finished `dev` profile [unoptimized] target(s) in 49.63s          → 0 errors, 125 warnings
```

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
    --lib -- --test-threads=1 selection refresh brush_mesh nakagin
test result: FAILED. 57 passed; 2 failed; 0 ignored; 0 measured; 690 filtered out; finished in 6.46s
failures:
    gumball_active_only_for_transform_utilities_with_object_selection
    open_vortex_suggestions_every_step_stays_below_the_interactive_ceiling_for_nakagin
```

Both failures are **pre-existing and not this wave's** (§9): the gumball one fails in ISOLATION too
(`an unattached gumball must never render: left: Some(true), right: Some(false)`) and touches nothing
this wave changed; the other is a 2 ms-per-turn unoptimized perf budget that measured 4.67 ms under a
live fleet. Wave B44 §7 already records the gumball law as red.

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
    --lib -- --test-threads=1 re_upload
test editor::puzzle3d::precompute::component::tests::a_resident_identity_never_stays_in_the_re_upload_request_set ... ok
```

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly \
    --lib -- --test-threads=1 --nocapture a_pick_reaches_the_world_body
[DEBUG] b48.worldLane example=concrete-forest objects=1 window=puzzle3d-main picked=seed-left-001 ids=["seed-left-001"]
… six rows, all three window instances × both documents …
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 748 filtered out; finished in 0.80s
```

```
$ RUST_MIN_STACK=134217728 cargo test -p semio-framework-plugin --lib -- --test-threads=1 \
    refresh_ui partial deferred reservable dropped_render tracker_initialization
test component::reactor::patches::tests::a_dropped_render_reservation_releases_its_surface_slot_and_its_output ... ok
test component::reactor::patches::tests::tracker_initialization_fits_the_component_stack_budget ... ok
test component::reactor::reconcile_budget_tests::a_retired_surface_publication_leaves_its_slot_reservable_for_the_next_dirty_render ... ok
test component::reactor::turn::deferred_render_drain_laws::a_drain_over_an_empty_deferred_ring_dirties_nothing ... ok
test component::reactor::turn::deferred_render_drain_laws::one_turn_redirties_every_acknowledged_deferred_surface ... ok
test result: FAILED. 16 passed; 1 failed; 0 ignored; 0 measured; 672 filtered out; finished in 0.12s
failures:
    component::plugin_runtime::plugin_builder_contract_tests::history_delivery_preserves_partial_ui_scope
```

That one failure is a PEER's: `viewPartialScope: Fault { code: FaultCode("interactive-job.missing-factory"),
message: "typed command 'viewPartialScope' has no exact controller/owner/factory/tool/schema proof" }` —
the factory-proof admission, nothing this wave touches. **Name diff against the B44 baseline for this
family: `deferred_render_drain_laws::*` and `a_dropped_render_reservation_…` are new here; no previously
green name in the filter went red.**

```
$ bunx nx run @semio-tech/framework-renderer-react:test-long -- "🔬️engine-contract" -t "brush mesh" --reporter=verbose
 ✓ … > puzzle3d brush mesh paged upload > claims a guest re-upload request a bounded number of times per guest instantiation 0ms
 … 10 of 10 green …
 Test Files  1 passed (1)
      Tests  10 passed | 576 skipped (586)
```

(The default `test` target resolves to the `⚡️quick` suite only — the engine suites need `test-long`.
A `test` run of an engine suite reports `No test files found` with the resolved `include` printed, which
is worth knowing before concluding a suite is green.)

## 8 Files changed

| file | change |
| --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` | one `MountedReconcileGrant::release` shared by `cancel` and `Drop` (the leak); the `#[cfg(test)] commit` early returns release through it; `reserve_refusal` diagnostic on all fifteen refusal predicates, printed by `debug_state`, stored as `(u8, &'static str)` to keep the 256-byte state budget |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `redirty_acknowledged_deferred_surfaces` (all, not one); a refused `commit_source` is `ui.surface-render-uncommitted`; a refused `defer` is `ui.dirty-surface-deferred-capacity`; new `🕹️deferred-render-drain` test module registered |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/🕹️deferred-render-drain/🦀️.rs` | **new** — two laws on the drain |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs` | `a_dropped_render_reservation_releases_its_surface_slot_and_its_output` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs` | `a_retired_surface_publication_leaves_its_slot_reservable_for_the_next_dirty_render` |
| `✏️s/…/🧊️3d/…/✏️editor/⏳️precompute/🦀️.rs` | `retire_mesh_reupload`, called from `adopt_shared_mesh`'s both `true` exits and `place_collision_mesh`'s already-resident bail |
| `✏️s/…/🧊️3d/…/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` | `a_resident_identity_never_stays_in_the_re_upload_request_set` |
| `✏️s/…/🧊️3d/…/✏️editor/🧪️tests/🔬️selection-scale/🦀️.rs` | `a_pick_reaches_the_world_body_of_every_window_instance_the_host_refreshes` |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `PUZZLE3D_MESH_REUPLOAD_CLAIMS`; `claimReupload(url)` bounded per guest instantiation; `#repaged` → `#claims` |
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx` | `claimReupload` call site |
| `🧰️framework/…/🧪️tests/🔬️engine-contract/🟦️.ts` | the claim law rewritten to the bounded contract, `PUZZLE3D_MESH_REUPLOAD_CLAIMS` imported |
| `T/🔍️b48-selection-refresh.ts` | **new** (input file, kept) — diagnostics-armed refresh-hop probe, mesh-census buckets, `--inspect-navbar`, a switch that verifies itself |

## 9 Peer state observed while this wave ran (recorded, not worked around)

| what | evidence |
| --- | --- |
| `semio-framework-os-kernel` / `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs` mid-refactor out of `os_store` | every cargo invocation failed there 18:20–18:40 (`let chains are only allowed in Rust 2024`, `E0425 cannot find type Edit`, `E0433 ArtifactStoreDecodedEditRetirement`); error count fell 12 → 9 → 0 while polling |
| `semio-framework-plugin`'s `🧪️tests/🧩️composition/🦀️.rs` carried a 2024-edition let-chain in a 2021 crate | blocked the lib-test target until ~18:50; peer's file, resolved itself |
| `semio-framework-ui-runtime/🧠️runtime/♻️reconcile/🦀️.rs` edited 17:07, and its retirement ladder no longer terminates | `close_instance_to_empty` → `instance N did not reach terminal empty` in `issued_obsolete_reconcile_feedback_retires_only_the_old_pending_owner`, `mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes`, `mounted_output_admission_direct_receiver_preserves_captured_lifetime_generation_and_callback_roots` — all three fail **in isolation** and fail **identically with this wave's `release` change both on and off**, so they are the peer's, not ours. `a_nakagin_scale_mixed_surface_turn_retires_every_ladder_within_a_handful_of_reactor_turns` consequently runs to its 100 000-turn bound (45+ minutes) instead of the single digits it asserts |
| `gumball_active_only_for_transform_utilities_with_object_selection` red in isolation | `an unattached gumball must never render: left: Some(true) right: Some(false)`; already recorded red by B44 §7 |

## 10 Residual / handed over

1. **What #60 must answer, in one line.** With `reserve_refusal` in the guest, the reactor's own
   more-work streak trace names the predicate holding the twelve clean slots (§6.5). Run
   `bun 🔍️b48-selection-refresh.ts --port=6013 --label=w60 --census=60` and read
   `reserve_refusal=<surface>:<reason>`. Expected outcomes: `none` (the four fixes closed it),
   `slot-key-stale` (the instance's live `NativeCloseKey` generation drifted from the key its slots were
   reserved under — a lifecycle-generation defect, and the next wave's whole subject), or
   `output-reservation-unavailable` (the process-wide reconcile registry is drained, which is what the
   `Drop` leak of §4.2 did and should now be fixed).
2. **The leftover `window` alias surface holds a job that never publishes.** `1:window#g14:-J-:ack1/rev0`
   owns the only reserved output across 57 451 turns. That alias is minted host-side for EVERY refresh
   (`windowHostContextBindings`, `🔌️PluginRuntime/🟦️.tsx:1622`, one extra binding carrying the LAST
   window's body key), so every `puzzle3d_selection_scope` refresh renders the 180-object world body a
   FOURTH time under a surface no pane reads. Whether the alias is still needed at all
   (`DEFAULT_LEFTOVER_WINDOW_SURFACE` dates from the leftover-Viewport patches) is worth deciding before
   optimising anything else on this path: it is 25 % of every world publication.
3. **The refresh protocol has no "rendered, unchanged" signal.** `refreshUi` waits only on bodies with
   NO retained surface (`missingSurfaceIds`, `🔌️PluginRuntime/🟦️.tsx:2586`), because a guest that
   re-renders an identical tree legitimately publishes nothing. So "the guest rendered and nothing
   changed" and "the guest never rendered" are the same answer on the wire, which is why this defect
   survived B4, B9, B25, B32b, B39, B44 and B46. A per-dirty-surface acknowledgement ("rendered at
   revision N") would make it impossible; it is a wire change and was out of scope here.
4. **`interactionSelect` settle latency at 180 objects** (B46 residual 3) is unmeasured by this wave —
   no pick could be established on Nakagin at #58 because of §6.4. B46's topology memo and this wave's
   four fixes all ride #60; re-measure then.
5. **Every probe inherited from B44/B46 switches the example blind** (§6.3). `🔍️b44-mutation-latency.ts`
   and `🔍️b46-selection-nakagin.ts` probe for a `<select>` that does not exist, then click after a fixed
   800 ms sleep and swallow the failure, so a run can report flagship numbers taken on Concrete Forest.
   `🔍️b48-selection-refresh.ts` has the self-verifying version; port it before reusing either.
6. **The `♻️reconcile` retirement ladder is currently broken by a peer** (§9). Three `patches::` laws and
   the W-B2 mixed-surface budget law are red because of it. Nothing in this wave depends on them, but a
   coordinator battery over `semio-framework-plugin --lib` will read red until that peer lands.
7. **Rides #60**: `retire_mesh_reupload`, `MountedReconcileGrant::release`, `reserve_refusal`,
   `redirty_acknowledged_deferred_surfaces`, `ui.surface-render-uncommitted`,
   `ui.dirty-surface-deferred-capacity`. The host halves (`claimReupload`, the call site, the probe) are
   already live off the repo and measured in §6.2.

## 11 Two stale Vite module graphs hit and cleared while probing

Both are the class the coordinator's brief names (`does not provide an export named …`), and in both the
export DOES exist in source — the dev server was holding a transform from a peer's mid-edit second:

| symptom | the export | cleared by |
| --- | --- | --- |
| `'…/🧱️elements/🐚️Shell/🟦️.tsx' does not provide an export named 'shouldAutoStartIntroduction'` | `🐚️Shell/🟦️.tsx:326`, present | `touch` on `🐚️Shell/🟦️.tsx`, `🏛️ShellHost/🟦️.tsx`, `🎯️targets/⚛️react/🟦️.tsx`, `🔄️ShellSync/🟦️.tsx` |
| `'🧰️framework/📦️packages/🟦️typescript/🟦️.ts' does not provide an export named 'parseViewport2d'` | re-exported at `📦️packages/🟦️typescript/🟦️.ts:18` from `🖱️ui/🪟️viewport/◻️2d/🧬️schema/🟦️.ts:7`, present | `touch` on those two plus `🕸️NodeGraph/🟦️.tsx` (its importer) |

Each cost one 200-second boot budget. Any wave whose battery reports `boot instances=0` should read the
console tail before believing anything about the guest: `🔍️b48-selection-refresh.ts` now prints it.
