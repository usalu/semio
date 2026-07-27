# Diagnosis D4 — Paragraph spacing (2.6) & Framed-component system (2.14)

Read-only diagnosis. Source paths are absolute. Line numbers are current-tree
(branch `kinan/bericht-diff`; `print/tex/semio-table.sty` has uncommitted
working changes — noted where relevant). The compiled PDF could not be
rasterised in this environment (no poppler); findings are traced from source,
which fully determines the mechanisms below.

---

## Issue 2.6 — Paragraph spacing

### 2.6 (1) Root cause / current state

A document-level `\parskip` **already exists** and is correctly scoped — this
issue is NOT "no paragraph gap", it is "the gap leaks into a handful of compact
contexts that still lack a local reset."

- Document-level setting, applied late enough to win over the class/KOMA:
  `E:\semio\print\tex\semio-window.sty:1749-1754`
  ```
  \AfterEndPreamble {%
    \setlength { \semio@block@sep@skip } { \semio@spacing@single } % 0.2em
    \setlength { \parskip }  { \semio@spacing@par }               % 0.5em
    \setlength { \parindent } { 0pt }
  ```
- Spacing tokens:
  - `\semio@spacing@par` = **0.5em** — `E:\semio\print\tex\semio-window.sty:31`
    (fallback dup at `E:\semio\print\tex\semio-table.sty:14`)
  - `\semio@spacing@single` / `\semio@spacing@unit` = **0.2em** — `E:\semio\print\tex\semio-tokens.sty:50-51`
  - `\semio@spacing@double` = 0.4em — `E:\semio\print\tex\semio-tokens.sty:52`
- Block/paragraph junction design (single choke point, max-merging `\addvspace`):
  `E:\semio\print\tex\semio-core.sty:591-605`
  (`\semio@block@sep@skip`, `\semio@block@before`, `\semio@noindent@noparskip`).
  Paragraph↔paragraph = `\parskip` (0.5em); block↔paragraph and block↔block =
  `max(\parskip, 0.2em)` via `\addvspace`. So body text is consistent at 0.5em.
- The class does **not** touch `\parskip`/`\parindent`/lists
  (`E:\semio\print\tex\zukunftbau.cls` — no matches), so line 1753 is authoritative.

**The reason the reviewer still saw "not consistent"**: the reviewed PDF is
`zwischenbericht(3).pdf`, an earlier build. The 0.5em document `\parskip` is
present in the current source, but the *compact-context guarding* the spec
demands is only **partially** in place: tables are guarded, header/caption rows
are guarded, but **lists and the bibliography are not**, so those contexts
inflate (and, per issue 2.17, the bibliography inflation is a contributing
cause of the sparse final page).

**Already-correct guards (verified):**
- Table cells — `\semio@table@setup` resets `\parskip` to `\z@`:
  `E:\semio\print\tex\semio-table.sty:154`. Reached by BOTH short tables
  (`\semio@table@render` :381, `\SemioTableBegin` :630 → `\semio@table@setup`)
  and long tables (`\semio@table@setup@long` :180 calls `\semio@table@setup`). ✓
- Title-bar / caption chip rows — built inside `\sbox`/`\hbox` (restricted
  horizontal mode, `\parskip` inert) and additionally wrapped by
  `\semio@noindent@noparskip` (`E:\semio\print\tex\semio-core.sty:605`) in
  `\semio@heading@row@wrap` (`...\semio-window.sty:2274`),
  `\semio@window@header@row@wrap` (`:2611`), and
  `\semio_window_header_muted_use:` (`:1131`). ✓

**Leaks that still need a local reset (root cause of 2.6 residue):**
1. **Lists** — `itemize` / `enumerate` / `description` have NO `\parskip`
   reset anywhere. LaTeX's `\list` does not zero `\parskip`; with `\parskip`
   = 0.5em, the inter-item glue becomes `max(\parskip, \itemsep)` ≈ 0.5em
   instead of the class `\itemsep` (~0.28em), so lists render looser than
   surrounding body text. Body actually uses one: `enumerate` at
   `E:\semio\mit-bestand\bericht\zwischenbericht\zwischenbericht.tex:367-371`.
