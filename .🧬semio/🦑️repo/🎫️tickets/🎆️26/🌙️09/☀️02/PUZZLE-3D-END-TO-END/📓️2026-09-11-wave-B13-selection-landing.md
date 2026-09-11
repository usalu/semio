# Wave B13 — First-pick selection landing

Implementation pass, 2026-09-11. Ticket `26/09/02/PUZZLE-3D-END-TO-END` (moon `🌙️09/☀️02`).
Scope: C1 leftover→Inspection hop — leftover can publish `selectedIds:["seed-left-001"]` while Inspection stays
`id=null` because B4's `uiRefreshSectionUnchanged` keeps the empty-summary tree. Also carry leftover
selection across hover leftovers so Delete / Duplicate / Focus / outliner Hide have a live id.

No git write. No `:6013`. Probe and vite-live host only on `:6014`. **B9 guest topology was not reverted.**

Filename is this file. Coordinator log also reserved `B13` for export/history/locale
(`📓️2026-09-11-wave-B13-export-history-locale.md`) — different wave, same numeral.

---

## 0. Verification status

| lane | result |
|---|---|
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib` (4 filters) | **4 passed**, 0 failed, 691 filtered. Tails in `🗑️generated/b13-cargo.txt` |
| renderer-react vitest leftover / window-host-context | **5 passed**, 870 skipped. Tails in `🗑️generated/b13-vitest.txt` |
| `:6014` `curl` | `200` (vite pid 18776) |
| browser `--only=selection-surfaces,selection-keybindings,outliner-rows --port=6014` | **booted on wasm #46** (`dist/release/…/🧩️puzzle` mtime 2026-09-11 17:13). Verdicts below. `probe-2026-09-11T15-24-52.md` |
| wasm `component-release` | **not started** — #46 already on disk (deploy 17:13); no other `rustc`/`wasm32` job. Guest hover-carry landed after that stamp and rides the next wasm. |

Guest laws (exact rustc names):

```
test editor::puzzle3d::component::tests::an_absent_fixture_meta_member_never_empties_the_typed_authority ... ok
test editor::puzzle3d::component::tests::first_pick_of_a_fresh_session_renders_the_object_and_its_lock_row ... ok
test editor::puzzle3d::component::tests::hover_after_first_pick_keeps_the_object_and_its_lock_row ... ok
test editor::puzzle3d::component::tests::interaction_topology_names_every_id_the_world_lane_paints ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 691 filtered out; finished in 0.75s
```

Vitest:

```
Test Files  2 passed | 21 skipped (23)
     Tests  5 passed | 870 skipped (875)
```

---

## 1. C1 hop — leftover ids present, Inspection empty

B6 hop table already proved leftover can carry `selectedIds:["seed-left-001"]` and the leftover Inspection
epoch fires a **full** refresh. Guest then answered `framework.panel.inspection#…=same` for 73 s because
`uiRefreshSectionUnchanged` compares the **request** hash to the retained empty-summary hash.

`leftoverInspectionRefreshScope` already returned `{kind:"full"}`, but `buildUiRefreshRequest` still stamped
the cached Inspection hash. The leftover effect called the scope with a dummy `["leftover"]` and never
deleted `panel:framework.panel.inspection` from `uiRefreshCacheRef`.

### Fix (vite-live)

1. `leftoverInspectionPanelHash(selectedIds, cachedHash)` → `undefined` when `selectedIds.length > 0`.
   `uiRefreshSectionUnchanged(undefined, {root, hash})` is **false**, so the host must apply the new tree.
2. ShellHost leftover Inspection effect now:
   - gates on `leftoverWorldSelectionOverlayV1()?.ids` (not the dummy `["leftover"]`);
   - deletes `panel:${FRAMEWORK_PANEL_TAB_INSPECTION_ID}` from `uiRefreshCacheRef` when the hash is omitted;
   - logs `[DEBUG] leftover Inspection refresh { epoch, selectedIds, hashBust, cached }`.
3. `leftoverInteractionStateV1` synthesizes `{ vortex: { granularity:"object", ids } }` when the leftover
   selection map is empty but `selectedIds` is not.

B9 stays the guest topology authority: `interaction_topology` still reads
`puzzle3d_fixture_from_projection` (one projection read). `Puzzle3dFixtureMeta` still
`skip_serializing_if = "Option::is_none"`.

---

## 2. Hover leftover must not wipe the pick

