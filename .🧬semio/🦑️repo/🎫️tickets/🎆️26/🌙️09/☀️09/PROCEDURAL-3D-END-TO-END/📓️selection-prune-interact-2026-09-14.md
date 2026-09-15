# Selection pruning on example switch — generation3d React, 2026-09-14/15 (lane `selection-prune-interact`, port 6025)

Scope: §4.4 of `📓️react-oracle-hardening-2026-09-14.md` — a selection made in one bundled example
surviving into every later one, the plain click that then failed to REPLACE, the Inspection panel
describing a widget no longer in the document, and the `orbit`/`hover` rows of the same lane.
Everything below ran on `http://127.0.0.1:6025/?plugin=generation3d` (direct vite serve,
`SEMIO_VITE_HMR=0`, `📜️serve-generation3d-react-6025.sh`). Evidence under
`🗑️generated/react-interact/`.

---

## 1. Which half kept the stale id

**The HOST kept it. The guest was already pruning correctly.** That is the question §4.4 left open, and
the probe's new `data-guest-selection-json` lane answers it outright.

Baseline run, `🗑️generated/react-interact/results-before.json` (2026-09-14 18:57–18:59, before any
change in this lane). Per example: the two selection lanes read immediately after the example switch,
then after one plain click on the geometry the probe had just hovered.

| after switching to | `data-selection-json` (pane PAINTS) | `data-guest-selection-json` (GUEST sent) |
|---|---|---|
| Face Sweep Extrude | `["shell@solid"]` | `[]` |
| Sphere Cut With Torus | `["extrude@solid"]` | `[]` |
| Sphere Box Fuse | `["brep_bool_cut_5@solid","extrude@solid"]` | `[]` |
| Rectangle Wire Preview | `["fuse@solid","extrude@solid"]` | `[]` |
| Box Fillet Preview | `["extrude@solid"]` | `[]` |

| plain REPLACE click in | hovered | pane PAINTS | GUEST sent |
|---|---|---|---|
| Sphere Cut With Torus | `brep_bool_cut_5@solid` | `["brep_bool_cut_5@solid","extrude@solid"]` | `["brep_bool_cut_5@solid#0"]` |
| Sphere Box Fuse | `fuse@solid` | `["fuse@solid","extrude@solid"]` | `["fuse@solid#0"]` |
| Box Fillet Preview | `fillet@solid` | `["fillet@solid","extrude@solid"]` | `["fillet@solid#0"]` |

The guest's own lane is EXACTLY one id in every row — the merge is a REPLACE and it worked. The pane
published two, because the host's leftover overlay
(`mergeWorldSelectionWithLeftoverV1`, `🌐️World3dHost/🟦️.tsx`) replaced the guest's list with its own,
and its own still carried the pick from a document that no longer exists.

### 1.1 Why nothing ever retired it

The host overlay is published from exactly two kinds of place
(`publishLeftoverWorldSelectionV1` callers in `🏛️ShellHost/🟦️.tsx`): the leftover `interactionView`
a framework-reserved PICK answers with, and a utility/tool arm. A command that replaces the whole
document is neither.

It cannot become one, either — measured, not assumed. A temporary `[DEBUG] leftover peel` log in
`applyLeftoverInteractionView` (`🏛️ShellHost/🟦️.tsx`, removed again) recorded what every response
actually carried on that lane (`🗑️generated/react-interact/peel/console.txt`):

```
3796  leftover peel {"hasOutput":true,"keys":["generation","operationId"],"published":null,"addressedWindowId":"procedural-main"}   ← setActiveExample
37386 leftover peel {"hasOutput":true,"keys":["interactionView"],"published":{"selectedIds":["shell@solid"],…}}                    ← interactionSelect
45612 leftover peel {"hasOutput":true,"keys":["generation","operationId"],"published":null,…}                                       ← setActiveExample (the switch)
47032 leftover peel {"hasOutput":false,"keys":null,"published":null}                                                               ← its completion
```

