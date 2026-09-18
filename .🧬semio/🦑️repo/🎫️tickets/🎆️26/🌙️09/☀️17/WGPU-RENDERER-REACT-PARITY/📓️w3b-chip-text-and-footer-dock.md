# 🏷️ W3b — Chip text, font face, footer dock and navbar bands

Packet W3b of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Input evidence: the first real wgpu boot
screenshot `🗑️generated/wgpu-boot-2/shot-20s.png` (1440×900, dpr 1) against the React reference at
`http://127.0.0.1:6013/?plugin=puzzle3d`. The React side was read twice: a headless screenshot pass
(`🗑️generated/w3b-react/`, probe copy of `🐍️wgpu-console-dump-probe.mjs` with `SEMIO_PROBE_URL`
pointing at 6013) and — because the boot tour overlay blurs the chrome — a live DOM geometry read of
every navbar/footer/window control's `getBoundingClientRect` at 1024×768. All React numbers below are
that measured geometry, not a reading of the source.

---

## 1. Chip/tab labels wrapped onto two lines inside every chip

**Symptom.** Every chrome chip label broke mid-word inside its own 22 px chip: `Concrete F/orest`,
`Edit/or`, `View/er`, `Catalo/gue`, `Artif/act`, `Ch/at`, `Tool r/uns`, `Inspec/tion`, `Fullsc/reen`,
`Transf/orm`, `Bru/sh`, `Volume B/rush`, `Reloc/ate`, `Fi/le`, `Fold/er`, `Remo/te`, `To/ol`,
`Comm/and`, `Setti/ngs`, `Marketp/lace`, `Hist/ory`.

**Root cause (measure ≠ paint).**
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11765` `retained_chrome_group_item_width` priced a chip's label at

```rust
label_bytes as f32 * theme.font_size_small * 0.6
```

a per-BYTE guess — 6.72 px/byte at `--text-xs` (11.2 px, `ui_styling::metrics::typography::TEXT_XS_PX`)
— while the painter (`🖌️paint/🦀️.rs` `paint_retained_glyph_step`, reached through
`chrome_text_step`, `🐚️Shell/…:11713`) walks `FontAtlas`'s own advances. The atlas the frame worker
booted with pens a flat **10 px** per ASCII glyph (defect 2), so every chip laid out at 0.672 × its
own painted label and the painter's wrap branch (`cursor.pen_x + advance > bounds.w` → next line)
broke the run mid-word. The observed break positions reproduce this exactly: `Concrete Forest` fits
10 of 15 glyphs (15 × 6.72 = 100.8 px ⇒ 10 × 10 px), `Editor` 4 of 6, `Chat` 2 of 4.

**React reference.** Chip labels are `whitespace-nowrap` + `truncate` and the box hugs the measured
text (`🧰️framework/🔨️modules/🖱️ui/🔨️modules/🎛️chrome-control-presentation/🟦️.ts`; measured widths at
`text-xs`: `Artifact` 39 px label in a 76 px chip, `Catalogue` 53/90, `Inspection` 55/92,
`Tool runs` 48/85, `Chat` 25/62, `Fullscreen` 54/86, `Concrete Forest` 98/192).

**Fix.**
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11765` — `retained_chrome_group_item_width(atlas, theme, item)`
  now measures: `padding*2 + icon + atlas.measure_text(label, theme.font_size_small).0`, read off the
  very atlas that paints it. All 13 call sites (navbar clusters, navbar/footer tab rows, fullscreen
  chip, dropdown rows, footer utility nodes, sync/check-in pills, tutorial bar, surface controls)
  updated. A multi-byte scalar no longer inflates its chip by its UTF-8 length either.
- `🖌️paint/🦀️.rs` — new `RetainedTextFlow { Wrap, Clip }` + `paint_retained_glyph_step_flowed`.
  `chrome_text_step` (`🐚️Shell/…:11713`) now flows `Clip`: chrome text stays on ONE line and loses
  its tail if a box is ever undersized, which is React's `nowrap + truncate`, never a second line
  inside a chip.
- `🖌️paint/🦀️.rs` — `RETAINED_TEXT_FIT_EPSILON = 1/64 px` (a browser `LayoutUnit` grain) on the
  overflow test. A box sized from `measure_text` is the SUM of the advances the painter
  re-accumulates and f32 addition need not land on the same last bit; without the slack a perfectly
  measured `Chat` lost its final glyph. This was caught by the new law, not by inspection.
- `🐚️Shell/…:17649` — the chrome tooltip's box now measures its text too (same class of bug).

---

## 2. Glyphs looked monospace with wide tracking

