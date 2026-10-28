# One-Off Assignment PDF Authoring

The repository-native TeX compilation remains unavailable in this session. The requested German PDF is therefore authored as a one-off vector PDF through bundled ReportLab, invoked behind the registered repo Python tool. This creates no production dependency and changes no framework engine source.

The retained input script is `create-assignment-pdf.py`. It reads the actual German authored TeX, shared TikZ geometry, and language-neutral fixture in place, without copying tracked source. It validates the explicit five-page sequence and every geometry tuple before drawing. The output is initially `🗑️generated/Fingeruebung-Parametrisierung.pdf` for root visual review.

The document uses A4 monochrome vector artwork, the repository's embedded Anta heading font, standard Helvetica body text, three common-scale worked boxes with a good dimension table, all three isolated counterexamples, two blank eight-row parameter tables, matching complete K8 graphs, and 28 separate handwritten edge entries per graph. Exercise tables contain no type names or row numbers. All five parameter headings and all parameter value cells remain blank. Box example values do not reveal exercise answers.

Root completed the PDF skill operation marker before this authoring pass. Root owns rendered page inspection, final retained destination and delivery. No separate marker is emitted by this script.

## Generated PDF Structural Validation

Bundled pypdf reopened the created file and confirmed exactly five pages and `/Lang=de-DE`. Page text extraction contains all three criteria. Each graph page contains every one of the 28 edge labels exactly as authored from the complete unordered pair list. The PDF contains zero embedded raster images, so all diagrams and tables remain vector artwork. Extracted text lengths by page are 1640, 647, 933, 681 and 921 characters. Visual review remains root-owned; these checks do not replace it.

## Font and Legibility Review

Root's first raster review exposed unreliable unembedded standard Helvetica rendering. The script now embeds bundled Vera regular and bold as an explicitly mapped Body font family, retaining embedded repository Anta headings. Body text is 10.5pt with 14.2pt leading; smaller explanatory text is 9.4pt with 12.6pt leading. Metadata and table captions use at least 9pt where space permits. The ledger column captions are shortened to avoid crossing the right margin with the wider font.

Bundled pypdf confirms all three actual fonts (Anta, Vera regular, Vera bold) have embedded FontFile2 descriptors. pdfplumber places the final body text on pages 1-5 at 240.56, 261.56, 270.56, 257.11 and 270.56mm from the top, before the footer rule at 278mm. The retained check script is `check-pdf-layout.py`. Output remains five pages with `/Lang=de-DE`.

## Second Render Repairs

Root's second raster pass found two concrete overlaps after font embedding. Both are repaired: graph ledger captions now read only `Parameteränderungen`, preventing collision with the next Kante column. Each table-page second instruction paragraph is positioned from the measured lower edge of the first paragraph plus 3mm, rather than a fixed top. The script asserts that the second paragraph ends by 75mm before the table begins at 78mm. Both table-page assertions pass. The regenerated PDF remains five pages; root is re-rendering all pages for final visual verification.

## Superseding Final-Output Requirement: 2026-10-27

The user explicitly requires the real LaTeX-based print product, the new assignment class and the same established styling. Therefore the historical one-off ReportLab main-document output and script are not the final rendering path. No new main PDF was generated after this clarification. The corrected document consumes the exact original Metabolism capsule and ellipsoid-capsule SVGs through a vector PDF resource derivation plan; root owns native asset staging and LaTeX compilation. Historical rendering scripts remain preserved as required ticket inputs, but must not generate the final deliverable.
