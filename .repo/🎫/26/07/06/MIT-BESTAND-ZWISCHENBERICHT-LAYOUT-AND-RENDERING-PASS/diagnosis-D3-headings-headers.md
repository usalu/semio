# Diagnosis D3 — Headings & Running Headers (issues 2.8, 2.9, 2.10)

READ-ONLY diagnosis. No source was edited.

## Include chain (traced)

`zwischenbericht.tex` → `\documentclass[type=zwischenbericht]{zukunftbau}`
→ `zukunftbau.cls:22` `\LoadClass[...type=paper...]{semio}` + `\SemioSetup{appendix-level=part}`
→ `semio.cls:40` `\LoadClass[twoside=true]{scrartcl}`; then `semio.cls:47-79` requires, in order,
`semio-tokens`, `semio-fonts`, `semio-core`, `semio-logo`, `semio-table`, `semio-window`, `semio-components`.

All heading-bar and running-header machinery lives in **`E:\semio\print\tex\semio-window.sty`**.
The appendix (pages 53-61) reuses the same `\section`-level heading path via `semio-components.sty`
(`appendix-level=part`, so appendix entries are ordinary `\section`s inside `\part`s).

The page style is a single **centered** fancyhdr band, not classic L/C/R marks:
`semio-window.sty:1717-1725` defines every page style (`fancy`, `plain`, `scrplain`, `scrheadings`)
with `\fancyhead[C]{\SemioChromeNavbarHead}` and `\fancyfoot[C]{\SemioChromeFooterHead}`.
The navbar itself paints three internal zones (left emblem/project, center report-type, right = running section).

---

## Issue 2.8 — Running-header timing (header advances before the section begins)

### Root cause
The running section name is stored in an ordinary global macro set with `\gdef`, **not** with TeX's
mark mechanism, so its value at ship-out time is "whatever was last assigned during token processing,"
which runs ahead of the actual page break.

- `semio-window.sty:2032-2034`
  ```
  \newcommand{\semio@chrome@heading@set}[1]{%
    \gdef\semio@chrome@heading{#1}%
  }
  ```
- `semio-window.sty:2036-2052` `\semio@chrome@heading@track` calls `\semio@chrome@heading@set{#2}`
  for part / chapter / section levels.
- It fires from the heading formatter: `semio-window.sty:2382`
  `\semio@chrome@heading@track{#1}{#3}` inside `\semio@koma@heading@lines`
  (installed as `\sectionlinesformat` at `2450-2452`, and analogously for part/chapter at `2456`/`2467`).
- The navbar reads that global directly: `semio-window.sty:3010-3013`
  ```
  \if\relax\detokenize{\semio@chrome@heading}\relax\else
    \semio@chrome@navbar@chip@text{\semio@chrome@font@chip}{\semio@chrome@heading}%
  \fi
  ```

Because `\gdef` takes effect the instant the heading command is *tokenized* in the main vertical list,
and TeX's page builder / output routine fires asynchronously (reading ahead of the break), a page whose
break lands *before* a heading can still ship carrying the *next* section's name. Standard LaTeX avoids
this precisely by using `\mark`/`\leftmark`/`\rightmark` (or the l3 `\InsertMark`/`\FirstMark`), whose
values are extracted positionally from the material that actually made it onto the shipped page. This
code bypasses that machinery entirely — hence "Depot-Shops" / "Ressourcenkataloge" / "Leseschlüssel"
appearing one page early (spec pages 54, 56, 58). The interaction is aggravated by the
`\Needspace*{...}` at `2378` (which can push the heading to the next page while the `\gdef` at `2382`
has already run), but the `\gdef`-vs-`\mark` design is the true cause; even without Needspace the
asynchronous page builder makes a raw global unreliable for running heads.

Note the appendix path is the *same* code — appendix section titles route through `\sectionlinesformat`
→ `\semio@koma@heading@lines` → `\semio@chrome@heading@track`, so no separate appendix mark macro exists
to patch; fixing the shared macro fixes the appendix too. (Frontmatter/mainmatter also poke the global
directly: `semio-components.sty:41` and `:68` call `\semio@chrome@heading@set`.)

### Concrete fix (respects §4 — no per-page header text, no manual spacing)
Convert the running head from a global macro to a real mark:

1. Replace the storage. Define a mark class once (l3 API, matches the codebase style):
   `\NewMarkClass{semioheading}` (or classic `\newcommand` around `\markright`).
