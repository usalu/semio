# LaTeX Implementation Binding Audit — 2026-09-12

## Decision

`.sty` and `.cls` names are native public identities only where a TeX consumer resolves them through `\usepackage`, `\RequirePackage`, `\documentclass`, or `\LoadClass`. They must remain tiny, fixed-name shims containing their TeX declaration and delegation. Their macro bodies are executable implementation and belong under the semantic owner as the canonical anonymous TeX leaf `📐️.tex`; no blanket `.sty`/`.cls` implementation exemption is justified.

The compiler cannot directly load that repository filename. The portable materialization proposed for the print staging lane is exact and narrow:

1. retain repository `📐️.tex` beneath the semantic owner directory;
2. materialize that one canonical anonymous TeX leaf as `_.tex` in the compiler staging tree;
3. rewrite TeX source references in every TeX-bearing text form that can contain one (`.tex`, `.bib`, `.sty`, and `.cls`) from the canonical repository path to the staged path; and
4. leave fixed public package/class names unchanged in the staging tree.

`_.tex` is a compiler-only native filename, never a repository taxonomy exception. It is anonymous, has a nonempty ASCII basename, and loaded under the pinned native compiler. The current staging behavior instead removes the `📐️` glyph and emits `.tex`; that happens to work in the audited Tectonic run because the engine also drops the glyph while parsing the shim reference. It is observed behavior, not a portable contract or the recommended implementation path.

## Native evidence

The pinned offline executable was `.🧬semio/🦑️repo/⚡️cache/tools/tectonic/0.16.9/aarch64-apple-darwin/tectonic`; every native control used locked bundle `2f375cdacdf7f982138c778c3e12d8cd11e4aa3af3d854deb1467d560e8d71b0`, `--only-cached`, `-Z deterministic-mode`, `SOURCE_DATE_EPOCH=1782864000`, and `TZ=UTC`. All resulting PDFs stayed under this ticket's generated subtree and were removed after this report.

| Control | Result | What it establishes |
| --- | --- | --- |
| Named `audit-graph.sty` → ASCII test body `implementation.tex` | Two passes, byte-identical PDF `cda4a861…287f2c89` | Native named package resolution may delegate with `\input`; `implementation.tex` is a test-only name, not a source proposal. |
| Direct `🕸️graph/📐️.tex` | Fails: `(.tex File ignored)`, then undefined macro | The repository canonical leaf is not directly loadable by the native engine. |
| Direct `🕸️graph/_.tex` | Two passes, byte-identical PDF `22de7c86…ced0895` | The exact staged anonymous leaf is native-loadable. |
| Current `stagePrintSources` of `🕸️graph/📐️.tex` | Emits `graph/.tex`; its copied shim keeps `\input{📐️.tex}`; two staged runs passed with `61f58b8c…2be9046` | Current `.tex` behavior exists, but depends on two independent glyph-dropping behaviors. |
| Full staged source → root shim → `graph/_.tex` | Two passes, byte-identical PDF `f9b8ac27…d41127f` | The proposed materialization works from the staged root, with root entry `canonical-control.tex`, root `audit-canonical.sty`, semantic `graph/_.tex`, `TEXINPUTS=<staged-root>//:`, and `search-path=<staged-root>`. |
| Unmodified `\documentclass{semio}` | Two passes, byte-identical PDF `db585f46…44369df9`; control called `\SemioGraphState` and `\SemioGraphEdgeKind` | The actual named `semio.cls` class-to-package relation loads and executes substantive graph registry macros. Fonts were supplied through the production `dist/fonts` search path. |

The full staged control first invokes the actual `stagePrintSources` over ticket-owned sources, then changes only its generated materialization from `graph/.tex` to `graph/_.tex` and changes the generated shim reference from `\input{🕸️graph/📐️.tex}` to `\input{graph/_.tex}`. It does not change product sources. This separates the proposed staging change from current native behavior.

The native graph/class document used the actual `semio` class, which requires `semio-graph`; its two real substantive registry calls defined a graph state and edge kind. The separate trivial named-shim control exists solely to prove the loader mechanism. A first direct graph-figure drawing control was not included as acceptance evidence because it exceeded this audit's bounded compiler interval; it neither weakens the completed registry execution nor warrants a broad document compilation sweep.

## Current staging seam

`🧰️framework/🛍️products/📓️print/🔨️modules/📥️source-staging/🟦️.ts` has two relevant current rules:

- `printCompilerName` removes an initial pictograph from every staged path component. Thus canonical `📐️.tex` becomes `.tex`, while the candidate compiler leaf `_.tex` remains `_.tex`.
- `stagePrintSources` rewrites references only in `.tex` and `.bib`; it copies `.sty` and `.cls` byte-for-byte. Therefore a future explicit `_.tex` materialization must extend its deliberately scoped TeX-reference rewrite to shim/class text. The full staged control proves the exact before/after reference needed, rather than asserting a general filename exception.