**Root cause.** `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs:549` booted the frame
worker's atlas with `FontAtlas::from_bytes(&[])`. Empty bytes is the documented "deterministic 8×16
ASCII bitmap" contract (`🖱️ui/🎯️targets/🧊️wgpu/📝️text/🦀️.rs:447`), i.e. `FontAtlas::builtin()`:
fixed 8 px glyph boxes, a flat `advance = 10` at every size, `raster_scale = 1.0` (so it ignores dpr
too). A pixel dump of the boot screenshot's chip row confirms it — uniform 10 px pitch, 5–6 row tall
blocky glyphs, exactly `BITMAP_FONT`'s patterns. **The frame worker paints the browser boot** (the
screenshot's own fault text says the canvas belongs to the worker), so this — not the UI-thread path
at `🧊️renderer/🦀️.rs:14571`, which loads real Anta — is what the user saw.

**React reference.** `--font-sans: Anta, …` (`🖱️ui/🎨️styling/🎨️palette/🎨️.css:217`), the same face
the atlas's `FAMILY_SANS` registers from `🖼️assets/🔤️fonts/🚀️anta`.

**Fix.** New `FontAtlas::shaped_default()` (`📝️text/🦀️.rs`) — full `Shaped` mode over the embedded
Anta / Kelly Slab / Share Tech Mono / Noto Emoji families with no host override — and the worker now
calls it. Measured cost in the new native law: building two shaped atlases plus every chip
measurement is 0.07 s, so this is not the multi-second `font-atlas` boot phase the frame-worker
docstring warns about.

---

## 3. Footer dock

The boot's footer chips were, left to right: `Transform · Brush · Volume Brush · Relocate · File ·
Folder · Remote · Tool · Command · Settings · Marketplace · History`.

**Correction to the packet brief.** The first seven are NOT the Tool branch's leaves. They are the
footer UTILITY rail: `render_footer_step`'s phase 2 walks `self.active_utilities` (the app's declared
utilities scoped to the active window kind, `derive_utility_nodes`) with `framework_sync_utilities`
(`File`/`Folder`/`Remote`, `🐚️Shell/…:12009`) appended. `Tool` and `Command` are already correct —
collapsed BRANCH chips, exactly as React renders them.

React's footer, measured (vw 1024): `Display`(x3 w75) `Remote: detached`(#s-sync-status, x78 w133) |
`Tool`(x439) `Command`(x497) — group x439 w147, centred on 512 | `No one else is here`
(#s-presence-peers, x660) | `Settings`(x764) `Marketplace`(x845) `History`(x946, ends 1021 = vw−3).
Composition source: `🏛️ShellHost/🟦️.tsx:10613-10635` — bottom-left bar, bottom-middle bar with
`centered: true`, a fill, the presence pill, bottom-right bar.

### 3a. No `Display` chip — FIXED
`default_dock` built the Display branch out of the APP's bottom-left panel tabs and skipped the
branch entirely when there were none (puzzle3d declares none). React's branch children are the two
FRAMEWORK display leaves, always present (`createFrameworkDisplayPanelTabs`,
`📌️ChromePanels/🟦️.tsx:249` → `framework.display.windows`, `framework.display.layout`), and the
app's own display-group tabs are **siblings** of the branch (`🏛️ShellHost/🟦️.tsx:9431-9433`).
Fixed in `🐚️Shell/…:6615` plus a new `ShellState::framework_display_tabs` (ids and
`display.tab.windows`/`display.tab.layout` strings already existed in this file and were unreachable).

### 3b. All three bars left-packed — FIXED
New `ShellState::footer_tab_row_rect` (`🐚️Shell/…:16256`) places each anchor in its own band:
bottom-left at the leading edge, bottom-middle centred on the footer, bottom-right ending at
`width − padding`, every band in the anchor's declared order, all widths measured on the painting
atlas. `render_footer_step`'s phase 5 consumes those rects instead of one running cursor.

### 3c. Remaining, out of this packet's lane (reported, not fixed)
- **The footer utility rail has no React counterpart.** React's utilities live on the window's own
  `Utilities` chip (`framework.window.<id>.utilityBar.unfold`, measured at the pane's bottom-left).
  Removing the rail deletes `render_footer_utility_node(s)`, its hit routing and its tour notes, and
  the wgpu window chrome has no utilities surface to move them to — that is the window/utilities
  lane's call (Architecture Decision 5 is written into these docstrings). The new banding keeps the
  bottom-left band opening AFTER the rail (`lead_x`) so nothing overlaps in the meantime.
- **`Remote: detached` is native-only.** `sync_status`/`sync_channel`/`sync_bootstrap_progress` are
  `#[cfg(not(target_arch = "wasm32"))]` fields (`🐚️Shell/…:2802-2808`) and phase 3 of
  `render_footer_step` is gated with them, so the browser boot paints no sync pill at all. React has
  a browser sync lane; wiring one is a structural gap, not a composition bug.
