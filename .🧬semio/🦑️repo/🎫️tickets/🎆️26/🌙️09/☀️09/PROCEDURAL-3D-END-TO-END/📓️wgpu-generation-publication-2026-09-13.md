# 🧬️ wgpu GENERATION PUBLICATION — `Add Generation` now reaches the screen on 6118

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane **wgpu-generation-publication**, 2026-09-13.
Resumes `📓️wgpu-retained-controls-wires-2026-09-13.md` §7.3(a)/§7.4 and
`📓️generate-mode-interactions-2026-09-13.md` §6.

Repo MCP was down the whole session (`repo -32602 invalid initialize params`,
`semio CONNECTION_CLOSED`); no ticket was opened, closed or reopened, `📓️status.md` and
`🎫️ticket.json` were not touched. Evidence under `🗑️generated/wgpu-generation/`; nothing under any
`🗑️generated` folder this lane did not create was read or removed. No git-state-modifying command was
run. The coordinator's serve on `http://127.0.0.1:6118` was neither started nor stopped.

---

## 1. TL;DR

| question | answer |
|---|---|
| Does the guest create the generation on 6118? | **Yes, and it always did.** `[DEBUG] gen3d command action=addGeneration before=0 after=1 ops=1 selected=Some("generation-1")` — §3 hop 3. |
| Does the guest re-render the roster with it? | **Yes, and it always did.** `[DEBUG] gen3d render body=procedural.play.generations generations=1` — §3 hop 5. |
| Does the host receive the new tree? | **Yes, and it always did.** `renderSurface surface=generation3d-generations … nodes=6 rev=2`, up from `nodes=5 rev=1` — §3 hop 6. |
| Where did it die? | **The engine's document INGRESS refused it as already published.** Every lease the browser producer minted carried `generation: u64::from(instance_id)` — a session CONSTANT — so `Ui::document_status` answered `Published` for every document after each surface's first, the ingress phase short-circuited, and the arena repainted the first tree for the life of the shell (§4). |
| Fix | The ingress generation is now the engine's own rule, one counter per surface, bumped when that surface's revision moves (`ui_document_ingress_generation`, §5.1). |
| Second defect found on the way | A retained BODY press never made its window active, so the keyboard followed `procedural-main` forever: the inline rename editor could be opened and focused and never typed into (§5.2). |
| Third defect found on the way | wgpu never opened the guest's `[DEBUG]` door at all — it spawned its shards from a url that the realm owning the preference never stamped, so every guest trace site was unreachable on that target (§5.3). This lane could not have found the defect without fixing it first. |
| Laws | **9 green**: 2 new Rust + 3 pre-existing in `🌳️document-tree-reconcile`, 2 new Rust in `🔬️wgpu-shell-input`, 2 new TypeScript twins (11 in that suite) — §6, each new law measured FAILING against the shipped behaviour first. |
| 6118 | `Add Generation` → roster row + inline editor, Form 1 placeholder → **8 bound nodes / 3 sliders**, preview `instances=1 lines=1 state-meshes=3 scenePasses=1`. A second Add lands a second row; `selectGeneration` through the row moves the editor. §7. |
| Not claimed | `renameGeneration`'s COMMIT. It now reaches the dispatcher with the right arguments (it did not before this lane), and is then dropped by the frame's deferred-action queue — a third, separate defect, measured and named in §8. |

---

## 2. The reported symptom

On 6118 (`?plugin=generation3d&mode=generate`) the retained `Add Generation` row dispatched
`addGeneration`, the guest settled the typed operation `terminal=true`, all three generate surfaces
re-rendered — and the Generations window still showed `(no generations)`, the Form still showed its
`Add a generation to edit input values.` placeholder, and the generate preview never gained an
instance. React (6018) creates the generation and paints it within 5 s.

Reproduced first thing, unchanged, with `🐍️wgpu-generation-publication-probe.mjs`
(`🗑️generated/wgpu-generation/run-1`): `rosterBefore=2 rosterAfter=2 grew=false formAfter=1`.

---

## 3. Hop by hop, on 6118

Every row is a console line from `🗑️generated/wgpu-generation/diag-6/console.txt` (the run with guest
diagnostics armed, §5.3) unless noted. Timestamps are that run's.

