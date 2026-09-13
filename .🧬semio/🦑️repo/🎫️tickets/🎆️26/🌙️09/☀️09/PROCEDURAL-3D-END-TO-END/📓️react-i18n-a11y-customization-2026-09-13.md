# React procedural-3d — i18n, Accessibility, Customization (2026-09-13)

Lane: `react-i18n-a11y-customization`. Closes `📓️audit-user-journey-gaps-2026-09-13.md` §2 (i18n), §3
(accessibility), §4 (customization) and P2 items **#13, #14, #15**.

All runtime evidence is from **6018** (`http://127.0.0.1:6018/?plugin=generation3d`), the coordinator's
React dev serve, after a successful `activate-generation3d-react-dev` restage at 13m47s / exit 0
(11 components, "changed"). No git-modifying command was run; the ticket status file was not touched.

---

## 0. TL;DR

| # | Item | Before this lane | Now |
|---|---|---|---|
| 1 | **i18n** | 32 labels declared en+de, but only 2 ever *observed* in German at runtime (§2 of the audit) | German proven at runtime across **edit mode, generate mode, viewer role and the shell chrome**; 4 new canvas labels added; a **two-implementation law** (Rust + TS twin over one fixture) now fails if any label ships without German |
| 2 | **Accessibility** | The 4 scene canvases were **anonymous, unfocusable and silent** — `renderComponentSceneHost` dropped `AccessibilitySpec` entirely; `live`/`shortcut` were **unreachable from the builder** | All 4 canvases are named `role="application"`, `tabindex="0"`, localized, with `aria-describedby` help text; 3 carry `aria-live="polite"`; the compute-status live region stays mounted so phase changes are actually announced; contract law extended, both halves green |
| 3 | **Customization** | "NOT AUDITED … insufficient evidence either way" | The framework already owns a complete event-sourced customization lane; **proven** that locale, appearance and dock UI state of the generation3d editor survive a reload. No new system invented |

**The headline defect this lane found and fixed:** the React interpreter rendered `AccessibilitySpec`
into real ARIA for *every* component except the one that needed it most. A `<canvas>` has no
accessible children, so its record's spec is the only thing assistive technology can ever learn about
it — and it was being discarded. A CDP accessibility-tree probe on 6018 confirmed the before-state:
three canvases, **no `role`, no `aria-label`, no `tabindex`** (`🗑️generated/react-i18n-a11y/recon/`).

**Honest scope limit:** §4 below lists what is *not* claimed — most importantly that arrow-key
node-by-node navigation *inside* the graph canvas is **not** implemented, and 5 pre-existing
`wgpu::prepared` test failures are a peer lane's, not mine.

---

## 1. i18n

### 1.1 What was already right

The audit's §2 was pessimistic about the mechanism and correct about the proof. Two label systems
exist, and both were already fully bilingual:

- **`app_labels!` block** — `✏️editor/🗣️terminology/🦀️.rs`. The macro makes a missing locale a
  *compile error*: a field cannot be named without all four of `native_en`/`native_de`/`reuse_en`/`reuse_de`.
- **Action/command labels** — `LocalizedLabel::native(en, de)` in the registry. A histogram of every
  `LocalizedLabel::` constructor under the generation3d tree returned **118 uses, 100% `::native`**,
  i.e. zero single-language labels. The Import/Export labels the brief mentions live here, not in the
  `app_labels!` block, and are already bilingual:
  - `✏️editor/🦀️.rs:2203` — `"Import Document…"` / `"Dokument importieren…"`
  - `✏️editor/🦀️.rs:2204` — `"Export Document…"` / `"Dokument exportieren…"`
  - `✏️editor/🦀️.rs:2207` — `"Import Document File"` / `"Dokumentdatei importieren"`
- The **eval phase labels** are framework-owned and bilingual at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:3669`
  (`SamplingEdges => ("Sampling edges", "Kanten werden abgetastet")`).

### 1.2 The runtime route to German

Not obvious from source, so recording it: the shell exposes locale three ways — the Settings panel
select `#framework.settings.language` (`📌️ChromePanels/🟦️.tsx:431`), the `os.setLocale` palette
command (`🛠️ShellHelpers/🟦️.tsx:4167`), and `ShellHost`'s `setLocale` callback
(`🏛️ShellHost/🟦️.tsx:8164`). The probes drive the Settings select, which is what a user reaches.

