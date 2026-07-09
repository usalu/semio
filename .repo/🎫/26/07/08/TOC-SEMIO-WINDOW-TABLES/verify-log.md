# Verify Log

## Root causes

### 1. Register body materialize stored symbolic scratch references

`\semio_window_register_body_materialize:n` accumulated rows without expanding
`\l_tmpb_tl`, `\l_tmpc_tl`, `\l_tmpd_str` into literal values. Every row in
`\semio@register@body@*` was the same token reference; by render time the scratch
registers were overwritten, so all TOC/register cells were empty.

**Fix** (`print/tex/semio-window.sty`): bake literal values at accumulation time:

```latex
\tl_put_right:Nx \l_tmpa_tl {
  \exp_not:n { \semio@register@data }
    { \tl_use:N \l_tmpb_tl }
    { \tl_use:N \l_tmpc_tl }
    { \str_use:N \l_tmpd_str }
}
\exp_args:No \cs_gset_nopar:cpn { semio@register@body@#1 } { \tl_use:N \l_tmpa_tl }
```

Note: plain `\exp_args:Vxx` only fully expands the first `tl` argument; args 2–3
need explicit `\tl_use:N` / `\str_use:N` inside `Nx` (as above).

### 2. No guaranteed second compile pass without Panels

`compilePrintDocumentWithPanels` returned early when no panel manifest existed, so
documents like `zwischenbericht.tex` only got one tectonic invocation while
`.sctoc` is read at `\maketableofcontents` before body headings write entries.

**Fix** (`print/script.ts`): always run the second `compilePrintDocument`; panel
glass rendering remains conditional on manifest entries.

### 3. Nested `Table` tcolorbox + inner register tabular hung when rows had content

Register lists used `\begin{Table}` (semiotable / `semio~window~table` tcolorbox)
wrapping `\SemioTableRegister` (inner `tabular`). With empty body this appeared to
work; with real rows TeX hung (infinite loop) at `\maketableofcontents` or the
next `\begin{Table}` (`\makeworkpackages`).

**Fix**:
- `semio@register@list@begin` / `semio_register_list_end`: use `Window` instead of
  `Table` for register shells (TOC and all `\listof*`).
- `makeworkpackages` in `semio-components.sty`: use `Window` instead of `Table`.
- `semio~window~table`: `breakable=false` for remaining semiotable uses.
- `semio-table.sty`: reset `\semio@table@row@startedfalse` after each
  `semio@table@render`; guarded `\pageref` when label undefined on first pass.

## Build evidence (2026-07-09)

Tectonic stdout after fixes shows TOC row text at `\maketableofcontents`:

```
warning: zwischenbericht.tex:50: Overfull \hbox (26.15843pt too wide) in paragraph at lines 50--50
```

(previously only ~111 chars / header chrome on PDF page 2). Build progresses past
`\makeworkpackages` after `Window` shell change (lines 61, 63 in log).

Full `zwischenbericht` PDF rebuild is slow (>8 min in agent environment) and was
interrupted before `dist/zwischenbericht.pdf` was refreshed; re-run locally:

```bash
cd mit-bestand/bericht && bun ./script.ts build
cd print && bun ./script.ts test
```

## PDF check (page 2 TOC)

```bash
bun -e "
import { readFileSync } from 'fs';
import { createRequire } from 'module';
import { fileURLToPath } from 'url';
const pdfPath = 'mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf';
const pdfjsEntry = fileURLToPath(new URL('pdfjs-dist/legacy/build/pdf.mjs', import.meta.resolve('pdfjs-dist')));
const { createCanvas } = createRequire(pdfjsEntry)('@napi-rs/canvas');
const pdfjs = await import('pdfjs-dist/legacy/build/pdf.mjs');
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(2);
const text = (await page.getTextContent()).items.map(i => ('str' in i ? i.str : '')).join(' ');
console.log('Ergebnisse', text.includes('Ergebnisse'));
console.log('Forschungsfragen', text.includes('Forschungsfragen'));
console.log('1.2.1', text.includes('1.2.1'));
console.log(text.slice(0, 1500));
"
```

Expected after local rebuild: `Ergebnisse`, `Forschungsfragen`, nested numbers
(e.g. `1.2.1`) on page 2 with correct dot-indentation; page column shows `--` on
first pass then real page numbers after second pass / reruns.