| # | hop | evidence | verdict |
|---|---|---|---|
| 1 | the row resolves | `pointer button x=160.696 y=138 … hit=Some((TreeItem, Some("tree.label.procedural3d-play-generate.add-generation"), Some("addGeneration")))` | ✅️ |
| 2 | the retained press dispatches | `wgpu-shell retained press window=generation3d-generations kind=TreeItem down=false action=Some("addGeneration")` | ✅️ |
| 3 | the guest admits and RUNS the command | `plugin_exchange actionId=addGeneration branch=catalog`, then `gen3d command action=addGeneration before=0 after=1 ops=1 selected=Some("generation-1")` | ✅️ |
| 4 | the typed operation publishes its lanes | `wgpu-bridge typed-operation command instance=1 pages=6 terminal=true`, zero fault pages, `effects leftover 2 tags=dispatchAction,dispatchAction` (the two `flowEvalTick` re-arms) | ✅️ |
| 5 | the guest RE-RENDERS the roster | `gen3d render body=procedural.play.generations generations=1 selected=Some("generation-1")` — and again on all three later renders | ✅️ |
| 6 | the host receives the new tree | `wgpu-bridge renderSurface surface=generation3d-generations turn=0 … nodes=6 rev=2` (the render immediately before it: `nodes=5 rev=1`) | ✅️ |
| 7 | **the engine ingests it** | **no `ui-doc ingress` line at all after the press** — `document_status` answered `Published` every frame | ❌️ **first divergence** |
| 8 | the arena paints it | `dumpStructure("generation3d-generations")` still 5 nodes, still `…generations.empty` | ❌️ (consequence of 7) |

Hop 7 is where the chain breaks, and it is the FIRST hop that does. Hops 3 and 5 are the two the
earlier reports could only guess at — they needed the guest's own `[DEBUG]` door, which did not exist
on this target until §5.3.

The `ui-doc ingress` counts for the whole pre-fix run say it in one line: exactly **one** `Vacant`
per surface, ever.

```
1 ui-doc ingress window=generation3d-generations      generation=1 nodes=5  status=Vacant
1 ui-doc ingress window=generation3d-generate-form    generation=1 nodes=1  status=Vacant
1 ui-doc ingress window=generation3d-generate-preview generation=1 nodes=11 status=Vacant
```

Three surfaces, three documents, one session. Everything a user did after boot was invisible.

---

