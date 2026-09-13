# wgpu World3d Status, Accessibility and Chrome i18n (2026-09-13)

Lane `wgpu-status-a11y-i18n`. Closes gaps **#4 (World3d compute status parity)**, **#3 (accessibility
metadata parsed but dropped)** and **#8 (chrome i18n)** of `📓️audit-wgpu-parity-2026-09-13.md`.

---

## TL;DR

| gap | source | laws | runtime on 6118 |
|---|---|---|---|
| **#4** World3d compute status | ✅ the full `World3dComputeStatusV1` contract is read on wgpu and painted as a status pill (phase label + `unitsDone/unitsTotal (pct%)` + a ratio bar), with the cancel control moved onto the same row after it | ✅ Rust 3 laws + TypeScript 3 laws over **11 new shared fixture rows** | ❌ **not claimed** — the bundled example settles ~20 s before the World3d surface first attaches, so the shell never observes a non-idle status in a boot session (§5.2) |
| **#3** accessibility | ✅ a real accessibility projection: per-node semantics in `ui_contract`, the tree walk in the wgpu target, a `dumpAccessibility` export, an `accessibility` transport probe, and an **ARIA mirror DOM** the host maintains beside the canvas | ✅ Rust 3 contract laws + Rust 4 wgpu-target laws + TypeScript **116 assertions**, one shared fixture | ✅ **proven, repeatably** — 34 ARIA nodes for `procedural-main`, 11 for `procedural-preview`, with real `role`/`aria-label` (§5.1) |
| **#8** chrome i18n | ✅ every user-visible chrome literal now carries EN **and** DE — the Command Palette's 19 strings plus 8 more the audit had not found | ✅ 3 grep laws (26 keys, 0 escapes, 6 declared proper nouns) | ❌ **not claimed** — the palette is the ONLY runtime route to `os.setLocale` on wgpu and it did not dispatch in the probe (§5.3) |

Two peer compile breaks were fixed forward (§7). One new lane-owned probe:
`🐍️wgpu-status-a11y-i18n-probe.mjs`.

---

## 1. Gap #4 — World3d compute status parity

React renders the whole `WorldComputeStatusPane` (`🌐️World3dHost/🟦️.tsx:3755-3795`): a spinner, the
producer's own phase label, `unitsDone/unitsTotal (pct%)`, and a cancel button. wgpu read only
`cancellable`/`cancelAction` and threw the rest of the producer's report away.

### What changed

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, region `🔀️ChromeParity`:

- **`World3dComputeStatus` + `world3d_compute_status()`** (`:9391-9422`, `:9548-9580`) — the Rust twin
  of `world3dComputeStatusV1` (`🖱️ui/🎬️scene/🟦️.ts:382`), field for field: `computing`, `phase`,
  `phaseLabel`, `unitsDone`/`unitsTotal`/`facesDone`/`facesTotal`/`inFlight`, `ratio`, `cancellable`,
  `cancelAction`. Total by construction — malformed JSON, a missing field or a hostile type degrades
  to the neutral status, never a throw inside a render.
- **`world3d_cancel_affordance` is now a projection of it** (`:9535-9538`), not a second parser. The
  chrome-parity lane's own laws still answer through it unchanged; there is exactly one reader of
  `statusJson` on this target now.
- **`World3dStatusPill` + `world3d_status_pill_for` + `world3d_status_is_visible`** (`:9424-9461`) —
  the visibility gate is byte-for-byte React's (`computing || cancellable || phase == "cancelled"`),
  so a settled producer annotates nothing and an idle preview is never covered by chrome.
- **`surface_status_pills_for`** (`:9476-9490`) lays the pill out at the surface's top-left, and
  **`surface_overlay_controls_for`** (`:9497-9516`) now anchors the cancel control *after* the pill on
  the same row — React's own single-row layout, and the reason the two never paint on top of
  each other.
- **Paint**: a new overlay phase (`:12856-12904`) draws the pill — glass row, a muted track with an
  accent fill at the published ratio, and one glyph run `"<phase> · 3/8 (38%)"`. It is resumable (one
  unit per grant, like every other chrome step) and **registers no hit**, so it can never steal a
  press from the surface beneath it.