- **No presence pill.** React always shows `#s-presence-peers`. `ShellState::presence_peers` exists
  (`🐚️Shell/…:2872`) but `render_presence_bar` no longer does — only docstrings reference it — and
  there is no `presence.empty` chrome string on this side.

---

## 4. Navbar anchor → side mapping — FIXED

React, measured: `Artifact`(x3) `Catalogue`(x79) at the LEADING edge; `Inspection`(x691)
`Tool runs`(x783) `Chat`(x868) on the trailing side; `Fullscreen`(x933) trailing-most; the
logo/title/example/mode/role cluster is an absolutely CENTRED overlay layer (x169 w522).
Source: `navbarItems` = `[top-left bar, fill, top-right bar, centred cluster]`
(`🏛️ShellHost/🟦️.tsx:10087-10100`).

wgpu had ONE right-to-left row over `[TopRight, TopMiddle, TopLeft]`
(`navbar_tab_row_item`), which is why the boot read `Catalogue · Artifact · Chat · Tool runs ·
Inspection` all on the right, in reverse.

- `navbar_leading_tab_row_item` + `navbar_leading_tab_row_rect` — top-left's tabs at the leading
  edge in declared order; painted by a new `render_navbar_step` phase that runs between the navbar
  hairline and the logo, so the logo/title cluster opens after the band instead of under it.
- `navbar_trailing_tab_row_item` — `[TopMiddle, TopRight]` enumerated in REVERSE, so the existing
  right-to-left `cursor.right` walk lays them out left-to-right with the last tab nearest
  `Fullscreen`.
- Declared divergence: React carries top-middle inside the centred cluster; this renderer still
  left-packs that cluster, so top-middle heads the trailing band. Unobservable while the default dock
  leaves top-middle empty; pinned as such in the fixture.

---

## 5. Window title chips — reference captured, not fixed

The boot screenshot shows only one painted pane (the other is the DOM fault overlay), so there is no
honest before/after yet. React's measured per-pane geometry, for whoever takes this:

| row | leading | centre | trailing |
| --- | --- | --- | --- |
| cap, y≈35 | window tab carrying the WINDOW title (`Top`, `Perspective`; id `mode-dock-tab-<i>-<windowId>`) + `Focus`, `Close` icon chips | — | — |
| pane top, y=64 | `Actions` (`framework.window.<id>.engagement.toggle`) | `Search` (`….search.toggle`), centred on the pane | `Window Options` (`….measures.unfold`), ending at pane right − 9 |
| pane bottom, y=710 | `Utilities` (`….utilityBar.unfold`) | — | `Projection` (`framework.worldOrbit.projection.<id>.pane.fold`) |

So a pane's overlay rows use the same leading/centre/trailing banding as the footer, which
`footer_tab_row_rect` now implements and could be shared. wgpu's cap showed the app label
(`Puzzle 3D`) where React shows the window title — needs the rebuilt boot to confirm whether that is
a label-source bug or simply this session's only window.

---

## Tests added

`🐚️Shell/🧪️tests/🔬️wgpu-retained-chrome-text-laws/🦀️.rs`
- `every_chip_lays_out_at_least_its_labels_painted_width` — **the chip law**. For the whole shell
  chip label set (the 29 labels of the boot evidence + both framework branch label sets + both
  locales of 18 chrome-string keys), with and without a leading icon, on BOTH atlases
  (`builtin()` and `shaped_default()`): the granted label box ≥ the measured paint width, clipping
  drops no glyph, and painting the same label WRAPPING keeps every glyph on line 0.
- `chrome_text_clips_instead_of_wrapping_a_narrow_box` — chrome never wraps, whatever box it gets.
- `measured_label_advance_equals_the_painted_advance` — one glyph per scalar inside the measured
  width, last glyph ending inside it, on both atlases.
- The two pre-existing width tests were updated for the atlas-taking signature.

`🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs` (W1h's shared-fixture home)
- `default_dock_display_branch_carries_the_framework_display_leaves`
- `navbar_bands_match_the_react_chrome_band_fixture`
- `footer_bands_match_the_react_chrome_band_fixture` (leading/centred/trailing + no overlap)

`🧫️fixtures/🧭️default-dock/🔣️.json` (W1h's shared fixture, extended)
- `displayBranch` — the framework display children, with the sibling rule written down.
- `chromeBands` — which side of the navbar/footer each chrome-hosted anchor occupies.
- `expected["bottom-left"]` and `mobilePanelTabs.ids` corrected to React's arrangement (the fixture
  previously pinned the wgpu-only "app tabs are the branch's children" shape).

`🧪️tests/⌨️wgpu-shell-shortcuts-palette/🦀️.rs` — `each_panel_anchor_chord_toggles_its_own_anchor`:
bottom-left now always has the Display branch to open, so the empty-anchor no-op law moved to
left-middle (the one anchor no source populates).

---

## Verification (all foreground, logs under `🗑️generated/`)

| command | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib -j 4` | ok, 4 warnings | `w3b-check-ui.txt` |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -j 4 -- text chrome widget glyph` | **87 passed** | `w3b-test-ui.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib -j 4` | ok | `w3b-check-renderer.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --tests -j 4` | ok, 82 warnings | `w3b-check-renderer-tests.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown -j 4` | ok, 67 warnings | `w3b-check-renderer-wasm.txt` |

One wasm run in between failed on `E0599 no method named stack` at
`🌐️browser-worker/🦀️.rs:708` — W3a's panic-hook edit, in flight in the same file, which they had
already rewritten to `js_sys::Reflect::get` nine seconds later. Re-run clean. Worth knowing because
that file now carries edits from both packets (their panic hook at :701, this packet's atlas at :549).
| new laws + dock fixture tests, `--test-threads=1` | **10 passed, 0 failed** | `w3b-test-final.txt` |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- --test-threads=1 shell::` | 291 passed, 10 failed | `w3b-test-shell-serial.txt` |

### The 10 remaining `shell::` failures are not this packet's

The tree carries 138 uncommitted files from concurrent packets, so there is no clean baseline to
diff against; each failure was instead attributed by code path.

- `shell_document_retirement_*` (2), `tool_run_panel_*` (3), `window_measures_*` (1) — all fail with
  resident-arena/permit capacity messages (`ArenaFull`, `Permit { fault: Capacity }`,
  `resident capacity exhausted … committed items 24595/131076`). `🎟️prepared/🦀️.rs` and
  `🏟️arena/🦀️.rs` carry a peer's in-flight edit (both mtime 11:06, distinct from every file here);
  the same capacity failures appear in `semio-framework-ui`'s own suite
  (`w3b-test-ui-all.txt`: 546 passed, 5 failed, all engine/prepared/reconcile, none referencing
  `paint_retained_glyph_step` or `RetainedTextFlow`).
- `ui_prefs_themes_i18n_*` (2) — process-global env locks under a parallel runner.
- `window_silhouette_border_emits_notched_outline_segments` — pure
  `WindowSilhouette::from_measured_top` + `push_window_silhouette_border` geometry: no atlas, no
  dock, no chip width.
- `the_overlay_row_steps_clear_of_an_open_floating_panel` — asserts the World3d cancel control is
  flush (7.2) while `surface_overlay_controls_for` now leads it by the status pill (151.2); that lead
  comes from `world3d_status_pill_width`, which this packet did not touch.
- `directory_home_bootstrap_retries_cancels_and_rebootstraps_without_cursor_loss` — directory
  bootstrap frontier, unrelated lane.

### Needs the live rebuild (W3a owns the only wasm activation)

Nothing here was verified in a browser: no `activate-*`, no `framework-renderer-wgpu:wasm`, no trunk.
After the next real boot, check in order:
1. every chip label on one line, in Anta rather than the ASCII bitmap face;
2. the footer reading `Display · … · Tool · Command · Settings · Marketplace · History` with the
   Tool/Command pair centred and the Settings group flush right;
3. the navbar reading `Artifact · Catalogue` on the left and `Inspection · Tool runs · Chat ·
   Fullscreen` on the right;
4. worker boot timing — the worker now builds a shaped atlas (15 registered faces) where it used to
   build the bitmap fallback. Native cost is negligible, but the `font-atlas` boot phase is the one
   the frame-worker docstring already flags as multi-second, so watch `declare_boot_subphase`.

## Same-class sites left estimating (deliberate, need an atlas they cannot reach)

- `world3d_status_pill_width` (`🐚️Shell/…:12768`) and `surface_status_pills_for` /
  `surface_overlay_controls_for` — pure, law-driven fns with no atlas parameter; threading one
  through changes their public shape and the tests that drive them.
- `transient_notice_rect` / `transient_notice_close_rect` (`🐚️Shell/…:14474-14486`) — geometry fns
  whose own docstring admits the "monospace approximation".
- `dock_tab_drop_index` (`🐚️Shell/…:16547`) — priced a chip at `×0.5` while its painting siblings
  (`:16549`, `:16592`) measure, so a tab drop lands off the chip it was previewed on. It runs from
  `handle_pointer_button`, which has no atlas; the honest fix is to read the painter's registered
  `shell.panel.tab.*` hit rects instead of re-deriving widths at all.
- `context_menu_level_width` and the context-menu shortcut widths (`:18466`, `:18545`) are
  `#[cfg(test)]`-only paths.
