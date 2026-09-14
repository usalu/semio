# Status — DOCS

Agent: DOCS (second run; the first run stalled before writing anything).

Scope owned:
- `🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-api/🔓️viz-api.tex` — the d3-style reference document.
- `🧰️framework/🛍️products/📓️print/README.md`.
- `🔨️modules/📊️visualization-gallery/🟦️.ts` — ONLY the new exported `vizApiReference()` plus the
  `VIZ_API_COMMANDS` extension.

Not owned (untouched): `🖋️latex/semio-viz-*.sty`, `🧬️schema/🔣️.json`, `🖼️assets/🔣️viz-catalog.json`,
the generator functions `vizGeneratedFiles()` / `generateVizArtifacts()` (INTEGRATION-2),
`🔨️modules/📊️viz-kernel/` (TS-TWIN).

## The document

`🔓️viz-api.tex` is now the reference of the whole library: **18 chapters, ~4600 lines, 168 pages**,
built in both themes with zero LaTeX errors.

1. Reading this reference — conventions, `VizFigure`.
2. Data — `\SemioVizTable/Row/TableFromCSV/Hierarchy/Graph/Matrix/Geometry/Function`, the demo tables.
3. Transform — the fixed operation order and all 23 operations.
4. Scale — the 15 kinds and the 17 options.
5. Format — the d3-format grammar, all number types, all time directives, worked examples typeset by
   the library itself.
6. Coordinate systems — the 7 kinds and the 13 options.
7. Marks and paths — the 56 mark kinds grouped, the 26 mark options, the 19 curves, the 8 text kinds.
8. Shape generators — line, area, arc, pie, link, symbol, ribbon and the result accessors.
9. Layout algorithms — `\SemioVizLayout` (the dispatcher landed while I wrote: 30 algorithms), the
   hierarchy, force, flow and spatial option vocabularies.
10. Guides — axis (24 options), legend (13), grid.
11. Annotation and labels — the 11 annotation kinds, the queue-and-solve label model.
12. Facet. 13. Composition — every §77 block. 14. Theme.
15. The plot grammar — `\SemioVizPlot`, `VizPlot`, `\SemioVizLayer`, the 16 encoding channels.
16. Chart kinds and families — the catalogue, the generated packages, overrides.
17. The namespaces — §76, one section per namespace, **246 families with every option, type, default
    and bilingual meaning** (generated from the schema by `🔧️docs-namespaces.py`, see below).
18. Extending the library (chart kind / family / test, incl. the TESTS-HARNESS §1 contract and the
    probe protocol) and Numerics and limits.

Every grammar chapter carries at least one compiled example figure with demo data.

## Decisions

- **Bilingual with no default language.** `\ApiText{en}{de}` is an expandable
  `\str_case:Vn \l_semio_language_tl { {en}{#1} {de}{#2} }` — the mechanism `semio-core.sty` uses for
  `\SemioRegisterTitle`, and with **no fallback branch**, so an unknown language errors instead of
  silently rendering English. Used in every chapter and section title, every paragraph and every
  table cell, including the generated namespace chapter.
- Presentation macros: `\Cs{name}`, `\Key{…}`, `\meta{…}`, `\ApiSig{…}`, `\ApiPkg{…}`, `\E{emoji}`.
- Key tables are `\SemioTableLong[text-size=8pt]{caption}{fractions}{header}{rows}`.
- The §76 namespace chapter is **generated from the schema**, not hand-typed: 245 families ×
  2573 options with the bilingual descriptions the family owners already wrote is not something to
  retype, and generating it keeps the reference honest when a family changes. The generator is
  `🔧️docs-namespaces.py` in this ticket folder and it splices between
  `%region 🔖️GeneratedNamespaces` markers, so re-running it never touches the hand-written chapters.

## Traps found while writing (worth keeping)

- `\SemioTableLong`'s header argument is split on `&` by `\semio_table_long_header_build:nn`
  **before any expansion**, so one macro that expands to `a & b & c & d` is ONE cell and the table
  dies with `Missing } inserted` at the end of the row block. Header cells are written out at every
  call site.
