# User-Journey Gap Punchlist — generation3d, React + wgpu (2026-09-13)

Read-only audit. Lane: `audit-user-journey-gaps`. No source edited, no builds/servers run. Every
claim below is either (a) a direct read of source on disk at
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/` (abbreviated
`generation3d/` below) checked 2026-09-13, or (b) a citation of a prior ticket report, named
explicitly. Where a report's runtime claim is stale relative to source, that is called out.

## 0. Live-state caveat — read this before trusting any "proven" row

Session 4 (today, `📓️status.md` "Session 4" section) found the **React renderer currently
regressed**: `setContributions` never yields on 6018 as of 12:10 today (watchdog kills shard 0 after
18–24 s, loops forever, `hosts: []` — `🗑️generated/s4-boot-check-1/`), and the **wgpu serve was down**
(missing activation receipt). A fix lane (`fix-forward-set-contributions-hang`) is in flight but has
not landed as of this audit. **Every "proven at runtime" row below cites the run that proved it
(journey #9, 2026-09-12 ~00:05–06:35, and the wgpu lanes through 09-13 07:55); none of it has been
re-confirmed against the current tree.** Treat "proven" as "proven once, on a since-moved tree" until
a fresh restage + journey-probe run happens. This is itself gap **P0-0** below.

---

## 1. Window/panel/mode/role inventory — what a user can reach and its proof state

Legend: **PROVEN** = a cited browser/probe run observed it working. **WIRED/UNPROVEN** = source-complete,
dispatch chain exists, no browser observation found in any report. **MISSING/DEAD** = no UI control,
or a control that cannot be hit (dead handler, unreachable code, or a declared-but-unrouted verb).

### 1.1 Editor, mode `edit` (default) — `generation3d-plugin.json` mount at `✏️s/🔌️plugins/🌀️procedural/🦀️.rs:105`

| Window | Controls (from source + `window_kind_action_refs`, `✏️editor/🦀️.rs:1930-1950`) | State | Proof |
|---|---|---|---|
| **Flow** (`procedural-main`, `🎭️modes/✏️edit/🪟️windows/🕸️flow/`) | render node graph (`NodeGraphScene`); `nodeGraphEdit` (select/drag/param-patch node), `nodeGraphViewport` (pan/zoom the graph canvas), `setLodMode` | **PROVEN** graph paints all 7 widgets + camera auto-fits + readable captions (`📓️node-graph-camera-fit-labels-2026-09-12.md` §0); **PROVEN** node drag dispatches `nodeGraphEdit` (`📓️gap-inventory-2026-09-10.md` §4 "Node drag"); **PROVEN** catalogue-drag→`addWidget` | journey-9 screenshots; `🐍️flow-window-probe.mjs` |
| ‑ Wire connect/disconnect between ports | part of `nodeGraphEdit`'s mutation surface (`🎮️commands/✏️node-graph-edit/🦀️.rs`) — no separate command exists | **WIRED/UNPROVEN** — no report or probe step drags a wire between two ports and asserts a new synapse; `📓️gap-inventory-2026-09-10.md` §4 only proves node-move and catalogue-drop, not port-to-port wiring | none found — see gap #3 below |
| ‑ Add / remove node | catalogue drag → `addWidget`; delete key / context menu → `removeWidget`/`deleteSelection` | **PROVEN** add (catalogue drag, gap-inventory §4); `removeWidget`/`deleteSelection` **WIRED/UNPROVEN** — dispatch chain exists (`🎮️commands/➖️remove-widget/`, `🎮️commands/❌️delete-selection/`, both with native unit tests per `📓️interaction-coverage-2026-09-12.md` §2), no browser click-to-delete observed in any report | native tests only |
| ‑ Undo / redo | `mod+z` / `mod+shift+z`, app-level keybindings (`✏️editor/🦀️.rs:1978-1979`), framework history verbs | **DEGRADED** — native law `undo_redo_round_trips_flow_graph_edits` is in the **flaky/failing** set as of 2026-09-12 (`📓️interaction-coverage-2026-09-12.md` §4.1, listed among the 9 pre-existing generation3d `--lib` reds that "swap between runs"); zero browser proof of `mod+z` actually undoing a graph edit found in any report | see gap #4 |
| **Preview** (`procedural-preview`, World3d) | camera orbit/pan/zoom, hover, single/multi select, gumball (`translateSelection`/`rotateSelection`/`scaleSelection`), show mode, sun, cancel | **PROVEN** hover→`hoverTarget`, click→deduped `selectedIds`, gumball on, orbit→`setCamera` (`🗑️generated/probe-interact-final-*/`, cited `summary-2026-09-12.md`); **PROVEN** marquee/shift-add/empty-click/orbit-debounce as jsdom+Node-twin laws (`📓️interaction-coverage-2026-09-12.md` §1.3, 8/8); **PROVEN** cancel button appears + click fires (native+TS, `📓️preview-eval-cancellation-2026-09-12.md` §3); meshes render for all 8 examples (journey-9) | multiple, cited above |
| ‑ Fit (camera fit) | `Fit graph` control, keyboard shortcut `F`, localized en/de | **PROVEN** — landed + runtime-proven on a side serve, port 6048, not yet re-verified on 6018 (`📓️node-graph-camera-fit-labels-2026-09-12.md` §0, "6018 was never restarted") | needs re-check post-restage |
| ‑ Cancel long eval | `cancelPreviewEval`, button appears when `cancellable:true` | **DEGRADED** — the button-and-dispatch mechanism is proven, but at probe time the **served wasm** still published `cancellable:false` throughout a 51 s eval (`📓️preview-eval-cancellation-2026-09-12.md` §4.1); fixed source needs a restage the lane could not do. Session 3 later reports "evaluate budgeted/resumable" (`summary-2026-09-12.md` root cause #6) which should close this — **not independently re-probed for the cancel-button-appears case after that landed** | gap #6 |
| ‑ Rotate/scale/delete selection dispatch-and-assert | | **PROVEN** at the Rust-unit level only (`📓️interaction-coverage-2026-09-12.md` §2, 8 new laws); **no browser proof** a gumball rotate/scale handle drag reaches the same command | native only |

### 1.2 Editor, mode `generate` — layout `[22,43,35]` (`🎭️modes/🧬️generate/🦀️.rs:21-33`)

| Window | Controls | State | Proof |
|---|---|---|---|
| **Generations** (`generation3d-generations`) | `addGeneration`, `selectGeneration`, `renameGeneration`, `removeGeneration` | **PROVEN** add — journey probe clicks the `Add Generation` tree row, `addGeneration` settles, generate preview shows `meshes=1` in 5 s (`📓️summary-2026-09-12.md`, `🐍️generate-mode-probe.mjs`) | `🗑️generated/generate-mode-2/` |
| ‑ select / rename / remove generation | tree-row actions | **WIRED/UNPROVEN** — declared, native-tested (per §5 of `📓️audit-window-inventory-2026-09-12.md`), no browser click observed for select/rename/remove specifically (only add was probed) | gap #7 |
| **Form** (`generation3d-generate-form`) | `updateGenerationValues` — edit input widget values for the active generation | **WIRED/UNPROVEN** — renders hint or real form chrome per `📓️gap-inventory-2026-09-10.md` §1.2; value-edit round trip has no cited browser probe (the journey probe never types into the Form window) | gap #8 |
| **Preview** (`generation3d-generate-preview`) | show mode, sun, camera — **no gumball** (`translateSelection`/`rotateSelection`/`scaleSelection` intentionally absent from `window_kind_action_refs`, `✏️editor/🦀️.rs:1944-1950` vs `:1931-1940` for edit preview) | **PROVEN** mesh renders after Add Generation (journey-9); gumball **MISSING BY DESIGN** — a user cannot transform a generated instance directly in generate mode | `📓️audit-window-inventory-2026-09-12.md` §4 item 4 |

No `enterGenerate` command exists; mode switch is shell-only (`playground.navbar.modes.generate` button,
`ShellHost/🟦️.tsx:8429`, or `mod+alt+→` chord added by `📓️role-switch-keyboard-2026-09-12.md`).

### 1.3 Viewer role — `s.procedural.generation3d@1/*#viewer`, mode `view`

| Window | Controls | State | Proof |
|---|---|---|---|
| **Preview** (`procedural-view-preview`) | all 7 view commands (`setShowMode`, `setLodMode`, `setCamera`, `toggleSun`, `setSunAzimuth/Elevation/Intensity`) | **PROVEN reachable** — role switch fixed at source (`?role=viewer` query param wired into the React dev entry, app-pin-outranks-role bug fixed, `📓️role-switch-keyboard-2026-09-12.md` §1.1-1.2) and **PROVEN meshes render**: all 8 examples show meshes in the viewer per journey-9 (`📓️summary-2026-09-12.md` table row "all windows") | `🐍️role-switch-runtime-probe.mjs`, journey-9 results.json |
| ‑ Cancel eval in viewer | none — `👁️viewer/🦀️.rs` declares no `cancelPreviewEval` counterpart | **MISSING** — "the viewer should probably own the verb too" (`📓️preview-eval-cancellation-2026-09-12.md` §5 item 3); a long viewer-side eval has no cancel affordance at all | gap #9 |
| ‑ Mode/role switch keyboard | `mod+alt+e`/`mod+alt+v` chords, `playground.navbar.roles.*` buttons | **PROVEN** wired (`📓️role-switch-keyboard-2026-09-12.md`) | same report |

### 1.4 Panels (framework-injected tabs, not window-layout panes; independent of mode)

| Panel | Controls | State | Proof |
|---|---|---|---|
| Document (artifact tree) | selection only, no per-item action | **PROVEN** (renders; framework `graph` selection) | `📓️gap-inventory-2026-09-10.md` §2 |
| Catalogue | `addWidget` per row, paginated | **DEGRADED** — only first page of operators shown per group, continuation row omits count (`catalogue/🦀️.rs:78-84`) | same report |
| Inspection | `patchFlowWidgets` on slider widgets, selection-driven | **PROVEN live** — "a genuinely live selection-driven inspector" (`📓️feature-inventory-2026-09-09.md` §8 closing line); form-field edit → parameter update **not independently re-probed** after the 2026-09-11/12 preview-chain fixes | native + one prior probe |
| History (framework default) | undo/redo, no generation3d-specific chrome | **DEGRADED** — cosmetic-only gap, framework default applies | `📓️gap-inventory-2026-09-10.md` §2 |

### 1.5 Assembly artifact (`s.assembly@1/*`) — separate app, ports 6019/6019(wgpu 6119)

Mounted (`✏️s/🔌️plugins/🌀️procedural/🦀️.rs:32-33,107-110`) but **not reachable from the 6018
generation3d URL at all** — a different playground variant. Tree window (`framework.window.tree`),
9 mutation actions, native-tested (28 passed) but **zero browser probe ever run against it**
(`📓️audit-window-inventory-2026-09-12.md` §1.4). Listed because the ticket brief names it; ranked low
because it is out of the generation3d user flow by construction, not a regression.

---

## 2. Terminology / i18n

`✏️editor/🗣️terminology/🦀️.rs` — `Generation3dLabels` (`app_labels!` macro), 30 label fields, **every
field carries all four of `native_en`/`native_de`/`reuse_en`/`reuse_de`** — no default language, matches
CLAUDE.md. Covers window titles, status words, catalogue kinds, hints, `delete_selection`. **Gap**: no
report shows a runtime screenshot with the locale actually switched to German and UI text observed in
German — the 30-field coverage is a static/native fact, not a browser-proven one. `cancelPreviewEval`'s
label ("Vorschauberechnung abbrechen") and `Fit graph`'s en/de pair are the only two labels with cited
runtime text (`📓️preview-eval-cancellation-2026-09-12.md` §1.1 row 6; `📓️node-graph-camera-fit-labels-2026-09-12.md`
§0). The other 28+ labels — flow window titles, status pills, catalogue group names — are **WIRED/UNPROVEN**
in German at runtime.

---

## 3. Accessibility / keyboard reachability

| Path | State | Source |
|---|---|---|
| Mode switch (edit↔generate) | **PROVEN** — button + `mod+alt+←/→` chord | `📓️role-switch-keyboard-2026-09-12.md` |
| Role switch (editor↔viewer) | **PROVEN** — button + `mod+alt+e/v` chord | same |
| Undo/redo | **PROVEN wired** (`mod+z`/`mod+shift+z`, `✏️editor/🦀️.rs:1978-1979`) but the underlying law is flaky (§1.1 above) | `📓️audit-window-inventory-2026-09-12.md` §3.3 |
| `Fit graph` | **PROVEN** — `F` shortcut, focusable control | `📓️node-graph-camera-fit-labels-2026-09-12.md` |
| Every other one of the 30 editor commands (node select/drag, wire connect, gumball, generation add/select/rename/remove, form field edit, sun/camera/show controls) | **MISSING keyboard path** — "mode switching and window opening have no keyboard path today — mouse-only via the navbar buttons and panel tabs" was true as of 2026-09-12 for windows/panels; per-action keybindings for graph/preview/generate verbs remain mouse-only (only the two chords above and undo/redo exist app-wide) | `📓️audit-window-inventory-2026-09-12.md` §3.3 |
| ARIA / screen-reader labeling | **NOT AUDITED** — no report inspects `aria-*` attributes on the Flow/Preview/Generations DOM beyond `aria-pressed` on mode/role buttons (added by the role-switch lane); node-graph canvas, World3d canvas and tree rows are almost certainly canvas/SVG-painted with no accessible fallback — flag as unverified, not proven either way | data gap, this audit |

---

## 4. Customization

**NOT AUDITED by any prior report and not found by this pass.** No `🎨️` theming/customization directory
under `generation3d/`; CLAUDE.md's "develop customizable UIs" rule has no generation3d-specific
implementation found (framework-level theming, if any, is out of this artifact's tree). Flag as an
open question for the next wave rather than a scored gap — insufficient evidence either way.

---

## 5. Cancellation / progress for expensive operations

| Operation | Progress UI | Cancel | State |
|---|---|---|---|
| Flow eval + tessellate (edit preview) | status pill, phase/ratio, localized phaseLabel | `cancelPreviewEval` button | **PROVEN** button+dispatch; **DEGRADED** `cancellable` predicate was stale on served wasm at probe time (§1.1 above), evaluate now budgeted per session-3 summary but not re-probed for the button case |
| Flow eval (generate preview) | same status contract | same command exists app-wide (undeclared on the window, so dispatchable but not offered — `✏️editor/🦀️.rs:1965-1985` per `📓️preview-eval-cancellation-2026-09-12.md` §1.2) | **WIRED/UNPROVEN** whether the cancel button actually appears in generate mode's preview pane |
| Flow eval (viewer) | status contract present | **MISSING** — no `cancelAction` published, no command declared | gap #9 |
| Example switch | none observed | n/a | not designed as cancellable; switches are fast (3-78s) per journey-9, no report flags this as needed |
| Undo/redo | n/a | n/a | instantaneous, not expensive |

---

## 6. IO (import/export via `🚪️io`)

**The single most severe, concrete, currently-open gap this audit found.**

`generation3d/🚪️io/` implements 9 formats each direction (`json`/`txt`/`las`/`png`/`stl`/`dwg`/`obj`/
`gltf`/`ply`) with real round-trip codecs (`📓️io-codecs-2026-09-09.md`; json/dwg/stl/obj/etc. are real,
txt is an honest `Err` stub, "no `todo!`/`unimplemented!` anywhere" per `📓️audit-window-inventory-2026-09-12.md`
§4 "Verified NOT present"). **But grep of the entire editor and viewer source
(`grep -rln "🚪️io" generation3d/✏️editor generation3d/👁️viewer` → 0 hits, re-run this pass) finds
zero references to the `🚪️io` module from any command, menu, or action.** There is no `importDocument`/
`exportDocument`-shaped entry in the 30-row `Generation3dCommand` enum (`✏️editor/🦀️.rs:64-94`, listed
in full in §7 below) and no `👁️viewer` command either (`👁️viewer/🦀️.rs`, 8 commands total, all view-only
per `📓️audit-window-inventory-2026-09-12.md` §1.3). **A user cannot import or export a file from the
procedural 3d editor at all, through any control, today** — the entire IO subsystem is
library-complete and round-trip-tested but has no UI surface. This exact framing was flagged back in
`📓️feature-inventory-2026-09-09.md` §8 item 1 (then about wrong-format silent corruption); the codec
correctness has since been fixed, but the **UI-unreachability half was never addressed and remains
true today**, confirmed by a fresh grep this pass.

---

## 7. Full editor command roster (current source, `✏️editor/🦀️.rs:64-94`) — 30 rows

`setActiveExample`, `nodeGraphEdit`, `deleteSelection`, `removeWidget`, `moveMediaNode`, `addWidget`,
`patchFlowWidgets`, `reorganize`, `translateSelection`, `rotateSelection`, `scaleSelection`,
`addGeneration`, `removeGeneration`, `renameGeneration`, `updateGenerationValues`, `nodeGraphViewport`,
`setLodMode`, `setShowMode`, `toggleSun`, `setSunAzimuth`, `setSunElevation`, `setSunIntensity`,
`setCamera`, `selectGeneration`, `flowEvalTick`, `flowEvalResolve`, `flowTessellateResolve`,
`cancelPreviewEval`, `flowTessellateCancelResolve`, `setContributions`.

(Earlier reports cite 27 or 29 — the roster has grown by 1-3 rows since 2026-09-09/10; `rotateSelection`/
`scaleSelection`/`deleteSelection` all now have dedicated native unit-test dirs per
`📓️interaction-coverage-2026-09-12.md` §2.) Four are host/runtime-only, not palette items
(`flowEvalTick`, `flowEvalResolve`, `flowTessellateResolve`, `setContributions`); `cancelPreviewEval`
has UI affordance via the status chrome, not a toolbar button; `reorganize`, `moveMediaNode` have no
cited browser proof anywhere in this ticket — flag as **WIRED/UNPROVEN**, gap #10.

---

## 8. wgpu renderer (secondary target) — punchlist, from `📓️audit-wgpu-journey-readiness-2026-09-13.md`

The wgpu shell is materially behind React on UI surface, independent of the input-routing blocker:

| Gap | wgpu-shell.rs owning site | Rank |
|---|---|---|
| Input never reaches `Ui::dispatch_event` on the currently-served host (blocks everything below) | resolved by `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` (mailbox pumped every frame) + `📓️wgpu-retained-hit-registry-2026-09-13.md` (hit targets) — **landed 2026-09-13 07:30/08:30**, but the wasm that carries it was blocked on a peer's `DslField` migration until 07:55 and has not been journey-probed since | was P0, now **needs a fresh probe run** |
| Example picker is dead code — handler exists, nothing paints the control, stale non-dialect-keyed filter | `wgpu-shell.rs:6488-6491,6511-6518,11640` (dead), `:3516-3521` (stale filter) | fixed by `📓️wgpu-chrome-parity-2026-09-13.md` (13/13 Rust laws) — **runtime blocked on the input hop above at write time** |
| Mode/role buttons + chords | absent, `wgpu-shell.rs:8585-8596` | fixed same lane, same runtime caveat |
| `Fit graph` on wgpu | camera hardcoded `[4,-4,3]→[0,0,0]`, no fit pass | fixed by chrome-parity lane per its summary line — **not independently re-verified by this audit** |
| Preview cancel button on wgpu | guest already emits `cancellable`/`cancelAction`; no chrome control found | fixed by chrome-parity lane per its summary — **not independently re-verified** |
| Generate-mode panel bodies (Generations/Form/Preview chrome) | guest didn't publish them; root cause (`ViewModel::for_panel` stale `focused_window_id`) fixed | `📓️generate-mode-panels-2026-09-13.md`, 15/15 Rust + 14 vitest, native-proven |
| Full 8-example × edit/viewer/generate journey + hover/selection on wgpu | no `wgpu-journey-probe.mjs` run has completed (lane dispatched 07:15, blocked on wasm rebuild, session 3 ended before a run) | **open**, gap #1 below |

---

## 9. Ranked punchlist

### P0 — blocks "works end to end" outright, or reverses a prior proof

0. **Live state is currently regressed and unverified against the cited proofs.** React: `setContributions`
   never yields on 6018 as of today 12:10 (`s4-boot-check-1`, `hosts: []` after 73 s). wgpu: serve was
   down (missing activation receipt) at audit time. **Every PROVEN row in §1 needs a fresh restage +
   `🐍️journey-probe.mjs` run before being trusted for a live user.** Probe: re-run
   `📜️serve-generation3d-react-direct.sh`, then `🐍️journey-probe.mjs`, once
   `fix-forward-set-contributions-hang` lands.

1. **IO import/export has zero UI surface** (§6). 9 codecs, round-trip tested, unreachable by any
   command/menu/button in editor or viewer. Owning files: `generation3d/🚪️io/📤️export/`,
   `📥️import/` (codecs, real); `generation3d/✏️editor/🦀️.rs:64-94` (command enum, no IO entry);
   `generation3d/👁️viewer/🦀️.rs` (8 commands, no IO entry). Probe: none exists — would need a new
   `importDocument`/`exportDocument` command + menu/button before any probe is possible.

2. **wgpu full journey has never run** (§8). The input-routing fix landed today but the wasm carrying
   it (puzzle3d DslField migration) only became green at 07:55; no `wgpu-journey-probe.mjs` run has
   completed against it. Owning files: `🎯️targets/🧊️wgpu/🌐️server/🟦️.ts`, `wgpu-shell.rs`. Probe:
   the skeleton in `📓️audit-wgpu-journey-readiness-2026-09-13.md` §4, or the (possibly unfinished)
   `🐍️wgpu-journey-probe.mjs` dispatched 07:15 today.

### P1 — real, user-visible capability gaps on the (nominally working) React path

3. **Wire connect/disconnect between node-graph ports has no dedicated runtime proof.** Node-move and
   catalogue-drop are proven; port-to-port wiring is not independently exercised by any report.
   Owning file: `generation3d/✏️editor/🎮️commands/✏️node-graph-edit/🦀️.rs`; React drag source
   `NodeGraph/🟦️.tsx:1031`. Probe: extend `🐍️flow-window-probe.mjs` with a drag-from-output-port-to-
   input-port step and assert a new wire in the fixture JSON.

4. **Undo/redo law is flaky/failing natively, unproven in browser.**
   `undo_redo_round_trips_flow_graph_edits` is in the 9-failure set as of 2026-09-12 (§1.1). Owning
   file: `generation3d/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (test name), command dispatch via
   `mod+z`/`mod+shift+z` (`✏️editor/🦀️.rs:1978-1979`). Probe: `cargo test -p
   semio-s-artifact-procedural-generation3d --lib -- undo_redo_round_trips_flow_graph_edits
   --test-threads=1` run 5× to confirm flake, then a browser step (edit graph → `mod+z` → assert
   fixture reverts).

5. **Generate-mode preview has no gumball** — by design, but it is a real, visible capability gap
   between the two preview windows a user will notice. `✏️editor/🦀️.rs:1944-1950` vs `:1931-1940`.
   No probe until a product decision is made to add it.

6. **Cancel-button `cancellable` predicate was stale on served wasm** at last probe (51 s eval, always
   `false`). Session 3 later reports evaluate is now budgeted/resumable, which should fix this, but
   no report re-runs `🐍️cancel-preview-probe.mjs` after that landed. Owning files:
   `✏️editor/🦀️.rs:2413`ish (status projection), `🌊️flow/🖥️host/🦀️.rs`. Probe:
   `🐍️cancel-preview-probe.mjs` against `Sphere Cut With Torus` or `Sphere Box Fuse`, confirm a
   `cancellable:true` frame appears before completion.

7. **Generations window: select/rename/remove never independently browser-probed** — only `addGeneration`
   was clicked by the journey probe. Owning file: `🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs`.
   Probe: extend `🐍️generate-mode-probe.mjs` to click an existing generation row, rename it, remove it,
   and assert the roster + preview update.

8. **Form window: value-edit round trip never independently browser-probed.** Owning file:
   `🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs`. Probe: type into a Form field after Add Generation,
   assert `updateGenerationValues` dispatches and the preview mesh changes.

9. **Viewer has no cancel-eval affordance** for a long viewer-side evaluation. Owning file:
   `generation3d/👁️viewer/🦀️.rs` (8 commands, no `cancelPreviewEval` counterpart). Probe: open viewer
   on a slow example, confirm no cancel control ever appears (negative proof, then fix).

10. **`reorganize` and `moveMediaNode` commands have no cited browser proof anywhere in this ticket.**
    Owning files: `🎮️commands/🗺️reorganize/🦀️.rs`, `🎮️commands/🚚️move-media-node/🦀️.rs`. Probe: none
    exists; needs a UI trigger identified first (neither appears in the interaction table of
    `📓️gap-inventory-2026-09-10.md` §4).

### P2 — degraded but low severity, or explicitly out of scope

11. Catalogue panel pagination shows only the first page of operators per group, continuation row omits
    count (`panels/catalogue/🦀️.rs:78-84`).
12. `demo-session` example (9th, complete) still excluded from the picker (`examples()`,
    `✏️editor/🦀️.rs`); unreachable by design/oversight, unchanged since 2026-09-09.
13. i18n: 30 labels declared en+de with no default, but only 2 (cancel button, Fit graph) have
    cited runtime-observed German text; the other 28 are static-only proof (§2).
14. Accessibility: ARIA/screen-reader behavior of the Flow/Preview canvases and tree rows is
    unaudited by any report and by this pass — flagged as a research gap, not scored as failing.
15. Customization: no CLAUDE.md-mandated customizable-UI surface found under `generation3d/`; flagged
    as unaudited, not scored.
16. Assembly app (`s.assembly@1/*`) is fully out of the generation3d URL/flow; native-tested only,
    zero browser probe, by design (separate port/variant) — not a generation3d regression.
17. Cosmetic debug object (`evalLen`/`evalHead`) still unconditionally present in edit preview's
    `status_json` (`…/✏️edit/🪟️windows/👁️preview/🦀️.rs:84` area, per `📓️gap-inventory-2026-09-10.md`
    Lane J, not re-verified line-exact this pass).

---

## 10. Method

Read (no edits): `✏️editor/🦀️.rs`, `✏️editor/🗣️terminology/🦀️.rs`, `✏️editor/🎮️commands/` (directory
listing, 30 entries), `🚪️io/` (directory listing + `📓️io-codecs-2026-09-09.md`), `👁️viewer/🦀️.rs`
(via cited reports), plus in full: `📓️summary-2026-09-12.md`, `📓️audit-window-inventory-2026-09-12.md`,
`📓️interaction-coverage-2026-09-12.md`, `📓️audit-wgpu-journey-readiness-2026-09-13.md`,
`📓️gap-inventory-2026-09-10.md` (§§1-6), `📓️feature-inventory-2026-09-09.md` (§§7-8),
`📓️preview-eval-cancellation-2026-09-12.md`, `📓️node-graph-camera-fit-labels-2026-09-12.md` (headline),
`📓️role-switch-keyboard-2026-09-12.md` (§1), `🐍️journey-probe.mjs` (full), and the tail (~400 lines) of
`📓️status.md` covering session 3's close and all of session 4 to date. `grep -rln "🚪️io"` re-run fresh
against `✏️editor`/`👁️viewer` this pass, confirming 0 hits (the IO-unreachability finding, §6). Window
kind action counts cross-checked against current `✏️editor/🦀️.rs:64-94` (30 rows, not 27/29 as older
reports state — roster grew). No cargo/nx/browser commands were run by this lane.