### 1.3 Runtime proof (`🐍️i18n-a11y-customization-probe.mjs`, `🗑️generated/react-i18n-a11y/deliverable/`)

`2-edit-de.png` is the money shot: the whole editor in German with the mesh rendering and the layout
unchanged. Observed German, per window/mode/role:

| Surface | English → German observed |
|---|---|
| Shell chrome | Artifact→**Dokument**, Catalogue→**Katalog**, Inspection→**Inspektion**, Fullscreen→**Vollbild**, Example→**Beispiel**, Edit→**Bearbeiten**, Generate→**Generieren**, Viewer→**Betrachter**, Settings→**Einstellungen**, History→**Verlauf**, Command→**Befehl**, Display→**Anzeige**, "Remote: detached"→**"Remote: getrennt"**, "No one else is here"→**"Niemand sonst ist hier"**, "Agent disconnected"→**"Agent getrennt"** |
| Flow window | Flow→**Workflow**, NODES→**KNOTEN**, WIRES→**LEITUNGEN**, Input→**Eingang**, Output→**Ausgang**, Evaluated→**Ausgewertet**, "Fit graph"→**"Graph einpassen"** |
| Preview window | Preview→**Vorschau**, "Frame visible"→**"Sichtbares einpassen"**, Projection→**Projektion** |
| Window chrome | "Window Options"→**Fensteroptionen**, Actions→**Aktionen**, Utilities→**Hilfsmittel** |
| Generate mode | `window_generations`→**Generationen**, `window_generate_form`→**Formular** (observed in step `4-generate-de`) |
| Viewer role | **Betrachter**, **Vorschau**, plus the full German canvas description (see §2) — `5-viewer-de.png` |
| Panels | Document/Catalogue/Inspection panels in German — `widgets`→**Elemente**, `widget_group`→**Element**, `schema_prefix`→**Schema:**, `widgets_prefix`→**Elemente:**, `id_field`→**ID**, `catalog_preview`→**Vorschau** (`🗑️generated/react-i18n-a11y/panels/`) |
| Inspection w/ selection | Selecting the `Column Height` node renders **"ID: height"** and **"Bereich: 0..10"** — `range_field` in German (`🗑️generated/react-i18n-a11y/inspection/`) |

The example name itself is localized too: *Hexagonal Mushroom Column* → **Sechseckige Pilzsäule**.

**Strings not reachable in the default example's UI state** (so observed statically, not at runtime):
`catalog_slider` (Schieberegler), `catalog_note` (Notiz), `catalog_neuron` (Neuron), `no_selection`
(Keine Auswahl), `value_field` (Wert). These are gated on document/selection state the bundled
example never enters — the Catalogue lists *operator* groups (Brep/Math/Inputs/Outputs/Contract),
while those four labels describe *document widget kinds*. The same code path
(`generation3d_catalog_label`) **was** observed in German via `catalog_preview`→Vorschau, so the
mechanism is proven; the individual strings are covered by the law in §1.5.

### 1.4 New labels added

Four fields, all four locale spellings each, `✏️editor/🗣️terminology/🦀️.rs:40-43`:
`graph_canvas` ("Node graph canvas" / **"Knotengraph-Leinwand"**), `graph_canvas_hint`,
`preview_canvas` ("3D preview canvas" / **"3D-Vorschau-Leinwand"**), `preview_canvas_hint`.
Roster is now **36 fields**. Both new labels were observed in German at runtime as the canvases'
accessible names (§2.4).

### 1.5 The law — two implementations, one fixture

Test-driven, matching the repo's existing `♿️accessibility-projection.json` convention.

- **Fixture (language-neutral):** `🧫️fixtures/🗣️terminology.json` — 36 rows × 4 locale spellings,
  plus `identicalByDesign` naming the only two rows whose German legitimately equals the English
  (`schema_prefix` "Schema:", `catalog_neuron` "Neuron").
