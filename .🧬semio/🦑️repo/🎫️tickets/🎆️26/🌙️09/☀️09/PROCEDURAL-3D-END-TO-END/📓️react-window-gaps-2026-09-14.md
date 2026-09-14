# 🪟 React window gaps — generation3d editor, 2026-09-14 (lane `react-window-gaps`)

Scope: the **React** renderer of the generation3d playground on **:6023** (`📜️serve-generation3d-react-6023.sh`,
direct vite serve, `SEMIO_VITE_HMR=0`). Closes `📓️window-coverage-audit-2026-09-14.md` §5 items **2, 4, 5, 6,
7, 8, 9, 10** — the unwired probes, the unlocalized chrome, the undriven status states, the edit-mode
gumball, the viewer's own actions and the two option lists.

Every battery row this lane added or hardened asserts a **user-visible effect** — a `data-published-value`
the guest published, the `data-sun-json` the scene's light is given, `data-selection-json`'s
`transformMode`/`showEdges`, the Inspection panel's Id field, a rendered outline row's status word, a
menu row's label — never a `[DEBUG]` console line alone.

---

## 1. Window × action coverage after this lane

`✓` = a battery step drives it, asserts a user-visible effect, and **ran green**. `▲` = a step exists and
ran, but its verdict is weaker or its precondition failed (named in §7). `✗` = a step exists and is red.
Nothing here is marked on the strength of the step merely existing.

| Window / surface | Action | Before | After | Battery row · step |
|---|---|---|---|---|
| **Flow** (`procedural-main`) | LOD picker, clicked | UNCOVERED (and **unclickable**) | ✓ | `preview-chrome` · `lod-mode:coarse/fine/medium` |
| Flow | `Window Options` / `Actions` / `Utilities` chips reachable by pointer | **dead to a pointer** | ✓ | `preview-chrome` · `measure-rails-open` |
| Flow | arrow-key traversal (`↓ ↑ ← →`) | UNCOVERED | ✓ | `graph-keyboard` · `arrow-*`, each asserting its OWN verb |
| Flow | `Enter` → `activateSelection` | UNCOVERED | ✓ | `graph-keyboard` · `enter-activate` |
| Flow | traversal changes what is selected | UNCOVERED | ✗ | nothing observable moved — see F1; the hardened step has not been run |
| Flow | right-click context menu, unselected | UNCOVERED | ✓ | `menus` · `context-menu-unselected` |
| Flow | right-click context menu, with a selection | UNCOVERED | ✗ | `menus` · `context-menu-selected` — precondition (a selection) not armed |
| Flow | a menu row actually dispatches | UNCOVERED | ✓ | `menus` · `context-menu-acts` (widget positions move) |
| Flow | node status word per row, all six states | only `Evaluated` | ✓ | `status-states` · `en:ok/stale/queued/computing/error/blocked` |
| Flow | the same six in German | never | ✓ | `status-states` · `de:*` |
| **Preview** (`procedural-preview`) | Show-mode picker, clicked (4 rows) | UNCOVERED (chord only) | ✓ | `preview-chrome` · `show-mode:*` (+ `showEdges` flips) |
| Preview | Sun enable toggle | UNCOVERED | ✓ | `preview-chrome` · `sun-toggle` (light `enabled` flips) |
| Preview | Sun azimuth / elevation / intensity | UNCOVERED | ✓ | `preview-chrome` · `sun-azimuth/elevation/intensity` (light `position`/`intensity` move) |
| Preview | edit-mode gumball Move / Rotate / Scale | UNCOVERED (generate only) | ✓ | `preview-chrome` · `gumball-rail`, `gumball-move/rotate/scale` |
| **Viewer Preview** (`procedural-view-preview`) | Show picker | only "a window mounted" | ✓ | `viewer-actions` · `viewer-show:*` |
| Viewer Preview | Detail (LOD) picker | only "a window mounted" | ✓ | `viewer-actions` · `viewer-lod:*` |
| Viewer Preview | Sun toggle + azimuth | only "a window mounted" | ✓ | `viewer-actions` · `viewer-sun-*` |
| Viewer Preview | example switch from the viewer | only "a window mounted" | ✓ | `viewer-actions` · `viewer-example-switch` |
| Viewer Preview | Export | UNCOVERED | ✗ | `viewer-actions` · `viewer-export` — row not reached, no download |
| Viewer Preview | import / gumball / node graph **absent** | UNCHECKED | ✓ | `viewer-actions` · `viewer-withholds-editor-verbs` |
| **Navbar / chrome** | actions palette German, all rows | 5 of ~38 | ▲ | `gaps` · `actions-pane-de` widened to every row; the `gaps` row was not re-run by this lane |
| **Export form** | the seven `EXPORT_FORMATS` rows | UNCOVERED | ✗ | `menus` · `export-formats` — staged form never appeared |
| **Panels** (Catalogue / Inspection / Document) | German for every reachable label | 7 of 39 | ▲ 26/33 | `panel-i18n` · one step per field; 5 unreached (§7) |
| Role switch | the role/mode actually changed | `windows.length > 0` | ▲ | `role-switch` verdict now decides on `roles`/`modes`; that row was not re-run by this lane |

