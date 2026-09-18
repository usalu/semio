# 🫧 W8a — tour crispness, the veil's blur, and the symbol glyphs no shipped face carries

Packet W8a of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Input evidence: the wgpu boot
`🗑️generated/w7b-fix-1/run-1/shot-30s.png` against React `🗑️generated/react-6313/final.png`, both
1440×900 dpr 1. Line numbers are post-edit. Every gate under **VERIFY** was RUN, in the foreground,
with `-j 4`, and its log is under `🗑️generated/w8a-*.txt`.

I did NOT run `activate-*`, trunk or the wgpu wasm task — W8b owns the activation. Everything below is
verified natively (checks, a wasm32 `cargo check`, and laws that read the real `DrawList` and the real
`FontAtlas`); §6 lists what only a live boot can settle.

---

## 1 — The two defects, measured

Cropping the boot screenshot at 6× (`🗑️generated/w7b-fix-1/run-1/shot-30s.png`) says both things at
once:

| what | wgpu, before | React `react-6313/final.png` |
| --- | --- | --- |
| tour card, `Skip`, body, `1 / 5`, `Next` | **blurred**, at `[528,398,382,100]` | crisp |
| navbar / footer / window-cap chips | crisp | **blurred** (the veil's backdrop) |
| the scene behind the veil | sharp | blurred |
| navbar role chips | `Editor □□□□E` / `Viewer □□□□V` | `Editor ⌘⌥E` / `Viewer ⌘⌥V` |

So the blur was applied to exactly the wrong layer, and the chord glyphs were four `.notdef` boxes for
two symbols.

---

## 2 — Root cause A: the tour card painted its own copy into the texture the glass pass samples

`render_chrome_tour_step` pushed the card's glass region (`push_glass`, scalar 1) and then painted the
title, `Skip`, `Next`, `Back`, the counter, the body paragraphs and the checklist straight into the
ACTIVE layer. Nothing opened a `begin_glass_content` layer, so `prepared_draw_scalar_is_glass_foreground`
answered `false` for every one of those scalars and the prepared ladder encoded them into
`PreparedDrawTarget::Scene` — the very texture `GlassCommands` then mips and samples back over the
composite. The card blurred away its own labels.

This is **exactly** `📓️w3c` §3's defect, which was fixed for the window cap (`🛰️Dock` /
`🐚️Shell/…:13455`,`:13588`) and for the floating panel (`:17700`) and never given to the tour. The
prepared ladder is `EnsureTarget → ClearScene → Commands → BlurScene → EncodeComposite → GlassCommands
→ ForegroundCommands → Present` (`🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:537`); the split is the whole
mechanism by which anything survives a glass region.

### Root cause B: the veil was a solid quad, so nothing behind it was ever blurred

`introduction_veil_bands`' bands were pushed with `push_solid(…, theme.veil(Level::Dialog))` — the fill
half of React's `ui-veil` and none of its `backdrop-filter: blur(var(--veil-blur))
saturate(var(--glass-saturate))` (`🖱️ui/🎨️styling/🖌️ui/🎨️.css:7005`). `Theme::veil_blur_px()` had had
no production consumer at all since it was introduced (`📓️w4a` hand-off 2, `📓️w7a` hand-off 2).

**The architecture note that said this was impossible was reading the wrong renderer.**
`introduction_veil_bands`' own docstring argued a real glass veil is infeasible because "in
`composite_to_swapchain`, overlay glass regions composite *before* the overlay's own instance pass".
That function is `#[cfg(test)]` (`🖍️draw/🦀️.rs:3841`). The path that actually presents encodes every
non-glass-content layer of BOTH draw lists into the scene the blur chain mips, so a glass region
composites the **frosted navbar, footer and panes**, not blurred bare canvas. The claim was true of the
batch path and false of the production one; the docstring is now corrected in place
(`🐚️Shell/…:15477`).

### The fix