- **Rust half:** `✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs`
  - `every_user_visible_label_is_declared_in_every_locale_exactly_as_the_shared_fixture_says` —
    walks `FIELD_NAMES`, asserts set-equality with the roster and exact text for all four consts.
  - `german_is_a_translation_rather_than_a_copy_of_the_english` — every non-whitelisted field's
    German must differ from its English.
- **TypeScript twin:** `✏️editor/🗣️terminology/🧪️tests/🔬️unit/🟦️.ts` — structural laws over the same
  fixture (bilingual, non-blank, German distinct, whitelist honoured), registered to actually run via
  a new `twins` hook on the shared artifact runner.

**Enabling framework change:** the `app_labels!` macro had no way to enumerate its own fields, so the
law could not be written generically. Added `FIELD_NAMES` and `for_each_label` (allocation-free) at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6222,6226`. Every app gets this, not just
generation3d. Docstring states exactly why: the macro already catches a *missing* locale; these
catch a *present but wrong* one.

**Counts** — Rust `cargo test -p semio-s-artifact-procedural-generation3d --features
component-app-assembly --lib -- terminology`: **4 passed, 0 failed** (2 pre-existing + 2 new).
TS twin: **364 checks**.

**Negative control (the law actually bites):** setting `window_flow.nativeDe := "Flow"` in the fixture
produced `AssertionError: window_flow: the German slot still holds the English text`; fixture restored,
364 checks green again.

---

## 2. Accessibility

### 2.1 The gap, precisely

Three independent findings, all confirmed:

1. **The interpreter discarded the spec for canvases.** `accessibilityAriaProps`
   (`🗣️Interpreter/🟦️.tsx:893`) turns `AccessibilitySpec` into real ARIA, and `TextView`/`ButtonView`/
   `ContainerView` all use it. `SurfaceView` did **not** — it called `renderComponentSceneHost`, whose
   signature never took `record.accessibility`. So every scene canvas rendered with no name, no
   description, no live setting.
2. **The contract said canvases are not focusable.** `accessibility_is_focusable` omitted
   `Component::Surface`, and that same predicate drives the wgpu `EventRouter`'s Tab traversal — so
   the canvas was unreachable by keyboard on *both* renderers.
3. **`live` and `shortcut` were unreachable from the builder.** `HasBase` had `try_label` and
   `try_describe` only; no method existed to set `AccessibilitySpec.live` at all. A status surface
   could not have been made a live region even by an author who wanted to.

And generation3d itself set **zero** accessibility explicitly anywhere — the only accessible names in
the whole app came free from `tree_item()`'s auto-derived label.

**Before-state, measured** (`🐍️locale-a11y-recon-probe.mjs`, `🗑️generated/react-i18n-a11y/recon/`):
3 canvases, all `role: null, ariaLabel: null, tabIndex: null`. The CDP accessibility tree contained
two bare `Canvas` nodes and **zero** `application` nodes.

### 2.2 Fixes, at the owning layer

| Layer | File:line | Change |
|---|---|---|
| UI contract (Rust) | `🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:122` | `Component::Surface` joins the focusable set |
| UI contract (TS twin) | `🖱️ui/🧬️contract/♿️accessibility/🟦️.ts:119` | same, in the TypeScript mirror |
| UI contract builder | `🖱️ui/🧬️contract/🏗️builder/🦀️.rs:704,713` | new `live(Liveness)` and `try_shortcut(..)` on `HasBase` |
| React interpreter | `🗣️Interpreter/🟦️.tsx:1520` | new `SurfaceAccessibilityShell` — `role="application"`, `tabIndex`, `aria-label`/`describedby`/`live`/`keyshortcuts`, `aria-busy`, focus ring, Enter/Space → `activate` |
| React World3d host | `🌐️World3dHost/🟦️.tsx:3854,3866` | the compute-status `role="status"` region **stays mounted while idle** |
| generation3d shared UI | `🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs:67` | new `accessible_scene_surface(..)` helper |
| generation3d canvases | `🕸️flow/🦀️.rs:279`, `✏️edit/…/👁️preview/🦀️.rs:88`, `🧬️generate/…/👁️preview/🦀️.rs:99`, `👁️viewer/…/👁️preview/🦀️.rs:375` | all four wired to it with localized name + hint |

Two subtleties worth recording:

- **The live region must already exist.** `WorldComputeStatusPane` used to `return null` while idle.
  That is the classic ARIA trap: a reader only announces mutations *inside a region it is already
  observing*, so a `role="status"` element that first appears at the moment it has something to say is
  announced by nothing — idle → "Sampling edges" → "Evaluated" was completely silent. It now renders
  an empty `sr-only` region while idle, so every later phase is an observed-subtree mutation.
- **Descriptions use `UiText::clipped`, not `try_from_str`.** A description that fails admission would
  fail the whole `refreshUi` (the known 512-byte `UiText` overflow hazard), and the full text is always
  reachable in the window's own outline.

### 2.3 The law

Test-driven: the shared fixture `🖱️ui/🧬️contract/🧫️fixtures/♿️accessibility-projection.json` was edited
**first** to declare the new truth, both halves went red, then both implementations were fixed.

- Role row renamed `a-surface-is-an-application-you-can-tab-into`, `focusable: false → true`.
- New document node `#canvas` (id 6): a `world-3d` surface carrying label **"Preview"**, a description
  and `live: "polite"`, with its expected projection row and its entry in `announced` (`[2,5] → [2,5,6]`).