---

## 2. Defects found and fixed (product)

### D1 — The Flow window's floating pane chrome was dead to a pointer

**Symptom.** `Window Options` (which is where the LOD picker lives), `Actions` and `Utilities` on the Flow
window rendered, were focusable, and ate every click. The same chips on the sibling Preview window worked.

**Root cause.** `NodeGraphHost` paints a full-bleed pointer overlay at a raw `z-30`, a label canvas at
`z-40` and the marquee at `z-50`. The host root was `relative` with no `z-index`, so it created **no
stacking context** and those raw numbers competed directly with the shell's own ladder (`--z-pane: 20`).
The graph overlay therefore painted on top of the window's pane chrome. `World3dHost` has no such overlay,
which is why only the Flow window was affected.

**Evidence.** `elementsFromPoint` over the chip's centre answered the graph's `absolute inset-0 z-30` div;
a programmatic `.click()` on the chip opened the rail the pointer could not reach.

**Fix.** `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` —
new `NODE_GRAPH_HOST_CLASS` adding `isolate`, so the host's layer numbers stay inside its own stacking
context. **After:** `measure-rails-open` green, `lod-mode:*` 3/3 green, the rail lists `LOD` again.

### D2 — The sun's effect on the scene was unobservable

**Symptom.** Nothing outside React could tell a sun slider that moved the guest config from one wired to a
field no light reads. The measures rail's `data-published-value` proves only that the guest *accepted* a
value.

**Fix.** `🌐️World3dHost/🟦️.tsx` — new exported `world3dSunDomJson(environment)` and
`data-sun-json` on the host root, publishing exactly the values the scene's `<directionalLight>` is given
(`enabled`, `azimuth`, `elevation`, `intensity`, `color`, and the derived `position`). Also added
`showEdges` and `selectionMode` to `worldSurfaceSelectionDomV1`, the "what this pane paints" projection the
show-mode picker drives.

**Evidence.** `sun-azimuth`: published `45 → 360`, light `position [69.5,69.5,68.8] → [98.3,0,68.8]`.
`show-mode:shaded+edges`: `showEdges false → true`; `points`: `true → false`.

### D3 — A context-menu row showed the user an internal id

**Symptom.** The Flow context menu rendered a row reading literally `menu.group.io`, in both locales, next
to correctly labelled `Create` and `Methods` rows.

**Root cause.** A `Menu::group(category)` row travels to the shell with `label: None` **by contract**; the
shell resolves it from the closed 20-id ribbon-parent taxonomy. `io` is not in that table — it was the only
out-of-taxonomy group category in the whole repo — so the shell fell back to the raw id.

**Fix.** `✏️editor/🦀️.rs` — `CONTEXT_MENU_TRANSFER_CATEGORY = "transfer"` (the taxonomy's own name for
moving a document in and out), with a Rust law so the next group cannot drift out of the table.

### D4 — Every window-chrome measure label was hardcoded English

**Symptom.** A German-locale user read `Show / Shaded / Wireframe / Points`, `LOD / Coarse / Medium / Fine`
and `Sun / Enabled / Azimuth / Elevation / Intensity` beside a correctly translated palette entry for the
identical verb. Measured on the rail: `["LOD","Show","Sun","Enabled","Azimuth","Elevation","Intensity"]`.

**Fix.** The labels now resolve per locale:

- `🧰️framework/.../🔌️plugin/🦀️.rs` — `world3d_sun_measures(id_prefix, sun, is_de, action)`
  (`Sonne / Aktiviert / Azimut / Höhe / Intensität`), threaded through all **seven** call sites
  (generation3d editor + viewer, cad, process3d, puzzle3d, puzzle5d, lowpoly).
- `✏️editor/…/👁️preview/🦀️.rs` — `show_mode_measure(show_mode, is_de, …)` and `show_mode_row(mode, is_de)`
  (`Anzeige / Schattiert / Schattiert + Kanten / Drahtgitter / Punkte`).