`setActiveExample` is an interactive job: its synchronous answer is the admission
(`{operationId, generation}`, `🔌️plugin/🦀️.rs:24553`/`25867`/`26076`) and its real completion arrives
as a `TypedOperationCompletion` — a record of `operation`, `revision`, `ui_scope` and `history_patch`
(`🔌️plugin/🦀️.rs`, `take_typed_operation_completion`) with **no leftover `output` field at all**. So a
guest-side retirement has no channel to travel on for the route that needs it. (A first attempt in
this lane set `result.output` to a retiring `interactionView` in `dispatch_emit`; the peel log above
is the measurement that showed it is dropped for job-routed commands, and it was removed rather than
left half-wired.)

---

## 2. The fix — one law per cache, each on the layer that owns it

### 2.1 Host: the leftover cover is a cover over THIS pane's document

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`

New `leftoverWorldOverlayIdsInDocumentV1(ids, instances)`, applied inside
`mergeWorldSelectionWithLeftoverV1`. The pane knows which objects it is drawing, so it needs to ask
nobody: an overlay id offered by no instance of this pane — neither as the instance's own `id`, nor
as the coarser `interactionId` the topology declares, nor as the mesh it draws — is covering nothing
and is dropped, `gumballAnchorId` with it. A pane with **no** instances yet has no membership
information and keeps every id, the same rule the guest's topology pruning uses for a domain it has
no entry for. The cover the overlay exists for is never the one dropped: a just-picked id is by
construction an id of the instance whose geometry was clicked.

Also in that file: `leftoverWorldOverlayAppliesV1` now counts `selectionCleared`. An overlay whose
whole content is an ABSENCE — the guest's own retirement of a selection — answered "nothing to
overlay" and `mergeWorldSelectionWithLeftoverV1` returned the base record untouched, so the one
publication that exists to REMOVE a selection was the one the host could not act on. That was a
standing red in the engine contract (`first leftover pick keeps selection…`, line 8719) before this
lane arrived, and it is green now.

### 2.2 Guest: a document change prunes the state AND the leftover cache it is laid over

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`

New pure law `interaction_after_document_change_v1(validated, declared_domains, overlay, leftover_ids)`,
applied in `revalidate_and_persist_interaction_state` when
`origin == InteractionRevalidateOrigin::DocumentChange`. It closes two holes that the guest's own
readers fall into even though the browser never saw them on this app:

1. `protocol::validate_state` can only prune a domain the topology has an entry for, and
   `build_full_interaction_topology` builds entries for DECLARED domains only. The `vortex` mirror
   `overlay_leftover_ids_into_vortex` writes for the panels that read that name is not a declared
   domain of generation3d, so it was never checked against anything — and it is PERSISTED into
   `interaction_store`. A mirror is a projection of the real domains, so it may now only name what a
   declared domain still names after validation.
2. The leftover cache (`interaction_leftover_selection`, `interaction_leftover_ids`) exists to carry a
   pick the interaction store has not answered with yet — `interaction_selection_snapshot` lays it
   over an EMPTY store selection. A document change empties that store selection *legitimately*, so
   the cache papered straight over the prune and put the dead ids back on the very next read. A cover
   may not outlive the ids it was covering.

A document edit that touched nothing selected leaves all three untouched, so an in-flight pick during
an unrelated edit keeps its cover. This half is what stops `interaction.selection("graph")` — which
`deleteSelection` and the three transform verbs read directly (`✏️editor/🦀️.rs`) — from naming a
widget the new document does not have.

---

## 3. Laws