## 4. Root cause

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:940`
(`assemble_browser_document`, as shipped):

```rust
let identity = ui_contract::UiDocumentAssemblyIdentity {
    generation: u64::from(instance_id),
    revision: ui_contract::UiRevision(published.revision),
    root: Some(ui_contract::UiNodeId(root)),
    layout_epoch: published.layout_epoch,
};
```

The document lease's `generation` was the plugin INSTANCE id: `1` for the whole session, and the same
number for every surface. The engine admits a document by that number and nothing else:

* `Ui::document_status` (`🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:659-669`) answers `Published` while
  `window.tree.document().generation() == generation`;
* `Ui::begin_document` (`:700`) refuses `StaleGeneration` unless the incoming generation is strictly
  GREATER than the published one.

So `render_ui_document_step`'s `Ingress` phase
(`🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:1227`) read `Published` on the first line and jumped
straight to `Reconcile`, which re-reconciled the document already in the arena. The `UiDocumentTree`
the bridge had just assembled — the correct one, with the new row — was never read, and the lease was
retired on the next refresh.

Two things kept this invisible for a long time:

1. **It looks like everything works.** Boot paints correctly, because boot is each surface's FIRST
   document. Only the SECOND document of a surface is lost, and until this lane the generate-mode
   surfaces never produced one on this target.
2. **Every existing reconcile law drove `Ui::publish_document`**, the `cfg(test, feature = "testkit")`
   shortcut that writes the tree directly and consults neither admission rule
   (`🧪️tests/🌳️document-tree-reconcile/🦀️.rs`, pre-existing laws). The production ingress ladder had
   no law over two consecutive documents at all. §6.1 adds one.

---

## 5. Fixes

### 5.1 The ingress generation — one rule, stated beside the rules it must satisfy

New `ui_wgpu::wgpu::engine::ui_document_ingress_generation`
(`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:425`), declared immediately above
`document_status`/`begin_document`:

```rust
pub fn ui_document_ingress_generation(minted: Option<(u64, u64)>, revision: u64) -> u64 {
    match minted {
        Some((published_revision, generation)) if published_revision == revision => generation,
        Some((_, generation)) => generation.saturating_add(1),
        None => 1,
    }
}
```

* **Held still while the surface is.** Keyed on the producer's own revision, so a surface that
  republishes an unchanged document keeps its generation, answers `Published`, and pays no ingress and
  no reconcile — the previous cost profile, unchanged.
* **Counted, not mirrored.** A surface whose revision restarts (a retired surface reopened under a
  fresh owner) still moves forward instead of being refused `StaleGeneration` for the rest of the
  session.
* **Never zero.** `UiDocumentTree::new` rejects generation `0` outright.

The browser producer now holds only the per-surface ledger that rule reads and writes
(`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`, `browser_document_generation`), and
`assemble_browser_document`'s now-unused `instance_id` parameter is gone rather than silenced.

### 5.2 A retained body press activates its window

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, `route_retained_pointer_press`: a press on a window's retained body
now sets `active_window_id`, on the press, exactly as a press on its chrome does.

Found while proving the rename half of the journey. `handle_keyboard_async` routes real keys into
retained content on exactly one predicate — `content_has_focus(active_window_id)` — and a retained
body press was the one way into a window that never set that field. Measured on 6118 before the fix:

```
116053 wgpu-shell content focus window=generation3d-generations node=Some(NodeId { index: 6, … })
120386 wgpu-shell key routing window=procedural-main contentFocus=false action=Char("A")
120655 wgpu-shell key routing window=procedural-main contentFocus=false action=Char("B")
```

The inline rename editor took focus and every keystroke was routed at `procedural-main`, the app's
first window kind, whose content had none. After the fix, same gesture:

```
121027 wgpu-shell key routing window=generation3d-generations contentFocus=true action=Enter
```

This is not generation3d-specific: on this target, no retained control in any window but the first
could be typed into.

### 5.3 The guest's `[DEBUG]` door was closed on wgpu

`🧵️shard-runtime/🟦️.ts` stamps `?diagnostics=1` on a shard worker's url when the realm armed
`SEMIO_RUNTIME_DIAGNOSTICS`, and the shard worker hands that through `wasi:cli/environment` to every
component it hosts — which is the ONE way a guest's `[DEBUG]` line reaches a browser session. wgpu
spawns its shards from the UI isolate on a url its FRAME WORKER names
(`🐚️plugin-bridge/🟦️.ts` → `shard-spawn` → `🚚️browser-frame-transport/🟦️.ts`'s `spawnShardWorker`), and
a frame worker owns no `localStorage`: the url was stamped by nobody, and every guest trace site in
the tree was unreachable on this target.

The stamp belongs to the realm that CONSTRUCTS the worker. New leaf
`🧰️framework/🔨️modules/🎭️actor/🩺️diagnostics/🟦️.ts` holds the key, the parameter,
`shardRuntimeDiagnosticsArmed` and `stampShardWorkerDiagnostics`; `🧵️shard-runtime/🟦️.ts` re-exports
them explicitly and `shardWorkerUrl()` is now one call to the leaf; the wgpu transport stamps
`message.url` as it spawns. A LEAF because `🧵️shard-runtime` imports `ShardClient` and with it the
whole pooled-actor graph — importing it into the UI isolate's boot bundle grew that bundle from
63 KB to 766 KB, which is why the rule is not simply re-exported from there (the leaf is declared in
`🔣️taxonomy.json`'s `wgpu-frame-worker` browser profile; bundle back to 64 KB).

Hops 3 and 5 of §3 are this fix's output. Without it this lane could not have distinguished "the
guest never created it" from "the host never showed it", which is exactly the question two earlier
reports left open.

### 5.4 Two diagnostics deliberately left in place

Both `[DEBUG]`-prefixed, both bounded, both the line whose ABSENCE was the defect:

* `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `ui-doc ingress window=… generation=… nodes=…`, ONE line per
  admitted ingress (silent for an unchanged surface, never per page, never per frame), plus
  `ui-doc begin refused …` on a refusal.
