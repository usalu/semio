# 🎓️ W7a — the introduction tour, control-id parity, and the chord glyphs

Packet W7a of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. It answers
`📓️audit-visual-parity-puzzle3d.md` §7 items 6 and 7, `📓️w5b-interaction-parity-probe.md` §4 items
1–4, and `📓️w4a-boot-appearance-and-tour.md` hand-off 3. Line numbers are post-edit. Every gate named
under **VERIFY** was RUN, in the foreground, and its log is under `🗑️generated/w7a-*.txt`.

Reference captures: React `🗑️generated/react-6313/final.png` (`Welcome to Puzzle 3D`, body copy,
`Skip`, `1 / 5`, `Next ↵` over a blurred veil) vs wgpu `🗑️generated/w5c-probe-8/final.png` (no tour at
all; `Editor Ctrl+Alt+E` / `Viewer Ctrl+Alt+V` on the navbar where React reads `⌘⌥E` / `⌘⌥V`).

---

## 1 — Root cause of "the tour registers hits but paints nothing": **the wgpu canvas is frozen**

This is the packet's biggest finding and it is NOT an overlay-lane defect. It was measured, live, on
the running 6213 serve, and it is reproducible from artefacts that were already in this ticket.

### What was measured

New probe `🐍️w7a-tour-paint-probe.mjs` (samples `dumpChrome` + a screenshot at t = 4, 8, 12, 18, 25,
35, 50 s) → `🗑️generated/w7a-tour-paint/`:

| t | `armed` | generation | hits | tour rows |
| --- | --- | --- | --- | --- |
| 4 s | — (introspection not yet published) | — | — | — |
| 8 s | true | 1 | 43 | none |
| 12 s | true | 1 | 43 | none |
| 18 s | true | 28 | 45 | `shell.tour.skip` `[806.8,367.6,70,22.4]`, `shell.tour.next` `[786.8,508.4,90,22.4]` |
| 25 / 35 / 50 s | true | 63 / 117 / **191** | 45 | the same two rows |

So the tour arms at ~t = 13–18 s and stays armed for the rest of the run. **The screenshots from
t = 12 s onward are byte-identical** (`ImageChops.difference(...).getbbox()` is `None` for every
consecutive pair from `t012` to `t050`), and the 400 × 200 px region where the card's own hit rects
say it is contains exactly two colours: the page background and one scene diagonal.

Two further probes settle that this is the PRESENTATION lane, not the tour:

1. `🐍️w7a-canvas-liveness-probe.mjs` → `🗑️generated/w7a-live/`: boot, idle screenshot, hover the
   `Inspection` navbar chip, screenshot, hover a scene point, screenshot. **All three frames are
   byte-identical** — hovering chrome changes nothing on the canvas.
2. This ticket's OWN pre-existing artefacts say the same thing:
   `🗑️generated/audit-visual/wgpu-chrome/chrome-dump.json` records the hit registry growing
   43 → 51 → 59 rows as the audit clicked Artifact and then Inspection (the two panels really did
   open in the model), while `before.png`, `after-artifact.png` and `after-inspection.png` are
   **pixel-identical**.

**Verdict.** The shell's CPU-side chrome walk keeps completing (the hit registry is republished only
by a COMPLETE walk, and its generation reached 191), but no new frame reaches the swapchain after the
first few. Every "wgpu does not show X" observation taken against this build — the missing tour
included — is a photograph of a frame from t ≈ 12 s. This belongs to the presentation/GPU lane
(**W5c**, who owns the world pass and the wasm activation) and it must be settled before any visual
parity claim about this build is worth making. I did not touch that lane.

### What was NOT the cause (each ruled out by a measurement, not by reading)

- **The tour renderer.** A native law now drives `render_chrome_tour_step` to completion on a real
  `ShellState` and reads the overlay `DrawList`: veil quad, one glass card, its glyphs and its chips
  all arrive (`an_armed_tour_paints_a_veil_a_card_and_its_controls`).