| site | change |
| --- | --- |
| `🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs:429` | new `Theme::veil_glass(level) -> GlassStyle` — the WHOLE `ui-veil` utility: `surface(level)` tinted at `levels::VEIL_ALPHA`, blurred at `levels::VEIL_BLUR_PX`, saturated at `glass_saturate`. `Theme::veil_blur_px()`'s first production consumer |
| `🐚️Shell/…:19173` | the tour's veil bands are `push_glass(…, theme.veil_glass(Level::Dialog))` |
| `🐚️Shell/…:19177` | the card's region index is kept in `cursor.depth` (`region + 1`) and `begin_glass_content` opens immediately after `push_glass` |
| `🐚️Shell/…:19122` | the function docstring carries the law: **a tour armed means its card's glyphs are encoded in the foreground pass** |
| `🐚️Shell/…:15614` | new `close_chrome_overlay_glass_content(cursor, overlay)` — closes the layer on EVERY exit: the terminal step, the glyph-boundary fault, a closed session, a vanished step and a vanished step count. These ladders are stepped across host opportunities and can be abandoned mid-walk; an unbalanced `begin_glass_content` would classify every LATER overlay child as this sheet's foreground |
| `🐚️Shell/…:19074`, `:19077`, `:19037` | **the confirm dialog carried the identical defect** — a solid scrim plus a glass sheet whose title, body and two buttons painted into the scene — and gets the identical split. Same helper, same law |

`punch_introduction_cutout`'s docstring was also corrected: the geometric subtraction stays, but
because 3D window content lives in separate `scene_passes` that cannot be repainted above an overlay,
not because glass was impossible.

---

## 3 — Root cause C: no face this repo ships carries a single one of the chord glyphs

`🐍️w8a-font-coverage-sweep.py` (new, in the ticket folder) parses the `cmap` of **every**
`🔤️outline.ttf` under `🖼️assets/🔤️fonts` with no third-party module, unions the 15 faces the atlas
actually registers (`📝️text/🦀️.rs`'s `ANTA_LATIN` + `KELLY_SLAB_LATIN` + `SHARE_TECH_MONO_LATIN` + the
12 `NOTO_EMOJI_BUCKETS` — 1 651 codepoints), and scans every Rust string literal under the wgpu UI
target, the locale terminology and the os renderer's chrome elements for a codepoint that union lacks.

**Result (`🗑️generated/w8a-font-coverage.txt`): exactly 12 characters, and they are the whole chord
table.**

```
U+2318 ⌘  U+2325 ⌥  U+21E7 ⇧  U+2303 ⌃  U+238B ⎋  U+21B5 ↵
U+232B ⌫  U+2326 ⌦  U+2190 ←  U+2192 →  U+2191 ↑  U+2193 ↓
```

Nothing else in the chrome is uncovered: `·`, `—`, `–`, `…` and every German umlaut resolve through
Anta's latin subset; `⏯`, `⌨`, `⬇`, `▶`, `◀`, `✔` resolve through Noto Emoji. The window cap's
`⤢ ✕ ⠿` are `IconAtlas` vectors, not text, and were never at risk.

Why none of the twelve is there: Google's `symbols` subset (which Anta ships as
`🚀️anta/🔣️symbols`) deliberately skips the Miscellaneous-Technical keyboard block, and none of these
carries emoji presentation, so no Noto Emoji bucket claims them either. React falls back to the host's
system UI font; this atlas registers
`Collection::new(CollectionOptions { shared: false, system_fonts: false })` on purpose — one
deterministic, self-contained collection on every platform and inside the browser frame Worker. There
is no font to fall back TO, on either target.

### Root cause D: `⌘️` is FIVE codepoints, and the variation selector was a box too

The chip read `□□□□E` — **four** boxes for two symbols. This repo spells every emoji with a trailing
U+FE0F and the chord table is no exception (`format_keybinding_shortcut` answers `"⌘️⌥️E"`, and React's
`formatKeybindingShortcut` answers the same string,
`🖱️ui/🔨️modules/🔤️keybinding-text-interpretation/🟦️.ts`). `FontAtlas::ensure_glyph` is per-codepoint,
so each U+FE0F was shaped alone against the Anta-first stack and resolved to Anta's own `.notdef`.
A default-ignorable codepoint is zero-width and zero-ink in every browser.