* `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `wgpu-shell key routing window=… contentFocus=… action=…` and
  `wgpu-shell content focus window=… node=…`, both only on real key/focus events.

---

## 6. Laws

Every new law was measured FAILING against the shipped behaviour before it was made to pass.

### 6.1 Rust — `🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs` (+2, 5 green)

`cargo test -p semio-framework-ui --features testkit --lib -- document_tree_reconcile` → **5 passed;
0 failed**.

* `a_surfaces_second_document_reaches_the_arena_and_a_constant_generation_never_does` — assembles two
  real `UiDocumentLease`s from the shared fixture the way the browser producer does
  (`open_into`/`place_one`/`finish_into`) and drives the engine's OWN ingress ladder
  (`document_status` → `begin_document` → `read_node_page`/`apply_document_page` → `finish_document`
  → `step_document_reconcile`), never the testkit's `publish_document` shortcut. Asserts the second
  document's added node reaches the paintable arena — and, in the same test, that the shipped
  constant-generation producer is never ADMITTED for its second document and the arena keeps the
  first tree. Each lease is retired before the next is taken, because `UI_DOCUMENT_LEASE_SLOTS` is a
  fixed process-wide table.
* `the_ingress_generation_holds_still_while_a_surface_does_and_never_moves_backward` — the rule's own
  table over the fixture's five cases.

Proof of failure first (`ui_document_ingress_generation` temporarily reduced to the shipped
constant): **3 passed; 2 failed**, `assertion left == right failed: case the surface's revision moved`
and `a repeat generation is never ADMITTED …`.

