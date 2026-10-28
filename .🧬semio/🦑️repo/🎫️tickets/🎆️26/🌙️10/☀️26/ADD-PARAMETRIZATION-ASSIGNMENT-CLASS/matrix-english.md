# English Matrix PDF Verification

## Initial Compile

The original English-light parametrization-en.tex was compiled through the registered Tectonic tool with the exact compiler override, shared ticket cache, encoded bundle URL and search paths retained in latex-variants.md. No source files were copied or changed.

The initial compile exited 1 before producing pages: parametrization-en.content.tex:6 at begin-document reports undefined control sequence SemioChromeNavbarLabelsCompute from the shared semio-window.sty begin-document hook. The package source contains the invocation at line 1996 and an alias definition at line 3695. No earlier errors occurred. This precedes matrix rendering. The parent was notified to coordinate the shared-source defect.

Final verification is pending a successful compile.

## Final Original-Source Compile

After the shared sources settled, the exact original parametrization-en.tex was recompiled through the same registered Tectonic invocation. It completed with exit 0 through TeX, BibTeX, repeated TeX passes and xdvipdfmx. The shared semio-window.sty was not edited. The temporary initial failure and successful diagnostic reproduction are documented in matrix-navbar-fix.md.

## Independent Metadata and Geometry Verification

Bundled pypdf reports:

| Property | Result |
| --- | --- |
| Pages | 5 |
| Page size | 595.28 by 841.89 points on every page, A4 portrait |
| Title | Finger Exercise: Parametrization |
| Author | Architecture |
| Catalog language | en-US |
| PDF bytes | 83,681 |
| Overfull boxes in final TeX log | 0 |
| Underfull boxes in final TeX log | 0 |

The retained verify-matrix-pdf.py passed against the actual final original-source PDF. PDFplumber independently confirmed on both pages 2 and 3:

- Nine columns and nine rows.
- Eight canonical vector artwork cells in the top header.
- Eight blank parameter rows with no text and no artwork in the writing area.
- One exercise with no obsolete Exercise 1 or Exercise 2 headings.
- Native PDF title and language metadata.

## Rendered Visual Verification

The retained review-pdf.mjs rendered all five pages through PDF.js and canvas and reported outside=[]. The five-page contact sheet and full-resolution table pages 2 and 3 were inspected.

Both matrices span the page line width and provide substantial blank writing space. All sixteen canonical icons remain sharp and fit their respective header cells. The first page keeps the three-box example and criteria together. Both graph pages preserve all vertex icons, graph edges and answer rules. Headers, footers, section labels and body text fit within page boundaries with no visible clipping or collisions.

## Temporary Outputs

All compiler and rendering outputs remain under the ticket generated directory for the parent to review and remove:

- 🗑️generated/compiled/parametrization-en.pdf
- 🗑️generated/compiled/parametrization-en.log
- 🗑️generated/review/parametrization-en-contact.png
- 🗑️generated/review/parametrization-en-1.png through parametrization-en-5.png
- 🗑️generated/review/parametrization-en.json

No checked source was copied into the ticket. The authored navbar-probe.tex diagnostic input remains at the ticket root. No source edits were needed during compilation or visual review.