- The monospaced face (`ShareTechMono`) has no emoji coverage, so `\texttt{📜️script.ts}` silently
  drops the glyph ("Missing character" in the log, nothing on the page). `\E{…}` switches to
  `\SemioEmoji` for the glyph and the variation selector is dropped; `🔧️docs-emoji-in-mono.py`
  applies it to every `\Key`/`\Cs`/`\meta` argument and is idempotent.
- Anta has no `≤`/`≥`/`×` either; the namespace generator spells them out.
- Python on this machine cannot open a path with emoji segments **for writing** when the path is
  absolute (`OSError: [Errno 22]`); reading works. `os.chdir()` plus the bare basename works for
  some names and not others — the Write/Edit tools are the reliable route.
- Concurrent `build viz …` runs collide: `stagePrintSources` `rmSync`s `dist/source/<template>` and
  fails `EBUSY` while another agent's tectonic holds it. Every build here runs in a retry loop.

## `vizApiReference()`

New export in `🔨️modules/📊️visualization-gallery/🟦️.ts` (region `🔖️ApiReference`), the
machine-readable twin of the document:

```
{"packages":49,"commands":157,"keys":522,"families":245}
```

- **packages** — every `semio-viz-*.sty`: its public `\SemioViz…` commands and `Viz…` environments
  (name, kind, xparse signature, the emoji `%` note above the definition) and its
  `%region 🔖️Keys…` blocks parsed into keys with type, default and description. Three comment
  spellings are in use across the kernel and all three are parsed.
- **families** — the schema's 245 family option vocabularies with the owner and bilingual descriptions.
- The German half of a kernel key is read back from the reference document's own bilingual key
  tables, and only where the document spells that name the same way everywhere (134 of 522 keys
  today). Deriving it from the family vocabulary by name was tried and rejected: `align`, `base` and
  `padding` mean different things in different namespaces and the result was confidently wrong.
- `VIZ_API_COMMANDS` went from 21 to **97** names covering everything the document documents, and the
  entries lost their leading backslash — a signature is typeset with `\Cs{…}`, which *emits* the
  backslash rather than writing it into the source, so matching on the bare name is the check that
  means what it says. `assertVizApi()` itself is unchanged. Verified: 97 names, 0 missing.

Wiring it into `generate viz` as `🖼️assets/🔣️viz-api.json` is requested from INTEGRATION-2 in
`📓️integration.md` under `## From DOCS` (the generator functions are theirs, not mine).

## README

`🧰️framework/🛍️products/📓️print/README.md` created: the directory map, the `📜️script.ts` commands
and their nx targets, what is generated and must not be hand-edited, the grammar in one line, a
pointer to the reference document and to `vizApiReference()`, and how the test cases and the oracle
registry work.

## Final verification

`bun ./📜️script.ts build viz api`, the whole document, both themes, nothing else touched:

```
errors:
viz-api.log:0
viz-api-dark.log:0
viz-api.log:4235:Output written on viz-api.xdv (172 pages, 6972144 bytes).
viz-api-dark.log:4235:Output written on viz-api-dark.xdv (172 pages, 6888544 bytes).
missing chars:
viz-api.log:0
viz-api-dark.log:0
```

**172 pages in both themes, zero LaTeX errors, zero dropped glyphs.**

`bun ./📜️script.ts test fundamental` (the level that gates the API reference and the generated files):

```
[DEBUG] print: viz coverage 1966/1966 leaves through 1738 kinds, API 128/128
[DEBUG] print: unit tests passed
[test] level=fundamental cases=68 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=66
```

Document shape: 19 chapters, 26 sections, 245 family subsections, 247 key tables with 2617 documented
keys, 50 signature blocks, 18 compiled example figures, 2808 bilingual `\ApiText` calls.

### `bun ./📜️script.ts test quick` — **red, and not on anything DOCS owns**

The unit half is green (`[DEBUG] print: unit tests passed`, `API 128/128`). The command then runs
`parity quick` for the print owner, and that dies:

