# Fix Table Gaps (Glossar → Abkürzungen)

## Symptom
Page-cream hole between last Glossar row (`Test-Case`) and first Abkürzungen row (`AP`), with vertical border stubs into the gap and missing Abkürzungen chrome/column headers.

## Root cause
1. Empty `\endfirsthead` left a **non-void** empty `\LT@firsthead`. Longtable then shipped that blank box on page 1 and skipped `\LT@head` — so the second consecutive register longtable lost its title chip + column headers.
2. Mid-rule L/R pillars had downward extent that stuck into `LTpost` / the inter-table cream band.

## Fix
- `print/tex/semio-table.sty`: omit `\endfirsthead` (void `\LT@firsthead` before `\begin{longtable}`); page 1 uses `\LT@head`. Mid-rule pillars are upward-only.
- `print/tex/semio-window.sty`: `\clearpage` before each termlist after the first so Glossar / Abkürzungen do not stack on one page.

## QA (2026-08-05)
- PDF text: Abkürzungsverzeichnis + Abkürzung/Bedeutung/Seiten on p123; Glossar ends p122.
- Pixel: Test-Case bottom `stubBelowL/R = 0`. Abk chrome welded; AP present.
