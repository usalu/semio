# Verify Log

## Root cause (empty TOC body rows)

`\semio_window_register_body_materialize:n` used `\exp_args:Ne` when storing
`\semio@register@body@toc`. Full expansion (`e`) executed `\semio@register@data` →
`\SemioTableRegisterRow` → `\SemioTableRow` **during materialize**, outside the
`tabular` environment. Row markup (`\\`, `\noalign`, `\hline`) was discarded, so
`\maketableofcontents` rendered only the register header row.

## Fix

`print/tex/semio-window.sty`: use `\exp_args:No` (once-only expansion) so body rows
are stored as tokens and expanded inside `\SemioTableRegister` on the second pass
when `.sctoc` is loaded.

## Build

```bash
cd mit-bestand/bericht && bun ./script.ts build
```

Note: concurrent edits to `semio-window.sty` may cause slow/hung compiles at
`\makeworkpackages` (line ~54); TOC fix is independent. Rebuild when window
header forward-ref settles.

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
console.log(text.includes('Ergebnisse') ? 'TOC rows present' : 'TOC rows missing');
console.log(text.slice(0, 500));
"
```

Expected after fix: page 2 contains `Ergebnisse`, `Forschungsfragen`, section numbers.
