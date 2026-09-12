# Wave B32b — the world lane after a mutation's completion

Ticket `26/09/02/PUZZLE-3D-END-TO-END`. Relaunch of B32 (first instance stalled with no report and no
known edits). Written incrementally from the first tool call.

Assignment: the last convergent defect behind gumball / relocate / hide / delete — the mutation commits
(console `[DEBUG] gumball pose delta`, history `move-object … new-origin=13.0994,0,0`) yet
`data-instances-json` is byte-identical after 30 s and the artifact panel body is never re-taken
(B29 §4.5), plus the `Agent disconnected` presence flip in the same reading.

## 0 Reading log (what is established before any edit)

- Guest scope table already declares every one of the named verbs as `Puzzle3dScopeClass::Document`
  (`✏️editor/🦀️.rs:2340-2341`): `translateSelection`, `worldRelocate`, `setSelectionFlag`,
  `deleteSelection`, `addTargetVolume`, `addObjectKind`, `importFixture`. `Document` resolves to
  `UiDirtyScope::Partial { window_bodies: [main::BODY_KEY], panel_bodies: puzzle3d_document_panel_bodies(),
  measures: true }` (`:2315-2317`), and `main::BODY_KEY == "puzzle3d.play.composite"`
  (`🪟️windows/🧊️main/🦀️.rs:33`). So hypothesis (a) of the assignment — "the completion carries the
  mutation but no world/document scope because the publication contract omits them" — is FALSE at the
  scope-table level: `importFixture` and `translateSelection` share one class.
- The probe reads `worldHidden` out of `data-instances-json` itself (`🔍️browser-probe.ts:2750-2760`:
  `scale === [0,0,0]`), so B29's `worldHiddenAfter` carrying `seed-left-001` PROVES the world body
  republished for `setSelectionFlag` on the same build where relocate's `data-instances-json` never
  moved. The two verbs share a scope class, so the dropping hop cannot be the scope.
- The three-way split across B29's own lanes is the discriminator, and it is NOT the scope class:

  | verb | retained work (`build_tool_job`, `✏️editor/🦀️.rs:7798-7845`) | Emit carries `ui_scope` | B29 verdict |
  | --- | --- | --- | --- |
  | `setSelectionFlag` | `BoundedFirstStepCommandWork` → `puzzle3d_retained_reduce` → `handle_action_impl` | yes (`:3456`) | world lane APPLIES, panel body never re-taken |
  | `addTargetVolume` | `Puzzle3dWindowCommandWork` | yes | `volume-brush-add-target-volume` **PASS** 6.2 s |
  | `addObjectKind` | `Puzzle3dAddObjectKindWork` | yes (`:4368`) | `catalogue-add-object-kind` **PASS** 6.2/14.2 s |
  | `translateSelection` | `Puzzle3dScaleWork` | yes (`:4586`), plus `coalesce_key: "gumball-translate"` | `gumball-scene-delta` FAIL 30.2 s |
  | `worldRelocate` | `Puzzle3dWorldRelocateWork` | yes (`:5141`), no coalesce key | `relocate-pose-delta` FAIL 30.0 s |

- Two candidate mechanisms read and KILLED before measuring, so the report does not repeat them as
  hypotheses:
  - **attraction resolution snapping the pose back** — the default 3d example is `concrete-forest`
    (`✏️s/🔌️plugins/🧩️puzzle/🔣️.json`, `manifest.examples[2]`): ONE object `seed-left-001` at
    `origin [0,0,0]`, `attractions: []`, `locked:false`, `hidden:false`. With no attractions
    `resolve_puzzle3d_attractions` is a no-op, so it cannot be eating the move on these lanes.
  - **`coalesce_key` → `ArtifactCommand::AmendLast` leaving the app's projection cache stale** —
    `refresh_cache` keys on `self.store.generation()` (`🔌️plugin/🦀️.rs:21409`), and
    `ArtifactStore::amend_command` calls `bump()` on BOTH its amend and its fresh-edit branch
    (`🏪️store/🦀️.rs:16947, 16970`), which increments `generation`. So an amended edit does invalidate
    the snapshot; a stale projection is not the mechanism. (`AmendLast` is also
    `ArtifactProjectionCause::Apply`, `🏪️store/🦀️.rs:2747`.)