Red→green confirmed at each step:
- TS twin **116 → red (`a-surface-…: focusability, false !== true`) → 127 checks**.
- Rust: `cargo test -p semio-framework-ui-contract --lib accessibility` — **11 passed, 0 failed**,
  including the two fixture-driven laws
  (`every_component_implies_the_role_the_shared_fixture_declares`,
  `every_published_record_projects_the_way_the_shared_fixture_declares`).

### 2.4 Runtime proof on 6018

Accessibility-tree probe uses Chromium's own `Accessibility.getFullAXTree` over CDP — an oracle
*outside* our projection code, so it independently confirms what a screen reader would see.
(Playwright removed `page.accessibility`; CDP is the stronger replacement.)

| Step | Surface shells | Named | Focusable (`tabindex=0`) | Live | CDP `application` nodes |
|---|---|---|---|---|---|
| `1-edit-en` | 2 | 2 | 2 | 1 | `["3D preview canvas", "Node graph canvas"]` |
| `2-edit-de` | 2 | 2 | 2 | 1 | `["3D-Vorschau-Leinwand", "Knotengraph-Leinwand"]` |
| `4-generate-de` | 1 | 1 | 1 | 1 | `["3D-Vorschau-Leinwand"]` |
| `5-viewer-de` | 1 | 1 | 1 | 1 | `["3D-Vorschau-Leinwand"]` |
| `6-after-reload` | 2 | 2 | 2 | 1 | `["3D-Vorschau-Leinwand", "Knotengraph-Leinwand"]` |

All four canvases (`procedural.play`, `procedural.play.preview`,
`procedural.play.generate-preview`, `procedural.view.preview`) are named, localized and focusable.

**Keyboard focus, measured:**
`{"found":true,"focused":true,"activeRole":"application","activeLabel":"Knotengraph-Leinwand","tabIndex":"0"}`
— focusing the canvas makes it `document.activeElement` with its German accessible name.

**The description reaches the DOM as real text** — the viewer's body text contains, verbatim:
*"Interaktive 3D-Vorschau des ausgewerteten Workflows. Ziehen zum Umkreisen, Klicken zum Auswählen;
der Auswertungsstatus wird bei Änderung angesagt."*

**Layout is unchanged.** `🐍️canvas-dom-ancestry-probe.mjs` before vs after: the node-graph host is
`538x914` and the World3d host `503x914` in both runs; the new shell sits exactly where the scene
host's own root sat and carries the same `relative h-full min-h-0 w-full` sizing. Zero page errors in
the whole probe run.

---

## 3. Customization

### 3.1 The framework already owns this — generation3d just had to be shown using it

The audit called this unaudited. It is not missing; it is a complete, event-sourced, local-first lane,
exactly as CLAUDE.md prescribes. **No new system was invented.**

