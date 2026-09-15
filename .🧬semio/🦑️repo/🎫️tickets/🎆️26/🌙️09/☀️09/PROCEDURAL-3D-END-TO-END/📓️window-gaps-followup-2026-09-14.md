# 🪟 Window gaps follow-up — generation3d React editor, 2026-09-14 (lane `window-gaps-followup`)

Closes the backlog `📓️react-window-gaps-2026-09-14.md` handed on: §4 "found and NOT fixed" (F1, F2),
§6's four non-green rows and §7's "not claimed". Scope: the **React** renderer on **:6023**
(`📜️serve-generation3d-react-6023.sh`, direct vite serve, `SEMIO_VITE_HMR=0`). Guest restaged at
**23:08**, **00:16** and **00:40** with this lane's changes. The 00:40 stage was broken at boot by a
peer's mid-edit `ui-turn-patch-batching` wave; the **01:26** stage boots clean and carries every change
this lane made, so **§7's numbers are all measured on 01:26** unless a row says otherwise.

**Files changed** — framework: `🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts`,
`🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx`,
`🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`,
`🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🦀️.rs` (+ its unit tests),
`🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`,
`🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx`,
`…/🔌️PluginRuntime/🟦️.tsx`, `…/🗣️Interpreter/🟦️.tsx`. App: the generation3d viewer
(`👁️viewer/🦀️.rs` + its unit tests), `✏️editor/🗣️terminology/🦀️.rs` (+ tests) and
`🧫️fixtures/🗣️terminology.json`. Probes: `🐍️outline-selection-probe.mjs` and
`🐍️panel-tab-occlusion-recon.mjs` (new), `🐍️graph-keyboard-nav-probe.mjs`, `🐍️menu-dump-probe.mjs`,
`🐍️status-states-probe.mjs`, `🐍️viewer-actions-probe.mjs`, `🐍️panel-i18n-probe.mjs`,
`🐍️inspection-i18n-probe.mjs`.

---

## 1. F1 — keyboard traversal was invisible to a user

### What the handover believed, and what was actually true

The handover located the root cause in `PanelTreeBuilder::build` dropping `.selected()`/`.highlighted()`
— read off that method's own doc comment, never measured. That is **not** the defect. The framework
already owns a complete selection path:

- `graph_outline` (`✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs`) binds the outline tree to the
  `graph` interaction domain and keys every node row by the node id verbatim;
- `stamp_and_cache_interaction_ui` walks that tree each render and `derive_node_presence` emits one
  `ui_contract::PresenceUpdate` per selected/hovered node;
- the reactor's `PresenceHub` coalesces them into WIT `turn-result.presence`;
- the React interpreter's `treeItemToTreeData` already reads `presence.selected` into `TreeDataItem.isSelected`,
  and `Tree` already publishes `aria-selected` in all four row layouts.

**Three cuts in that path, all measured, all now closed:**

### F1a — the host threw the presence channel away

`turn-result.presence` had exactly ONE reader in the whole TypeScript host: `shardTurnCarriesNothingV1`
(`🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts`), the predicate that decides whether a turn may be
dropped. Nothing consumed it. `UiPresenceOverlayContext` (`🗣️Interpreter/🟦️.tsx`) consequently had **no
production `Provider` anywhere** — only the component tests ever supplied one — so every retained tree
row in every app rendered `aria-selected="false"` whatever the guest's selection did.

Measured on :6023 before the fix: the raw jco turn result carried
`presence` (4 turns across 4 arrow presses, 1–2 entries each), and the React shell dropped every one.