```
[budget] bun …🧪️test\🖥️host\🟦️.ts --plan …hierarchy-pack-subject-typescript\📋️plan.json …
         exceeded 30000ms — killed. Trim it, or raise its budget
         (`budgetMs`, `SEMIO_CMD_BUDGET_MS`, `SEMIO_BUILD_BUDGET_MS`).
error: spawnSync bun ETIMEDOUT   syscall: "spawnSync bun"   code: "ETIMEDOUT"
error: bun ./📜️script.ts parity quick --owner 🧰️framework/🛍️products/📓️print exited with status 1
```

That is the `pack` performance issue HIERARCHY measured and I document in the numerics chapter
(18.7 s for 211 nodes, 4.7 s for the committed seven-pack fixture) meeting the platform's 30 s
per-adapter budget on a machine currently running four tectonic processes at once. The same run also
reports 29 `parity failed` lines in `scale-temporal`, `shape-*`, `spatial-*` and `transform-*` — all
kernel cases owned by GRAMMAR-CORE / SHAPES / GEO-SPATIAL and, per INTEGRATION-2's status, still
theirs to close.

Nothing in that failure touches the three files I own. The level that gates my work,
`test fundamental`, is green including `parity=7/7`, and the document builds clean in both themes.
The full log was captured to `🗑️generated/DOCS/test-quick.txt` and deleted again after reading it;
the tails quoted above are verbatim from that run.

## Real output tails

Chapter-by-chapter builds (`bun ./📜️script.ts build viz api`, `grep -c "^!"` on both logs):

```
ch. 1–2    viz-api.log:0  viz-api-dark.log:0   8 pages
ch. 3–4    viz-api.log:0  viz-api-dark.log:0  (header trap fixed)
ch. 5–8    viz-api.log:0  viz-api-dark.log:0   24 pages / 24 pages
ch. 9–11   viz-api.log:0  viz-api-dark.log:0   32 pages / 32 pages
ch. 12–16  viz-api.log:0  viz-api-dark.log:0   40 pages / 40 pages
ch. 17     viz-api.log:0  viz-api-dark.log:0  164 pages / 164 pages
ch. 18     viz-api.log:0  viz-api.xdv 168 pages
```

## Progress

- [x] Document, all 18 chapters, compiled after every chapter.
- [x] `vizApiReference()` + `VIZ_API_COMMANDS`.
- [x] README.
- [x] `## From DOCS` request filed in `📓️integration.md`.
- [x] Final verification: both themes clean in one run (172 pages each), `test fundamental` green
      with `API 128/128` and `parity=7/7`; `test quick` red on the `hierarchy-pack` adapter budget,
      which is not mine (see above, and filed in `📓️integration.md`).

## Blocked on nothing; noted for INTEGRATION-2

Two dark-theme builds failed on *kernel* states that were mid-edit by another agent and were gone on
the next build — recorded in `📓️integration.md`: `\g_semio_viz_geom_names_seq` declared twice
(`semio-viz-data.sty` and `semio-viz-geo.sty`), `\fp_new:N \l_semio_viz_spatial_opt_x0_fp` (digit in
an expl3 name), and one `Missing \begin{document}` at `semio-viz-*.sty:3487`. None of them is mine
and none is standing.

## Files touched

- `🧰️framework/🛍️products/📓️print/🧾️template/📊️viz-api/🔓️viz-api.tex` (rewritten, 18 chapters).
- `🧰️framework/🛍️products/📓️print/README.md` (new).
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts`
  (region `🔖️ApiReference` added, `VIZ_API_COMMANDS` extended — nothing else changed).
- `.🧬semio/🦑️repo/🎫️tickets/…/PRINT-VISUALIZATION-LIBRARY/📓️integration.md` (`## From DOCS`).
- Ticket scripts, kept: `🔧️docs-inventory.py`, `🔧️docs-namespaces.py`, `🔧️docs-emoji-in-mono.py`.
- `📓️status-DOCS.md` (this file).