- `✏️editor/…/🕸️flow/🦀️.rs` — `window_measures(lod_mode, is_de, …)` (`Detailgrad / Grob / Mittel / Fein`).
- `👁️viewer/…/👁️preview/🦀️.rs` — the viewer's own twins of all three.

Threading the locale needed a locale-bearing value at each call site. Three of the seven only hold a
resolved `AppLabels`, so `app_labels!` now emits `locale()` / `is_de()` on the generated struct (a resolved
label set knowing which locale it resolved for), and the rest read `view_state.locale` directly.

### D5 — The German for "Computing" said the opposite

`status_computing` was `native_de "Berechnet"` — the past participle, i.e. *computed*, which collides with
`status_ok` (`Ausgewertet`). The framework's own preview pill already said `Berechnen` for the identical
English word, so one app showed a German user two different words for one state. Fixed to `Berechnen` in
`🗣️terminology/🦀️.rs` and in `🧫️fixtures/🗣️terminology.json`.

### D6 — `io-surface`-class vacuity in my own rows (fixed before shipping)

`graph-keyboard`'s first verdict passed `traversal moves the published selection` on a constant
`["Flow","Preview"]` — those are dock tabs, not graph rows. Replaced by the Inspection panel's Id field,
read by element id, which is fed by the same framework-owned `graph` selection a click moves. See §4/F1 for
what that measurement then exposed.

---

## 3. Peer-churn breakages repaired in passing

Not this lane's changes, but they blocked its own build and were forward-fixable:

- `🕸️flow/🦀️.rs` — `NodeGraphOperatorChannelRecord` gained `value_types`; the generation3d
  `operator_channel` builder now sets it.
- `✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `flow_eval_tick_budget()` gained a `turn_started_us` parameter;
  both call sites pass `None`.
- **Shared disk**: the build volume hit 100 % (120 MiB free) mid-lane, failing every cargo invocation in
  the fleet with `No space left on device`. Pruned `⚡️cache/cargo/build/debug/incremental` (119 GB), the
  documented remedy; 124 GB free afterwards. It regrows — see `📓️status.md`'s incremental note.

---

## 4. Defects found and NOT fixed here (named, with owner)

### F1 — Keyboard traversal of the node graph is invisible to a user

All five verbs dispatch correctly — `arrow-down → selectNextNode`, `arrow-right → selectDownstreamNode`,
`Enter → activateSelection`, every one asserted against its **own** action id, 9/9 — and
`🎮️commands/🧭️navigate-graph` emits a real `InteractionWrite::replace` on the `graph` domain. But with the
Flow outline in view and nothing else open, **no DOM state changes at all** across three arrow presses:
no row gains `aria-selected`, no row gains `data-selected`, the preview's `selectedIds` stays `[]`.

Root cause located: `PanelTreeBuilder` accepts `.selected(ids)` / `.highlighted(ids)` and
**`PanelTreeBuilder::build` drops both** — its own doc says "`.selected()`/`.highlighted()` no longer reach
the built tree this wave". The Flow outline therefore cannot mark a selected node at all, by click or by
key, and the shell-side `ui_tree_stamp_presence` is the only thing that ever stamps `presence.selected`.

Not fixed here because completing that framework wave is owned outside this lane and its blast radius is
every panel tree in every app. **Owner:** `🧰️framework/…/🔌️plugin/🦀️.rs` `PanelTreeBuilder::build`.
This lane's `graph-keyboard` row now measures the traversal through the Inspection panel instead, which is
the channel that does follow the selection — so the regression is covered while the outline gap stands.

### F2 — The generate-preview window kind's own `window_measures` is unreachable code

`generate_preview::window_measures` exists and is localized, but `Generation3dPlayApp::window_measures`
hands the **edit** preview's `measures.clone()` to both preview windows. Harmless today (the two are
byte-identical) and deliberately left alone: deleting it belongs with whoever owns that window's chrome.

---

## 5. Laws

| Law | File | Output |
|---|---|---|
| `context_menu_groups_are_taxonomy_categories` | `✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `ok` — `[DEBUG] generation3d context-menu groups: ["transfer", "create", "methods", "transfer", "create", "methods", "targets"]` |
| `window_measure_labels_are_localized` | same | `ok` — `[DEBUG] generation3d localized 14 window-chrome measure labels` |
| `sun_measures_are_exposed_on_preview_windows` (pre-existing, kept green across the signature change) | same | `ok` |

`cargo test -p semio-s-artifact-procedural-generation3d --lib --features component-app-assembly`
→ `test result: ok. 3 passed; 0 failed` (`🗑️generated/react-gaps/law-run-4.txt`).