Hover leftover is `selectedIds:[]` + `hoveredId`. `applyLeftoverInteractionView` used to replace the overlay
with empty `ids`, so the first pick looked hover-only and the Inspection epoch never bumped.

### Fix

- `leftoverOverlayCarryingSelectionV1` — hover leftover with empty `ids` + `hoveredId` **keeps `prior.ids`**.
  `hoveredId` is never treated as a selection.
- Interpreter `TreeView` subscribes to leftover overlay and marks
  `leftoverTreeItemSelectedV1(record.key, leftoverIds)` (id / `/${id}` / `.${id}`).
- Guest `🔌️plugin/🦀️.rs`: on `INTERACTION_HOVER` only, if current domain selection is empty and the prior
  leftover still has ids, copy prior selection / mode / granularity before
  `leftover_interaction_view_from`. Source-only until the next wasm (after #46).

---

## 3. Browser on `:6014` / wasm #46

`--only=inspection-object-fields,…` matches **verdict** names, not `add()` step names. The live plan
entries are `selection-surfaces`, `selection-keybindings`, `outliner-rows`.

| verdict | result | note |
|---|---|---|
| `inspection-object-fields` | **FAIL** `populated=false empty=true id=null` | leftover never published ids |
| `inspection-locked-flag-row` | **FAIL** `lockChrome=false` | same hop |
| `outliner-hide-control-present` | **PASS** | Hide/Lock buttons on `seed-left-001` |
| `outliner-hide-applies` | **FAIL** | row text unchanged after Hide click |
| `duplicate-selection` | **FAIL** `before=1 after=1` | no live leftover selection |
| `focus-selection` | **FAIL** camera JSON unchanged | same |
| `delete-selection` | **FAIL** `before=1 after=1` | same |

Leftover taps from this run (host log shape is the B13 `selectedIds`/`publishedIds` pair — vite-live confirmed):

```
leftover InteractionView {"selectedIds":[],"publishedIds":[],"locked":{},"gumball":false,"hoverTarget":null}
leftover InteractionView {"selectedIds":[],"publishedIds":[],"locked":{},"gumball":false,"hoverTarget":{"domain":"vortex","channel":"pointer","id":"seed-left-001"}}
```

`interactionSelect` settled several times. Guest leftover still answered `selectedIds:[]`. Hover did name
`seed-left-001`. C1's "leftover already has the id" hop was **not exercised** — epoch / hash-bust only
run when `overlay.ids.length > 0`. `Agent disconnected` was on the Inspection chrome dump.

#46 already includes B9 guest topology. Empty leftover after `interactionSelect` on this probe is a
**pick-target / leftover publication** miss (clickForestTable pixels + Frame, then hover-only leftover),
not a revert of B9 and not a failure of the hash-bust helper (laws prove the skip is gone when ids exist).

---

## 4. Files

| file | change |
|---|---|
| `🔌️PluginRuntime/🟦️.tsx` | `leftoverInspectionPanelHash`; export `uiRefreshSectionUnchanged` |
| `🌐️World3dHost/🟦️.tsx` | `leftoverOverlayCarryingSelectionV1`, `leftoverTreeItemSelectedV1`, `subscribeLeftoverWorldSelectionV1` |
| `🏛️ShellHost/🟦️.tsx` | carry selection; hash-bust Inspection cache on leftover epoch |
| `🛠️ShellHelpers/🟦️.tsx` | `leftoverInteractionStateV1` from `selectedIds` |
| `🗣️Interpreter/🟦️.tsx` | leftover tree `isSelected` |
| `🔌️plugin/🦀️.rs` | hover leftover carries prior selection |
| `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `hover_after_first_pick_keeps_the_object_and_its_lock_row` |
| `window-host-context` fixture + test | `omitHash` |
| `engine-contract` | first leftover pick / hash-bust law |

---

## 5. Next hop

1. Vite-live host is enough for C1 **when leftover.ids is non-empty**. Re-probe after a click that
   logs `selectedIds:["seed-left-001"]` and `[DEBUG] leftover Inspection refresh … hashBust:true`.
2. Guest hover-carry needs the **next** wasm after #46. Do not revert B9 to chase leftover ids.
3. Outliner Hide control is present; apply/Show still FAIL on #46 — separate from first-pick hash-bust.
4. Do not use `--only=inspection-object-fields` as a plan name; use `selection-surfaces`.
