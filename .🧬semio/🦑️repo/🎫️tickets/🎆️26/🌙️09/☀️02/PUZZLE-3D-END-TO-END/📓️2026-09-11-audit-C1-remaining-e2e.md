# Audit C1 — Remaining Puzzle 3D end-to-end gaps

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, read-only audit 2026-09-11 (Composer 2.5 auditor).
**Goal is not complete.** This report uses runtime evidence only — not intent from wave plans.

## Evidence stack (newest first)

| artifact | wasm / serve | what it proves |
|---|---|---|
| `🗑️generated/probe-2026-09-11T14-48-04.ndjson` + partial `battery-2026-09-11-45b-6013.txt` | **#45** (`a532fbe68602`, B0+B2+B3+B8+B10 host partial) | First clean run after slot-retirement fix; stalled mid-battery (~590 s at brush). No typed-operation saturation faults. |
| `🗑️generated/battery-2026-09-11-45-6013.txt` | **#45**, aborted on reload | Host mid-edit during B10; incomplete. |
| `🗑️generated/probe-2026-09-11T13-15-23.md` | **#44** (`8bef0b857f6c`) | Full 70-step battery; **45 hard faults**, 39× slot-retirement leak; collateral failures dominate second half. |
| `📓️2026-09-11-claude-coordination.md` | — | Wave ledger, build numbering, B12 launched (WindowConfig round-trip). |
| Waves B0–B11 reports | — | Source fixes + law status; browser columns marked “not run” where noted. |
| Working tree @ audit time | vite-live host chrome; guest fixes **not** in #45 wasm | B9 guest lanes, B11 editor actions, B10 Tree checkbox (`stopPropagation` only), B11 `openAddObjectDialog` promotion — present in source, **unproven in browser**. |

**#46** (per coordination): next `component-release` bundling B9 guest halves + B11 guest halves not shipped in #45.

---

## Status legend