- **Nine UI-preference mutations** in `💻️os/🎚️config/🧬️schema/🧬️mutations/🎨️ui-preferences/🟦️.ts`:
  `setAppearance`, `setLayout`, `setDriver`, `setCustomDriver`, `setLocale`, `setTerminology`,
  `setTheme`, `setCustomTheme`, `setKeybindingOverride`.
- **Event-sourced, not CRUD.** `commitUiPreference` (`🏛️ShellHost/🟦️.tsx:7924`) delegates to
  `commitUiPreferencesConfigMutation` (`🧑‍🎨engine/🎚️UiPreferences/🟦️.ts:101`), which **appends** the
  mutation to an event array and **replays** it to a projection.
- **One durable document:** `localStorage["semio.os.config"]` (`🖥️platform/🟦️.ts:265`), holding
  `preferences`, `namedLayouts`, `dockLayouts`, `dockUi`, `windowPanes`.
- **Rehydrated pre-paint** for locale (`🏛️ShellHost/🟦️.tsx:1824`) and at `initialShellState`
  (`🐚️Shell/🟦️.tsx:1138-1181`); live cross-tab sync via a `storage` listener.
- **User keybinding overrides genuinely exist** — a Settings "Hotkeys" tab
  (`framework.settings.keybindings`, `📌️ChromePanels/🟦️.tsx:842-894`) with per-control chord capture,
  conflict detection and reset, persisted through the same lane; user overrides outrank app
  declarations (`🕹️control-keybinding-context/🟦️.tsx:193-202`).

### 3.2 Runtime proof that it applies to and persists for generation3d

`🐍️customization-persistence-probe.mjs`, `🐍️appearance-scope-probe.mjs`, and step `6-after-reload`
of the deliverable probe:

| Customization | Applies to generation3d? | Survives reload? | Evidence |
|---|---|---|---|
| **Locale** (de) | Yes — every window (§1.3) | **Yes** | `htmlLang: de` after reload, German text intact |
| **Appearance** (dark) | Yes | **Yes** | `.semio-scope[data-ui-appearance]` = `dark` before *and* after reload; `3-dark-reloaded.png` shows the whole editor dark |
| **Dock UI state** | Yes | **Yes** | `dockUi.apps["s.procedural.generation3d@1/*#editor"]` records the panel's `visible`/`path` at `bottom-right` |
| Event log | — | **Yes** | `preferences["os.config.ui-preferences"]` = `{"version":1,"events":[{"mutation":"setLocale","locale":"de"},{"mutation":"setAppearance","appearance":"dark"}]}` — an append-only stream, not a state blob |

**One real defect found, characterized but NOT fixed** (out of this lane's blast radius, flagged for
the next wave): appearance is applied to the `.semio-scope` root, but the **pre-paint boot scripts read
a dead key**. `PLAYGROUND_PLAY_BOOT_APPEARANCE_SCRIPT` and `PLAYGROUND_PLAY_BOOT_THEME_SCRIPT`
(`🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts:406,412`) read `localStorage["ui.chrome.appearance"]` and
`["ui.chrome.theme.snapshot"]` — keys the OS shell **no longer writes** (it writes `semio.os.config`).
Measured consequence: with appearance = dark, `<html>` has no `.dark` class and `<body>` stays
`rgb(247,243,227)` while the app inside is `rgb(0,17,23)`. Invisible once mounted (the scope root fills
the viewport) but it is a guaranteed light flash on every reload of a dark-themed session. The fix is a
one-line source change in those two boot scripts to read the `semio.os.config` document; it belongs to
whoever owns the styling boot path, and changing it blind would have put a FOUC regression into a
shared serve mid-session.

---

## 4. What is NOT claimed

1. **Arrow-key navigation of graph nodes inside the canvas is not implemented.** The canvas is now
   focusable and announces itself as `role="application"` (which promises it handles its own keys), and
   Enter/Space dispatches an `activate` binding where one is declared. But node-by-node arrow traversal
   *within* the node graph needs guest-side keyboard verbs that do not exist. The canvas's German
   description deliberately points the reader at the sibling outline tree, which **does** expose every
   node, port and wire as real focusable `treeitem`s — that is today's honest keyboard path, not a
   claim of in-canvas navigation.
