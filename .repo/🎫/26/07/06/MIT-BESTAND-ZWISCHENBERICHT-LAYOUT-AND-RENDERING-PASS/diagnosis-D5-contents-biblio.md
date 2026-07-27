# Diagnosis D5 — Contents escapes & bibliography

Scope: issues **2.15** (visible LaTeX escape in contents) and **2.17** (near-empty
final bibliography page). READ-ONLY diagnosis; no source was edited.

Environment note: PDF page rendering was unavailable in this session (poppler /
`pdftoppm` not installed, no `pypdf`), so the two symptoms were confirmed by
tracing the source token path rather than by rasterising the PDF. Both root
causes are unambiguous at source level; the offending titles are literally
present in the source with `\&`, and the write-path that carries them to the
contents detokenizes them.

---

## Issue 2.15 — Visible `\&` in the contents/register

### Symptom
The on-page contents (the "Inhalt" register table, PDF pages ~2–6) shows the
project titles with a literal backslash:

- `ELYS Kultur- \& Gewerbehaus`
- `Melkinlaituri School \& Day-care`

### Where the titles are authored
Both are appendix `\subsection` headings, written with a correctly-escaped
ampersand (valid in body text):

- `E:\semio\mit-bestand\bericht\zwischenbericht\anhang\projekte.tex:605`
  `\subsection{ELYS Kultur- \& Gewerbehaus}`
- `E:\semio\mit-bestand\bericht\zwischenbericht\anhang\projekte.tex:869`
  `\subsection{Melkinlaituri School \& Day-care}`

The authoring is *correct*: `\&` is the proper way to typeset an ampersand in
running LaTeX text. The defect is entirely in the contents/register write-path,
not in the titles.