2. In `\semio@chrome@heading@track` (2036-2052) replace each `\semio@chrome@heading@set{#2}`
   with `\InsertMark{semioheading}{#2}` — keep the identical part/chapter/section level guard.
   Also replace the direct calls at `semio-components.sty:41` and `:68` with the same insert
   (or an insert of an empty mark to reset).
3. In `\SemioChromeNavbarHead` (3010-3013) read the mark instead of the macro:
   use `\FirstMark{semioheading}` (l3) or `\rightmark` (if using `\markright`). `\FirstMark`
   gives "the section in effect at the top of the page," which is exactly the desired
   "update only when the visible section begins" semantics and makes an early advance impossible,
   because the mark travels with the heading box.
4. Keep `\semio@chrome@heading` defined as an empty fallback (`2 974` `\def\semio@chrome@heading{}`)
   only if a non-mark call site remains; otherwise remove it.

Conservative fallback if the modern marks API is unavailable in the build's LaTeX: issue
`\markright{#2}` in `\semio@chrome@heading@track` and read `\rightmark` in the navbar — fancyhdr
is already loaded, so `\rightmark` is available and positionally correct.

### Risk notes
- `\semio@chrome@heading` is read only at `3010-3013`; the swap is localized.
- Marks must be inserted at the same vertical position the heading occupies — inserting inside
  `\semio@koma@heading@lines` (as the `\gdef` is today) is correct; do not move it before `\Needspace`.
- No content, numbering or title text changes.

### Shared macros to change
`\semio@chrome@heading@set`, `\semio@chrome@heading@track` (semio-window.sty:2032-2052),
`\SemioChromeNavbarHead` (3000-3016), and the two poke sites in semio-components.sty (41, 68).

---

## Issue 2.9 — Long running-header overlaps the centered "Zwischenbericht" box (pages 60-61)

### Root cause
The three navbar fields are laid out as **two independent full-width boxes overlaid at the same
origin**, with no width partitioning and no max width on the running-section chip. When the right
field is long ("Typologische Zuordnung und Wiederverwendungsrollen") its right-flushed chip extends
back to the horizontal center, where the centered report-type chip sits, and they collide.

- `semio-window.sty:2972-2998` `\semio@chrome@bar@threezone{#1height}{#2edge}{#3center}{#4other}`:
  ```
  \sbox\semio@chrome@bar@edge@box{\vbox to #1{... \hbox to \headwidth{%
        \Ifsemiochromemirrored{#4\hfil#2}{#2\hfil#4}}...}}%   % edge box: full headwidth
  \sbox\semio@chrome@bar@center@box{\vbox to #1{... \hbox to \headwidth{\hfil #3 \hfil}...}}%  % center box: full headwidth
  \hbox to \headwidth{%
    \usebox\semio@chrome@bar@edge@box
    \hskip-\wd\semio@chrome@bar@edge@box   % back up to origin
    \usebox\semio@chrome@bar@center@box}%  % overlaid on top
  ```
  Both boxes are `\hbox to \headwidth`. The edge box puts `#2` (emblem+project) flush left and `#4`
  (the running-section chip, on recto) flush right, separated only by `\hfil`. The center box centers
  `#3` ("Zwischenbericht"). They are drawn on top of each other with no reserved gutter, so the only
  thing keeping them apart is that the two `\hfil`-driven layouts *happen* not to reach the middle —
  which fails as soon as the right chip is wide.
- The running-section chip is set with no width limit: `\SemioChromeNavbarHead` (3009-3013) →
  `\semio@chrome@navbar@chip@text` (2955-2961) → `\semio@chrome@chip` (2831-2837) →
  `\fcolorbox{...}{...}{#1}` with the text in a single unbounded `\hbox` (`\semio@chrome@vcenter`
  2792-2794 wraps only vertically). Nothing clips, wraps, shrinks or truncates it.

### Concrete fix (respects §4 — keep visible title unchanged, no absolute positioning)
Replace the overlay in `\semio@chrome@bar@threezone` (2972-2998) with a true three-column band so
each field owns a bounded zone:

1. Measure the center chip once (`\sbox` #3, read `\wd`). Reserve a center zone of that width plus a
   fixed gutter (e.g. `2\semio@chrome@padding` each side).
2. Split the remaining `\headwidth` into a left and a right zone
   (`(\headwidth - centerzone)/2` each). Lay out
   `\hbox to \headwidth{\hbox to \leftzone{#2\hfil}\hbox to \centerzone{\hfil#3\hfil}\hbox to \rightzone{\hfil#4}}`
   (mirrored for verso via the existing `\Ifsemiochromemirrored`). This *structurally* prevents overlap.
3. Constrain the running-section chip to its zone. Preferred, per spec's allowed remedies:
   - **max width + short mark**: with 2.8 converted to marks, insert a short form alongside the full
     title (e.g. an optional `\SemioSectionShort{...}` that defaults to the full title) and let the
     navbar pick the short form when the full chip exceeds the right zone; or
   - **width-limited chip**: typeset the chip text inside a box of the right-zone width and allow it
     to reduce to the small chip font / ellipsize when it overflows.
   Do not change the visible section heading text (that is a separate render path via
   `\semio@heading@pair`).

### Risk notes
- `\semio@chrome@bar@threezone` is shared by the **footer** (`\SemioChromeFooterHead` 3018-3029:
  logos / authors / page number). The footer fields are short and centered page number is narrow, but
  any zone-splitting rewrite must keep the footer visually identical — test both bands.
- Center-zone measurement must run inside the header (where `\headwidth` is valid); keep it in the
  same `\sbox` pass already present.

### Shared macros to change
`\semio@chrome@bar@threezone` (2972-2998); the chip-width limiting touches
`\semio@chrome@navbar@chip@text` (2955-2961) / `\semio@chrome@chip` (2831-2837); optional short-mark
plumbing ties into the 2.8 mark work in `\SemioChromeNavbarHead` (3000-3016).

---

## Issue 2.10 — Colored/outlined heading-bar padding (text tight against borders)

### Root cause
Two distinct defects in the shared cap (chip) machinery:

**(a) Vertical padding does not scale with the enlarged heading font.**
Heading bars set their text at 14 pt (`semio-window.sty:2181` `\providecommand{\semio@chrome@font@heading}{14pt}`,
applied by `\semio@heading@bar@font` 2182-2184), but the padding token
`\semio@chrome@padding` = `0.2em` (`semio-window.sty:20`) is *evaluated against the ambient body font*,
not the 14 pt heading font, in the height math:
- `\semio@window@cap@body@h@set` (2069-2074) seeds `titleh` from `\semio@chrome@titlebar@height` (`1.4em`, line 19) — again ambient em.
- `\semio@window@cap@metrics@set` (2075-2086):
  ```
  \setlength{\semio@window@cap@titleh}{\dimexpr\ht\slot+\dp\slot
      +\semio@chrome@padding+\semio@chrome@padding+\semio@chrome@padding\relax}%
  ```
  The `3×\semio@chrome@padding` here is `3×0.2em` at the *ambient* size (~11 pt ⇒ ~6.6 pt total),
  added around a 14 pt glyph box. Relative to the larger letters the top/bottom breathing room is
  proportionally small, so the text reads as tight against the borders. The side padding has the same
  origin: `\hspace{\semio@chrome@padding}#1\hspace{\semio@chrome@padding}` inside the slot boxes
  (`\semio@window@cap` 2149, `\semio@heading@cap@nofill@core` 2211, `\semio@heading@cap@muted@core` 2236) —
  0.2em, and evaluated inside the group *after* the font is set there, but still only 0.2em.

**(b) Title box and number box are measured independently, so they can differ in height.**
`\semio@heading@pair` (2281-2296) builds the left title chip and the right number chip as two separate
`\semio@heading@chip`→`\semio@window@cap` calls; each call runs `\semio@window@cap@metrics@set`, which
sets `titleh` from *that box's own* `\ht+\dp`. A title with tall glyphs/umlauts and a number without
get different `titleh`, so the two fields end up unequal height — the spec's "aligned title and number
boxes / equal height" defect. Same pattern in the outline path `\semio@heading@pair@outline`
(2298-2312) and `\semio@heading@pair@nofill@num` (2314-2328).

Vertical centering itself is fine (`\semio@window@cap@paint` 2102-2115 and `@paint@nofill` 2117-2124
use `\vfil…slot…\vfil`), and the box *does* grow from content via `metrics@set`; the problem is the
*amount* of padding and its font reference, plus the per-box (not shared) height.

### Concrete fix (respects §4 — no manual resize, no manual \vspace)
1. **Make heading-bar padding font-aware and larger.** Introduce a dedicated heading-bar padding token
   (e.g. `\semio@heading@bar@pad`, default ~`0.45em`) and evaluate the vertical padding in
   `\semio@window@cap@metrics@set` / `body@h@set` *inside* `\semio@heading@bar@font` so the em resolves
   at 14 pt. Alternatively, express the height math relative to the heading font. Use a symmetric
   `2×pad` for top+bottom (keeping the strut/`\vfil` centering) rather than the current `3×` fudge, and
   apply the same token to the side `\hspace`. This grows the bar automatically and keeps letters clear.
2. **Share one height across title and number.** Measure both the tab slot and the ctrl slot, take the
   max `\ht+\dp`, compute a single `titleh`, then paint both boxes to it — so left and right fields are
   always equal height and aligned. Refactor `\semio@window@cap@metrics@set` to accept/keep a shared
   target, or compute the max in `\semio@heading@pair` / `@outline` / `@nofill@num` before painting.
3. Long-title height growth already works once (1) makes padding font-relative; a genuinely
   multi-line section title would need the slot to become a `\parbox`/`p{}` instead of a single `\hbox`
   (currently `\semio@window@cap@slot` is an `\hbox`), but for section-level titles a single line is
   normal — flag as optional.
4. The "consistent small gap before following text/tables" is governed by
   `\semio@heading@install@spacing` (2407-2417, `afterskip=1sp`) plus `\semio@block@before` /
   `\semio@block@sep@skip` (`= \semio@spacing@single`, 1752). If the after-bar gap reads as
   inconsistent, normalize it there — do **not** add manual `\vspace`.

### Risk notes — SHARED CHROME, coordinate with the table-title-bar agent (issues 2.2/2.3)
These macros are used by **both** section heading bars and **table title bars**:
- `semio-table.sty:241` `\semio@heading@cap@muted{...}` and `:252` `\semio@heading@cap@muted{number}` —
  the long-table title row is literally a `\semio@heading@cap@muted` chip, routing through
  `\semio@heading@cap@muted@core` → `\semio@window@cap@metrics@set`.
- `semio-table.sty:269-272` uses `\semio@heading@bar@font`; and `\semio@chrome@padding` appears
  throughout the table column spec (`:173,177,216,266,436,442,546`).

Therefore **do not simply bump `\semio@chrome@padding`** (line 20) — it is global chrome used by the
navbar chips, footer chips, panels, table cells and table title bars; enlarging it reflows every table
and every chip. Instead add a heading-bar-specific padding token and/or make `metrics@set` font-aware,
and decide *with the table agent* whether the table title bar should adopt the same increased,
font-relative padding (it likely should, for spec 2.3's "balanced vertical padding / equal height").
If `\semio@window@cap@metrics@set` is refactored to a shared-height model, verify the table title/number
pairing (`\semio@window@tab@box` / `\semio@window@ctrl@box`, reused in semio-table.sty) still aligns.

### Shared macros to change
`\semio@chrome@padding` (20 — leave as-is; add a new token instead),
`\semio@window@cap@body@h@set` (2069-2074), `\semio@window@cap@metrics@set` (2075-2086),
`\semio@window@cap` (2144-2159), `\semio@heading@cap@nofill@core` (2206-2217),
`\semio@heading@cap@muted@core` (2231-2242), and the pairing macros
`\semio@heading@pair` (2281-2296) / `@outline` (2298-2312) / `@nofill@num` (2314-2328) for shared height.

---

## Cross-issue shared-macro summary

| Macro (semio-window.sty) | 2.8 | 2.9 | 2.10 | Also used by tables? |
|---|---|---|---|---|
| `\semio@chrome@heading@set` / `@track` (2032-2052) | ✔ | | | no |
| `\SemioChromeNavbarHead` (3000-3016) | ✔ (read mark) | ✔ (short mark) | | no |
| `\semio@chrome@bar@threezone` (2972-2998) | | ✔ | | footer (shared band) |
| `\semio@chrome@navbar@chip@text` / `\semio@chrome@chip` (2955-2961 / 2831-2837) | | ✔ | | footer/panel chips |
| `\semio@chrome@padding` (20) | | | ⚠ leave | tables, navbar, footer, panels |
| `\semio@window@cap@metrics@set` / `body@h@set` (2069-2086) | | | ✔ | **tables (cap@muted)** |
| `\semio@window@cap` + cap cores (2144-2242) | | | ✔ | **tables (cap@muted)** |
| `\semio@heading@pair` / `@outline` / `@nofill@num` (2281-2328) | | | ✔ (shared height) | tab/ctrl boxes shared |

Priorities per spec §3: 2.8 (item 5) and 2.9 (item 6) are the "running-header" pair; 2.10 belongs with
the padding cluster (items 3). 2.8 is a self-contained mark swap and the safest to land first; 2.10 must
be coordinated with the table-title-bar agent because it touches `metrics@set`/`cap@muted`.

Note: the PDF could not be rasterized in this environment (`pdftoppm`/poppler not installed), so page
images were not inspected directly; symptoms are confirmed from the source render paths and match the
spec's page-by-page descriptions (pp. 54/56/58 timing, 60-61 overlap).