- **`common.loading`** (EN/DE) is the fallback phase label — used only when the producer published no
  `{en, de}` pair, exactly as React falls back to `ui.common.loading`.
- A `[DEBUG] wgpu world3d status pill surface=… phase=… label=… ratio=… rect=…` trace fires once per
  CHANGE (`:12395-12418`, backed by the new `ShellState.world3d_status_pill_trace` ledger), not once
  per frame.

### Laws

Shared fixture: `🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json`, new `statusPane` section — **11 rows**
covering a meshing producer with progress, an indeterminate producer, a producer-declared ratio that
outranks the counts, an out-of-range ratio, a count past its own total, a half-translated phase label,
a cancelled producer, a settled producer, no status, malformed JSON and hostile progress types.

- Rust — `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs`:
  `the_compute_status_is_read_field_for_field_the_way_the_shared_fixture_declares`,
  `the_status_pill_speaks_the_readers_language_and_never_defaults_to_one`,
  `the_status_pill_leads_the_row_the_cancel_control_follows`.
- TypeScript twin — `🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`, region `⏳️ComputeStatusPaneTwin`:
  three `it`s answering the same 11 rows through the shipped `world3dComputeStatusV1`.

```
cargo test -p semio-framework-os-renderer-wgpu --lib -- shell_chrome_parity_tests
  → 21 passed; 0 failed
  [DEBUG] wgpu world3d status: 8 of 11 shared fixture rows are unsettled and paint a pill
  [DEBUG] wgpu world3d status pill: leads the overlay row at [483.2, 43.2], cancel anchored at [667.52, 43.2]

SEMIO_TEST_LEVEL=long … --testNamePattern="world3d compute status pane|world3d cancel contract"
  → 6 passed | 598 skipped
  [DEBUG] world3d status pane reproduced all 11 shared fixture rows
```

---

## 2. Gap #3 — accessibility

`AccessibilitySpec { label, description, live, shortcut, hidden }` is a field on every `UiNodeRecord`
and React turns it into real ARIA; wgpu's `reconcile.rs`/`paint.rs` had **zero** references to it. A
GPU canvas has no elements to hang `aria-*` on, so the fix is to publish the tree as data and give it
elements on the host side.

### The four layers

1. **Contract (shared, schema-first)** — `🖱️ui/🧬️contract/♿️accessibility/🦀️.rs`, new region
   `🔖️AccessibilityProjection`:
   - `accessibility_role(component, activatable)` — the role a `Component` IMPLIES. The spec
     deliberately carries no `role` field ("a `Component::Button` is a button on every renderer"), so
     this is the one place that implication is written down. React reaches the same roles implicitly
     through the HTML element each view renders; a canvas has to say them out loud. A container you
     can press is a `button`, except where the author already declared a landmark (`form`/`toolbar`).
   - `accessibility_is_focusable`, `liveness_name`, `AccessibilityProjectionNode`,
     `accessibility_projection_node(record, depth)`.
   - A hidden node is **kept**, carrying `hidden: true`, never dropped — dropping it would disagree
     with the DOM renderer, which renders the element and marks it `aria-hidden`.
2. **wgpu target walk** — new module `🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs`:
   `accessibility_projection(tree)` walks the retained DOCUMENT in pre-order (the reading order),
   stamping from the ARENA the two things the document cannot carry: live `FOCUSED` and the absolute
   laid-out rect (from the double-buffered `mounted_layout` the paint pass consumes, never the
   immediate-mode bucket). `accessibility_announced()` is the reach subset.
3. **Transport** — `dumpAccessibility` (`🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:2143-2154`) →
   `BrowserFrameIntrospectionProbe` gains `"accessibility"` → the frame worker routes it
   (`🎞️frame-worker/🟦️.ts:45,319`). It is the one probe that is **not** a test hook: it is this
   target's production accessibility path.
4. **ARIA mirror** — `🚀️browser-boot/🟦️.ts`, new region `♿️AccessibilityMirror`: a
   `#semio-wgpu-accessibility` subtree beside the canvas, one element per projected node carrying
   `role`, `aria-label`, `aria-describedby` (+ its description span), `aria-live`, `aria-keyshortcuts`,
   `aria-hidden`, `aria-disabled` and `data-*` mirrors of the raw fields. Visually hidden by the
   standard clip rule — never `display:none`, which would remove it from the accessibility tree too —
   and every element is `tabindex="-1"`, because focus belongs to the canvas, which owns the
   renderer's own focus ring and Tab traversal. Refresh is event-driven (on a UI turn) with a 400 ms
   coalescing floor, so nothing ticks while nothing happens.