- **The chrome walk as a whole.** Driving the FULL `render_chrome_step` ladder (213 steps to
  `Complete`) natively also puts the card's quads in the overlay list — including a chip background
  at exactly `[806.8, 367.6, 70, 22.4]`, the same rect the live `dumpChrome` publishes. Geometry,
  arming and paint all agree; only the pixels are missing.
- **`session.app.introduction` being absent**, the `ForegroundCommands`/glass split, a leaked
  scissor/clip, the retained-output grant, and the prepared-command page budget: the first is
  disproved by the live rows themselves (W6a §6 already argued it statically), and the rest are
  all-or-nothing paths that would have faulted the whole frame or dropped the neighbouring chrome
  too — the navbar, pane chips and footer pills of that same overlay list ARE on screen.

---

## 2 — The overlay lane itself: what was actually wrong, and the fix

Frozen canvas or not, the tour that WOULD have painted was not React's. Four real divergences, all
fixed.

### 2.1 The card was a fixed 320 × 168 box, always centred

`render_chrome_tour_step` computed `Rect::new((width - 320.0) * 0.5, (height - 168.0) * 0.5, 320.0,
168.0)` and painted into it. React's card is `w-fit max-w-sm`, anchored to `step.introduce` through
`resolveIntroductionPlacement`, and grows to hold its own copy. W4a had already PORTED
`resolve_introduction_placement`, `introduction_veil_bands` and `punch_introduction_cutout` — all
three carried `#[cfg(test)]`, i.e. were production-dead (W4a hand-off 3).

| site | change |
| --- | --- |
| `🐚️Shell/…:15535` | `split_introduction_body_paragraphs` — React's `splitIntroductionBodyParagraphs` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:4900`): a blank line starts a paragraph |
| `🐚️Shell/…:15548` | `panel_tab_introduction_element_id` — React's `panelTabElementId` (`🔨️modules/🖥️platform/🟦️.ts:57`) |
| `🐚️Shell/…:15558` | `chrome_tour_anchor_rect` — the wgpu twin of `useIntroductionAnchorRect`: the per-frame element-rect registry first, then the dock's own `window_content_rects` for a `framework.window.<kind>` id resolved through `element_id_segment` |
| `🐚️Shell/…:15605` | `chrome_tour_layout` + `ChromeTourLayout` — one measured layout per step: cap row (title + Skip), body paragraphs wrapped to the content width, the interaction checklist, and a footer of Back / `i / n` / Next-or-Done, clamped to React's own `max-w-sm` (384 px) |
| `🐚️Shell/…:11881` | `chrome_text_wrapped_complete_step` — the WRAPPING twin of `chrome_text_complete_step`. Shell chrome text is `Clip` (chip labels are `truncate`); the card's body is the one place React wraps |
| `🐚️Shell/…:19052` | `render_chrome_tour_step` rewritten around the layout: veil BANDS around the cutouts, the glass card, a `push_introducing_border` pulse ring on the introduced element, chip fills, the title/Skip/Next/Back/counter text, the body paragraphs, the checklist rows with their check/circle bullets, then the hits |
| `🐚️Shell/…` (`introduction_veil_bands`, `punch_introduction_cutout`, `resolve_introduction_placement`, `INTRODUCTION_INFO_BOX_GAP`) | `#[cfg(test)]` removed — production code at last |

**The spotlight.** `step.introduce` (and every `step.show`) is resolved to a rect and punched out of
the veil, and the first cutout additionally gets `DrawList::push_introducing_border` — the instanced
breathing ring, which is the shader twin of React's `data-introduced` keyframes. React elevates the
element above the scrim instead; `introduction_veil_bands`'s own docstring records why geometric
subtraction is this renderer's only way to realise the same picture, and that decision stands.

To make the spotlight resolvable at all, three new registrations were added, because only utility-bar
node ids were ever registered before: `framework.panelTab.<tabId>` at the open panel's tab row
(`:17597`), the navbar's leading tab row (`:17905`) and the footer's tab row (`:18421`).