2. **Bibliography** — `\defbibenvironment{semioreferences}` sets `\itemsep`
   and `\parsep` but **not** `\parskip`:
   `E:\semio\print\tex\semio-window.sty:1884-1894` (options block :1887-1893).
   Bib entries therefore inherit 0.5em, inflating the list (feeds 2.17).
3. **Title-page / cover blocks** — the `titlepage` opened by `\makecoverpages`
   (`E:\semio\print\tex\semio-components.sty:119`) and the address helper
   `\semio_cover_address:n` (`:29-36`, sets `\parindent` only) do not reset
   `\parskip`. Low visual risk (values are single-line or `\\`-separated, not
   `\par`-separated), but the spec explicitly lists title-page blocks.
4. **Panels / other minipage chrome** — `Panel`
   (`E:\semio\print\tex\semio-window.sty:1704`, minipage at `:1670`) inherits
   `\parskip`; low risk but unguarded.
5. **Header/footer chrome** — navbar/footer builders run in the fancyhdr
   head/foot boxes; `\parskip` is practically inert there (box-built), but the
   spec lists headers/footers, so a defensive reset belongs in the chrome
   builder for safety.

### 2.6 (2) Concrete fix (respects §4: no manual per-paragraph `\vspace`, no negatives)

Keep the existing shared document-level `\parskip = \semio@spacing@par`
(`...semio-window.sty:1753`) — it is the correct "shared setting". Add **local
resets** only, via environment hooks and the two existing shared list macros:

- Lists: add `\setlength{\parskip}{\z@}` at the start of the standard list
  environments through the LaTeX hook system, e.g.
  `\AddToHook{env/itemize/begin}{\setlength{\parskip}{\z@}}` and the same for
  `enumerate` and `description` (place next to the `\AfterEndPreamble` block in
  `...semio-window.sty:1749`). A hook (not a redefinition) keeps it shared and
  non-invasive.
- Bibliography: add `\setlength{\parskip}{\z@}` inside the options group of
  `\defbibenvironment{semioreferences}` (`...semio-window.sty:1887-1893`),
  alongside the existing `\parsep`/`\itemsep` sets. Doubles as part of 2.17.
- Cover / title-page: add a scoped `\setlength{\parskip}{\z@}` inside the
  `titlepage` in `\makecoverpages` (`...semio-components.sty:119`) and in
  `\semio_cover_address:n` (`:33`, beside the existing `\parindent` set).
- Panel / chrome: add the same one-line reset inside `\semio_panel_begin:n`
  (`...semio-window.sty:1670` minipage) and the navbar/footer builders — cheap,
  defensive, matches the spec's enumerated contexts.

Do NOT lower the global value; do NOT add per-paragraph spacing.

### 2.6 (3) Full list of compact environments needing a `\parskip` reset

| Context | Location | Status |
|---|---|---|
| Table cells (short + long) | `semio-table.sty:154` (`\semio@table@setup`) | **already reset** ✓ |
| Title-bar / caption chip rows | `semio-core.sty:605` via `semio-window.sty:1131,2274,2611` | **already guarded** ✓ (restricted mode + `\semio@noindent@noparskip`) |
| `itemize` | LaTeX list; no reset | **NEEDS reset** |
| `enumerate` | LaTeX list; used `zwischenbericht.tex:367` | **NEEDS reset** |
| `description` | LaTeX list; no reset | **NEEDS reset** |
| Bibliography (`semioreferences`) | `semio-window.sty:1884-1894` | **NEEDS reset** (also 2.17) |
| Cover / `titlepage` block | `semio-components.sty:119` | **NEEDS reset** (low risk) |
| Cover address helper | `semio-components.sty:29-36` | **NEEDS reset** (low risk) |
| `Panel` (minipage) | `semio-window.sty:1670,1704` | **NEEDS reset** (low risk) |
| Navbar / footer chrome | `semio-components.sty:1717-1747` builders | defensive reset (very low risk) |