### Laws

Shared fixture: `🖱️ui/🧬️contract/🧫️fixtures/♿️accessibility-projection.json` — a `roles` table with
**24 rows covering all 18 contract components** (including every `ContainerRole` and the pressable
container), and a `document` section: a real flat snapshot (section → field → labelled+shortcut'd
numeric stepper, a decorative `aria-hidden` ornament, a polite live region, a disabled pressable card)
with its full expected pre-order projection and its `announced` subset.

```
cargo test -p semio-framework-ui-contract --lib -- accessibility
  → 11 passed; 0 failed
  [DEBUG] ui contract accessibility: 24 role rows over all 18 components
  [DEBUG] ui contract accessibility: 6 projected nodes, 2 reachable by name

cargo test -p semio-framework-ui --features wgpu-engine --lib -- accessibility
  → 4 passed; 0 failed
  [DEBUG] wgpu accessibility projection: 6 nodes published in pre-order from a mounted document
  [DEBUG] wgpu accessibility projection: 2 of 6 nodes reachable by name
  [DEBUG] wgpu accessibility projection: focus stamped from the arena onto node [2]

bun … accessibilityProjectionSelfTests()
  [DEBUG] accessibility-projection-twin checks= 116
```

The law the audit asked for is
`every_labelled_reachable_node_appears_in_the_projection` (`🖱️ui/🧪️tests/🔬️targets-wgpu-accessibility-projection/🦀️.rs`):
every focusable-or-actionable node carrying an `AccessibilitySpec` label appears in the projection,
verbatim, and nothing is invented. It runs against a document that is actually reconciled into the
live arena, not against a hand-built list.

---

## 3. Gap #8 — chrome i18n

The audit named the Command Palette's ~19 strings. The sweep found **8 more** the audit had not, and
the law that found them is the durable half of this fix.

### What changed (all in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`)

- **Command Palette** (`:9866-9906`): every `LocalizedLabel::data("…")` became
  `LocalizedLabel::native(en, de)` — `Set Appearance`/`Erscheinungsbild festlegen`,
  `Appearance`/`Erscheinungsbild`, `Light`/`Hell`, `Dark`/`Dunkel`, `Set Driver`/`Treiber festlegen`,
  `Driver`/`Treiber`, `Default`/`Standard`, `Compact`/`Kompakt`, `Set Locale`/`Sprache festlegen`,
  `Locale`/`Sprache`, `Set Terminology`/`Terminologie festlegen`, `Terminology`/`Terminologie`,
  `Set Theme`/`Design festlegen`, `Theme`/`Design`, `Reset Dock Layout`/`Dock-Layout zurücksetzen`,
  `Native`/`Nativ`, plus `Recording`/`Aufnahme` (`:11165`).
  `native()` was chosen over the `shell_chrome_string` key table wherever the field is already typed
  `LocalizedLabel`: its `match locale` is exhaustive with no catch-all, so **adding a locale fails the
  build until translated** — a stronger guarantee than a keyed table with a fallback.
- **Eight more sites the audit missed**, routed through `shell_chrome_string` with new EN/DE keys:
  `contextMenu.goHome` (`Go Home`/`Zur Startseite`, `:7674`), `overlay.search.title`
  (`Search`/`Suchen`) and `overlay.find.title` (`Find in page`/`Auf Seite suchen`) — both were raw
  literals in the overlay title match (`:12894`) — `common.close`, `common.focus`,
  `command.applyLayout` (`:10213-10221`), `command.checkIn` (`:8801`), `command.moveWindow` (`:6783`)
  and `command.resizeWindow` (`:6487`).
- `common.loading` was added for the status pill (§1).

The table is now **26 distinct keys asked for, all answered in EN and DE**.

### Laws (`🔬️wgpu-shell-chrome-parity/🦀️.rs`)