**The veil is still NOT blurred.** React paints `backdrop-filter`. W4a's architecture note stands
unchanged (the blur chain mips only the MAIN draw list's scene texture; chrome paints into the
overlay list) — see §5 hand-off 2.

### 2.2 The overlay leaked pointer input into the scene — W5b §4.3

**Root cause.** `ShellState::pointer_press_belongs_to_shell_chrome` (`🐚️Shell/…:9496`) matched only
`HitKind::{NavbarItem, DropdownItem, ContextMenu, Select}`. The tour's controls are
`HitKind::Button`, so a press on Skip was NOT claimed by the shell; the renderer's own press path
then walked `world3d_states` and handed the same coordinates to the 3D surface under the card, which
is why clicking `shell.tour.skip` also dispatched `interactionHover` + `interactionSelect`.

**Fix.** Three parts:

1. `:9496` — the predicate also claims any hit whose control id starts with `ui.introduction.`.
   Deliberately narrow: claiming `Button` in general would steal a surface's own press, which is the
   exact defect the function's existing comment records.
2. `:19052` scalar 13 — the veil is now a REAL hit target (`ui.introduction.veil`, the whole
   viewport) registered whenever `veil_blocks_pointer`, which ports React's own predicate
   (`step.introduce == null || the target resolved`) so an unresolved target never traps the user.
   It registers BEFORE the three card controls, so `InputState::hit_at`'s reverse-order resolution
   still gives Skip/Next/Back the press.
3. `:9853` — the four controls are answered in `handle_shell_hit` instead of by a rect test inside
   the painter. React wires them as ordinary `onClick`/`onPointerDown` handlers on real elements, and
   routing them through the one press funnel is what lets the veil swallow a press at all. A press on
   the veil ends the tour AND persists the answer, exactly as React's `onPointerDown={skip}` →
   `onDismiss(false)` → `dismissIntroduction` does.

### 2.3 Keyboard