### The fix — an owned symbol face, no new dependency and no new asset

| site | change |
| --- | --- |
| `📝️text/🦀️.rs:295` | new `//#region 🔣️SymbolFace`: `SYMBOL_FACE`, twelve SVG path outlines authored in 1 000 units per em with the **baseline at `y = 0` and y growing downward**, so a cap-height stroke runs `-620 → 0` and a symbol sits on the same baseline the shaped faces do |
| `📝️text/🦀️.rs:311`,`:315`,`:320` | `SYMBOL_EM_UNITS` / `SYMBOL_STROKE_UNITS` (78) / `SYMBOL_SIDE_BEARING_UNITS` (60) |
| `📝️text/🦀️.rs:335` | `is_zero_width_format_char` — the variation selectors, the joiners, the bidi marks, the soft hyphen, the word joiner and the BOM |
| `📝️text/🦀️.rs:583` | `rasterize_glyph` now states the ONE resolution order: a default-ignorable is blank and zero-advance → a `SYMBOL_FACE` codepoint is drawn from these outlines **in both atlas modes, on native and wasm** → everything else goes to the mode's own pipeline |
| `📝️text/🦀️.rs:611` | `rasterize_symbol_glyph` — strokes the path through `swash::zeno`, the rasterizer swash already brings (`render` ⇒ `scale` ⇒ `dep:zeno`), so owning these glyphs adds **no dependency and no font file**. `zeno`'s `Placement::top` is the bitmap's top edge in the path's y-down space, the sign-flipped twin of swash's; both land in `GlyphEntry` as the same baseline-to-bitmap-bottom bearing every paint call site reads |

The advance is the **measured ink plus both side bearings**, so `measure_text` and
`paint_retained_glyph_step` agree by construction rather than through a hand-declared per-glyph advance
that could drift from the outline. Everything stays logical: the raster honours
`FontAtlas::raster_scale`, so a symbol is crisp at 2× like any other glyph (`📓️w1g`'s unit model).

**Drawn against the real thing.** I rendered all twelve at 64 px out of the atlas and stacked them
under macOS's own `SFNSMono` — `🗑️generated/w8a-symbol-face.png` (top two rows system, bottom two rows
this face). The first pass got ⌘ and ⌥ wrong (⌘'s loops sat at the square's corners instead of the
square's sides running tangentially past them into the loops; ⌥ was flipped, running bottom-left to
top-right instead of top-left to bottom-right). Both are corrected in the shipped table. `⎋` and `↵`
have no glyph in `SFNSMono` at all, which is the same hole React would hit on this host.

---

## 4 — Tests

| law | file | pins |
| --- | --- | --- |
| `an_armed_tours_card_is_encoded_in_the_foreground_pass` | `🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs` | **the crispness law** — ≥ 8 quads on a `foreground_of.is_some()` layer, every one of them inside the card, and NOTHING the card carries left in the scene the glass pass samples |
| `the_veil_carries_reacts_own_blur_and_tint` | same | every veil band is a glass region at `Theme::veil_blur_px()`, `--veil-alpha`, the dialog level's own surface and React's `saturate(--glass-saturate)` |
| `a_confirm_dialogs_sheet_is_encoded_in_the_foreground_pass_over_a_blurred_scrim` | same | the identical split for the dialog: one full-viewport blurred scrim, the sheet's own buttons and copy in the foreground pass |
| `every_chord_glyph_the_formatter_emits_rasterises` | same | **the glyph law** — every character of BOTH platform columns of `🧫️fixtures/⌨️keybinding-glyphs/🔣️.json`, through `FontAtlas::shaped_default()`, rasterises to a bitmap with at least one non-zero texel; a glyph that paints nothing must also advance nothing; and the fixture must contain at least one such zero-width codepoint (it spells its symbols with U+FE0F). A new fixture row naming a glyph the face lacks fails HERE, not in a screenshot |
| `every_shortcut_symbol_rasterises_to_real_ink` | `🖱️ui/🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs` | all twelve, at 11.2 / 12.8 / 16 px, on BOTH a `shaped_default()` and a `builtin()` atlas, each with real ink and a positive advance and on the alpha page |
| `a_default_ignorable_codepoint_paints_nothing_and_moves_no_pen` | same | U+FE0F, U+FE0E, U+200D, U+200B, U+FEFF, U+00AD are zero-texel and zero-advance, and `measure_text("⌘️⌥️E")` equals `measure_text("⌘⌥E")` exactly |
| `a_symbol_glyph_shares_the_faces_baseline_and_honours_the_raster_scale` | same | a symbol's cap stays inside React's own line box, never hangs below the baseline further than a descender, sits within half an em of the shaped face's own cap, and keeps its LOGICAL advance while its ATLAS extent grows at 2× |