`window_measure_labels_are_localized` walks both locales' whole measure tree and fails on any label a
German user still reads in English, so the next measure added is caught here rather than in front of a user.
`context_menu_groups_are_taxonomy_categories` asks the menu with and without a selection and asserts every
`menu.group.<category>` is a `RIBBON_PARENT_CATEGORIES` member.

**TS twin:** `world3dSunDomJson` is a pure exported function beside `world3dCameraDomJson`; `showEdges` /
`selectionMode` ride `worldSurfaceSelectionDomV1`, which the World3dHost component tests already cover.

Cargo scope check: `cargo check -p semio-s-plugin-procedural -p semio-s-plugin-puzzle` → **exit 0, 0
errors** (`🗑️generated/react-gaps/cargo-check-mine.txt`). `cad` / `process` / `lowpoly` did not compile in
the same sweep for reasons unrelated to this lane (a peer's in-flight `🔺️diff` module and an unresolved
`semio_framework_tool_run`); their `world3d_sun_measures` call sites were updated and are covered by the
restage.

---

## 6. Battery

`🐍️react-battery.mjs` now takes **`SEMIO_BATTERY_ROOT`** (default `react-verify`), so this lane writes to
`🗑️generated/react-gaps/` and never clobbers `react-verify/`. Rows went **16 → 23**:

| Row | Script | Status |
|---|---|---|
| `preview-chrome` | `🐍️preview-chrome-probe.mjs` (new) | **16/16 green** |
| `status-states` | `🐍️status-states-probe.mjs` (new) | **13/13 green** |
| `graph-keyboard` | `🐍️graph-keyboard-nav-probe.mjs` (wired) | **12/12 green**, hardened afterwards (§4/F1) |
| `menus` | `🐍️menu-dump-probe.mjs` (rewritten + wired) | rerun pending — see §7 |
| `viewer-actions` | `🐍️viewer-actions-probe.mjs` (new) | run pending — see §7 |
| `panel-i18n` | `🐍️panel-i18n-probe.mjs` (rewritten + wired) | run pending — see §7 |
| `inspection-i18n` | `🐍️inspection-i18n-probe.mjs` (wired) | run pending — see §7 |

Hardened existing rows: `role-switch` (decides on `roles`/`modes`, not `windows.length > 0`), `gaps` →
`actions-pane-de` (every pane row compared against its own English pass, plus `vanished`/`unlabelled`
checks, instead of five hardcoded pairs).

### Numbers actually run on :6023

Two stages. The first (pre-19:00 guest) carried the host fixes but not the guest ones; the second
(19:36 guest, after lane `contributions-ingress-ceiling` lifted the command-ingress caps) carried both.

| Row | Steps | Stage | Notes |
|---|---|---|---|
| `preview-chrome` | **16/16**, 0 page errors | first | `show-mode` 4/4, `sun` 4/4, `lod` 3/3, `gumball` 4/4 |
| `status-states` | **13/13**, 0 page errors | first | all six states, both locales |
| `graph-keyboard` | **12/12**, 0 page errors | first | every arrow and `Enter` reached its OWN verb |
| `viewer-actions` | **12/13**, 0 page errors | second | only `viewer-export` red |
| `menus` | **4/6**, 0 page errors | second | `context-menu-unselected` ✓, `context-menu-acts` ✓ |
| `panel-i18n` | **26/33**, 0 page errors | second | 5 fields genuinely unreached (§7) |
| `inspection-i18n` | **2/8**, 0 page errors | second | same five fields, from its own angle |

Evidence worth naming:

- `status-states`: `Column Height Evaluated` · `Vector Stale` · `Brep.curve.polygon Queued` ·
  `Math.vector Computing` · `Polygon Error` · `ExtrudeCurve Blocked`, and in German
  `Column Height Ausgewertet` · `Rectangle Veraltet` · `Vector In Warteschlange` · `Rectangle Berechnet` ·
  `Polygon Fehler` · `ExtrudeCurve Blockiert`. `error` was driven by typing a degenerate value into the
  Inspection panel; `blocked` by an unwired port.
- `sun-azimuth`: published `45 → 360`, the scene's light `position [69.5,69.5,68.8] → [98.3,0,68.8]`.
- `show-mode`: `shaded+edges` flips `showEdges false → true`, `points` flips it back.
- **D3 proven live:** `groups: ["menu.group.transfer","menu.group.create","menu.group.methods"]`,
  `rawIdRows: []`.