| status | meaning |
|---|---|
| **proven** | Browser probe/battery PASS on #45 or later (or #44 where #45 did not re-run and no regression expected). |
| **source-fixed-unproven** | Fix + laws green in tree; no post-fix browser PASS on a wasm that includes the fix. |
| **broken** | Browser FAIL on best available evidence (#45b preferred over #44). |
| **unknown** | Step missing, battery incomplete, or actor/session failure contaminates the read. |

---

## Requirement table — checklist §1–§25

| § | requirement (summary) | evidence | status | next hop |
|---|---|---|---|---|
| **1** | Top + Perspective windows render; fixed 2-pane layout | #45b: `window-both-present`, `one-canvas-each`, `same-document-both-views`, `distinct-camera` PASS | **proven** | — |
| **2** | Camera orbit / pan / zoom; per-window; no artifact history | #45b: `camera-orbit/pan/zoom`, `camera-json-attribute`, `camera-per-window`, `camera-emits-no-artifact-history` PASS (B10 history guard on #45) | **proven** | — |
| **3** | Projection pane options | #45b: `projection-measures-present`, `projection-control-flips` PASS; `projection-repaints-camera` FAIL (camera JSON unchanged after puzzle measure nudge) | **broken** | B12 or guest: wire `setProjection`/`setProjectionParam` → `data-camera-json` / viewport repaint; probe `--only=projection-options` on #46 |
| **4** | Window options (grid, LOD, vortex, sun, select filters) | #45b: sliders `grid-spacing`, `lod-value` PASS; all toggles + vortex selects + sun toggle FAIL (controlled value unchanged). B10 Tree checkbox fix is **vite-live** but toggles still fail on #45 → not a checkbox-only defect. | **broken** | **B12** WindowConfig publish/refresh round-trip (`📓️2026-09-11-wave-B12-windowconfig-roundtrip.md`); then `--only=window-options` on #46 |
| **5** | Example switcher; empty-doc `addObjectKind` | #44: `undo-unwind` PASS; example-switch step ok in timeline. Nakagin E2E not in #45b (aborted). Empty-doc path: laws only. | **source-fixed-unproven** | `--only=example-switch` on #46; hand-check camera/grid reset on switch (checklist §5 nuance) |
| **6** | Selection (viewport, tree, marquee, same-kind, clear) | #45b: viewport click → `inspection-object-fields` FAIL (`id=null`); leftover lane shows `selectedIds:["seed-left-001"]` but Inspection panel empty. `Agent disconnected` notices throughout mutate group. | **broken** | Deploy **#46** (B9 first-pick + topology); re-run `--only=selection-surfaces`; if still FAIL → host inspection refresh / panel body key |
| **7** | Hover highlight | #45b brush step: `interactionHover` dispatches (47×) but `hovered:[]` in vortex flags; no visible hover-driven UI | **broken** | After §6 + brush arm (§9): `--only=brush-stroke`; guest hover flags in world lane |
| **8** | Gumball / transform | #45b: `gumball-handle-enter` PASS; `gumball-scene-delta` FAIL (`poseLen` unchanged). `translateSelection` synthesized in console but world JSON static. | **broken** | **#46** + B9 dirty-scope (vite-live in tree); requires working selection first; `--only=gumball-drag` |
| **9** | Brush utility (picker, cycle, place) | #45b: utility stays `select` after Brush click (50 polls); `brush-preview-place` FAIL. B9 utility instance-scope **not in #45**. | **source-fixed-unproven** | **#46** deploy; `--only=brush-stroke`; then brush placement / `addBrushObject` |
| **10** | Volume Brush (paint target volume, voxel dims) | #44: `volume-brush-arm` FAIL (`activeUtility=select`); `add-target-volume` FAIL. B11 host ground-ray gesture **vite-live**; notice path **#46**. | **source-fixed-unproven** | Reload renderer (vite); `--only=volume-brush` on **#46** |
| **11** | Relocate utility | #44: `relocate-arm` PASS; `relocate-pose-delta` FAIL. B5 wired gesture; extent not the blocker (A3). | **broken** | `--only=relocate` with selection + drag; verify `worldRelocate` mutation reaches `data-instances-json` |
| **12** | Fill tool (activate, count, cancel, weights, tick) | #45b: fill tab arms; `fill-wait-ready` count stays **0**; `fill-history-entry` FAIL (`entries=0`). #44: `fill-history-entry` PASS (pre-slot-storm). | **broken** | Isolate fill planner pump (`fillBuildTick` / job spawn) on #46 fresh `--only=fill-tab,fill-apply-max`; not collateral from slot leak |
| **13** | Vortex suggestions (open, hover preview, accept, close) | #44: `suggestions` PASS. Hover-preview empty-candidates gap from checklist still listed; not re-tested on #45b. | **proven** (open/accept path) | Optional: `--only=suggestions-open` for hover-preview candidates on #46 |
| **14** | Engagement bar | #44: `engagement-input-present` FAIL (`input=0`). B10 engagement pane fix **vite-live**, unproven. | **source-fixed-unproven** | Renderer reload; `--only=engagement-bar` |
| **15** | Context menu rows + zoom action id | #44: `context-menu-object-vocabulary` FAIL (shell fallback menu). B11 hits + `defaultPrevented` guard in tree (**#46** + vite). Checklist `zoomToSelection` vs `focusSelection` not re-tested. | **source-fixed-unproven** | **#46** + vite reload; `--only=context-menu-rows` without prior selection |
| **16** | Inspection panel per-entity fields | #45b: `inspection-object-fields`, `inspection-locked-flag-row` FAIL | **broken** | Same hop as §6 (B9 #46) |
| **17** | Artifact / outliner panel | #45b: outliner shows `Hide \| Lock` labels (EN). #44: `outliner-hide-applies` FAIL on stale wasm. B10 law: guest correct. | **source-fixed-unproven** | `--only=outliner-rows` on **#46** after selection works |
| **18** | Catalogue add + drag-drop | #44: `catalogue-kind-rows` PASS; `catalogue-add-object-kind`, `catalogue-drag-drop` FAIL. B11 drag payload mirror **vite-live**. | **source-fixed-unproven** | vite reload + **#46**; `--only=catalogue-panel`; add blocked on §6 selection |
| **19** | Settings panel steppers | #45b: `puzzle3d.panel.settings` tab mounts (B10); `settings-steppers-present` FAIL (only root `puzzle3d-play-settings` tree item, no four stepper ids) | **broken** | Guest panel render: `📌️panels/⚙️settings/🦀️.rs` body not expanding steppers in UI refresh; fix + `--only=settings-panel` |
| **20** | History undo / redo / checkpoint | #44: `undo-unwind` PASS; `undo-redo` FAIL late (slot storm). #45b: no `AliasCapacity` faults (B9 host fix). Full redo not reached in #45b. | **source-fixed-unproven** | Full battery on **#46** `--only=undo-unwind,undo-redo` |
| **21** | Copy / cut / paste | #45b: `clipboard` FAIL (`delta=0`); copy control absent; paste UI appears but no instance delta | **broken** | After §6 selection: clipboard framework route + `send-message` wasm effect mapping (`wireEffectToFriendly` drops) |
| **22** | Delete / duplicate / focus | #44/#45b: all FAIL with `before=after` (no effective selection). B11 `focusSelection` empty-doc fix **#46**. | **source-fixed-unproven** (focus) / **broken** (delete/dup until select) | §6 first; then `--only=selection-keybindings` on #46 |
| **23** | Add Object dialog | #44/#45b: `add-object-dialog-opens` FAIL; trigger clicked wrong row (`addObjectKind` vs `openAddObjectDialog`). B11 promotion in **working tree**, not #45. | **source-fixed-unproven** | **#46**; `--only=add-object-dialog` |
| **24** | Import / export | #44: `import-same-file-idempotent` PASS; `import-distinct` FAIL; `export-only` FAIL (`download=none`). A2: import wired; distinct file may be probe fixture issue (B9). | **broken** (export) / **unknown** (distinct import) | Export: guest `exportFixture` download path on #46; import-distinct: read B9 `puzzle3d.import.*` taps on #46 |
| **25** | Locale DE labels | #45b: `locale-control-present` FAIL (no language control found); EN outliner labels present. B10: DE law green; empty DE tree correlated with `actor-activation.revoked` on #44. | **unknown** | Isolated `--only=locale-switch` on stable actor (#46); fix settings language control discoverability |

---

## Tool rows (explicit)

| tool | evidence | status | next hop |
|---|---|---|---|
| **Fill** | Tab activates; planner never advances (`fill count=0`, `fill-history-entry` FAIL #45b) | **broken** | `fillBuildTick` / worker pump / `setFillCount` precompute lane on #46; `--only=fill-tab,fill-apply-max` |
| **Brush** | Utility never arms on #45 (`utility=select`); preview null | **source-fixed-unproven** | **#46** (B9 `puzzle3d_addressed_window_id`); `--only=brush-stroke` |
| **Volume brush** | #44 arm FAIL; B11 host Alt+ground gesture in tree, not in #45 battery | **source-fixed-unproven** | vite reload + **#46**; `--only=volume-brush` |
| **Relocate** | Arm PASS #44; no pose delta | **broken** | Selection + drag hop; `--only=relocate` |
| **Gumball** | Handle hit PASS; scene pose unchanged | **broken** | Selection + B9 dirty refresh + `--only=gumball-drag` on #46 |

---

## Top remaining user-visible blockers (ranked)

After B8 slot retirement and B9/B10/B11 **land in source** (browser proof still pending on **#46**):

1. **Viewport selection does not reach Inspection / lock chrome / downstream tools** — blocks §6, §16, §21, §22, gumball commit, clipboard, catalogue add. Root: B9 guest fixes ride **#46**; #45b still FAIL with `Agent disconnected` noise.
2. **WindowConfig toggles & selects do not round-trip** (grid visible/snap, LOD auto, vortex show/direction, sun enabled) — sliders work; checkboxes/selects do not. B10 fixed Tree activation (vite-live) but #45b toggles still FAIL → **B12** guest/host WindowConfig publish path.
3. **Brush / Volume Brush utilities do not arm in world lane** (`activeUtility=select`) — B9 instance-scope fix not in #45 wasm.
4. **Fill planner never produces objects** (`fill count` stuck at 0) — user-visible “Fill does nothing” even when tab is on.
5. **Settings panel missing four steppers** — tab exists (`puzzle3d.panel.settings`) but body does not expose overlap/proximity/chunk/grid controls.
6. **Add Object dialog unreachable** — B11 manifest ordering fix in tree, not proven until **#46** + menu open path.
7. **Export download** — still `download=none` (#44); import/export half-working.
8. **Engagement command line** — B10 host fix vite-live; never PASS in battery.
9. **Session stability** — `Agent disconnected` during #45b mutate group corrupts many steps; needs stable serve before trusting partial PASSes.

---

## wasm **#46** vs **vite-live**

| needs **wasm #46** (guest `semio-s-plugin-puzzle` rebuild) | **vite-live** (renderer reload only) |
|---|---|
| B9: utility publication per window instance; first-pick / typed topology; import taps verification | B9: `browserActorDispatchUiScopeV1` mutation dirty (already in host bundle) |
| B9: history alias paging (host Rust in plugin crate — actually shipped in #45 framework; benefit already in #45b) | B10: Tree checkbox activation (`🌳️Tree/🟦️.tsx`) |
| B10: projection/sun measure ids + `default_open` (guest-linked framework plugin) — **partially in #45** (measures visible in #45b) | B10: Settings/Display dock tabs, engagement pane id + single-fold, `Pane` id |
| B10: camera View dispatch no history row — **in #45** (PASS #45b) | B11: Volume Brush host ground gesture (`World3dHost`) |
| B11: `openAddObjectDialog` menu promotion, context menu hits, `focusSelection` empty selection, target-volume notice | B11: ShellHost `defaultPrevented` guard, `treeRowDragPayloadAttributes` |
| B8/B0/B2/B3: already in **#45** | B4: `uiRefreshSectionUnchanged` (host PluginRuntime) — live |
| Fill planner, settings steppers body, export download, projection repaint | — |

**Practical rule:** anything reading `activeUtility`, Inspection body, dialog manifest, or document mutation → **#46**. Chrome-only gestures (volume brush raycast, engagement DOM, tree checkbox, catalogue drag payload) → **vite reload** first, then confirm on #46 for guest halves.

---

## Suggested verification sequence (post-#46 materialize)

1. `component-release` → materialize **#46** (HEAD: B9 + B11 guest + any B12).
2. Restart `:6013` serve (avoid 1-day wedged vite; coordination 21:12 note).
3. `🔍️browser-probe.ts --battery --reload-between-groups --port=6013` → compare to `probe-2026-09-11T13-15-23.md` fault count (expect **0** slot-leak messages).
4. If mutate group still noisy: run B11 recipe isolates (`--only=selection-surfaces`, `brush-stroke`, `add-object-dialog`, …) on fresh pages.

---

## Goal verdict

**Puzzle 3D is not end-to-end for a user.** Windows and camera are largely proven on #45b; the tool surface (fill, brush, volume brush, selection-driven edit, settings, add-object, export) remains broken or unproven on the newest wasm. Working-tree fixes from B9–B11 are the critical path; **#46 browser battery** is the gating evidence before calling any checklist row **proven**.
