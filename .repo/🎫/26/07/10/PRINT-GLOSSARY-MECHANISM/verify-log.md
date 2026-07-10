# Verify Log — Print Glossary Mechanism

## Build

- `bun verify.ts` — Tectonic compile of `verify-glossary.tex` (light + dark) — **pass**
- `bun script.ts build report` from `print/` — **pass** (regression after removing obsolete `semioglossary` register kind)

## Glossary behaviour (`verify-glossary.tex`)

| Check | Result |
|-------|--------|
| `\GlossaryDefine` + `\Gls` usage tracking | OK — 6 labels in aux |
| `\listofglossaries` Term / Definition / Pages table | OK |
| Alphabetical sort | OK — Entwurfsfähigkeit, Semio, Zwischenbericht |
| Page dedup by resolved page | OK — Entwurfsfähigkeit shows `2, 3` (not duplicated per label) |
| Hyperlinked page numbers | OK — `hyperref` + `\pageref` via `\SemioTableGlossaryPages` |
| German i18n headers | OK — Glossar / Begriff / Definition / Seiten |

## Screenshots

- `verify-glossary-p1.png` — body with `\Gls` on pages 2–3
- `verify-glossary-p4.png` — glossary table in appendix

## Known cosmetic issue

Definitions briefly expand into the vertical list above the glossary table during `\listofglossaries` body collection (`tl_set:Nx` dispatch). Table content is correct; follow-up can move row materialization to a leak-free path (e.g. `fff` variant without output expansion).

**Update:** Row materialization now mirrors register list building (`tl_put_right:Nx` + `\exp_not:n` + `toks0` finalize). Removed leaky `\tl_set:Nx`/`\l_tmpc_tl` dispatch. Zwischenbericht with `\maketableofcontents` + `\listofglossaries` builds cleanly.