1. `no_user_visible_chrome_literal_bypasses_the_bilingual_paths` — the grep law the audit asked for:
   no `Label::data("…")`/`LocalizedLabel::data("…")` literal may remain, except 6 declared
   locale-invariant call sites, each named with its reason (`Semio`, `Mono` — product/theme proper
   nouns; `English`, `Deutsch` — language ENDONYMS, since a locale picker names each tongue in its own
   tongue and never translates them).
2. `no_capitalised_chrome_phrase_escapes_the_bilingual_tables` — the stronger one. ANY capitalised
   multi-word literal in the 14 k-line shell is user-visible copy unless it is a
   `LocalizedLabel::native` argument, an arm of one of the two EN/DE tables, or an internal `Shell …`
   fault diagnostic. **This is what found `Find in page`, `Go Home`, `Apply Layout`, `Check In`,
   `Move Window` and `Resize Window` after the Command Palette sweep had already declared itself
   done.**
3. `every_chrome_key_the_shell_asks_for_answers_in_both_tongues` — no key falls through to the raw-key
   fallback, which is what a missing translation looks like.

```
[DEBUG] wgpu shell chrome i18n: 0 English-only literals, 6 declared locale-invariant proper nouns
[DEBUG] wgpu shell chrome i18n: 0 English-only chrome phrases outside the EN/DE tables, 176 declared exemptions
[DEBUG] wgpu shell chrome i18n: 26 distinct keys asked for, all answered in EN and DE
```

---

## 4. Files

**Changed**
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — status contract + pill + paint + i18n.
- `…/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` — `statusPane` (11 rows).
- `…/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` — 6 new laws.
- `…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — `⏳️ComputeStatusPaneTwin`.
- `…/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — `dumpAccessibility` + `DumpAccessibility`.
- `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts` — `"accessibility"` probe.
- `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts` — probe routing.
- `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts` — `♿️AccessibilityMirror` + beacon export.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs` — projection region.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️accessibility-unit/🦀️.rs` — 3 new laws.
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/📜️script.ts` — runs the TS twin.
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🦀️.rs` — mounts the new module.

**Added**
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/♿️accessibility-projection.json`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🔬️accessibility-projection/🟦️.ts`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-accessibility-projection/🦀️.rs`
- `…/PROCEDURAL-3D-END-TO-END/🐍️wgpu-status-a11y-i18n-probe.mjs`

**Fixed forward for a peer** — see §7.

---

## 5. Runtime on `http://127.0.0.1:6118/?plugin=generation3d`

Probe: `🐍️wgpu-status-a11y-i18n-probe.mjs`; evidence under
`🗑️generated/wgpu-status-a11y/run-{1..4}/`. Renderer wasm rebuilt and served
(`dist/wasm-dev` 14:25). **Screenshots are not evidence here**: the canvas is an `OffscreenCanvas`
owned by the frame Worker, and headless Chromium captures it blank — every claim below rests on the
DOM, on the renderer's own introspection exports, or on its `[DEBUG]` console traces.

### 5.1 Accessibility — PROVEN (runs 1, 3, 4)

```
accessibility:mirror       pass  {"mirrorRole":"region","mirrorLabel":"Semio controls","nodeCount":34,
                                  "labelled":28,"roles":["group","tree","treeitem","application"],
                                  "firstLabels":["treeitem:Column Height","treeitem:Profile Radius",
                                                 "treeitem:Side Count","treeitem:Polygon", …]}
accessibility:per-window   pass  {"hooks":["dumpStructure","dumpFrameStats","dumpAccessibility"],
                                  "perWindow":{"procedural-main":{"count":34},
                                               "procedural-preview":{"count":11}}}
```

Both generation3d windows publish a tree; the host mirrors it into real ARIA elements with real
`role`s and `aria-label`s taken from the authored document. Before this lane the whole wgpu renderer
had exactly one `aria-label` (a static one on the canvas) and no `role` beyond `status`/`alert`.
Full dumps: `run-3/accessibility.json`, `run-4/accessibility.json`.

### 5.2 Status pill — NOT CLAIMED, with the reason measured

The pill never painted in any probe window, and the trace shows why rather than leaving it open:

```
status:pill-while-computing  fail  {"computing":0,"total":0,"producerEverComputing":0}
```

