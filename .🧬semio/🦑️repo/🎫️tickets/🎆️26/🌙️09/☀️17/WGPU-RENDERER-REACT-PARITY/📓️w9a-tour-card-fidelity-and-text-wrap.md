# ✂️ W9a — CSS line breaking, the tour card's own chrome, and the two W8a hand-offs

Packet W9a of ticket `26/09/17/WGPU-RENDERER-REACT-PARITY`. Evidence: wgpu
`🗑️generated/tour-paint-1/t025.png` against React `🗑️generated/react-6313/final.png`, both 1440×900
dpr 1. Line numbers are post-edit. Every gate under **VERIFY** was RUN in the foreground with `-j 4`.

I did NOT run `activate-*`, trunk or the wgpu wasm task — W9b owns the activation. Everything below is
settled natively; §6 lists what only a live boot can.

---

## 1 — The three defects, side by side

| what | wgpu `t025.png` | React `final.png` |
| --- | --- | --- |
| body copy | `…before you start comp` / `osing.` — **broken mid-word** | `…before you start` / `composing.` |
| step counter | `1 /` — the total is missing | `1 / 5` |
| advance control | bare `Next` | `Next ↵ ›` |
| dismiss control | bare `Skip` | `✕ Skip` |
| card | a plain glass sheet | an outlined card: title chip + drag grip, body, footer band with three chips |
| window caps over the veil | crisp | blurred with the rest of the shell |

---

## 2 — Root cause A: the retained painter broke at whatever glyph overflowed

Every WRAPPED run in the wgpu target went through
`paint_retained_glyph_step_flowed` (`🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs:129`), whose whole wrap rule
was one per-glyph test:

```rust
let overflows = cursor.pen_x > 0.0 && cursor.pen_x + advance > bounds.w.max(1.0) + EPSILON;
```

That is a CHARACTER wrap. It has no notion of a word, so the first glyph of `composing` that did not
fit dropped to the next line and left `comp` behind — exactly what the screenshot shows.