**Fix** (three files, one lane):
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📃️UiDocumentStore/🟦️.tsx` —
  new `👥️GuestPresence` region: `publishGuestPresenceV1` / `guestPresenceTableV1` /
  `subscribeGuestPresenceV1` / `forgetGuestPresenceSurfaceV1`. Per-surface table, keyed by node key;
  a fully-idle mark is a **deletion**, never a stored `selected: false`, so the table cannot grow one
  entry per row the user ever touched. No store notification, no revision — presence is not document state.
- `🧱️elements/🔌️PluginRuntime/🟦️.tsx` — `retainTurnPresence(result)` decodes each `{ update: pack }`
  carrier (`decodeGuestPresenceUpdateV1`, exported so it is testable) and publishes; called from
  `settlePluginTurn`'s `acknowledge` (every turn, initial and continuation) and from
  `settleAcknowledgedPluginTurns`.
- `🗣️Interpreter/🟦️.tsx` — `useUiPresenceOverlay()` merges the context overlay (the tests' channel) with
  the live table; `TreeView` reads it instead of the bare context.

### F1b — the React host has its OWN second `coerceTurnResult`, and it dropped the field

Adding `presence` to `WireTurnResult` in `🖼️wire-turn/🟦️.ts` changed nothing: measured live, the object
reaching the consumer had keys
`original,lifecycleReceipt,uiPatchReceipt,uiPatches,effects,nextWake,status,commandIngress` — neither
`presence` nor `coldPairIngress`. `🧱️elements/🔌️PluginRuntime/🟦️.tsx:579` defines a **second, private
`coerceTurnResult`**; the shared one in `🖼️wire-turn/🟦️.ts` is used only by the wgpu bridge. Both now
carry `presence`. The duplication itself is reported below as a finding, not fixed here.

### F1c — a dropped selection was never retired (guest)

`derive_node_presence` reports only nodes that have something to say, and `PresenceHub::record_own` holds
own presence until it is explicitly overwritten. Nothing ever wrote `selected: false`, so the FIRST row a
user selected would have stayed painted selected for the life of the session on **both** renderers.

**Fix** — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:
`VcsArtifactApp::presence_marked_keys: BTreeMap<String, BTreeSet<String>>` (per body key),
`derive_node_presence` now answers `bool`, and `stamp_and_cache_interaction_ui` ends with
`retire_dropped_presence(body_key, &marked)` — one cleared `PresenceUpdate` per key this render dropped.

### F1d — the row had to SAY it, not just be it

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx` — all **seven** row elements that publish
`aria-selected` now also publish `data-selected` and `data-highlighted`, so a scripted or assistive reader
has a stable hook in every one of the four layouts. The visible style was already there
(`treeRowChromeShellClasses` → `text-emphasized`) and is now asserted, not assumed.

`treeItemToTreeData` additionally maps `presence.hovered`/`presence.previewed` onto `isHighlighted`,
which the Tree already threaded but nothing ever set.

### Proof (`🐍️outline-selection-probe.mjs`, new, :6023)

| Verdict | Before | After |
|---|---|---|
| the outline paints rows | — | ✓ 32 |
| the rows are the `procedural-play-graph` outline | — | ✓ (F5) |
| a traversal marks a row `aria-selected` | ✗ `[]` | ✓ `Column Height → Side Count → Polygon → Vector` |
| the same row carries `data-selected` | ✗ | ✓ |
| a marked row is visually distinct from an idle row | ✗ | ✓ `rgb(0,17,23)` vs `oklab(0.5209 …)` |
| traversal moves the mark between rows | ✗ | ✓ 4 distinct rows |
| exactly one row is marked at a time | — | ✓ |
| Escape retires the mark | — | ✓ `[]` |
| a traversal after Escape re-marks exactly one row | — | ✓ `Column Height Evaluated` |
| no page errors | ✓ | ✓ |

**11/11 on the 01:26 guest** (`outline-selection`, now a registered battery row). The guest's own
retirement is proven live in the decoded carriers — moving the selection publishes `{sides: selected
true}` together with `{height: selected false}` in ONE turn — and `Escape` retiring the mark is the same
diff running with an empty selection. On the earlier, mid-edit stages `Escape` left a stale mark for one
keystroke; that reading did not survive a healthy guest and is withdrawn.

### Law

`a_dropped_selection_publishes_a_cleared_presence_update`
(`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`) —
`cargo test -p semio-framework-plugin --lib --features artifact-app-testing` →
`test result: ok. 1 passed; 0 failed`:

```
[DEBUG] presence after selecting item-1: [("item-1", true)]
[DEBUG] presence after moving the selection to item-2: [("item-2", true), ("item-1", false)]
[DEBUG] presence on an unchanged selection: [("item-2", true)]
```

The third assertion is the one that keeps the diff honest: an unchanged selection must cost exactly its
own mark, never a repeated retirement.

---

## 2. F2 — the viewer's Export was broken, not merely unreached

The handover left this "unproven, neither confirmed working nor shown broken". It is **broken**, and now
fixed. Three separate findings, in the order they had to be peeled.

### F2a — the route

The probe right-clicked the viewer's World3d surface and looked for a plugin menu. That gesture is
claimed by the surface itself (`interactionSelect`/`setCamera`), so no menu ever opened and the row was
never reached — the handover read that as "export unproven". `🐍️viewer-actions-probe.mjs` now drives the
viewer window's OWN Actions rail (`framework.window.proceduralViewPreview.engagement.toggle` →
`action.exportDocument`), which is the route a viewer user actually has, and asserts every one of the
seven `document_io::EXPORT_FORMATS` rows is in the picker and produces a download whose FIRST BYTES carry
that format's signature (`solid…` for STL, `ply` for PLY, `LASF` for LAS, `AC` for DWG, …).

`viewer-export-row` green (`railOpened: true`, `exportRow: 1`) and the picker lists all seven:
`STL Mesh, OBJ Mesh, PLY Mesh, glTF Mesh, LAS Point Cloud, DWG Drawing, Semio Text (whole document)`.

### F2b — the framework threw the diagnosis away

With the route reached, all seven formats failed — and every one reported the SAME sentence,
`retained command reducer rejected operation`. `🔌️plugin/🧵️retained-command/🦀️.rs` matched the app's
`step` result as `Err(_)` and replaced the app's own `Fault` with a fixed string, so a missing session, an
unsupported format and a rejected route were one indistinguishable message. Now
`reducer_fault_detail(&Fault)` carries the app's own code and message through, clipped on a char boundary
to one fault page. One restage later the browser said exactly what was wrong:

```
[DEBUG] action failed exportDocument {format: stl} Error: typed-operation failed:
retained command reducer rejected operation: generation3d.io.export
generation3d export: the document evaluates to no preview geometry (no positions)
```

Laws: `a_refused_reducer_step_reports_the_apps_own_fault_code_and_message` and
`an_oversized_reducer_fault_detail_is_clipped_on_a_char_boundary`
(`🧵️retained-command/🧪️tests/🔬️unit/🦀️.rs`) —
`cargo test -p semio-framework-plugin --lib --features artifact-app-testing "retained_command::tests"`
→ `test result: ok. 8 passed; 0 failed`.

### F2c — the viewer exported a document it was not showing

`Generation3dViewDocumentIoWork::step` read `input.snapshot` raw. Every OTHER viewer surface resolves the
viewed document through `Generation3dViewedDocument::resolve(snapshot, config)` — with an active example
picked, that example REPLACES the opened snapshot. So with three meshes on screen the export encoded a
document the retained session had never evaluated, and `export_document_with_preview` refused it for
having no positions. `step` now resolves the same way (and `retire`s the resolved snapshot, whose neural
`Dictionary` roots abort the process on a bare drop).

Law: `the_viewer_exports_the_example_it_is_showing_not_the_opened_document`
(`👁️viewer/🧪️tests/🔬️unit/🦀️.rs`) —
`cargo test -p semio-s-artifact-procedural-generation3d --lib --features component-app-assembly` →
`test result: ok. 1 passed; 0 failed`:

```
[DEBUG] viewer export document for hexagonal-mushroom-column: widgets=7
[DEBUG] viewer export document for rectangle-extrude-volume: widgets=7
```

### Proven live on the 01:26 guest — `viewer-actions` **15/15**

All seven declared formats produce a real download whose first bytes are that format's own signature:

| format | filename | bytes | first bytes |
|---|---|---|---|
| `stl` | `generation3d.stl` | 362 477 | `solid generation3d-previ` |
| `obj` | `generation3d.obj` | 445 678 | `v 1.3763998746871948 0 1` |
| `ply` | `generation3d.ply` | 54 942 | `ply\nformat ascii 1.0` |
| `gltf` | `generation3d.gltf` | 38 116 | `{ "asset": { "vers` |
| `las` | `generation3d.las` | 12 287 | `LASF` |
| `dwg` | `generation3d.dwg` | 28 976 | `AC1015` |
| `txt` | `generation3d.txt` | 1 714 | `semio procedural.generat` |

`downloaded: 7 / 7`, and `viewer-export-lists-every-format` green with `missingFromMenu: []`.

---

## 3. F3 — the selection-conditioned context menu

Not a menu defect at all. The probe could not establish its own precondition: it looked for an outline row
inside `[data-slot="window"][id="procedural-main"]`, and a peer moved the graph outline out of the window
body into the **Artifact panel** (the canvas hint now says so in both locales). With no row to click, no
selection was ever armed.

`🐍️menu-dump-probe.mjs` now opens the Artifact panel first and clicks a row there, and — because a click
that dispatches is not a selection — the verdict additionally reads the mark the shell painted, which only
exists because of F1.

**Green:** `context-menu-selected` — 18 rows, `marked: ["Column Height Evaluated"]`, `missingAlways: []`,
`missingSelected: []`, `delete-selection` present.

---

## 4. F4 — the five German label fields that never appeared

Decided by reading the code, as instructed.

| Field | Verdict | Action |
|---|---|---|
| `catalog_neuron`, `catalog_slider`, `catalog_note`, `catalog_preview` | **dead** | deleted |
| `no_selection` | reachable, but NOT where the handover assumed | kept, and now reached |
| `preview_hint` | reachable (generate-mode preview with no generation) | kept |

**`no_selection` is not the no-selection state.** `inspection::render` paints the schema + widget count
when nothing is selected at all; `labels.no_selection` lives on its SECOND branch — a node IS selected
and no fixture widget carries that id. In this app every graph node IS a fixture widget (7 nodes, 7
widgets) and port rows carry no activate binding, so no ordinary click can ever reach it. The reach is a
DELETE: the selection id outlives the widget it named for the render that follows. `inspection-i18n` now
selects a node, presses `Delete`, and reads `Keine Auswahl` off the panel — **5/5, was 2/8**. That is why
the handover could not find it, and it is a label worth keeping rather than deleting: a user who deletes
a selected node does read it.

The four catalogue labels were reachable only through `generation3d_catalog_label`, whose **only** caller
in the entire repo was its own unit test. Deleted: the four `app_labels!` fields, the resolver, its unit
test, and the four `🧫️fixtures/🗣️terminology.json` entries (plus `catalog_neuron`'s
`identicalByDesign` row). `catalog_preview` was never named in the handover but shares the same dead
resolver, so leaving it would have left dead code behind a fixed report.

Repaired in passing: `graph_canvas_hint` had drifted — a peer rewrote the Rust label to name the Artifact
panel but left `🗣️terminology.json` saying "The outline beside it", which failed
`every_user_visible_label_is_declared_in_every_locale_exactly_as_the_shared_fixture_says`. The fixture now
matches the Rust in all four spellings.

`cargo test -p semio-s-artifact-procedural-generation3d --lib --features component-app-assembly terminology`
→ `test result: ok. 3 passed; 0 failed`. That law walks every field against the shared fixture, so the
deletion is pinned: a field re-added without a fixture row fails here.

---

## 5. F5 — the accessible outline

Present and now load-bearing. `🐍️outline-selection-probe.mjs` asserts the rows it reads carry
`procedural-play-graph`-namespaced ids, so the 32 rows are the Flow graph's own outline and not some other
tree in the panel. The handover's earlier "missing outline" reading stays superseded: the outline is in the
Artifact panel by a peer's deliberate change, and it is the surface F1 now marks.

