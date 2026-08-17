---
name: Print Glossary Mechanism
overview: Add a term/definition/usage-tracking glossary mechanism to the semio print (LaTeX) stack, replacing the unused generic "Glossary" register stub with a real glossary that collects `\GlossaryDefine`/`\Gls` usage and renders a Term / Definition / Pages table at the document end via `\listofglossaries`.
todos:
 - id: open-ticket
   content: Open ticket under goal r2602/runningsketchpad for the glossary mechanism
   status: completed
 - id: remove-old-kind
   content: Remove obsolete semioglossary register kind from semio-window.sty and unused i18n entries from semio-core.sty
   status: completed
 - id: data-model
   content: "Add Glossary region to semio-window.sty: term/definition/pages data structures, GlossaryDefine, Gls, listofglossaries render"
   status: completed
 - id: table-macros
   content: Add SemioTableGlossary colspec/row/table macros to semio-table.sty
   status: completed
 - id: i18n
   content: Add SemioGlossaryTitle and Term/Definition/Pages header i18n strings to semio-core.sty
   status: completed
 - id: verify
   content: Build ticket-local verify-glossary.tex + verify.ts, compile with Tectonic, screenshot, confirm dedup/hyperlink behavior
   status: in_progress
 - id: regression-build
   content: Rebuild an existing print template to confirm removal of old kind doesn't break other documents
   status: pending
 - id: close-ticket
   content: Close ticket with summary and full list of touched files
   status: pending
isProject: false
---

# Add Glossary Mechanism to Print (semio LaTeX stack)

## Context

`print/` is a LaTeX-first stack (no compiler/generator — documents are handwritten `.tex`, built via [print/script.ts](print/script.ts) + Tectonic). Reusable styling lives in `print/tex/*.sty`. The "register" pattern (collect entries while the body compiles, render as a table at the end) already exists and powers List of Figures/Tables/etc. and the Table of Contents (see the `TOC-SEMIO-WINDOW-TABLES` ticket). A `Glossary` window "kind" and `\listofglossaries` already exist in [print/tex/semio-window.sty](print/tex/semio-window.sty) but they only capture **title + page** (like List of Figures) — never term + definition — and are unused anywhere in the repo.

Per the clarifying answers: the new glossary uses **usage-tracking** (definitions declared once, `\Gls{term}` marks each occurrence in running text), and the end table shows **Term | Definition | Pages**, with each page number individually hyperlinked to its usage location.

## 1. Remove the obsolete generic Glossary register stub

Since the existing `semioglossary` kind/window has the wrong shape (title+page, no definition) and is unused, drop it cleanly rather than layering two "glossary" concepts:

- [print/tex/semio-window.sty](print/tex/semio-window.sty):126 — remove the `\semio_window_kind_define:nnnnn { semioglossary } { glossary } { Glossary } { structural } { glossaries }` line (this also removes the `Glossary` environment and the auto-generated `\listofglossaries`, which we replace below).
- [print/tex/semio-core.sty](print/tex/semio-core.sty) — remove the now-unused `glossary`/`glossaries` entries from `\semio@kind@register@en/de` and `@plural@en/de` (they only existed for the old kind label).

## 2. New Glossary data model + commands (`semio-window.sty`, new `%region Glossary`)

Following the existing register/TOC pattern but keyed by term string instead of a running counter:

- `\seq_new:N \g_semio_glossary_terms_seq` — insertion-order, de-duplicated term keys.
- `\prop_new:N \g_semio_glossary_definition_prop` — term → definition text.
- `\prop_new:N \g_semio_glossary_pages_prop` — term → comma-list of unique usage labels.
- `\newcounter{semioglossaryusage}` — generates a unique `\label` per `\Gls` occurrence (needed for per-occurrence `\pageref`/`\hyperref`).

Public commands:

- `\GlossaryDefine{term}{definition}` — registers/overwrites a term's definition; adds it to the terms seq if new.
- `\Gls{term}` — typesets `term` in place, steps the usage counter, `\label`s that spot, and appends the label to the term's page-usage list (adding the term to the terms seq if it's the first time it's seen, so usage-before-definition is safe).
- `\listofglossaries` — replaces the removed auto-generated command. Renders nothing if no terms were collected; otherwise sorts a local copy of the terms seq alphabetically (`\seq_sort:Nn` with `\str_if_compare:nNnTF`), materializes each row (term, definition, deduplicated hyperlinked page list), and renders through the same `Window`-wrapper used by TOC/registers (`\semio@register@window@open{\SemioGlossaryTitle}` … `\semio@register@window@close`).

Page-list rendering dedupes by **resolved page number** (not label) so multiple `\Gls` calls landing on the same page collapse to one link, each rendered as `\hyperref[label]{\pageref{label}}` (hyperref is already loaded via [print/tex/semio.cls](print/tex/semio.cls):60).

`\listofglossaries` stays wired into `\makeregisters` in [print/tex/semio-components.sty](print/tex/semio-components.sty):197 — no change needed there.

## 3. New table shape (`semio-table.sty`)

Add a dedicated 3-column colspec (Term ~22% / Definition ~56% / Pages ~22%, matching the proportions of `semio@table@colspec@register`) plus row/table macros:

- `\semio@table@colspec@glossary`
- `\SemioTableGlossaryRow{term}{definition}{pages}` → builds on the existing `\SemioTableRow`
- `\SemioTableGlossary{rows}` → `\semio@table@render{\semio@table@colspec@glossary}{rows}`

## 4. New i18n strings (`semio-core.sty`)

Dedicated strings for the glossary section (not the generic "List of X" register title, since "List of Glossaries" reads wrong for a term/definition table):

- `\SemioGlossaryTitle` → "Glossar" (de) / "Glossary" (en)
- Column headers: Begriff/Term, Definition/Definition, Seiten/Pages

## 5. Verification (ticket-local, no permanent example files)

Following the pattern used by `FIX-PRINT-PARAGRAPH-CHIP-ALIGNMENT` (ticket-local `verify-*.tex` + `verify.ts`):

- Open a ticket under goal `r2602/runningsketchpad` (same goal used by the recent print tickets).
- Add a ticket-local `verify-glossary.tex` fixture (using `semio.cls`) exercising: several `\GlossaryDefine` calls, multiple `\Gls{term}` uses of the same term across different pages (to prove page dedup + multi-page linking), and `\listofglossaries` at the end.
- Add/extend a ticket-local `verify.ts` to compile it via Tectonic (same invocation style as the existing ticket) and produce a PDF → PNG screenshot to visually confirm: table renders, columns are Term/Definition/Pages, page numbers are distinct hyperlinks.
- Cross-check `print/dist/*.pdf` for one of the existing templates still builds cleanly after removing the old `semioglossary` kind (run `print/script.ts build` or the relevant `bun`/`nx` target).

## Files touched

- [print/tex/semio-window.sty](print/tex/semio-window.sty) — remove old kind, add Glossary region (data model, `\GlossaryDefine`, `\Gls`, `\listofglossaries` render)
- [print/tex/semio-table.sty](print/tex/semio-table.sty) — glossary colspec + row/table macros
- [print/tex/semio-core.sty](print/tex/semio-core.sty) — remove unused glossary kind-label i18n; add glossary title/header i18n
- Ticket folder under `.repo/🎫️/26/07/09/...` — `verify-glossary.tex`, `verify.ts`/extended verify script, rendered screenshot, `verify-log.md`, `ticket.json`