2. **The live region is wired but its announcement was not heard.** `aria-live="polite"` and the
   always-mounted `role="status"` region are verified present in the DOM; no screen reader was run, so
   "a status change is announced" is a structural argument (a mutation inside an already-observed live
   region), not an observation.
3. **Five `wgpu::prepared` tests fail and are not mine.** `cargo test -p semio-framework-ui --features
   wgpu --lib` → 136 passed, 5 failed, all in `wgpu::prepared::tests` (eviction byte cap, input-drop
   permits, item budget, receiver/worker ownership, retained codec). Those files contain **zero**
   references to `focusable`, `accessibility`, `Liveness` or `for_each_label`; the working tree carries
   heavy concurrent peer modification. Reported as observed, attributed to a peer lane.
4. **Repo-wide `tsc` shows 3556 errors, all pre-existing.** Every one is in generated
   `storybook-static/**/*.d.ts`. Zero errors in any file this lane touched.
5. **Five German strings were proven by law, not by screenshot** — `catalog_slider`, `catalog_note`,
   `catalog_neuron`, `no_selection`, `value_field`; see §1.3 for why the default example cannot reach
   that UI state.
6. **wgpu (6118) was not re-verified.** Making `Component::Surface` focusable changes the wgpu
   `EventRouter`'s Tab traversal too. That is the intended cross-renderer effect and its contract law is
   green, but no 6118 run was made — the wgpu lanes own that port.
7. **Assembly artifact untouched**, as in the audit — out of the generation3d flow by construction.

---

## 5. Files

**Framework (owning-layer fixes)**
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs` — `Surface` focusable
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🟦️.ts` — TS twin
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` — `live` / `try_shortcut`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/♿️accessibility-projection.json` — the law
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — `app_labels!` `FIELD_NAMES` / `for_each_label`
- `🧰️framework/🛍️products/💻️os/…/🧱️elements/🗣️Interpreter/🟦️.tsx` — `SurfaceAccessibilityShell`
- `🧰️framework/🛍️products/💻️os/…/🧱️elements/🌐️World3dHost/🟦️.tsx` — live region stays mounted
- `🧰️framework/🛍️products/🦑️repo/…/📦️artifacts/🦀️rust/📜️script.ts` — `twins` hook

**generation3d**
- `✏️s/🔌️plugins/🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs` — `accessible_scene_surface`
- `…/✳️any/✏️editor/🗣️terminology/🦀️.rs` — 4 new labels (36 total)
- `…/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🦀️.rs` — 2 new laws
- `…/✳️any/✏️editor/🗣️terminology/🧪️tests/🔬️unit/🟦️.ts` — **new**, TS twin
- `…/✳️any/🧫️fixtures/🗣️terminology.json` — **new**, language-neutral roster
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs` — node-graph canvas
- `…/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` — canvas + `labels` threaded in
- `…/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs` — canvas
- `…/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs` — canvas
- `…/✳️any/✏️editor/🦀️.rs`, `…/✳️any/👁️viewer/🦀️.rs` — call sites
- `…/🧊️generation3d/📦️packages/🦀️rust/📜️script.ts` — registers the twin

**Probes (ticket root, kept)** — `🐍️locale-a11y-recon-probe.mjs`, `🐍️settings-route-probe.mjs`,
`🐍️customization-persistence-probe.mjs`, `🐍️appearance-scope-probe.mjs`,
`🐍️canvas-dom-ancestry-probe.mjs`, `🐍️i18n-a11y-customization-probe.mjs`, `🐍️panel-i18n-probe.mjs`,
`🐍️inspection-i18n-probe.mjs`

**Probe output (`🗑️generated/react-i18n-a11y/`, deletable)** — `recon/`, `settings-route/`,
`persistence/`, `appearance-scope/`, `canvas-dom/`, `pre-restage/`, `deliverable/`, `panels/`,
`inspection/`

**Commands run** — `cargo check -p semio-s-artifact-procedural-generation3d --target wasm32-wasip2`
(exit 0, warnings present); `cargo check -p semio-framework-ui-contract -p semio-framework-plugin`
(exit 0); `cargo test -p semio-framework-ui-contract --lib accessibility` (11/11);
`cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --
terminology` (4/4); `bun nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev` (exit 0).