`producerEverComputing: 0` — across 32 status samples the producer's own `statusJson` was
`{"phase":"idle","progress":{"unitsDone":44,"unitsTotal":44,…}}` every single time. The cause is a
timing one, visible in the console: the boot evaluation runs at **~7 s**
(`[DEBUG] wgpu-shell command flowEvalTick settled`, `run-2/console.txt:64`) while the World3d surface
first attaches at **~27 s** (`[DEBUG] world3d surface=procedural-preview …`, `:404`). The shell's
`world3d_status` map is filled by the per-frame attach walk, so in a boot session it never sees a
non-idle status — and the pill correctly paints nothing. **The pill is therefore unproven at runtime,
not disproven**; it needs an evaluation that starts *after* the surface is live.

The one user route that does that is the example picker, whose trigger the probe did locate
(`playground.navbar.fixture`, run 4 — alongside `playground.navbar.modes.{edit,generate}`,
`playground.navbar.roles.{editor,viewer}` and the dock tabs). Clicking the trigger opens the dropdown;
driving a *row* click and holding the session long enough to watch the pill appear and then vanish is
the remaining step, and it is a probe-driving step, not a source step.

### 5.3 German chrome — NOT CLAIMED, with the blocker named

```
locale:german-via-palette      fail   (no "setLocale" dispatch anywhere in the console)
locale:german-via-boot-locale  fail   (labels identical — see below)
```

Two findings, both worth recording:

1. **The quick-search palette is the only runtime route to `os.setLocale` on wgpu.** The
   `build_command_panel_row` Select that would offer it is `#[cfg(test)]` (`:10184`), so it does not
   exist in a running shell. The probe pressed `ctrl+p` (the palette chord, `:8042`), typed `Deutsch`
   and pressed Enter; the console shows the key events reaching
   `os_host dispatch_normalized_event` and **no `setLocale` dispatch following**. Whether the palette
   opened at all is not observable from outside the canvas. This sits in the input/dispatch lane's
   territory, so it is reported rather than chased.
2. **A `de-DE` browser locale is not a valid discriminator here.** The labels the DOM can see
   (`Column Height`, `Profile Radius`, `Polygon`, …) are the example graph's own NODE NAMES —
   user data, identical in every locale. generation3d's translated strings
   (`Knoten`/`Leitungen`/`Eingang`/`Vorschau`, `🗣️terminology/🦀️.rs:15-40`) live in window chrome that
   is painted on the canvas, not in the retained document, so they are invisible to a DOM probe. A
   German proof needs either a canvas OCR/pixel oracle or a shell-side `[DEBUG]` trace of the resolved
   chrome strings.

---

## 6. What a follow-up should do

1. Drive the example-picker **row** (not just the trigger) in the probe and watch
   `[DEBUG] wgpu world3d status pill …` appear with `computing=true` and a ratio, then disappear —
   that closes §5.2 in one probe run, no source change.
2. Give the shell a `[DEBUG]` trace of the resolved locale + a couple of chrome strings on every
   locale change, so §5.3 becomes provable without pixels.
3. Find out why `ctrl+p` produces no palette dispatch (input/dispatch lane).
4. The status pill is the first piece of shell chrome that paints *without* registering a hit. If a
   later lane wants the pane clickable (React's is not), that is where to start.

---

## 7. Peer compile breaks fixed forward

Both blocked the renderer wasm build this lane needs (the renderer links the plugin registry, so a
plugin that does not compile stops the renderer's own build), and both were one-line omissions in an
in-flight refactor:

- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:18` — added
  `cancel_fill_build` to the `use crate::editor::puzzle5d::commands::{…}` list. The module was already
  declared (`🖐️5d/🦀️.rs:2006`) and already called (`:4268`); only the import was missing.
- An earlier break in `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` (a private `debug_log` and two borrow
  conflicts around `host.dag`) and one in `🧧️puzzle/🧊️3d` were fixed by their own lanes while this one
  waited; nothing was changed here for either.

**Environment note for the coordinator:** the machine hit `no space left on device` mid-lane with the
shared cargo build-dir at **359 GB** (`build/debug` alone 188 GB). `bun nx run repo:cache-prune`
reclaimed only 14 MiB — it correctly refuses to evict anything touched in the last 48 h, which under a
four-lane fleet is everything. Disk recovered on its own to ~92 GB free when a peer's build finished,
but the guard means the prune tool cannot help a fleet that is continuously warm.