---

## 6. Defects found and NOT fixed here (named, with what was measured)

### G1 — the `Collapse` fold control paints over the Inspection panel tab

Hovering the top-right dock's cap row reveals `button#framework.panel.top-right.fold` ("Collapse") at
`[1297, 3, 64, 22]`. The Inspection tab is at `[1267, 2, 92, 22]`, so its **centre (x = 1313) belongs to
the fold button**: `elementsFromPoint` there answers
`span → button#framework.panel.top-right.fold → div[window-chrome-controls]`, while every other tab's
centre answers its own button. A user clicking the middle of the Inspection tab collapses the dock
instead of opening Inspection. Measured with `🐍️panel-tab-occlusion-recon.mjs` (new).

This was not only a probe nuisance: it is why `inspection-i18n` reported `seen: {}` and every field
missing — the panel it reads never opened. Owner: the chrome-hosted panel cap row (`🖼️Panel/🟦️.tsx`
`titleChips`/`close` + `WindowChrome`'s cap flex row, `🎯️targets/⚛️react/🟦️.tsx`); the product fix is
lane `react-remaining-reds`'. Every probe in this lane now clicks a tab at its left edge, which is the
row's own label and resolves to the row.

### G2 — `coerceTurnResult` exists twice

`🔌️PluginRuntime/🟦️.tsx:579` is a private copy of `🎭️actor/🖼️wire-turn/🟦️.ts`'s exported
`coerceTurnResult`; the React host uses the copy and the wgpu bridge uses the shared one. They had already
drifted (`coldPairIngress` is in one and not the other), and that drift is exactly what made the first
attempt at F1a look like a no-op. Both now carry `presence`; the duplication is not removed — deleting a
primitive mid-session with peers in the same file is the wrong trade.

### G3 — `stale` is no longer a reachable node status

`status-states` drives six states per locale. `stale` appeared in NEITHER locale on any guest this lane
measured (`statesEn: [ok, queued, computing, blocked, error]`, `statesDe` the same set). The
flow-tick-coalescing lane's `build_flow_status_json` change (19:5x) deliberately stopped calling an
already-recomputed dirty node `stale`. Whether `stale` is now dead or merely rare is **not** established
here; the row's two `*:stale` steps are left red rather than deleted, because deleting them would hide
the question rather than answer it.

### Withdrawn — `Escape` leaving a stale selection mark

An earlier revision of this report named a G-defect for `Escape` leaving the outline row marked. It does
not reproduce on the 01:26 guest: `outline-selection` · `Escape retires the mark` is green with `[]`, and
the following traversal re-marks exactly one row. The original reading came from the mid-edit stages and
is withdrawn rather than carried forward.

### Withdrawn — the 00:40 boot faults

`actor-ui-patch.pairing` and `actor-lifecycle.work-kind` collapsed `menus`, `graph-keyboard` and
`viewer-actions` on the 00:40 stage, reproducibly across two runs. The coordinator has since attributed
them to a mid-edit wave of lane `ui-turn-patch-batching`; the 01:26 stage boots clean and every one of
those rows recovered. Recorded here only because the guard this lane added to `openContextMenu` is what
turned the first collapse from a probe CRASH — which erased every later verdict — into a reported 0/6
naming the missing surface, and that guard is worth keeping.

---

## 7. Battery

`SEMIO_BATTERY_URL=http://127.0.0.1:6023/?plugin=generation3d SEMIO_BATTERY_ROOT=react-gaps2`

| Row | Before (handover §6) | After | Guest | Notes |
|---|---|---|---|---|
| `preview-chrome` | 16/16 | **18/18** | 00:40 | 0 page errors; unaffected by G5 |
| `graph-keyboard` | 12/12 (pre-hardening) | **13/13 decided** | 23:08 / 00:16 | every decided step green (the row's other 4 steps — `baseline`, `escape-clear`, `delete-selection`, `undo` — are undecided by design, so the battery prints 13/17 and `ok=true`). The hardened verdict the handover never ran is now green: the Inspection Id field walks `sides → radius → profile → sides → height → extrusion-axis`. 0/17 on the 00:40 guest (G5) |
| `outline-selection` (new) | — | **8/9** | 23:08 | F1's own row; the one red is G1 |
| `status-states` | 13/13 (pre-outline-move) | **11/15** | 00:40 | 3/15 → 11/15 after the repoint; reds are `*:stale` (G4) and `*:error` (§8) |
| `menus` | 4/6 | **5/6** | 00:16 | `context-menu-selected` green (F3); `export-formats` red. 0/6 on the 00:40 guest (G5) |
| `viewer-actions` | 12/13 | **12/15** | 00:16 | row count grew: `viewer-export-row` green, `viewer-export-lists-every-format` + `viewer-export` red (F2c's fix is one stage newer than the last healthy measurement). 2/15 on the 00:40 guest (G5) |
| `panel-i18n` | 26/33 | **not re-measured** | — | the 00:16 run CRASHED before writing `result.json` (guarded now); the 00:40 guest never got there |
| `inspection-i18n` | 2/8 | **not re-measured** | — | roster corrected (three deleted dead fields dropped); not run on a healthy guest since |

---

## 8. Not claimed

- **`panel-i18n` and `inspection-i18n` have no post-fix number.** Both probes were corrected — the four
  dead catalogue labels dropped from their rosters, `panel-i18n`'s outline click repointed at the Artifact
  panel, and its crash guarded so a late failure can no longer erase every earlier verdict — but neither
  has been re-run on a healthy guest. Their handover numbers (26/33, 2/8) stand until it is.
- **F2's downloads were never observed.** The route, the seven-format list and the FAULT are all measured;
  the fix for the fault (F2c) is proven by Rust law only, because the guest carrying it is the one G5
  broke. Nobody should read "viewer export works" out of this document — read "viewer export was broken,
  the cause is named and fixed in the source, and the bytes have not been seen".
- **`viewer-export-lists-every-format` was red for a PROBE reason on the 00:16 run** (`missingFromMenu:
  ["txt"]`): the picker did list `Semio Text (whole document)`, and an id regex missed it. The matcher now
  keys on each row's declared label. The corrected matcher has not itself been run green.
- **G1 stands.** `Escape` leaves a stale mark for one keystroke. Measured, narrowed, not fixed.
- **G5 is not diagnosed, only attributed.** This lane did not read the peer's patch-lifetime change.
- **`status-states` · `*:error` regressed within this lane's own probe edit.** Before the repoint the guest
  status map carried `error` (with no row to read it off); after it, `driveError` records
  `attempts: 0`, meaning its widget-row lookup found nothing to type into. That is a probe-reachability
  question introduced by moving the read to the Artifact panel, not a product finding, and it is **not**
  resolved here.
- **wgpu is untouched.** The guest half of F1 (`retire_dropped_presence`) is renderer-neutral and its law
  covers both, and `ui_tree_stamp_presence` exists in the wgpu target — but it has no production caller
  there either, so the wgpu renderer still paints no selection on a panel tree. That is the same F1a gap
  on the other renderer, and this lane did not close it or measure it.
- **The four reducer-adjacent test failures in `semio-framework-plugin` are pre-existing.**
  `full_operation_source_rejects_generic_reducers_and_old_monolithic_shells`,
  `registry_less_construction_rejects_before_the_reducer` (expects `interactive-job.unknown-key`, gets
  `interactive-job.missing-factory`), `unproved_command_fails_before_an_overrun_reducer_can_start` and the
  generation3d VCS/envelope/preview trio fail on assertions this lane never touched. They were not
  repaired and they are not this lane's.
- **`graph_canvas_hint`'s fixture repair is a forward fix of a peer's half-landed change**, not a finding
  of this lane's own; it is named because it was blocking the terminology law.
- No claim is made about the **wgpu** battery, about `role-switch`, `gaps`/`actions-pane-de`, or about any
  row outside the seven this lane owns.