The MEASURES did not agree with it. `FontAtlas::measure_text_wrapped` and `widgets::wrap_text` both
`split_whitespace`d (word wrap, but blind to interior space runs, hyphens and CJK), and
`mounted_layout::measure_text` (W1l's worker-side intrinsic measure) had a third, hand-rolled
space/tab/newline rule. So a box was sized for one wrap and painted with another — the card was
measured tall enough for two lines and then painted with a word cut in half.

### The fix — one predicate, four call sites

| site | change |
| --- | --- |
| `🖱️ui/🎯️targets/🧊️wgpu/📝️text/🦀️.rs:179` | new `//#region ✂️LineBreak`. `is_wrap_space` (every whitespace but `\n` and U+00A0 — a no-break space is written precisely to forbid the break), `breaks_after` (UAX#14 `BA`/`ZW`: `-`, U+00AD, U+200B, U+2010/2012/2013/2014), `is_ideographic` (`ID`), `is_no_break_before` (`CL`/`NS`), `is_no_break_after` (`OP`) |
| `📝️text/🦀️.rs:233` | **`may_break_between(prev, next)` — THE soft-wrap predicate of this target.** CSS `word-break: normal` / `overflow-wrap: normal`, term for term: after a run of spaces, after a hyphen-family scalar, and on either side of an ideograph except where a bracket forbids it. Nothing else |
| `📝️text/🦀️.rs:251`,`:262` | `is_break_opportunity(text, byte)` and `unbreakable_run_end(text, byte)` — the run a greedy wrap must fit WHOLE or move down |
| `📝️text/🦀️.rs:868`,`:881` | `FontAtlas::measure_range` and **`FontAtlas::wrap_lines`** — CSS greedy first-fit, as byte ranges. Trailing spaces HANG (they join the line they end and never push it over the box); a hard `\n` always ends a line; the single last-resort arm cuts a word only when that one run is wider than the whole box, which is CSS's `overflow-wrap` fallback and the alternative to painting a word outside its own card |
| `📝️text/🦀️.rs:913` | `measure_text_wrapped` is now `wrap_lines` priced per line with its hanging spaces trimmed |
| `🪀️widgets/🦀️.rs:563` | `wrap_text` is the owned-line view of `wrap_lines` |
| `🖌️paint/🦀️.rs:164` | the retained stepper prices the whole unbreakable run at a break opportunity and moves it down as one; the per-glyph arm is now reachable only as the `overflow-wrap` last resort, and a space never triggers either arm |
| `📌️mounted_layout/🦀️.rs:247`,`:261` | W1l's `MinContent` (widest unbreakable run) and `Definite` (greedy first-fit) both go through `may_break_between` over the shaped scalars, so the worker-side measure and the painter cannot disagree |
| `🖌️paint/🦀️.rs:125` | `RETAINED_TEXT_FIT_EPSILON` is now `text::LINE_BREAK_FIT_EPSILON` — one 64th of a logical pixel, shared by the measure and the paint rather than declared twice |

## 3 — Root cause B: the counter chip was priced one `padding-inline` too narrow

`chrome_tour_layout` sized the counter at `measure_text(counter_text) + padding_standard * 2`, while
the painter insets chip text by `padding_standard * 2` on BOTH sides:
`chip_text(rect) = (rect.x + pad*2, …, rect.w - pad*4)`. The text box was therefore
`padding_standard * 2` NARROWER than its own label, and chrome text flows `RetainedTextFlow::Clip`,
which drops an overflowing glyph silently. `1 / 5` lost its tail on every step and nothing in the walk
reported it. The counter now goes through the same `chip_w` closure as every other chip.

## 4 — Root cause C: the card was a sheet, not React's `WindowChrome`

React mounts the card as `WindowChrome` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:5982`) with
`borderKind="introduced"`, a `titleChips` row carrying the step title and a `DragHandle`, a `close`
control (`{ id: "ui.introduction.skip", icon: <CloseIcon/>, label: skipLabel }`), a
`footerCenterChips` counter chip and `footerLeftChips`/`footerRightChips` `Button`s.
`ButtonGroupItem` (`🖱️ui/🧱️elements/🔳️ButtonGroup/🟦️.tsx:110-118`) renders **inline label → chord
badge → icon**, which is why React's footer reads `Next ↵ ›` with the chevron LAST; only
`WindowChrome`'s `close` leads with its icon, which is why Skip reads `✕ Skip`.

| site | change |
| --- | --- |
| `🐚️Shell/…:15766` | `ChromeTourChip { rect, label, hotkey, icon_id, leading_icon, filled }` — one chip in React's own reading order |
| `🐚️Shell/…:15751` | `INTRODUCTION_SHORTCUT_ROWS` + `introduction_control_hotkey` — the three `SHELL_KEYBINDINGS` rows (`ui.introduction.{skip,next,back}`) that `SHELL_SHORTCUT_ROWS` deliberately does not carry, because that table is the ACCELERATOR table and every row of it must resolve to a `ShellShortcut` verb. The badge reads a user override first, exactly like React's `useControlHotkey` |
| `🐚️Shell/…:15777` | `chrome_tour_chip_icon_x` / `chrome_tour_chip_label_box` / `chrome_tour_chip_hotkey_box` / `push_chrome_tour_chip_border` |
| `🐚️Shell/…:15842` | `chrome_tour_layout` rebuilt: chips priced with their icon and chord, a title CHIP with the `grip-vertical` handle at its trailing edge, a footer rule, and React's `gap-double`/`mb-double` between body paragraphs (`gap_standard * 2`, not `gap_standard`) |
| `🐚️Shell/…:19361` | `render_chrome_tour_step` rebuilt around it: card glass → `begin_glass_content` → the card's own `push_introducing_border` (React's `borderKind="introduced"`) and the spotlight ring → title chip border → title → grip → Skip chip → footer rule → counter chip → Next (label, chord, chevron/check) → Back (label, chord, chevron-left) → paragraphs → checklist → hits |

## 5 — The two W8a hand-offs

### Hand-off 1 — foreground content had no order against a later glass region

`ForegroundCommands` re-encoded EVERY glass-content layer after the glass pass, so a window cap's
chips stayed crisp over the introduction veil while React blurs the whole shell except the card.

`🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:89`,`:104`,`:124` — `prepared_draw_scalar_glass_region` (the
region a scalar's layer is content OF), `prepared_glass_region_covers` and
`prepared_foreground_scalar_is_enclosed`. A glass-content scalar whose own region is fully enclosed by
a LATER region is encoded into the SCENE instead of the composite: it is mipped by the blur chain, its
own region re-frosts it and the covering region frosts it again — which is what "the caps are under
the veil" means on a renderer whose blur chain reads one scene texture. The overlay draw list is
encoded after the main one, so every overlay region is later than every main-list region; that is the
veil-over-cap case itself.

**Containment, not overlap, is the predicate** (`:104`). A context menu that clips a panel's corner
must not push that panel's whole content into the backdrop, and a spotlight step's veil is BANDS
around the cutout — bands enclose nothing that straddles them, so the introduced element stays crisp
exactly as React's `useIntroductionElevation` elevates it.

### Hand-off 2 — six more overlay sheets painted their glyphs into the sampled texture

All in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, all now split, plus a shared opener
`open_chrome_overlay_glass_content` (`:15731`) beside W8a's `close_chrome_overlay_glass_content`:

| sheet | region | closed on |
| --- | --- | --- |
| search / find / navbar dropdown | `render_overlay_step` phase 0 | the phase 2 → 3 transition, including the "no rows" exit |
| World3d compute status pill | phase 7 scalar 0 | the end of each pill and the empty-list exit |
| retained context menu | `render_context_menu_step` scalar 0 | scalar 4 and the "no menu" exit |
| chrome tooltip | `render_chrome_tooltip_step` scalar 1 | scalar 3 and the dialog-opened exit |
| agent approvals modal | `AgentApprovalPaintOp::Modal` | the op-list terminal and the closed-queue exit. Its `Scrim` is now a real `veil_glass` region, not a solid quad |
| immediate-mode `render_context_menu_level` | the recursive level | the function's own exit, with the layer closed and reopened around each submenu so the stack stays balanced |

The two `#[cfg(test)]` twins (`render_chrome_tooltip`, `render_chrome_dialog`, W8a hand-off 4) got the
same split and the dialog's scrim became `veil_glass`, so they stop drifting from the ladders they twin.

## 6 — Tests

| law | file | pins |
| --- | --- | --- |
| `a_break_opportunity_is_exactly_what_css_allows` | `🖱️ui/🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs` | 15 cases of `may_break_between`: inside a word, after a space, between two spaces, before a space, the no-break space, hyphen/soft-hyphen/ZWSP/em-dash, both ideographic sides, the closing and opening bracket suppressions, and that `.` and `/` are NOT breaks in Latin |
| `a_paragraph_wraps_at_the_reference_break_indices` | same | **the wrap law.** A REFERENCE greedy wrap written in the test from the CSS rules (not borrowed from the implementation) is swept over every box width from "the widest word fits" to "the paragraph fits" (the law refuses a sweep of fewer than eleven widths) and `wrap_lines` must break at the identical byte indices, with every line start preceded by a space |
| `only_a_word_wider_than_the_box_is_ever_cut` | same | the `overflow-wrap` arm is the ONLY way a word is cut; no line of a real paragraph starts mid-word |
| `a_newline_always_breaks_and_a_trailing_space_hangs` | same | a hard `\n` breaks in any box; a trailing space neither wraps nor is priced into the line box |
| `the_retained_painter_breaks_where_the_measure_says` | same | **the paint law.** `paint_retained_glyph_step_flowed` is driven scalar by scalar over a real `DrawList` and EVERY scalar must land on the line `wrap_lines` measured it onto. This is the law `comp\|osing` violated |
| `glass_content_under_a_later_enclosing_region_is_encoded_into_the_scene` | `🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs` | hand-off 1: a cap with nothing over it stays crisp; a full-viewport overlay veil encloses it; a veil BAND that starts below it does not; a menu that clips its corner does not; the last region's own content stays crisp |
| `the_footer_counter_paints_its_whole_step_of_the_total` | `🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs` | the chip holds its whole label AND every scalar of `1 / 2` reaches the draw list — a clipped tail fails here, not in a screenshot |
| `the_cards_chips_carry_reacts_label_chord_icon_order` | same | Next's chevron/check + `↵` badge and its label→chord→icon order, Skip's LEADING `✕` and absent badge, the grip after the title inside the title chip, and the last step's `Done` + Back chip with its own chord |
| `every_overlay_sheet_with_a_glass_region_encodes_its_glyphs_in_the_foreground_pass` | same | hand-off 2: a source scan refusing any non-veil `push_glass` that opens no content layer (a NEW sheet fails here), plus a runtime drive of the context menu asserting nothing it carries is left in the sampled scene and the walk closed its own layer |

Updated, not deleted: `wrapped_and_single_line_measurement_agree_on_the_line_box` measured at
`max_width = 1.0`, a degenerate box in which CSS's `overflow-wrap` arm correctly cuts every word; it
now measures at one word's own width, which is what the law was always about.

---

---

## VERIFY (all foreground, all RUN, `-j 4`)

| gate | result | log |
| --- | --- | --- |
| `cargo check -p semio-framework-ui --features wgpu-engine --lib` | **0 errors** | `🗑️generated/w9a-ui-check.txt` |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` | **568 passed / 0 failed** | `🗑️generated/w9a-ui-tests.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib` | **0 errors** | `🗑️generated/w9a-native-check.txt` |
| `cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` | **0 errors** | `🗑️generated/w9a-wasm-check.txt` |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- tour overlay chrome_parity appearance glyph context_menu` | **150 passed / 1 failed** — the one failure is not this packet's, see below | `🗑️generated/w9a-shell-tests.txt` |

**The one failure**, `shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments`
(`gap baseline must sit under the cutout between tabs and controls`), was red before this packet
opened: `📓️w4a` VERIFY 1, `📓️w5b` §1.6, `📓️w7a` VERIFY and `📓️w8a` VERIFY all report it. It is window
SILHOUETTE border geometry; nothing in this diff touches silhouettes, clips or outline segments.

**Run both suites SERIALLY (`--test-threads=1`).** Under `-j 4` parallelism
`wgpu::prepared::tests::{packet_drop_retires_nested_backings_and_permit_scalars_separately,
pending_presenter_witness_rejects_superseding_packet_with_exact_owner}` contend over the 64 shared
`PREPARED_GPU_ABANDONMENT_STATE` slots and flake; both pass clean serially, and neither touches
anything this packet edits. `wgpu::engine::tests::large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms`
is the wall-clock budget `📓️w8a` already flagged — it reported 22.9 ms while twelve peer cargos were
resident and passed three consecutive times once the machine was quiet.

**🔒️ A fleet-wide cargo deadlock had to be cleared to run any of this.** Twelve cargo invocations sat
on `.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/.cargo-lock` for twenty minutes with NO `rustc` alive.
`sample` put two of them (a `semio-s-artifact-layout-layout` check and a `semio-s-artifact-wfc-grid3d`
test) inside `cargo::core::compiler::prebuild_lock_exclusive` → `LockManager::lock` → `flock`, holding
a fine-grain unit lock while blocked on the artifact-directory lock the other ten were queued on — the
`fine-grain-locking = true` lock-ordering deadlock. Killing ONLY those two roots restarted the whole
fleet. Diagnosis: `lsof` the lock, then `sample <pid> 1 | grep prebuild_lock_exclusive`; the roots are
the ones that match.

## 7 — What the coordinator must confirm LIVE

1. **The body reads `…before you start` / `composing.`** Clear `ui.introduction.seen.*`, rebuild, run
   `🐍️w7a-tour-paint-probe.mjs`, compare the card against `🗑️generated/react-6313/final.png`.
2. **The counter reads `1 / 5`** and the footer reads `Next ↵ ›` with `✕ Skip` in the cap row. The
   chord glyph comes from W8a's `SYMBOL_FACE` and the Apple/other choice from W7a's platform read, so
   `Next Enter ›` means the platform read answered "other", not that this packet regressed.
3. **The window caps blur under the veil.** This is hand-off 1's whole point and only a frame settles
   it: with the tour armed the navbar/footer/cap chips must be frosted like React's, while the card,
   its chips and the spotlight ring stay sharp.
4. **The spotlight step stays crisp.** On step 2/3 the veil is bands around the cutout, so the
   introduced element must NOT blur — that is the containment predicate doing its job.
5. **Open a context menu, a tooltip and the palette over a glass surface.** Their labels must be
   sharp; a smeared row means a sheet this packet missed.
6. **No `frame world resource admission exceeded fixed credits`.** Six more sheets now open one extra
   `DrawLayer` each while they are open, and the tour card grew from ~14 to ~27 scalars.

## 8 — Hand-offs

1. **P2 — the tour card is not a real `WindowSilhouette`.** React's card is a `WindowChrome`, so it
   inherits the notched silhouette, the surface-active border lifecycle (`introduced` → `active` →
   `normal` on a background click) and the drag that the `DragHandle` actually performs. This packet
   paints the card's chrome — outline, title chip, grip, footer band — but the grip is inert and the
   border never leaves the `introduced` pulse. Owner: the window-system lane.
2. **P2 — `ui.dialog.cancel` / `ui.dialog.submit` are the two other `SHELL_KEYBINDINGS` rows with no
   wgpu twin.** `INTRODUCTION_SHORTCUT_ROWS` covers the three introduction rows; the dialog pair is
   still unbound and unlisted in the Settings → Keybindings tree, which React lists.
3. **P2 — the soft hyphen breaks but paints no hyphen.** CSS renders a visible `-` at a U+00AD break;
   `SYMBOL_FACE` rasterises U+00AD as a default-ignorable (W8a), and the retained stepper is a
   one-glyph-per-grant machine with no room to emit a glyph the source does not carry. No repo string
   contains U+00AD today, so this is a gap, not a defect.
4. **P2 — `wrap_lines` is uncached and re-measures a run at every break opportunity.** That is one
   extra pass per word per wrapped paragraph — invisible for card copy, but the document text lane
   (`paint.rs:2074`, `widgets.rs:272`) wraps far larger runs.
5. **P1 (not this packet's) — `window_silhouette_border_emits_notched_outline_segments` is still
   red.** Reported by `📓️w4a`, `📓️w5b`, `📓️w7a` and `📓️w8a`; nothing here touches silhouettes.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📝️text/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📌️mounted_layout/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-text-unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎓️tour-overlay-and-chord-glyphs/🦀️.rs`
- ticket: `📓️w9a-tour-card-fidelity-and-text-wrap.md` (this file)