- The guest's refresh is a REAL re-render, not a mirror read: `plugin_refresh_ui`
  (`🔌️plugin/🦀️.rs:32557`) calls `instance.app.render(entry.body_key, …)` per requested window/panel and
  omits `value` only when the freshly rendered tree's hash equals the host's cached hash
  (`ui_refresh_section`). So "the guest answered unchanged" and "the host never asked" are the only two
  shapes a frozen body can have, and they are distinguishable in one reading.

---

## 1 The headline: there is no dropping hop. The world lane publishes — in 10–14 s.

Measured live on `:6013` (wasm #51), one fresh page, diagnostics armed, three Document-scope verbs
dispatched through the world host's own `onAction` via a temporary `__b32Dispatch` tap, each polled to
`data-instances-json` on `#puzzle3d-main-perspective`
(`🗑️generated/b32-tap-2026-09-12T03-45-27.txt`, probe `🔍️b32-completion-tap.ts`):

| verb | args | `data-instances-json` before → after | waitedMs |
| --- | --- | --- | --- |
| `translateSelection` | `{ids:[seed-left-001], mode:mesh, dx:7}` | `position [0,0,0]` → **`[7,0,0]`** | **10 285** |
| `worldRelocate` | `{objectId:seed-left-001, position:[11,4,0]}` | `[7,0,0]` → **`[11,4,0]`** | **14 417** |
| `addTargetVolume` | `{origin:[3,3,0]}` | `data-target-volumes-json` `[]` → **one `target-volume-0`** | **11 642** |

All three landed. So hypotheses (a), (b) and (c) of the assignment are all FALSE as stated:

- **(a) the completion carries no world/document scope** — every one of them does. Quoted verbatim from
  `translateSelection`'s own completion:
  ```
  [DEBUG] completion apply {"operation":256,"scope":{"kind":"partial","measures":true,
    "panelBodies":["puzzle.3d.play.inspector","puzzle.3d.play.document","framework.body.history","puzzle.3d.play.kinds"],
    "windowBodies":["puzzle3d.play.composite"]},"refresh":{…same…},"historyPatch":true,…}
  ```
  `typedOperationCompletionRefreshV1` returned the mutation's scope VERBATIM (`refresh` === `scope`),
  never `null`, on every mutating completion in every run. B27's rule is not the defect.
- **(b) the guest answers the world lane as unchanged** — it answers with a VALUE. Host tap on
  `refreshUi`'s response: `answeredWindows:["puzzle3d-main=value","puzzle3d-main-top=value","puzzle3d-main-perspective=value"]`,
  `answeredPanels:["framework.panel.artifact=value",…]`. And the guest's refresh is a real re-render per
  requested body (`plugin_refresh_ui`, `🔌️plugin/🦀️.rs:32557`), so "unchanged" would have been a genuine
  content answer, not a mirror read.
- **(c) the host refresh never runs** — it runs, it is never superseded, and it is never lost. A second
  tap logged the generation guard at `🏛️ShellHost/🟦️.tsx:4571` across a whole gumball lane:
  `superseded:false` on **11 of 11** passes, generations 1→11 contiguous. The coalescing loop
  (`🏛️ShellHost/🟦️.tsx:4762-4772`) drains its own `uiRefreshOwedRef` after each pass, so an owed
  follow-up is never dropped either.

## 2 The hop table, in order, for one gumball `translateSelection` (the PASSING run)

`🗑️generated/probe-2026-09-12T04-00-10.md`, `--only=gumball-drag`, `gumball-scene-delta` **PASS**
(`sceneDelta=true poseAfterLen=282`), with the completion log forced on for the reading:

| # | hop | evidence |
| --- | --- | --- |
| 1 | host dispatches | `[DEBUG] gumball pose delta {action: translateSelection, ids: Array(1), mode: mesh}` |
| 2 | a refresh pass answers | `b32 refreshUi answered {"superseded":false,"generation":10,"current":10,"windows":["puzzle3d-main=value","puzzle3d-main-top=value","puzzle3d-main-perspective=value"]}` |
| 3 | leftover interaction view republishes | `leftover InteractionView {"selectedIds":["seed-left-001"],"gumball":true}` |
| 4 | two viewport-scope completions pass through | `completion apply {"operation":704,"scope":{"kind":"partial","windowBodies":["puzzle3d.play.composite"]},"refresh":{…same…},"historyPatch":false}` and `operation":768` identically |
| 5 | a FULL completion | `completion apply {"operation":832,"scope":{"kind":"full"},"refresh":{"kind":"full"},"historyPatch":false}` |
| 6 | the document edit lands | `history patch applied {"replace":false,"currentCursor":3,"patchCursor":12,"upserts":1,"labels":["move-object id=seed-left-001 new-origin=0,0,-1.9244830755406204"],"canUndo":true}` |
| 7 | **the mutation's own Document completion** | `completion apply {"operation":896,"scope":{"kind":"partial","measures":true,"panelBodies":["puzzle.3d.play.inspector","puzzle.3d.play.document","framework.body.history","puzzle.3d.play.kinds"],"windowBodies":["puzzle3d.play.composite"]},…}` |
| 8 | another FULL completion | `completion apply {"operation":960,"scope":{"kind":"full"},"refresh":{"kind":"full"},…}` |
| 9 | the world lane moves | `gumball sceneDelta=true poseAfterLen=282` |

Nothing is dropped anywhere in that chain. What the table shows instead is the **cost**: in this one
80 s lane, **16 completions, 3 of them `{kind:"full"}` (every window body + every panel body + labels +
measures) and 12 of them viewport-scope (all three world bodies), and ZERO answered `refresh:null`** —
against **11 refresh passes, 3 of which answered every world body `unchanged`** and **24
`interactionSelect`/`interactionHover` ingresses**.

## 3 Root cause: the mutation's refresh pass queues behind a refresh storm, and B27's `null` gate never fires any more

Two named, measured storm generators. Both are declared-scope defects; neither is a dropped hop.

### 3.1 `registerBrushMesh` republishes the world body on every REPEAT refusal — FIXED here

`✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs:96-99` (`request_reupload`) widened the arm's declared
`Puzzle3dScopeClass::Quiet` scope to `puzzle3d_viewport_scope()` **unconditionally**, while
`Puzzle3dCollision::request_mesh_reupload` (`✏️editor/⏳️precompute/🦀️.rs:1824`) is idempotent and
sorted — a re-announcement of an already-pending id leaves the published `meshReuploadUrls` array
byte-identical. So every re-announcement of the same dead id republished a world body that had nothing
new to say, and the republished body re-drives the client's `BrushMeshRegistrar`
(`🌐️World3dHost/🟦️.tsx:5122-5134`), which re-announces. Measured on the run where an object was hidden
(`🗑️generated/b32-tap-2026-09-12T03-26-48.txt`): **21 `registerBrushMesh` ingresses, 23 of 33
completions carrying that viewport scope, only 2 answering `refresh:null`, and 23 of 31 refresh passes
answering every world body `unchanged`** in a 75 s window with four user actions.

**Fix** — `request_mesh_reupload` now answers whether the standing set actually GREW, and the scope is
widened only then:

```rust
fn request_reupload(ctx: &mut Puzzle3dActionCtx<'_>, url: &str) {
    if ctx.app.precompute.borrow_mut().request_mesh_reupload(url) {
        *ctx.ui_scope = crate::editor::puzzle3d::puzzle3d_viewport_scope();
    }
}
```

### 3.2 Every pick AND every hover costs a FULL shell refresh — NOT fixed here, and it is now measured

`🔌️plugin/🦀️.rs:22445` `dispatch_interaction_action` — the one place any app's selection or hover
mutates — ends every one of the six `INTERACTION_ACTION_IDS` (`interactionSelect`, `interactionHover`,
`clearSelection`, `selectAll`, `setSelectionMode`, `setInteractionGranularity`) with
`UiDirtyScope::Full`. Its own docstring says so and says why:

> "and always returns `UiDirtyScope::Full` … narrower scoping would need per-window interaction-domain
> bookkeeping this wave doesn't build"

`interactionHover` fires on pointer motion. That is the single largest refresh source in the shell —
24 interaction ingresses and 3 `{kind:"full"}` completions in one 80 s gumball lane — and it is what
the Interactive-lane mutation's own pass queues behind. It is framework-owned and cross-app
(`puzzle3d` already declares the right answer for its own half:
`Puzzle3dScopeClass::Selection` → `puzzle3d_selection_panel_bodies()`), so narrowing it properly means
giving `ArtifactApp` a declared interaction scope the framework asks for — a wave of its own, named and
handed over rather than half-done here.

## 4 The three browser reds, re-measured and re-attributed

| lane | verdict this wave | attribution |
| --- | --- | --- |
| `--only=outliner-rows` | `outliner-hide-applies` **PASS waitedMs=13365**, `outliner-show-restores` **PASS** | B29 handover item 1 is a LATENCY red, not a regression: the same 10–14 s the tap measures, against a 30 s budget under battery contention |
| `--only=gumball-drag` | `gumball-handle-enter` PASS; `gumball-scene-delta` **FAIL waitedMs=30417** then **PASS** on the next run (`poseAfterLen=282`) | intermittent at the 30 s budget; the commit always lands (`move-object … new-origin=13.0602721003248` / `0,0,-1.9244830755406`), and §2 shows the publication chain intact. `battery PASS=5 FAIL=1 FAULTS=0` both runs |
| `--only=relocate` | `relocate-arm` PASS, `relocate-no-hard-fault` PASS, `relocate-pose-delta` **FAIL waitedMs=30584** | **not a publication defect at all**: the string `worldRelocate` occurs exactly ONCE in the whole probe stream — as the `activeUtility` verdict text. The lane's drag never dispatched the verb. `beginRelocateDrag` (`🌐️World3dHost/🟦️.tsx:5832`) needs `resolveClickInstanceId` to hit the mesh under the press and `world3dRelocateDragTargetV1` to find that id inside the live selection; the lane's press point satisfies neither. Same family as B28's gumball-handle geometry — B31's lane, named here with proof. The verb itself publishes in 14.4 s (§1) |

## 5 The `Agent disconnected` presence banner is not a mutation-induced flip

`"Agent disconnected"` is the `AgentBridge` presence label
(`🔗️AgentBridge/🟦️.tsx:39`, `os.agent.presence.disconnected`). The bridge's status starts at
`IDLE_PRESENCE`/`disconnected` (`:286`) and only leaves it when a ShellBridge WebSocket to an MCP
gateway opens (`:407-409`) — there is no gateway on a dev serve, so `disconnected` is the STEADY state.
Measured: my tap's whole 75 s window across four mutations produced **zero** presence/disconnect/detach
console lines, and the panel-tab census taken at BOOT, before any dispatch, already carried
`{"id":"s-sync-status","text":"Remote: detached"}`. B29 read a boot-time banner in the same
`chromeState()` sample as the drag and hypothesised a presence-scope defect; the probe's own
`COLLATERAL_NOTICE_RE = /agent disconnected|remote: detached|reconnect/i`
(`🔍️browser-probe.ts:93`) had already classified it as collateral. **No presence defect. Nothing to
fix.**

## 6 Laws

`✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs` —
`an_identity_this_guest_cannot_serve_becomes_a_request_for_the_bytes` now pins the return contract, and
`the_residency_counter_only_climbs_and_the_request_set_is_bounded` pins it at the bounded-set ceiling and
for an over-long id.

`✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes`
gains the arm that is the whole fix: a SECOND refusal of the same id answers `UiDirtyScope::None` and
leaves the published `meshReuploadUrls` byte-identical.

Green with the fix:

```
test editor::puzzle3d::component::tests::an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes ... ok
test editor::puzzle3d::precompute::component::tests::an_identity_this_guest_cannot_serve_becomes_a_request_for_the_bytes ... ok
test editor::puzzle3d::precompute::component::tests::the_residency_counter_only_climbs_and_the_request_set_is_bounded ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 720 filtered out; finished in 0.46s
```

Red with the widening put back to unconditional (same three laws, same command):

```
thread '…an_id_only_announcement_this_guest_cannot_serve_asks_for_the_bytes' panicked at ✏️editor/🧪️tests/🔬️unit/🦀️.rs:5780:5:
a re-announcement of an ALREADY standing request republishes nothing and must stay Quiet; got Partial { window_bodies: ["puzzle3d.play.composite"], panel_bodies: [], utilities: false, tools: false, engagements: false, measures: false, labels: false }
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 722 filtered out; finished in 0.19s
```

## 7 Verification (all foreground)

```
cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly
    Finished `dev` profile [unoptimized] target(s) in 0.71s          (0 errors; 88 pre-existing warnings)

cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2
    Checking semio-s-plugin-puzzle v0.1.0
    Finished `dev` profile [unoptimized] target(s) in 4.78s

RUST_MIN_STACK=134217728 cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- --test-threads=1
    test result: FAILED. 711 passed; 12 failed; 0 ignored; 0 measured; 0 filtered out; finished in 74.28s
```

The whole-suite `711 passed; 12 failed` is **byte-identical to B31's recorded baseline**
(`📓️2026-09-12-wave-B31-delete-abort-gumball.md`:321 `711 passed; 12 failed` — "zero new reds") — so
**zero new failures from this wave**. Each of the 12 passes in isolation (checked:
`the_catalogue_opens_its_object_kinds_and_folds_the_template_catalogs … ok, 1 passed`), i.e. they are the
known order-dependent family B33 is bisecting.

No renderer-react vitest lane and no `tsc --noEmit` run is quoted because **this wave's net TypeScript
diff is zero**: both host taps (`__b32Dispatch` + the `dispatchBrushMesh` announce log in
`🌐️World3dHost/🟦️.tsx`, and the `refreshUi` request/response + generation-guard logs plus the forced
`completion apply` in `🏛️ShellHost/🟦️.tsx`) were measurement instruments and are removed — verified by a
repo-wide sweep for `[DEBUG] b32`, `__b32Dispatch` and `true || runtimeDiagnosticsEnabled`, which returns
nothing outside the ticket folder.

The live after-measurement of §3.1 rides the guest wasm build (#52 was building during this wave —
`🗑️generated/build-2026-09-12-52.txt`); the laws are what pin it until then.

## 8 Files

Product (guest, rides a wasm build):

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️register-brush-mesh/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🦀️.rs`

Laws:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`

Probe (this wave owns it; B31's `🔍️browser-probe.ts` was RUN, never edited):

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️b32-completion-tap.ts`

Host: **unchanged** (taps added for the measurement and removed; zero net diff).

## 9 Handover

1. **The highest-value remaining fix is §3.2**: `dispatch_interaction_action`
   (`🔌️plugin/🦀️.rs:22445`) returns `UiDirtyScope::Full` for all six `INTERACTION_ACTION_IDS`,
   `interactionHover` included, so pointer motion repaints the whole shell. Measured: 24 interaction
   ingresses and 3 full completions per 80 s lane. The clean fix is a declared per-app interaction
   scope (`puzzle3d` already has `Puzzle3dScopeClass::Selection`), not a hard-coded narrowing.
2. **`relocate-pose-delta` is a press/hit-test red, not a publication red** (§4): the lane never
   dispatches `worldRelocate`. Start at `beginRelocateDrag` (`🌐️World3dHost/🟦️.tsx:5832`) /
   `world3dRelocateDragTargetV1` (`:4069`), same family as B28's gumball-handle geometry. The verb
   itself is proven live at 14.4 s.
3. **`gumball-scene-delta` is intermittent at the 30 s budget, not broken** (§4). Whoever owns the
   probe should raise that lane's budget or assert on the history row plus the world lane separately —
   a mutation on this shell costs 10–14 s at HEAD.
4. **`outliner-hide-applies`/`outliner-show-restores` are GREEN** (13.4 s) — B29 handover item 1 can be
   closed as a budget artifact.
5. **`worldRelocate`'s locked/hidden refusal already answers with a notice** (`✏️editor/🦀️.rs:5190`,
   `Puzzle3dWorldRelocateStage::Object`) — confirmed live: dispatching it at a hidden object produced
   `completion apply {"operation":448,"scope":{"kind":"none"},"refresh":null}` plus one `Notify`
   effect. Nothing owed there.
6. **`typedOperationCompletionRefreshV1` needs no change** (§1c) — but note that its `null` arm now
   almost never fires (0 of 16 in the gumball lane, 2 of 33 in the worst tap run) because the
   background verbs above hand it real scopes. Its no-storm property is only as good as the scopes the
   app declares, which is what §3.1 fixed and §3.2 still owes.