Updated, not deleted: `paint_tour` in the tour law file now separates the veil regions, the card
region (found through `foreground_of`), and the foreground vs background quads, and the two existing
paint/spotlight laws read `painted.veil` where they used to read full-screen `push_solid` quads.

`🔬️wgpu-tutorial/🦀️.rs:38` needed `..Default::default()` on its `OrbitController` literal: W8b added
`projection`, `up` and `zoom` to that struct mid-session and the whole crate's test target would not
compile. That is a one-field-list repair in a peer's test, not a change to their lane.

---

## VERIFY (all foreground, all RUN, `-j 4`)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib` | **0 errors** | `🗑️generated/w8a-ui-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | **0 errors** | `🗑️generated/w8a-native-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | **0 errors** | `🗑️generated/w8a-wasm-check.txt` |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- text glyph draw gpu theme` | **562 passed / 0 failed** | `🗑️generated/w8a-ui-tests.txt` |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- tour overlay chrome_parity appearance glyph` | **128 passed / 1 failed** — the one failure is not this packet's, see below | `🗑️generated/w8a-shell-tour-tests.txt` |
| `python3 🐍️w8a-font-coverage-sweep.py` | 12 uncovered characters, all now owned | `🗑️generated/w8a-font-coverage.txt` |

**The one failure**, `shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments`
(`gap baseline must sit under the cutout between tabs and controls`), was **already red before this
packet opened** — `📓️w4a` VERIFY 1 and `📓️w5b` §1.6 both report it, and `📓️w7a` VERIFY lists it among
four pre-existing reds. It is the window-silhouette border geometry; nothing in this diff touches
silhouettes, clips or outline segments.

**One environment-sensitive test to know about.**
`wgpu::engine::tests::large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms` is a
wall-clock budget over one `step_layouts` slice; under a loaded machine it reported 48 ms, 28 ms and
15 ms, and passed clean on the last three runs (it is green in the recorded log). It cannot be this
packet's: it drives a `builtin()` atlas over the ASCII text `node-0…node-1023`, i.e. 15 distinct
characters, so the whole of my addition to that path is 15 × (a 12-entry `char` compare plus one
`matches!`). Worth a quieter re-run by the integrator all the same.

---

## 5 — What this changes about the picture, honestly

After this packet the veil blurs the scene, the panes and the navbar/footer chrome — because all of
those are encoded into the scene the blur chain mips — and the tour card, the dialog sheet and the
spotlight ring are re-encoded crisply on top. That is React's picture.