Note: `SemioNest` (`semio-window.sty:2572`), `Window`, `Blockquote`, `Epigraph`
are NOT compact contexts — they hold ordinary body paragraphs and should keep
the 0.5em `\parskip`. Do not reset those.

---

## Issue 2.14 — Framed-component system

### 2.14 (1) Audit — one system or divergent?

**Verdict: it is mostly ONE system, unified by two shared tokens, with two real
divergences and one fragile join.** Inner padding and border width are already
single-sourced:

- Inner-padding token `\semio@chrome@padding` = **0.2em** —
  `E:\semio\print\tex\semio-tokens.sty:58` (and `...semio-window.sty:20`).
- Border-width token `\semio@stroke@hairline` = **0.75pt** —
  `E:\semio\print\tex\semio-tokens.sty:53`.
  (`\semio@stroke@default` 1.5pt / `@focus` 2.25pt exist but are NOT used by any
  frame — so border thickness is genuinely consistent across all frames.)

Every visible framed component maps to one of two shared engines:

1. **Box body — the `semio~window` tcolorbox style**:
   `E:\semio\print\tex\semio-window.sty:712-743`.
   `boxrule=\semio@stroke@hairline`, `boxsep=0pt`,
   `top/left/right/bottom=\semio@chrome@padding`, `colframe`/`colback` chrome
   colours, `before/after skip=\semio@spacing@table@outer` (0.85em).
   Variants: `semio~window~table` (`:744-752`, zero inner pad + custom left
   border `finish`), `semio~window~row` (`:753-756`), `image~cover`
   (`:757-764`). Used by every `Window`/kind window, cover cards, logo panels,
   quote boxes, Figure/Image/Table via `\semio_window_kind_define:nnnnn`
   (`:1251-1262`) and the cover `\begin{Window}[...]` cards
   (`...semio-components.sty:131-275`). **All one path.** ✓

2. **Title-tab — the cap engine** (`\semio@window@cap*` / `\semio@heading@cap*`):
   `...semio-window.sty:2054-2263`. Horizontal pad = `\semio@chrome@padding`
   hspace (`:2149,2211,2236`); vertical height = `\semio@window@cap@metrics@set`
   (`:2075-2086`); border strokes `\semio@window@stroke@v/@h` at hairline
   (`:2088-2096`). Drives window title bars (`\semio@window@header@muted` :2618),
   section/subsection heading bars (`\semio@heading@pair`/`@outline`
   :2281-2312), and nested-paragraph chips (`\semio@heading@pair@nofill@num`
   :2314). **All one path.** ✓

**Divergence A — the long-table title bar is a parallel renderer.**
On this branch it was rebuilt (uncommitted working diff) from the cap engine
into a native bordered `\multicolumn` table row:
`E:\semio\print\tex\semio-table.sty:265-277`
```
\multicolumn{#1}{|@{\hspace{\semio@chrome@padding}}m{...\semio@table@long@inner@w...}@{\hspace{\semio@chrome@padding}}|}{%
  \cellcolor{semio-chrome-canvas}%
  \rule[-0.5\semio@table@title@rowh]{0pt}{\semio@table@title@rowh}%
  {\semio@heading@bar@font ... }\hfill{...}%
}\\ \hhline{|*{#1}{-}|}
```
It is **reconciled on tokens** — it uses `\semio@chrome@padding`,
`\semio@stroke@hairline` (via the tabular `\arrayrulewidth`, `...table.sty:146`),
`\semio@heading@bar@font` (14pt), and, crucially, its row height
`\semio@table@title@rowh` is taken from the cap chip's own natural height
(`...semio-table.sty:245`, boxing `\semio@heading@cap@muted`). But it is a
**second, independently maintained code path** for "a title tab with balanced
padding", so it can drift from the cap engine on any future edit.