Escape / Enter / ArrowRight / ArrowLeft were already routed (W4a). One divergence remained: the arm
was guarded by `input.focused_id.is_none()`. React binds all three with `enableOnFormTags: true`
(`UIIntroduction`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:5942-5944`) — a veil that owns every pointer must
stay answerable from the keyboard even while a field has focus. The guard is now the sync-attach card
alone (`🐚️Shell/…:11083`).

### 2.4 Dismissal persistence

Unchanged in mechanism and confirmed by law: Skip, a last-step Done and now a veil press all route
through `dismiss_introduction`, which queues `introduction_seen_writes`; the `IntroductionWrite`
phase drives that through W5a's host-storage door to `ui.introduction.seen.<appId>`.

---

## 3 — Control-id parity, and the panel journal

### 3.1 The tour chips — W5b §4.1

`shell.tour.{skip,next,back}` → `ui.introduction.{skip,next,back}`, byte-identical to React's
`close={{ id: "ui.introduction.skip" }}` / `<Button id="ui.introduction.back">` /
`<Button id="ui.introduction.next">` and to the keybinding rows in
`🖱️ui/🔨️modules/🕹️control-keybinding-context/🟦️.tsx:164-166`. New constants at `🐚️Shell/…:19583`,
plus `ui.introduction.veil` for the scrim. The parity probe's `CONTROL_ALIASES` row for the tour is
now dead weight and can go.

### 3.2 The panel tabs — W5b §4.2

wgpu minted `shell.panel.tab.<anchor>.<tabId>`; React mints the BARE `<tabId>`
(`panelTabDefinitionToNode` → `singleTreeLeaf({ id: tabId })`, `🛠️ShellHelpers/🟦️.tsx:1897`). All
three desktop registration sites now publish `node.id` (`:17592`, `:17887`, `:18403`), and the anchor
— which the id used to carry — is derived where React derives it, from the tab's own place in the
tree: new `ShellState::panel_tab_anchor` (`:10507`) over `dock_tab_node_carries` (`:12081`).
`dock_tab_control` is deleted (no alias, no second vocabulary). `handle_shell_hit`'s arm is now
`id if self.panel_tab_anchor(id).is_some()` (`:9887`), in the same position the prefixed arm held, so
every more specific arm still outranks it. The mobile row keeps `shell.panel.tab.mobile.<tabId>`:
React's mobile panel derives its own id separately too.

### 3.3 Panel toggles journalled nothing — W5b §4.4

New `ShellState::journal_panel_selection` (`:10517`), called by `select_panel_tab`, journals what
React journals for the same gesture: `shell.panelToggle` `{anchor, visible}` on a visibility flip
(`🏛️ShellHost/🟦️.tsx:10014`) and `shell.panelTab` `{anchor, tabId}` on a path move (`:10042`),
through the existing `note_shell_command_action` funnel. Two new bilingual chrome strings at
`🐚️Shell/…:20588`, term for term with React's `ui.shellCommand.panelToggle` / `…panelTab`
(`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2752`/`:3594`): `Toggle Panel`/`Panel umschalten`,
`Switch Panel Tab`/`Panel-Tab wechseln`.

---

## 4 — Chord badge glyphs

### Root cause

`format_keybinding_shortcut` (`🐚️Shell/…:12637`) resolved Apple-ness with
`let apple = cfg!(target_os = "macos")`. That is a COMPILE-time fact and it is **false on
`wasm32-unknown-unknown` whatever machine the page runs on**, so every browser boot — macOS included
— formatted `mod` as `Ctrl`. React reads the live navigator
(`keybindingPlatformUsesMetaV1`, `🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`), which is
why the same navbar chip reads `Editor ⌘⌥E` there and `Editor Ctrl+Alt+E` here.

### Fix

| site | change |
| --- | --- |
| `🧊️renderer/🦀️.rs:9207` | new `//#region ⌨️HostPlatform`: `keybinding_platform_uses_meta` (`:9213`, React's `/mac|iphone|ipad|ipod/i` and its `userAgentData.platform === "macOS"` arm in one predicate), a `thread_local` **initialised from `cfg!(target_os = "macos")`** — natively that IS the OS answer and no door call is needed — `set_host_platform_uses_meta` (`:9227`), `host_platform_uses_meta` (`:9232`) and the `#[wasm_bindgen(js_name = semioWgpuSetHostPlatform)]` hook (`:9243`) |
| `🐚️Shell/…:12637` | `let apple = crate::host_platform_uses_meta();` — the table itself is unchanged and stays glyph-for-glyph React's |
| `🧭️boot-descriptor/🟦️.ts:256` | new `⌨️HostPlatform` region: `WgpuHostPlatform` and `resolveWgpuHostPlatform(view)` — `userAgentData.platform` first, then `navigator.platform`, `""` on any throw |
| `🚚️browser-frame-transport/🟦️.ts:179` | `platform` on `BrowserFrameWorkerBoot`, beside `appearance` |
| `🎞️frame-worker/🟦️.ts:61`, `:65` | the `semioWgpuSetHostPlatform` binding and its application in the `runtime-environment` owned step |
| `🚀️browser-boot/🟦️.ts:50` | `hostPlatform()` on the page thread, into the boot message |
| `🎬️renderer-boot/🟦️.ts:202` | the embeddable door publishes it itself at mount |

**Why not the boot descriptor.** The packet asked for the boot descriptor; it travels as an
environment field on the boot MESSAGE instead, exactly as W4a's `appearance` does and for the same
reason: `🧪️tests/🧭️boot-axis-parity/🦀️.rs` holds `WgpuBootDescriptor` to the shape of React's
`FrameworkOsBootOptions`, and React has no `platform` boot option at all — it reads the navigator
directly. Adding an axis no door wants would have meant a new allowlist row against a twin that does
not exist. There is no live-update channel either (unlike `appearance`): the platform is constant for
the life of a navigation.

**One source, pinned by a shared fixture.**
`🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json` — 17 chord rows × 2 platform columns, plus the
platform-spelling lists. It is read by BOTH renderers' laws (§5), so the two tables cannot drift.

---

## 5 — Tests

| law | file | pins |
| --- | --- | --- |
| `an_armed_tour_paints_a_veil_a_card_and_its_controls` | `🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs` | **the overlay paint law** — the veil quad, exactly one glass card sized inside React's `max-w-sm`, ≥ 8 quads inside it, the four control ids, Back absent on step 1, and the veil registering BEFORE the card's controls |
| `an_introduced_target_is_cut_out_of_the_veil_and_the_card_moves_beside_it` | same | the spotlight: the veil stops being one full-screen quad, a band ends exactly at the target's left edge, exactly ONE quad reaches the target (its own ring), and `placement: right` moves the card off centre |
| `the_veil_owns_every_pointer_it_covers` | same | every tour hit is the shell's press and an ordinary chrome `Button` still is not; Next advances, Back returns, a veil press ends the tour AND queues the seen flag once |
| `an_interaction_step_paints_a_checklist_and_no_next_button` | same | `advanceByButton`: a step with interactions hides Next, keeps Skip, and grows the card |
| `the_published_control_ids_are_reacts_own` | same | **the control-id parity law** — the three ids read out of React's own keybinding rows, `panelTabElementId`'s shape read out of `🖥️platform/🟦️.ts`, and that `shell.tour.*` and `dock_tab_control` are gone |
| `a_panel_toggle_journals_reacts_own_shell_commands` | same | both labels in both locales, React's two `noteShellCommand` call sites read out of `🏛️ShellHost/🟦️.tsx`, and this shell's own journal on every selection |
| `the_chord_formatter_answers_the_shared_platform_fixture` | same | **the glyph-formatter fixture** — both platform columns of every row, plus the platform predicate over both spelling lists |
| `the_platform_door_is_wired_end_to_end` | same | the hook is exported, the native default is the OS, the formatter reads the published value and never a `cfg!`, and all three browser doors make/forward the read |
| `⌨️ keybinding glyphs` (2 cases) | `🧑‍🎨engine/🧪️tests/⌨️keybinding-glyphs/🟦️.ts` (new) | **React's half of the same fixture**, through `formatKeybindingShortcut`/`keybindingPlatformUsesMetaV1`. Registered as `engineSuite("⌨️keybinding-glyphs")` in `⚛️react/🧪️tests/🎚️config/🟦️.ts` — an unregistered suite here is a gate that measures nothing |
| `carries the page realm's platform read across the boot seam` | `🧪️tests/📨️browser-frame-transport/🟦️.ts` | the boot field, the Worker's application of it, and `resolveWgpuHostPlatform`'s three arms |

Updated, not deleted:

- `🌓️appearance-tour-and-footer-pills/🦀️.rs` — `the_tour_card_carries_reacts_own_controls` now reads
  the LAYOUT function for the copy and the renderer for the new control-id constants.
- `🧭️wgpu-navbar-footer-parity/🦀️.rs` — the three band filters classified tabs by the
  `shell.panel.tab.<anchor>.` prefix, which no longer exists; they now ask `panel_tab_anchor`. The
  bottom bands additionally exclude `s-sync-status` — see hand-off 3.
- The three TS boot fixtures (`⏱️wgpu-ui-turn-budget`, `🎮️wgpu-browser-input-wire`,
  `📨️browser-frame-transport`) carry `platform`.

---

## VERIFY (all foreground, all RUN)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | **0 errors** | `🗑️generated/w7a-native-check.txt` |
| `cargo check … --lib --target wasm32-unknown-unknown -j 4` | **0 errors** | `🗑️generated/w7a-wasm-check.txt` |
| `cargo test … --lib -- --test-threads=1 tour_overlay_and_chord_glyph_tests` | **8 passed / 0 failed** | — |
| `cargo test … --lib -- --test-threads=1` (whole crate) | **846 passed / 13 failed** — none of the 13 is this packet's, see below | `🗑️generated/w7a-tests.txt` |
| `vitest run … ⌨️keybinding-glyphs` (`SEMIO_TEST_LEVEL=standard`) | **2 passed** | — |
| `nx run @semio-tech/framework-renderer-wgpu:lint` | **exit 0** | `🗑️generated/w7a-ts-lint.txt` |
| `…:generate-browser-boot` / `…:generate-frame-worker` | **exit 0** | `🗑️generated/w7a-generate-*.txt` |
| `…:check-browser-worker` / `…:check-frame-worker` | **exit 0** (the real gate: re-bundles both artifacts and byte-compares) | `🗑️generated/w7a-check-*.txt` |
| `…:test-browser-worker` | **8 files / 92 tests, all passed** (W4a's two pre-existing failures are gone — peers fixed them) | `🗑️generated/w7a-test-browser-worker.txt` |

All nx runs used `NX_DAEMON=false`.

### The 13 crate failures, attributed

Four were already reported red by earlier packets in this ticket, before this one opened:
`shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments` and
`shell::shell_chrome_parity_tests::the_overlay_row_steps_clear_of_an_open_floating_panel`
(`📓️w4a` VERIFY 1, `📓️w5b` §1.6), and
`async_boundary_tests::{presenter_ack_retirement_source_mutations_are_denied,
raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete}` (`📓️w5b` §1.6).

The other nine — `engine_canvas::paint2d_engine_tests` ×3,
`engine_canvas::saturated_graph_and_board_wheel_queues_preserve_cameras`,
`kernel_runtime::semantic_document_tests::command_batch_ninth_document_…`,
`runtime_publication_tests::runtime_single_enqueue_reader_…`,
`shell::command_registry_tests::directory_home_bootstrap_…`, `shell::tool_run_panel_tests` ×2 — are
all in the RETAINED-DOCUMENT paint/present family, and their sources sit in files a peer is editing
in the working tree right now: `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`, `…/🧊️gpu/🦀️.rs`,
`🧊️renderer/🧵️frame-job/🦀️.rs`, `⏱️turn-budget/🟦️.ts` and the metal/d3d12 world3d targets (W5c's
lane). The clearest tell is `tool_run_panel_of_a_running_run_paints_and_its_buttons_dispatch_the_run`,
which prints `[STATS] tool-run panel targets []`: the retained panel painted NO hit targets at all,
which is a `🖍️draw`/retained-paint outcome and cannot be reached from anything this packet touched
(no retained-document, widget, dock or interpreter code is in this diff). I could not establish a
before/after baseline — `git stash` is forbidden here and the tree is shared — so this is an
attribution by construction, not by bisect; the integrator should confirm it against their own
baseline.

### Environment note

The machine ran **out of disk** twice mid-run (`No space left on device` from rustc, and then from
the harness itself). I freed ~5 GB by pruning `⚡️cache/cargo/build/debug/incremental` sessions older
than 3 h, `build/debug/build` directories older than 2 days and the `⚡️cache/nx` entries older than
3 days. It is still under 5 GB free and peers are building continuously — worth watching.

---

## 6 — What the coordinator must confirm LIVE

1. **The frozen canvas, first — everything else is unverifiable until it is settled.** Rebuild, then
   run `🐍️w7a-canvas-liveness-probe.mjs`: the three PNGs must DIFFER (hovering `Inspection` must
   light the chip). If they are still identical, nothing in §2 can be seen and the finding belongs to
   W5c's present lane.
2. **The tour paints on a fresh profile.** Clear
   `ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor` (or use a fresh profile) and re-run
   `🐍️w7a-tour-paint-probe.mjs`. Expect `Welcome to Puzzle 3D`, the body line, `Skip` in the cap row,
   `1 / 5` centred, `Next` on the right, over a solid (not blurred) veil — and the card sized to its
   copy rather than a 320 × 168 box.
3. **Step 2 onward shows the spotlight.** Press Next: the step is `introduce:
   framework.window.puzzle3dMain`, so the right-hand 3D pane must be CUT OUT of the veil and carry a
   breathing ring, and the card must sit beside it, not centred. Step 3's target is
   `framework.panelTab.framework.panel.catalogue` — the Catalogue chip in the navbar.
4. **The overlay no longer leaks.** Re-run `🐍️parity-interact-probe.mjs`'s `dismiss-tour` step: it
   must dispatch NOTHING into the scene (React dispatches nothing), where it previously dispatched
   `interactionHover` + `interactionSelect`.
5. **One vocabulary.** The probe's `CONTROL_ALIASES` tour row should now resolve `exact` on both
   sides, and every panel step should resolve `exact` instead of `suffix`/`absent`. Panel toggles
   must now show `noteShellCommand` on both renderers.
6. **The glyphs.** The navbar role chips must read `Editor ⌘⌥E` / `Viewer ⌘⌥V` on this macOS host.
   This is the one claim that can only be settled in a browser: the platform read now happens on the
   page thread and is forwarded, so only a real page proves it.
7. **Native**: `cargo check … --lib --bins --features native-bin` was not run here (still W1d's open
   hand-off). The native platform default is a `cfg!`, so nothing new depends on winit.

---

## 7 — Hand-offs

1. **P0 — the presentation lane stops presenting** (§1). Owner: W5c / the present lane. Until this is
   fixed every visual observation in this ticket taken against 6213 is a photograph of an early frame.
2. **P1 — the overlay lane still cannot paint a blurred veil.** Unchanged from W4a hand-off 2;
   `Theme::veil_blur_px()` still has no production consumer. The spotlight cutouts now at least make
   the solid veil read as React's picture rather than an opaque sheet.
3. **P1 — `s-sync-status` now names two things.** React has ONE element: the bottom-left sync TAB,
   whose own folded chrome button IS the pill (`🏛️ShellHost/🟦️.tsx:9086`). This renderer paints a
   separate footer pill with the same id (W4a §3) plus the tab, so publishing React's bare tab ids
   made the collision visible — `🧭️wgpu-navbar-footer-parity`'s band law now has to exclude the id.
   The right fix is the footer lane's: paint the pill AS the tab's chrome button.
4. **P2 — the card is not draggable and has no logo row.** React's `UIIntroduction` gives the box a
   `DragHandle` in its cap and renders `step.logos`; neither has a twin here. Neither is used by
   puzzle3d's own five steps.
5. **P2 — `step.demonstrations` / the automatic "click Next" ghost cursor.** React plays one on every
   step (`INTRODUCTION_DEMO_NEXT_BUTTON_DEMONSTRATION`). The tutorial lane's
   `render_tutorial_gesture_overlay_node` is the machinery it would use; the introduction does not
   drive it.
6. **P2 — `🐍️parity-interact-probe.mjs`'s `CONTROL_ALIASES` table** can drop its tour rows and its
   panel-tab suffix rung once the coordinator's run confirms §6 item 5.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
- `…/🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts`, `…/🚚️browser-frame-transport/🟦️.ts`,
  `…/🎞️frame-worker/🟦️.ts`, `…/🚀️browser-boot/🟦️.ts`, `…/🎬️renderer-boot/🟦️.ts`
- `…/🧱️elements/🐚️Shell/🧫️fixtures/⌨️keybinding-glyphs/🔣️.json` (new)
- `…/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs` (new)
- `…/🧱️elements/🐚️Shell/🧪️tests/🌓️appearance-tour-and-footer-pills/🦀️.rs`,
  `…/🧭️wgpu-navbar-footer-parity/🦀️.rs`
- `…/🧪️tests/⌨️keybinding-glyphs/🟦️.ts` (new), `…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- `…/🧪️tests/📨️browser-frame-transport/🟦️.ts`, `…/⏱️wgpu-ui-turn-budget/🟦️.ts`,
  `…/🎮️wgpu-browser-input-wire/🟦️.ts`
- ticket: `🐍️w7a-tour-paint-probe.mjs`, `🐍️w7a-canvas-liveness-probe.mjs` (both new)
