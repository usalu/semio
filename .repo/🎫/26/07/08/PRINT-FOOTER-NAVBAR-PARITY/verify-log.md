# Verify Log — Print Footer Navbar Parity

## Problem
Footer chrome was painted via `eso-pic` at `\AtPageLowerLeft` + `\put(0,0)`, so it started at the physical page edge while the navbar uses `fancyhdr` centered in the text block (`\headwidth` = `\textwidth`).

## Fix
- Render footer through `\fancyfoot[C]{\SemioChromeFooterHead}` in `\semio_chrome_pagestyle_def:n` (same mechanism as navbar).
- Disable eso-pic footer shipout (`\semio_chrome_footer_install:` no-op).
- Keep inner `\hspace{\semio@spacing@single}` padding in navbar and footer hboxes.

## Build
```bash
bun ./script.ts build paper
```
`print/dist/paper.pdf` built successfully (2026-07-08).

## Visual check
```bash
bun .repo/🎫/26/07/08/PRINT-FOOTER-NAVBAR-PARITY/rasterize.ts
```
Artifacts: `report-p2-before.png`, `report-p2-footer-crop.png`.