**Divergence B — the top border is delegated to the chip row via a magic nudge.**
The window box sets `toprule=0pt` (`...semio-window.sty:715` style, top rule
absent); the visible top edge is the title-chip row's own baseline stroke,
welded on by `\semio_window_vskip_stroke_hairline:` =
`\vskip -3\semio@stroke@hairline` (`...semio-window.sty:344-349`), invoked from
`\semio_window_kind_begin:` (`:1208`) and `\semio_window_generic_begin:`
(`:1820`). The `-3×hairline` is a hand-tuned overlap constant, not a structural
join — the single most fragile point in the "complete border / cleanly
connected tab" requirement. If chip height, padding, or hairline changes, the
tab/box seam opens or overlaps.

**Minor — cap vertical metric is a 3-unit total.**
`\semio@window@cap@metrics@set` sets titlebar height = `slot(h+d) + 3×padding`
(`...semio-window.sty:2076-2079`), i.e. 1.5×padding of air per side (centred),
whereas the box interior uses 1×padding per side. Title-bar air is therefore
slightly larger than body air. This overlaps issues 2.3/2.10 (title-bar
padding) owned by other agents — flag, do not double-fix.

**Aligned adjacent boxes (cover):** cover cards are laid out by
`WindowArrangement`/`WindowRow`/`WindowColumn` with a shared gap
`\semio@window@gap` (set to 4pt on the cover, `...semio-components.sty:121`) and
weight-based equal heights (`\semio_window_weights_to_dims:nn`,
`...semio-window.sty:936`). Row items share height; each title tab is sized to
its own title text (tabs are per-title width by design), and the connecting gap
stroke fills to `\linewidth` (`\semio@window@header@gap@paint`,
`...semio-window.sty:2585`). No alignment defect in the shared macro.

### 2.14 (2) Concrete fix (respects §4: fix shared macros, no per-box, no absolute positioning, no margin changes)

1. **Keep the two shared tokens as the single source of truth** and require
   every frame to reference them — which they already do. No new per-component
   padding/stroke constants. `\semio@chrome@padding` (inner margin) and
   `\semio@stroke@hairline` (border) stay the only knobs.
2. **Bind both title-bar renderers to the cap engine's height.** The long-table
   row already derives `\semio@table@title@rowh` from the cap box height
   (`...semio-table.sty:245`) and uses `\semio@chrome@padding` for horizontal
   air — preserve that coupling and document it as the intended consolidation so
   the two paths cannot drift. (No structural rewrite of the tabular path is
   needed; it was rebuilt to satisfy 2.2/2.3 and is token-reconciled.)
3. **Replace the fragile top-edge join.** Express the tab↔box overlap in
   `\semio_window_vskip_stroke_hairline:` (`...semio-window.sty:344-349`) in
   terms of the actual chip baseline-stroke thickness (one hairline) rather than
   the magic `3×` multiplier, or give the `semio~window` style its own drawn top
   edge and let the chip sit flush on it — a change confined to the shared
   tcbset (`:712-743`) + that one macro. This is the only change that improves
   "complete borders / cleanly connected title tabs" without per-box edits.

### 2.14 (4) Consolidation plan — which macros, which shared params

| Concern | Shared param (keep single-sourced) | Shared macro(s) to touch |
|---|---|---|
| Inner margin | `\semio@chrome@padding` (0.2em, `tokens:58`) | `semio~window` tcbset `top/left/right/bottom` (`window:727-730`); cap hspace (`window:2149,2211,2236`); long-table cell pad (`table.sty:177,266`) — all already reference it, keep as-is |
| Border width | `\semio@stroke@hairline` (0.75pt, `tokens:53`) | `semio~window` `boxrule` (`window:721`); cap strokes (`window:2088-2096`); table `\arrayrulewidth` (`table.sty:146`) — already unified |
| Title-bar height | `\semio@window@cap@titleh` via `\semio@window@cap@metrics@set` (`window:2075`) | long-table row via `\semio@table@title@rowh` (`table.sty:245`) — keep coupled |
| Tab↔box top join | (new) baseline-stroke = one hairline | `\semio_window_vskip_stroke_hairline:` (`window:344`) + `semio~window` `toprule` (`window:715` region) |