**The one place it still diverges:** `ForegroundCommands` re-encodes EVERY glass-content layer after
the glass pass, with no notion of which glass region a later region should cover. So the window caps'
own chips (`🛰️Dock`'s `begin_glass_content`, `🐚️Shell/…:13455`/`:13588`) and the floating panel's
content (`:17700`) will stay CRISP over the veil, where React blurs them. Ordering foreground content
against a later glass region is a ladder-level change in `🧊️gpu/🦀️.rs` — hand-off 1. It is a smaller
divergence than the one it replaces, and it is now written into
`introduction_veil_bands`' docstring so the next reader does not have to rediscover it.

---

## 6 — What the coordinator must confirm LIVE

1. **The tour card is sharp and the backdrop is blurred.** Clear
   `ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor` (or a fresh profile), rebuild, run
   `🐍️w7a-tour-paint-probe.mjs`. Expect `Welcome to Puzzle 3D`, the body line, `Skip`, `1 / 5` and
   `Next` legible at 1:1 over a blurred shell — compare against `🗑️generated/react-6313/final.png`.
2. **The role chips read `Editor ⌘⌥E` / `Viewer ⌘⌥V`.** This is the one claim only a browser settles:
   the Apple/other choice comes from the page-thread platform read W7a wired, and the GLYPHS now come
   from `SYMBOL_FACE`. Four boxes would mean the platform read regressed OR the frame worker booted a
   stale wasm; a `Ctrl+Alt+E` spelling is the platform read alone.
3. **Nothing ELSE turned into a box.** The sweep says the chrome is clean, but app-declared labels
   (plugin manifests, example names) were out of its roots. A quick scan of the boot screenshot for
   `□` is enough.
4. **The blur radius reads as React's.** Both are 8 px (`--veil-blur`); a side-by-side crop of the
   region just outside the card is the check.
5. **No `frame world resource admission exceeded fixed credits`.** The veil now costs up to four glass
   regions instead of four solid quads, and each region is one `encode_prepared_glass_scalar`. That is
   four more glass commands per frame while a tour is armed — well inside the ladder's own budget, but
   it is new GPU work on the boot path and the fault banner would say so.
6. **A confirm dialog, if one can be provoked.** Its scrim should blur and its two buttons should stay
   sharp.

---

## 7 — Hand-offs

1. **P1 — foreground content has no order against a later glass region** (§5). Every glass-content
   layer is re-encoded after ALL glass regions, so a window cap stays crisp over a veil that should
   frost it. The honest fix is for `ForegroundCommands` to encode a layer's content only up to the
   next glass region that covers it, which means the glass regions and the foreground layers need one
   interleaved order rather than two independent walks. Owner: the present lane
   (`🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:611`).
2. **P1 — six more overlay sheets carry the defect this packet fixed twice.** Each pushes a glass
   region and then paints its own content into the scene under it, so its labels are blurred away the
   moment it opens. None was open in the boot screenshot, which is the only reason they have not been
   reported. All in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`: the navbar dropdown (`:18487`), the surface
   overlay row (`:18602`), the context menu and its submenus (`:18681`, `:18784`), the agent-approvals
   modal (`:18925`) and the command palette (`:19578`). The fix is three lines each — `cursor.depth`,
   `begin_glass_content`, and a `close_chrome_overlay_glass_content` on every exit of that ladder —
   plus one law apiece; the only real work is auditing each ladder's own early returns.
3. **P2 — `SYMBOL_FACE` is twelve glyphs, deliberately.** It is a bounded fallback for what no shipped
   face carries, not a general symbol font. Anything the UI starts writing that Anta and Noto Emoji
   both lack must either be added here or be given a real subset under `🖼️assets/🔤️fonts`.
   `🐍️w8a-font-coverage-sweep.py` is the instrument that finds them; running it in CI would turn
   "renders as a box" into a build failure.
4. **P2 — `render_chrome_tooltip` and `render_chrome_dialog` (`:19305`, `:19327`) are the `#[cfg(test)]`
   twins of ladders that now differ from them.** They were already drifting; the dialog one now paints
   a solid scrim where the production step paints glass.
5. **P2 — the chord fixture has no `⇪`/`⌤`/`␣` rows** and the formatter emits no such glyph, so the
   face carries none. If the keybinding vocabulary grows a caps-lock or space chord, §4's law is what
   will catch it.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-tutorial/🦀️.rs` (peer repair, §4)
- ticket: `🐍️w8a-font-coverage-sweep.py` (new), `📓️w8a-tour-crispness-and-symbol-glyphs.md` (this file)
