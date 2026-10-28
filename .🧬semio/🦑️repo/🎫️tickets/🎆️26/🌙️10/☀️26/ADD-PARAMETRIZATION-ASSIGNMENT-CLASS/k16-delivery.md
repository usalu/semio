# K16 Oval With Twenty Selected Parameter Changes

Delivered on 2026-10-27: `output/pdf/Fingeruebung-Parametrisierung.pdf`.

The final German assignment has three A4 pages: the worked box example, both compact capsule matrices, and one complete K16 graph with twenty selected transitions and writing spaces. The graph contains all 120 pairs faintly, with the sixteen outer-ring edges and four random middle chords highlighted. Only those twenty highlighted pairs require parameter changes.

The four reproducible middle chords are (1,10), (2,12), (4,10) and (5,13), selected with seed 20261027. Global vertices1–8 correspond to tableA and vertices9–16 to tableB. The ledger has exactly twenty matching blank entries in two columns of ten. A marked closed-cycle test and two reflection lines fit on the graph page.

## Final Artifact And Actual Measurements

- Size:114,937 bytes. SHA-256:`21470ff6110f23ee9fc612d32bee545d7e03ad1dbb11ab75a9453814c59d02d5`.
- Native title Fingerübung Parametrisierung, author Architektur, language de-DE; LaTeX/hyperref and xdvipdfmx output.
- Visible oval width170.001mm and height83.602mm; the prior visible graph height was83.60209mm. The vertical vertex radius remains32mm; only horizontal geometry expands.
- Independent actual-PDF checks confirm sixteen canonical vector node forms, sixteen global labels, every base node degree15 and all120 distinct base pairs, exactly sixteen ring plus four middle overlays, and exactly twenty matching blank ledger labels.
- Both matrices remain on page2 with nine rows and nine columns each. Actual writing-row pitch is6.14mm including rules; artwork padding2.13–2.23mm and maximum center offset0.030mm.
- All six fonts are embedded. All pages are A4. No overfull, underfull or missing-character diagnostics remain.

The parent rendered the final German bytes with PDFium and individually inspected all three pages. The graph fills the text width without clipping, vertex labels are clear, the selected ring/chords stand out, ledger rules have a clear gutter and all twenty writing spaces and reflection lines fit. English light and German dark independently pass the same native graph/matrix/metadata checks and were rendered and visually reviewed. The dark artwork panels make its visible graph height84.242mm while preserving the32mm vertical radius.

## Source And Tool Verification

Schema-first fixture version5 and bilingual content describe one K16 and the twenty selected transitions. The registered Bun verifier passes AJV schema mutation cases, mathjs combination/cycle checks, D3 edge and seeded selection checks, and canonical SVG/PDF source hashes. The bundled Python lxml oracle independently passes all sixteen original SVGs.

Initial actual checks correctly found two real defects: visible graph width165.18mm and a ledger-column collision. Zero horizontal label padding plus trimmed measurement whitespace now gives full visible width without vertical scaling. Adjusted label and rule positions provide a clear writing gutter.

Concurrent authored header, course, footer-logo and AssignmentTable chrome changes were retained. The direct native verification route needed the existing print product's reference-name projection before loading the footer PNG. The retained portable `prepare-k16-input.ts` calls the actual `printSourceReferenceMapV1` and `printSourceReferenceTextV1` functions to produce a derived compiler input. A directory junction on Windows or directory symlink on other platforms supplies the existing image directory without copying source files or changing PNG link counts. Its compiler search path must precede the original LaTeX directory. No shared-logo/style fallback, source backup, application runtime dependency or original resource modification was introduced.

All tool calls used the repository interface. The full managed build's unrelated source-admission prerequisite remains outside this document verification; the reported compilations are actual registered Tectonic0.16.9 runs against the product class, packages, fonts and original document content.

Evidence is retained in `k16-content.md`, `k16-geometry.md`, `k16-audit.md`, `k16-english.md`, `k16-dark.md` and `k16-footer-diagnosis.md`. Authored preparation, rendering, diagnostic and verification inputs remain in the ticket. The owned generated directory and the directory junction were removed after verification; the original footer image and its link count were checked before and after cleanup. The final PDF hash was checked after publication and cleanup.