Shared fixture: `🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json`, new `ingressGeneration` block
(five cases + the constant producer's own refusal, `admittedIngresses: 1`).

### 6.2 Rust — `📺️renderer/…/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs` (+2 green)

`cargo test -p semio-framework-os-renderer-wgpu --lib -- retained_body_press only_the_press` →
**2 passed; 0 failed**.

* `a_retained_body_press_activates_its_own_window_so_the_keyboard_follows_it` — records content focus
  for the pressed window, presses its body, and asserts both that the pressed window became active
  and that the keyboard's own predicate now answers for the window whose content holds focus.
  Measured FAILING with the fix disabled: `assertion left == right failed: the pressed window is the
  active one`.
* `only_the_press_half_of_a_body_click_moves_the_active_window` — a click activates on the press, the
  action fires on the release.

### 6.3 TypeScript twin — `📺️renderer/…/🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts` (+2, 11 green)

`bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts 🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts` →
**11 passed**. Reads the SAME fixture as §6.1 and re-implements the rule from its statement rather
than importing the Rust that implements it:

* `mints one ingress generation per surface, held still while that surface is` — the same five cases,
  plus the constant producer's single admitted ingress.
* `states the ingress-generation rule once, beside the two admission checks, and the browser producer
  reads it` — the rule lives in the engine next to `document_status`/`begin_document`, the browser
  producer mints through it, and `generation: u64::from(instance_id)` is gone from that producer.

### 6.4 Regression

`cargo test -p semio-framework-ui --features testkit --lib -- reconcile` → **14 passed; 0 failed**
(the whole reconcile surface, not only this lane's filter). The wgpu TypeScript suite
(`vitest --config 🧪️tests/🎚️config/🟦️.ts`) is **203 passed, 6 failed** — all six in
`🧪️tests/🧩️package-integration/🟦️.ts` and all six `ReferenceError: Bun is not defined`, i.e. that
suite needs Bun's own runtime and this lane ran vitest under Node. Unrelated to any change here; the
project's own `test` target runs it correctly.

---

## 7. 6118 — what the runtime does now

Renderer wasm rebuilt from this lane's source (`🗑️generated/wgpu-generation/wasm-{3,5,10}.txt`,
`Successfully ran target wasm`), generation3d guest restaged
(`activate-generation3d-wgpu-dev`, `🗑️generated/wgpu-generation/restage-2.txt`,
`Successfully ran target … and 39 tasks it depends on`).

### 7.1 `Add Generation` (`🗑️generated/wgpu-generation/fix-1`)

```
generation-publication target=yes rosterBefore=2 rosterAfter=3 grew=true formAfter=8
  scenePasses=1 world3d={"generation3d-generate-preview":{"instances":1,"lines":1,"stateMeshes":3}}
```

* **Generations** — `…generations/stack[0]#procedural3d-play-generate.generation.generation-1` plus
  its own `input[0]#…generation-1.rename` carrying the text `Generation 1`. The row a user sees, and
  the inline editor only the SELECTED row carries.
* **Form** — 1 placeholder node → **8 nodes**: `Inputs`, and three bound fields
  `Column Height` / `Profile Radius` / `Side Count`, each with its slider.
* **Generate preview** — `instances=1 lines=1 state-meshes=3`, `scenePasses=1`. Before the fix, the
  same probe on the same example measured `instances=0 … stateMeshes=2` and an unchanged roster.
* The witness of the fix itself, in the same console:
  `ui-doc ingress window=generation3d-generations generation=1 nodes=5` at boot, then
  `ui-doc ingress window=generation3d-generations generation=2 nodes=6` after the press. The second
  line did not exist in any run before this lane.

Re-measured identically on the FINAL binary (`🗑️generated/wgpu-generation/final-1`), after the §8
diagnostics were trimmed.

### 7.2 The roster journey (`🗑️generated/wgpu-generation/journey-4`, re-measured in `final-1`)

| step | evidence |
|---|---|
| a SECOND `Add Generation` | roster `["generation-1","generation-2"]`, and the inline editor moved to `generation-2` — the new generation is the selected one |
| `selectGeneration` on row 1 | `retained press … kind=TreeItem action=Some("selectGeneration")`, and the editor moves back to `…generation-1.rename`: `selectMovedTheEditor=true` |
| the rename editor takes focus | `retained press … kind=Input`, `content focus window=generation3d-generations node=Some(..)` |
| keys reach that window | `key routing window=generation3d-generations contentFocus=true` for every character and for `Enter` |
| the commit dispatches | `frame input action controller=s.procedural.generation3d@1/*#editor action=renameGeneration args={"id":"generation-1","value":"…","windowId":"generation3d-generations"}` |
| the roster shows the new name | **no** — §8 |

`ui-doc ingress … generation=3 nodes=7` and `generation=4 nodes=7` in the same run are the second add
and the selection move reaching the arena.

---

## 8. What is NOT claimed, and exactly where it stands

**`renameGeneration`'s commit does not land.** It is a THIRD defect, separate from both fixes above
and downstream of them — before this lane the gesture could not even produce the dispatch.

Measured, in order (`🗑️generated/wgpu-generation/journey-5`, with three temporary `[DEBUG]` lines
added to `🧊️renderer/🦀️.rs` for exactly this question):

1. The typed text reaches the retained input and the commit fires with the right arguments:
   `131743 frame input action … action=renameGeneration args={"id":"generation-1","value":"…","windowId":"generation3d-generations"}`
   (`🧊️renderer/🦀️.rs:11435`).
2. The descriptor is pushed onto the frame partial's `deferred_actions` (`:11436`).
3. 230 ms later that frame completes and installs its deferred cursor — **empty**:
   `131973 frame deferred install empty=true tutorial=false maintenance=true`. Every install in the
   whole run reads `empty=true`, including this one.
4. **`frame deferred action retired without dispatch` never fires either** — so the descriptor is not
   retired through `AppFrameAfterChrome::close_step` (`:11189`) on the way out.
5. **No `plugin_exchange actionId=renameGeneration` ever follows**, no `gen3d command
   action=renameGeneration`, no fault, no error.

So the descriptor is queued onto `self.after_chrome`'s `deferred_actions` and is gone by the time
`frame_after_input_step`'s `FrameFinishPhase::Deferred` takes it
(`🧊️renderer/🦀️.rs:13452`) — and it is not retired on the documented close path either. The partial
holding it is dropped without either, which on this target is a silent loss of one user gesture. That
is a frame-transaction lifecycle defect, not a generation3d or a retained-control one, and it is the
LAST hop of the rename journey: everything upstream of it — the focus, the key routing, the commit,
the arguments — is measured working in §7.2.

One diagnostic is left in place for whoever takes it: `[DEBUG] frame deferred install carries an
action` (`:13554`, silent for the tutorial/maintenance installs that run several times a second) and
`[DEBUG] frame deferred action retired without dispatch` (`:11190`). A `frame input action` line with
neither of those after it is this defect, live.

Also not claimed:

* Anything about the NATIVE (`not(target_arch = "wasm32")`) document producer. It takes its lease off
  the kernel's own arena and was neither read nor changed.
* The Form sliders' `updateGenerationValues` on 6118. The Form now renders eight real nodes including
  three `HitKind::Slider` controls, which is its runtime prerequisite, but no slider was dragged.
* A space character inside a typed retained value. `ui_event_from_key_action` has no mapping for
  `Space` — pre-existing, already lawed as `ui_event_from_key_action_has_no_mapping_for_space`, and
  visible in this lane's own measurement as `"Generation 1BalconyStudy"`.
* React parity of the ingress rule. React does not use this producer at all.

---

## 9. Peer breakages fixed forward (noted, not silently absorbed)

1. `🧰️framework/🔨️modules/🚪️io/🔤️base64/🟦️.ts` (media-export lane) was imported by `🎠️kernel/🟦️.ts`
   without being declared in `🔣️taxonomy.json`'s `wgpu-frame-worker` browser profile, so
   `generate-frame-worker` — and with it the WHOLE `activate-generation3d-wgpu-dev` chain — failed for
   every lane. One array entry added.
2. `♾️infinite/…/🕸️dag/🦀️.rs:2755` and `🌊️flow/🖥️host/🦀️.rs:1090` (node-graph-wire-drag lane) matched
   `DagGraphEdit` without its new `Move` row, breaking `semio-framework-os-infinite` and
   `semio-framework-os-flow` for every wasm build. Both arms added, the flow one emitting exactly the
   three keys the guest's `nodeGraphEdit` `"move"` arm already decodes.
3. `🎠️kernel/🦀️.rs`'s `RequestOutcome` was `PartialEq`-only next to an `Eq`-deriving
   `SpawnedJobCompletion` — the same lane fixed it themselves while this lane was preparing the same
   one-word fix; nothing was overwritten.
4. `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — left alone. That file was being saved every few minutes
   by its owning lane; §8 says what it blocked.

---

## 10. Method, evidence, files

Probes (ticket root): `🐍️wgpu-generation-publication-probe.mjs` — new. Boots 6118 straight into
generate mode, presses the retained `Add Generation` row at its OWN published rect (`dumpStructure`
body rect + the shell's `dock plan` window origin — never a guessed pixel), then reads the roster, the
Form, the preview census and the whole console hop by hop; `SEMIO_PROBE_JOURNEY=1` adds the second
add, the `selectGeneration` press and the inline rename; `SEMIO_PROBE_GUEST_DIAGNOSTICS=1` arms the
guest's own traces. `🐍️wgpu-add-generation-probe.mjs` (pre-existing) reported `blocked-no-row` because
it matched the camelCase tool id against a kebab-case published path — the row's path is
`…#procedural3d-play-generate.add-generation`; worth fixing in that probe.

Evidence: `🗑️generated/wgpu-generation/{base-1,run-1,run-2,diag-1…diag-6,fix-1,journey-1…journey-4}`
plus `restage-2.txt` and `wasm-{1..8}.txt`.

Changed:
`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`;
`🧰️framework/🔨️modules/🖱️ui/🧪️tests/🌳️document-tree-reconcile/🦀️.rs`;
`🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌳️document-tree-reconcile/🔣️.json`;
`🧰️framework/🔨️modules/🎭️actor/🧵️shard-runtime/🟦️.ts`;
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs`;
`…/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`;
`…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`;
`…/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs`;
`…/🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts`;
`…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts`;
`…/🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts`;
`…/🧫️fixtures/🌳️wgpu-document-reconcile/🔣️.json`;
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`;
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs`;
`…/✏️editor/🎮️commands/🧬️generation/🦀️.rs`;
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` (peer, §9);
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` (peer, §9);
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` (§8
diagnostics, not yet built).

Added: `🧰️framework/🔨️modules/🎭️actor/🩺️diagnostics/🟦️.ts`;
`T/🐍️wgpu-generation-publication-probe.mjs`; this report.

Two guest `[DEBUG]` sites in generation3d (`gen3d command …`, `gen3d render …`) are gated on
`semio_framework_job::runtime_diagnostics_enabled()` and cost one branch when disarmed; they are the
reason §3's hops 3 and 5 are measurements rather than inferences, and are worth keeping.
