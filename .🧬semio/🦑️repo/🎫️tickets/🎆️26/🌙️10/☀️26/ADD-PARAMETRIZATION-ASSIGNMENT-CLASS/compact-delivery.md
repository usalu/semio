> Superseded by k16-delivery.md: the final three-page assignment uses one full-width K16 oval and exactly twenty selected parameter changes.

# Compact Tables And Centered Capsule Icons

Delivered on2026-10-27: `output/pdf/Fingeruebung-Parametrisierung.pdf`.

The actual LaTeX assignment now has four A4 pages. Both complete nine-row, nine-column tables occupy page2. Each icon is centered horizontally and vertically in its header cell, with consistent padding around the original capsule artwork. The eight writable parameter rows in each matrix use a nominal6mm single-line height; nine total rows remain, including the icon header. The two K8 graphs and their28-entry ledgers remain on pages3and4.

## Final Artifact And Measured Geometry

- SHA-256: `7cba4378624fe33a5161efcee179c75a7cc6e371de3167f8fa0afbbcdab25e5a`;81,569bytes.
- Four A4pages; German title/author/de-DE metadata and native LaTeX/xdvipdfmx output.
- Independent actual-PDF pypdf/pdfplumber checks confirm both disjoint9×9 matrices on page2, eight unchanged vector icon headers per matrix and blank writing cells.
- Actual row pitch is6.14mm including rules. Minimum artwork inset is2.13–2.22mm across all16icons. Maximum measured horizontal center deviation is0.008mm and vertical deviation0.004mm.
- Final German compilation exits0 with no overfull/underfull boxes. PDF.js reports no out-of-page text. The parent inspected all four latest rendered pages individually; header icons, row boundaries, compact table placement, graph labels, ledgers and footers are clear and contained.
- English light and German dark each compile to four pages and pass the same table/grid/centering/padding checks. Their rendered table pages and contact sheets were independently inspected.

## Implementation And Tests

The reusable matrix separates a fixed16mm icon header from6mm writing rows and uses2mm padding. Fixed header boxes center their contents independently of font baselines. The image helper fits the actual original resource proportionally within the padded header; dark backing dimensions are included. All sixteen canonical vector resources, the existing assignment class, product fonts, colors and chrome remain in use. No new runtime dependency was added.

The generic language-neutral fixture and AJV suite pass17tests/64assertions after the expected old-height red run. The version4 content fixture and schema specify both matrices on page2 and four-page ordering; the source/AJV/mathjs/d3-array checks pass after the old-five-page red run. Content is condensed into one shared instruction paragraph and reconstruction check, with nonbreaking graph table references. The README documents current layout defaults and same-page usage.

See `compact-layout.md`, `compact-content.md`, `compact-audit.md`, `compact-english.md` and `compact-dark.md` for commands, independent measurements and changed-file evidence.

Temporary generated compiler output, caches, review PDFs/PNGs and replacement staging files were removed after final verification. Authored input scripts and Markdown reports remain. The published PDF checksum was checked before and after cleanup.