### Root cause — exact token path
semio does not use LaTeX's normal `.toc` typesetting for its "Inhalt" register.
It captures each heading title, extracts the plain title tokens, then stores the
title **as a detokenized string** in a sequence that is later typeset into the
register longtable. Detokenizing turns the single control sequence `\&` into the
two printable category-12 characters `\` + `&`, so the backslash becomes visible.

File: `E:\semio\print\tex\semio-window.sty`

1. Heading titles enter via `\semio_toc_track_heading:nn/:nnn`
   (`semio-window.sty:308` and `:312`) → `\semio_window_toc_append:nnn`
   (`:275`).
2. `\semio_window_toc_append:nnn` calls `\semio_window_toc_title_plain:n {#3}`
   (`:278`) which extracts the visible title into `\l_tmpa_tl`
   (definition `\semio_window_toc_title_plain:n` at `:242`). At this point
   `\l_tmpa_tl` still holds the *real* control sequence `\&`.
3. The title is then stored and written with `\tl_to_str:N` — this is the
   detokenization that produces the visible backslash. Three sites:
   - `semio-window.sty:285`
     `\seq_gput_right:Nx \g_semio_register_toc_title_seq { \tl_to_str:N \l_tmpa_tl }`
     (in-memory register sequence)
   - `semio-window.sty:293`
     `{ \tl_to_str:N \l_tmpa_tl }` inside `\SemioTocAuxLine`, written to the
     `\jobname.register-toc` aux file via `\iow_now` at `:296`
   - `semio-window.sty:334`
     `\seq_gput_right:Nx \g_semio_register_toc_title_seq { \tl_to_str:N \l_tmpa_tl }`
     in `\semio_window_toc_aux_line:nnn` (`:327`), which re-reads the aux file on
     the next run.

`\tl_to_str` on `\&` yields the escape char (`\`) followed by `&`, both catcode
12. The `&` is harmless (catcode-12, not an alignment tab, so it renders as an
ampersand glyph inside the longtable cell), but the leading `\` is printed
verbatim — hence `\&` on screen.

Why detokenizing is used at all: it neutralizes fragile/expandable tokens and a
raw catcode-4 `&` before the title is (a) written to an aux file and (b) dropped
into a `longtable` cell, where an un-escaped `&` would otherwise be read as a
column separator. So the mechanism is deliberate; it just fails to translate
LaTeX escape sequences back to their glyphs.

Note on PDF bookmarks: the actual PDF outline is produced by hyperref's
`\pdfstringdef`, which already knows `\&` and emits a correct `&`. The visible
backslash is specific to the semio register-toc path above, not to the bookmarks.
So the fix should target the register write-path; bookmarks need no change.

### Proposed fix (respects §2.15 rule and §4 "no content rewriting")
Do **not** change the titles to a bare `&` (that would be invalid in the normal
body text where `\subsection{...}` also typesets the heading). Instead, fix the
write-path so the detokenized title has its LaTeX escapes converted back to
glyphs.

`\semio_window_toc_title_plain:n` (`semio-window.sty:242`) is the single choke
point: every one of the three storage sites above is reached only *after* a call
to this function (site 285/293 via `:278`, site 334 via `:329`). Normalise the
title there, once, at the end of the macro (after the existing trims, i.e. after
`semio-window.sty:271`):

```latex
% inside \semio_window_toc_title_plain:n, after the Hy@SectionAnchorHref guard
\str_set:Nx \l_tmpa_str { \tl_to_str:N \l_tmpa_tl }
\str_replace_all:Nnn \l_tmpa_str { \c_backslash_str & } { & }
% (optionally also \% \_ \# \$ \{ \} for robustness)
\tl_set:Nx \l_tmpa_tl { \l_tmpa_str }
```

After this, `\l_tmpa_tl` already holds the cleaned string, so the three
`\tl_to_str:N \l_tmpa_tl` sites (285/293/334) become idempotent and emit a plain
`&`. The register longtable still receives a catcode-12 `&` (safe, prints as an
ampersand, never a column break); the aux file stores a plain `&`; the reload
path re-cleans harmlessly.

Alternative (equivalent) placement: wrap the three `\tl_to_str:N \l_tmpa_tl`
occurrences in a shared helper `\semio_toc_title_to_str:N` that does
`\tl_to_str` + `\str_replace_all`. The single-point fix in `title_plain` is
preferred because it touches one macro and covers both the live and the reloaded
paths.

`\str_replace_all:Nnn`'s search/replace arguments are themselves detokenized, so
`{ \c_backslash_str & }` matches the literal two-character string `\&`; use
`\c_backslash_str` (not a bare `\&`, which would be a control sequence).

### Risk notes
- Very low. The change only affects the detokenized *register* title string, and
  only rewrites the two-character sequence `\&` → `&`. Titles without escapes are
  untouched.
- The catcode-12 `&` in a longtable cell is safe — confirmed the register body is
  a longtable/`\SemioTableRegister` consumer, where cells are typeset from these
  stored strings, not re-parsed for `&` alignment.
- If extended to `\%`, `\_`, `\#`, `\$`, `\{`, `\}`, verify none of those glyphs
  are needed as active syntax in the register cell (they are not — cells are
  plain text), so extending is safe but optional; only `\&` is required for the
  two reported titles.
- Because the fix is idempotent, a stale `.register-toc` aux file from a previous
  run self-heals on the next compile (the reload path at `:329`/`:334` re-runs
  `title_plain`).

### Shared files / macros to change
- `E:\semio\print\tex\semio-window.sty`, macro `\semio_window_toc_title_plain:n`
  (starts `:242`, add normalization before its closing brace at `:273`).
- No change to `anhang\projekte.tex` (titles are correctly authored).
- No change to bookmarks/hyperref.

---

## Issue 2.17 — Near-empty final bibliography page

### Symptom
The last page of the bibliography carries only the final reference and is
otherwise nearly empty — the cumulative height of the entries overflows the
preceding page by roughly one entry, pushing the last one alone onto a new page.

### Relevant setup
File: `E:\semio\print\tex\semio-window.sty`

- Bibliography environment `semioreferences` (`\defbibenvironment` at `:1884`),
  used by `\SemioReferences` (`:1897`) → `\printbibliography[env=semioreferences,heading=none]` (`:1899`).
- The list parameters (`:1887`–`:1892`):
  - `\labelwidth`/`\leftmargin` = `\labelnumberwidth`
  - `\labelsep` = `\biblabelsep`
  - `\itemsep` = `\bibitemsep`  ← inter-entry vertical space
  - `\parsep`  = `\bibparsep`

Key finding: **`\bibitemsep`, `\bibparsep`, `\biblabelsep` are never set anywhere
in the project** (grep across all `*.cls`/`*.sty`/`*.tex` returns only the four
consuming `\setlength` lines above — no assignment). They therefore hold
biblatex's built-in defaults. biblatex's default `\bibitemsep` carries little or
no shrink glue, so the inter-entry space between ~25–30 entries is effectively
rigid: the page-breaker has no slack to compress the column and keep the last
entry on the preceding page.

Contributing factor: the document-global `\parskip` = `\semio@spacing@par` = `0.5em`
(`semio-window.sty:31`, applied `:1753`; also `semio-table.sty:14`). Inside the
`\list` the dominant inter-entry lever is `\itemsep` (=`\bibitemsep`), but the
non-zero global `\parskip` adds to intra-entry paragraph breaks and to the
list's top spacing, slightly inflating total height. The URL-stretch issue is
already handled in the preamble (`zwischenbericht.tex:5–12`: `xurl` +
`\appto\biburlsetup{\Urlmuskip=0mu}`), so URL line-breaking is **not** the cause
here.

### Root cause
Rigid (shrink-free) inter-entry spacing on a bibliography whose natural height is
just over one page boundary. Because `\bibitemsep` has no `minus` component, TeX
cannot absorb the ~1-line overflow, so `\printbibliography` breaks after the
penultimate entry and the final entry lands alone on a sparse last page.

### Proposed fix (small, bibliography-specific; respects §4: no global font shrink)
Give the inter-entry spacing a shrink component so the page-breaker can compress
the accumulated space by up to about one line across the last page, pulling the
final entry back. Set it once, bibliography-scoped, e.g. in the preamble after
`\addbibresource` in `zwischenbericht.tex`, or (cleaner, shared) alongside the
`\defbibenvironment` in `semio-window.sty`:

```latex
% inter-entry space keeps its visible value but gains shrink so the last
% entry is not orphaned onto a near-empty final page
\setlength{\bibitemsep}{0.5em plus 0.15em minus 0.4em}
```

- Keeps the visible ~0.5em gap between entries (no visual compression in the
  normal case), but grants ~0.4em of shrink per gap; across ~25 gaps that is
  several lines of reclaimable space — enough to keep the last entry on the
  previous page without any perceptible tightening.
- Font size, `\baselineskip`, margins, and per-entry content are untouched.

If a single shrink value is preferred to leave the rest of the column visually
identical, an equally safe alternative is to add shrink only, keeping the natural
default value, by expressing it relative to `\baselineskip`:
`\setlength{\bibitemsep}{\dimexpr 0.6\baselineskip\relax plus 0.1\baselineskip minus 0.4\baselineskip}`.

Do **not**: shrink the bib font, reduce margins, add negative `\vspace`, or force
a manual `\enlargethispage` — the shrinkable `\bibitemsep` is the minimal,
content-neutral lever the spec asks for.

### Risk notes
- Low. Only the bibliography's inter-entry glue changes; it becomes flexible
  rather than rigid. In the common case the entries render at the same spacing;
  shrink is only exercised on the page that would otherwise orphan the last entry.
- The exact numbers may need one compile-and-check iteration: if 0.4em shrink is
  not quite enough to reclaim a full entry, increase the `minus` component (e.g.
  to `minus 0.5em`) rather than reducing the base value, to preserve the normal
  look. Do not over-shrink (keep `minus` ≤ base value to avoid visibly cramped
  entries).
- Because `\bibitemsep` was previously unset (biblatex default), setting it
  explicitly also makes the spacing deterministic across biblatex versions — a
  side benefit, not a regression.

### Shared files / macros to change
- Preferred: `E:\semio\print\tex\semio-window.sty`, near the `semioreferences`
  `\defbibenvironment` (`:1884`) — set `\bibitemsep` (with shrink) so all reports
  using `\SemioReferences` benefit.
- Or document-local: `E:\semio\mit-bestand\bericht\zwischenbericht\zwischenbericht.tex`,
  after `\addbibresource{references.bib}` (`:13`).
- No change to `references.bib`, entry content, or citation data.

---

## Summary

- **2.15**: Correctly-authored `\&` in two appendix subsection titles
  (`anhang\projekte.tex:605`, `:869`) is **detokenized** on the semio contents
  write-path via `\tl_to_str:N` at `semio-window.sty:285`, `:293`, `:334`,
  turning `\&` into a visible `\&`. Fix at the single choke point
  `\semio_window_toc_title_plain:n` (`semio-window.sty:242`): after detokenizing,
  `\str_replace_all` `\&` → `&`. Titles and body text stay valid; bookmarks
  already correct via hyperref.
- **2.17**: `\bibitemsep` is never set (biblatex default, effectively rigid), so
  the bibliography overflows its last page boundary by ~one entry with no slack
  to compress. Fix: give `\bibitemsep` a shrink component
  (`0.5em plus 0.15em minus 0.4em`) at the `semioreferences` environment in
  `semio-window.sty:1884` (or in the preamble). No font shrink, no margin change.

Report path: `E:\semio\.repo\🎫\26\07\06\MIT-BESTAND-ZWISCHENBERICHT-LAYOUT-AND-RENDERING-PASS\diagnosis-D5-contents-biblio.md`