### 3.1 Rust — `cargo test -p semio-framework-plugin --lib interaction_selection_laws`, foreground

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🕹️interaction-selection-laws/🦀️.rs`, new
`a_document_change_prunes_the_vortex_mirror_and_the_leftover_cover_with_the_selection`: the fixture is
two documents — `shell@solid` picked in `Box Shell Preview`, then `Sphere Cut With Torus` loaded so
only its handles are in topology — and it states all four cases: the mirror pruned with the selection,
the cover retired with it, an id that SURVIVES the switch keeping both, and a `Flat`/entry-less
DECLARED domain keeping every id because it has no membership information.

```
running 3 tests
test component::app::interaction_selection_laws::interaction_selection_loss_names_every_way_a_pick_can_disappear ... ok
test component::app::interaction_selection_laws::an_app_selection_write_retires_the_leftover_overlay_of_the_domains_it_names ... ok
test component::app::interaction_selection_laws::a_document_change_prunes_the_vortex_mirror_and_the_leftover_cover_with_the_selection ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 739 filtered out
```

### 3.2 TS twin — engine contract, foreground

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, new
`retires a stale selection overlay against the pane's own document, and keeps the pick it exists for`:
the same two documents as instance rosters, asserting the cover STANDS in the document it was picked
in (the guest lane silent), goes empty in the next one, that a REPLACE click publishes only the new
id, and that an instance-less pane keeps everything.

```
✓ the leftover world overlay is per window INSTANCE: arming one pane leaves its sibling's record untouched
✓ carries an armed mode tool id across hover leftovers without overriding the published utility
✓ first leftover pick keeps selection across hover leftover and busts the Inspection hash skip
✓ empty-target interactionSelect clears selectedIds while hover may remain; hover leftover does not invent a pick
✓ retires a stale selection overlay against the pane's own document, and keeps the pick it exists for
     Tests  5 passed | 609 skipped (614)
```