This is a print source-staging concern. It does not belong to the active fixed-script body gate and does not authorize arbitrary ASCII source leaves.

## Actual ownership boundary

### Mandatory identity glue

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio.cls` is selected by `\documentclass{semio}`. It selects KOMA classes and requires tokens, core, window, components, graph, tree, and viz packages.
- `🧰️framework/🛍️products/📓️print/🖋️latex/zukunftbau.cls` is selected by documents using that class and `\LoadClass`es `semio` with its document-kind options.
- Named `.sty` package files are selected by the package names in those class declarations and by other `\RequirePackage` consumers. Each needs the fixed filename and matching `\ProvidesPackage`; that does not require it to contain its behavior.

### Substantive macro implementations

The following inspected bodies are authored macro domains, not prose or identity glue. The line/byte measures are current observations only.

| Current body | Inspected behavior | Size |
| --- | --- | --- |
| `semio-graph.sty` | Graph state/edge registries, diagnostics, TikZ styles, figures, primitives, legends | 431 lines / 18,726 bytes |
| `semio-window.sty` | Window chrome, panels, figures, navigation, rows, image and register behavior | 3,706 / 147,248 |
| `semio-table.sty` | Table rows, register/TOC tables, long and glossary table formatting | 1,457 / 67,862 |
| `semio-tree.sty` | Tree kinds, figures, boxes, rows, marks and legends | 609 / 29,990 |
| `semio-viz-layout.sty` | Visualization layout computation and `\SemioVizLayout` | 616 / 33,717 |
| `semio-core.sty` | Document setup, theme/language behavior, labels and front-matter mechanisms | 655 / 25,774 |
| `semio-components.sty` | Matter transitions, covers, appendices, registers and document components | 543 / 22,213 |
| `semio-viz-mark.sty` | Visualization marks, paths and text drawing | 364 / 27,182 |

Those are the first bounded set for extraction. Their semantic owner directory, rather than a language-first `latex` container or their present package basename, should name their domain. The shim becomes the only item retaining `semio-graph.sty`, `semio-window.sty`, and comparable public names.

### Generated token projection

`semio-tokens.sty` is generated, not an authored macro body. Its producer is `🧰️framework/🛍️products/📓️print/🔨️modules/🎨print-design-token-paints/🟦️.ts`; `📜️script.ts` writes the stylesheet and exposes `preview-generated`; `📦️packages/🟦️typescript/📋️project.json` registers the generation route. A direct Bun oracle rendered the producer and compared it to the current stylesheet: byte-identical, 4,151 bytes, containing `\ProvidesPackage{semio-tokens}`. Preserve the generator authority, native package identity, and generated output contract while relocating it.

### Documents, templates, and bibliography

The paper, report, flyer, and Zukunftbau template `.tex` files are documents and prose/template content, including `\input` relationships such as `🎓️paper.tex` → `🧩️paper.content.tex`. Their `.bib` companions hold bibliographic records. They are not evidence that every `.tex`/`.bib` file is a macro implementation. Their source references are nevertheless TeX-bearing text and must participate in compiler-materialization rewriting whenever their referenced canonical path changes.

## Bounded executable Sol lane

1. Add a single print staging contract for **canonical anonymous TeX implementation leaves only**: repository `📐️.tex` maps to compiler `_.tex`. Keep the reverse mapping out of source ownership and do not broaden the taxonomy's filename allowance.
2. Extend the existing source-staging text-reference transform to `.sty` and `.cls` along with `.tex`/`.bib`, with exact canonical-path substitutions. Preserve bytes for non-reference files and keep the present collision checks.
3. Add portable fixtures with a root entry, one named shim, one semantic graph directory, and repository `📐️.tex`. Assert staged `graph/_.tex` and a rewritten `\input{graph/_.tex}`; reject ambiguous or colliding materializations.
4. Add a native locked-Tectonic fixture using the same root entry/search-path coordinates as this audit. Require its real macro marker and two deterministic PDFs. Keep this fixture narrow; do not compile all 97 named contracts.
5. Extract one macro domain at a time, beginning with graph: fixed `semio-graph.sty` contains only native identity/delegation, while the semantic graph owner holds canonical `📐️.tex`. Rebase actual class/package consumers and preserve the generated token projection separately.

## Reviewed files and limits

Read in full: `📓️latex-package-implementation-followup-packet-2026-09-12.md` and `📓️fixed-filename-contract-review-2026-09-12.md`. Inspected the compilation owner, source stager, token producer/route, class declarations, template entry/content/bibliography examples, and the eight substantive macro bodies enumerated above.

No product source, schema, taxonomy, contract baseline, live PDF, or session output changed. No 97-file census was repeated. The compiler proof covers the pinned local Tectonic/bundle on this host; it does not establish behavior for another TeX engine. The current `.tex` staging behavior is recorded as observed, not promoted to a cross-engine invariant.

Authored controls retained at `📋️latex-binding-controls/`; all generated compiler/staging outputs were removed after recording results.