Net: the system does **not** need to be re-unified from scratch — it is already
a shared cap+box system on two tokens. The residual work is (a) documenting the
long-table-row↔cap-height coupling so it can't drift, and (b) making the top
seam structural instead of a `-3\hairline` constant.

### 2.14 (5) Risk notes + shared macros to change

- **Blast radius (high):** `\semio_window_vskip_stroke_hairline:` and the
  `semio~window` tcbset feed EVERY window — cover cards (pages 1-2), every
  project card (pages 23-49), Figure/Image/Table captions, Blockquotes. Any
  top-edge change must be re-checked on the cover, a project card, and a plain
  table.
- **In-flight coordination:** the long-table title row is an uncommitted working
  change in `semio-table.sty` (2.2/2.3). Do not re-refactor it for 2.14; only
  keep its token/height coupling.
- **Overlap with other agents:** the cap `3×padding` vertical metric
  (`window:2076-2079`) is really a 2.3/2.10 (title-bar padding) knob — flag, do
  not double-fix from D4.
- **2.6 ↔ 2.17 interaction:** the bibliography `\parskip` reset (2.6) also
  reduces the bib list's height and helps the sparse-final-page issue (2.17) —
  intended, but verify the last entry lands as 2.17 expects.
- **2.6 hook ordering:** add list/env `\parskip` resets after the
  `\AfterEndPreamble` block so they are not overwritten; environment hooks fire
  per-`\begin`, so they win locally regardless.

**Files / shared macros to change**
- `E:\semio\print\tex\semio-window.sty`
  - keep `\AfterEndPreamble` `\parskip` (`:1753`); add `env/itemize|enumerate|description/begin` `\parskip` resets near `:1749` (2.6)
  - `\defbibenvironment{semioreferences}` add `\parskip=\z@` at `:1887-1893` (2.6/2.17)
  - `Panel`/`\semio_panel_begin:n` `:1670` add reset (2.6)
  - `\semio_window_vskip_stroke_hairline:` `:344` + `semio~window` tcbset `:712-743` top edge (2.14)
- `E:\semio\print\tex\semio-components.sty`
  - `titlepage` in `\makecoverpages` `:119` and `\semio_cover_address:n` `:33` add reset (2.6)
  - navbar/footer builders `:1717-1747` defensive reset (2.6)
- `E:\semio\print\tex\semio-table.sty`
  - no structural change; keep title-row↔cap-height coupling (`:245`) and
    `\semio@chrome@padding`/hairline references (2.14). Table-cell `\parskip`
    reset already present (`:154`).

---

## Summary of root causes

- **2.6:** A shared document-level `\parskip = 0.5em` already exists
  (`semio-window.sty:1753`) and body text + tables + caption rows are correctly
  handled. The residual defect is that a set of **compact contexts still lack a
  local `\parskip` reset** and therefore inflate: `itemize`/`enumerate`/
  `description`, the `semioreferences` bibliography list
  (`semio-window.sty:1884`), the cover/title-page blocks, `Panel`, and
  (defensively) header/footer chrome. Fix = keep the global value, add one-line
  local resets in those environments.
- **2.14:** The framed components are **already one system** built on two
  single-sourced tokens — `\semio@chrome@padding` (0.2em) and
  `\semio@stroke@hairline` (0.75pt) — plus one shared cap/title engine and the
  `semio~window` tcolorbox style; cover cards, logo/metadata/quote boxes,
  project cards and captions all use it. The real divergences are (a) the
  **long-table title bar is a parallel renderer** (reconciled by tokens/height
  but maintained separately, `semio-table.sty:265`), and (b) the **window top
  border is welded on by a magic `\vskip -3\semio@stroke@hairline`**
  (`semio-window.sty:344`) instead of a structural join — the one fragile point
  behind "complete borders / cleanly connected tabs." Consolidation = keep the
  two tokens as the only knobs, document the long-table↔cap-height coupling, and
  make the top seam structural.

Report path:
`E:\semio\.repo\🎫\26\07\06\MIT-BESTAND-ZWISCHENBERICHT-LAYOUT-AND-RENDERING-PASS\diagnosis-D4-paragraphs-framed.md`