(The third row was RED before this lane's `selectionCleared` guard fix — see §2.1.)

---

## 4. `interact` before and after, per example

`SEMIO_BATTERY_URL=http://127.0.0.1:6025/?plugin=generation3d SEMIO_BATTERY_ROOT=react-interact bun 🐍️react-battery.mjs --only=interact`

- before: `🗑️generated/react-interact/results-before.json` (18:57–18:59, pre-change guest)
- after: `🗑️generated/react-interact/results-confirm.json` (2026-09-15 00:3x, on the newest staged guest, 00:16; host TS vite-live and verified served via `/@fs/`). `results-final.json` (00:0x, on this lane's own 23:52 restage) agrees on every hop but one `fit` flake.

Hops in order: `payload selection-reset hover select inspector orbit fit`.

| example | before | after |
|---|---|---|
| Box Shell Preview | ✅ ✅ ✅ ✅ ✅ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ❌ |
| Face Sweep Extrude | ✅ ❌ ✅ ❌ ❌ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ✅ |
| Hexagonal Mushroom Column | ✅ ✅ ✅ ❌ ❌ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ✅ |
| Sphere Cut With Torus | ✅ ❌ ✅ ❌ ✅ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ✅ |
| Sphere Box Fuse | ✅ ❌ ✅ ❌ ✅ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ✅ |
| Rectangle Wire Preview | ✅ ❌ ✅ ❌ ✅ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ✅ |
| Rectangle Extrude Volume | ✅ ✅ ✅ ✅ ✅ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ✅ |
| Box Fillet Preview | ✅ ❌ ✅ ❌ ✅ ✅ ❌ | ✅ ✅ ✅ ✅ ✅ ✅ ✅ |

| hop | before | after | owner |
|---|---|---|---|
| `payload` | 8/8 | 8/8 | — |
| `selection-reset` | 3/8 | **8/8** | this lane |
| `hover` | 8/8 | **8/8** | this lane |
| `select` | 2/8 | **8/8** | this lane |
| `inspector` | 6/8 | **8/8** | this lane |
| `orbit` | 8/8 | **8/8** | this lane |
| `fit` | 0/8 | 8/8 | **lane `chrome-panel-safe-area`, NOT this lane** |

The confirmation run is **56/56 — every hop 8/8 on all eight examples, 0 page errors, 0 shell faults,
battery `green=1/1 red=[]`**. Two earlier runs on this fix agree (`results-fix2.json`, 23:32, also
56/56); the one in between (`results-final.json`) flaked a single `fit` row on `Box Shell Preview`
(`no camera published: null` — the camera attribute read back null once), which is the chrome-panel
lane's surface either way.

After the fix the pane's two lanes agree in every row, e.g.
`Sphere Cut With Torus` → PAINTS `["brep_bool_cut_5@solid"]`, GUEST `["brep_bool_cut_5@solid#0"]`,
`activeObjectId` `brep_bool_cut_5@solid`, Inspection `Id: brep_bool_cut_5`.

The rule is not a blanket clear: `Hexagonal Mushroom Column` keeps `extrude@solid` across its own
switch and the row is green, because that id IS in the document now on screen.

---

## 5. Files

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — `leftoverWorldOverlayIdsInDocumentV1` + its use in `mergeWorldSelectionWithLeftoverV1`; `leftoverWorldOverlayAppliesV1` counts `selectionCleared` (§2.1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `interaction_after_document_change_v1` and its application on the `DocumentChange` revalidation; `revalidate_and_persist_interaction_state` answers the persisted state (§2.2).

Laws:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🕹️interaction-selection-laws/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`

Ticket:
- `📜️run-interact-6025.sh`, `📜️restage-selection-prune.sh` — this lane's runners.
- `🐍️selection-prune-probe.mjs` — picks in the NODE GRAPH rather than the 3d canvas, so the question can still be asked on a tree whose preview evaluation is down.

---

## 6. Not claimed

- **The wgpu twin was not run by this lane.** The standing gate
  (`until [ -z "$(pgrep -f 'wgpu-batter[y]|wgpu-.*-pro[b]e')" ]`) never cleared: port 6118 was held
  continuously by peer runs — first `--only=chrome,world3d-editor` (`SEMIO_BATTERY_ROOT=wgpu-safe-area`,
  44+ min), then `--only=chrome,status-a11y-i18n` (`wgpu-pace`), `🐍️wgpu-settle-pump-probe.mjs`, and
  `--only=examples,world3d-editor,world3d-viewer` all at once. The `world3d-editor` row is therefore
  being measured live by its own lane, on a guest that contains this lane's change: there is ONE
  staged `🌀️procedural` guest for both renderers and it was restaged at 00:16, after this lane's
  23:52 restage. No `activate-generation3d-wgpu-dev` was needed or run for the same reason. The guest
  half (§2.2) is renderer-agnostic and is covered by the native law; the host half (§2.1) is React-only
  (`🌐️World3dHost/🟦️.tsx`), so there is nothing in it for the wgpu shell to regress — but that is an
  argument, not a measurement.
- That `fit` was fixed by this lane — it was 0/8 at 18:59 and 7–8/8 afterwards purely because lane
  `chrome-panel-safe-area` unblocked the overlay rail in between.
- That `payload`, and the four viewer-role rows behind it, are this lane's. They were green in the
  baseline and are green now; two peer regressions crossed the middle of this lane's window and made
  them 0/8 for two runs — the `setContributions` ingress ceiling
  (`rejected 273136 raw bytes before decoding; maximum is 262144`, lane
  `contributions-ingress-ceiling`) and a `targeted window transient capture requires an exact
  ViewModel roster` fault that killed every extension evaluation. Both were fixed by their owners; the
  numbers in §4 are from runs with neither present.
- That the `Box Shell Preview` `fit` red in `results-final.json` is a product defect. It is one
  `camera: null` read on a hop that is green for the same example in both neighbouring runs, and `fit`
  is not this lane's surface.
- Anything about the `vortex` name being hard-coded in the framework
  (`overlay_leftover_ids_into_vortex`, `interaction_selection_snapshot`). §2.2 makes it harmless by
  treating it as the projection it is; removing the hard-coded name is a separate cut this lane did
  not make.