- `menus` · `context-menu-acts`: `Reorganize` clicked from the menu moved 7 widgets in `fixture.layout`.
- `viewer-actions` · `viewer-withholds-editor-verbs`: `offered: []`, `transformUtilities: []`,
  `nodeGraph: []` — the viewer really does withhold import, the gumball and the node graph.

---

## 7. Not claimed

- **Four rows are not green.** `menus` 4/6, `viewer-actions` 12/13, `panel-i18n` 26/33,
  `inspection-i18n` 2/8. Every remaining red is named below with what was measured; none is hidden behind
  a passing step.
- **`menus` · `context-menu-selected`** — red because the probe could not establish its own precondition:
  the outline-row click that would arm a selection found no row (see below). The menu's own always-on
  floor (8 rows across three submenus) is green, so what is unproven is the selection-CONDITIONED half,
  not the menu.
- **`menus` · `export-formats`** — the `Export Document` row is found and clicked (`menuRow` non-null) but
  the staged arg form never appeared to the probe, so the seven `EXPORT_FORMATS` rows are still unread.
  Whether that is a probe-reachability problem or a real staged-form defect is **undetermined**.
- **`viewer-actions` · `viewer-export`** — same shape: the context menu opened on the viewer surface and
  the export row was not reached (`invoked: ["interactionSelect","setCamera"]` — the right-click was
  claimed by the World3d surface). No download was produced, so viewer export is **unproven**, neither
  confirmed working nor shown broken.
- **Five `Generation3dLabels` fields never appeared in German on any surface**, with both panels
  confirmed open by their own body markers:
  `catalog_neuron` (identical-by-design, so unreachable either way), `catalog_slider` (`Schieberegler`),
  `catalog_note` (`Notiz`), `no_selection` (`Keine Auswahl`) and `preview_hint`. The Catalogue paints
  registry OPERATOR rows (`Brep Compound Cut`, `Fuse`, `Intersect`, …), not the four widget kinds
  `generation3d_catalog_label` maps, so those three catalogue labels may be **dead code rather than
  untranslated** — not established here. `no_selection` did not paint even with the Inspection panel
  confirmed open and nothing selected. `preview_hint` needs a generate mode with no generation.
  These are reported as findings to investigate, NOT as fixed.
- **The Flow window no longer carries an inline accessible outline**, and this is a peer's deliberate
  change, not a defect: the German canvas hint now reads "Interaktiver Knotengraph. **Das Artefakt-Panel**
  listet alle Knoten, Anschlüsse und Leitungen für Tastatur und Screenreader." The listing moved to the
  Artifact panel. This lane's earlier reading of a "missing outline" is superseded; what remains true is
  F1 below.
- **F1 stands and is not fixed**: all five traversal verbs dispatch (9/9, each against its own action id)
  and `🎮️commands/🧭️navigate-graph` emits a real `InteractionWrite::replace`, but no DOM state changed
  across three arrow presses on the surfaces this lane observed, and `PanelTreeBuilder::build` drops
  `.selected()`/`.highlighted()` by its own admission ("no longer reach the built tree this wave").
  The `graph-keyboard` row now measures the traversal through the Inspection panel; **that hardened
  verdict has not itself been run** — the 12/12 above predates it.
- **D3 is proven live. D4 and D5 are proven by Rust law only.** `window_measure_labels_are_localized`
  shows the German exists in the manifest for all 14 chrome labels; no browser run has yet read
  `Sonne`/`Detailgrad`/`Anzeige` off the rail. `panel-i18n` does not cover the chrome rail — that is the
  obvious next step.
- **D1 and D2 are proven live** (host TS, no restage needed): `preview-chrome` 16/16 ran on the fixed host.
- `cad`, `process3d` and `lowpoly` did not compile in a direct `cargo check` at any point during this lane
  (a peer's half-written `🔺️diff` module and an unresolved `semio_framework_tool_run`, 19 errors, none
  mentioning `sun`/`is_de`). Their `world3d_sun_measures` call sites were updated and the 19:36 restage
  built the dev plugin set successfully, but **this lane never saw those three crates compile green**.
- `status_computing`'s German is now `Berechnen`, matching the framework's preview-pill `phaseLabel`. The
  two remain separate tables that happen to agree; they were not unified.
- `preview-chrome`'s `gumball-*` steps prove the rail moves `transformMode`/`activeUtility`. They do **not**
  prove `gumballActive: true`.
- No claim is made about the **wgpu** renderer; this lane never touched it.
- The disk prune in §3 unblocked the fleet; it is not a fix, and the incremental cache regrows.
