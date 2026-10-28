# Final PDF Acceptance Audit

## Source Acceptance Checklist

- Exactly five German A4 portrait pages: overview and worked boxes; family one table; family one graph and ledger; family two table; family two graph and ledger. Title Fingerübung Parametrisierung.
- Overview shows exactly three good boxes A=(2,1,1), B=(3,1,1), C=(3,2,1), fixed L/B/H axes and consistent units. Eindeutigkeit counterexample omits B; B and C share (L,H)=(3,1). Konsistenz counterexample gives A→C=(+1,+2,0), conflicting with via B=(+1,+1,0) and cycle (0,-1,0). Minimalität counterexample uses invertible (p,q,r)=(L,L+B,H), requiring two changes rather than one for A→B.
- Family one row geometry is capsule/cylinder alternating at (length,diameter)=(12,6),(12,6),(18,6),(18,6),(12,9),(12,9),(18,9),(18,9). Family two independently uses (14,5),(14,5),(21,5),(21,5),(14,8),(14,8),(21,8),(21,8). All sixteen kind/length/diameter combinations are distinct globally. Cylinders have visibly planar ends; capsules have rounded ends.
- Each table contains eight unlabeled specimen rows, an unlabelled image/reference column, exactly five blank learner value columns, and the single header Parameter. No suggested parameter names or numerical answers. No row numbers or type names in the tables. Table source specifies 14 mm row height and 27 mm specimen column width.
- Each graph repeats that family geometry in table row order at vertices 1–8. Each unordered pair occurs exactly once, giving 28 straight edges and no self loops. Source ring radius is 32 mm. Vertex shapes overlay edges with a white backing.
- Each graph page includes 28 blank writing entries, exactly one for each pair 1–2 through 7–8, arranged in two columns of 14. Source ledgers use 6.3 mm baseline spacing and 67 mm writing rules. Directions are from smaller to larger row number; reverse signs for return. Require a cycle test and minimal-change comparison.
- Student name/date area, comfortably readable German typography, no overflow/clipping, monochrome print legibility, sufficient blank writing space. Root performs visual inspection.

## Pending Verification

Awaiting final PDF path from root. Source inspected through repo tools exec only. No source edits.

## Independent PDF Verification — Initial Final Artifact

Audited 🗑️generated/Fingeruebung-Parametrisierung.pdf using bundled pypdf and pdfplumber via repo tools exec --light python. Five pages each measure 595.3 × 841.9 pt (A4 portrait). Metadata title is Fingerübung Parametrisierung, author Semio - Architektur, subject states the three principles, and catalog /Lang is de-DE. German umlauts extract correctly; console replacement characters were terminal encoding only and escaped JSON confirmed the original Unicode.

Extracted text contains all three worked tuples, all three isolated counterexamples, inverse coordinate transformation, both independent exercises, explicit lower-to-higher edge direction, reversal of signs, cycle test and final comparison. Each graph page extracts all 28 unique edge labels exactly once.

Independent vector geometry check reconstructed the eight ring locations and matched actual PDF line endpoint bounds against every unordered pair: 28 of 28 on page 3 and 28 of 28 on page 5. Table pages each have seven vertical grid boundaries (six columns: specimen reference plus five learner columns). Cropping the complete learner-entry surface of each table yields zero text; cropping both ledger writing columns also yields zero text. Each table has 24 curve objects, consistently three per specimen, and eight alternating capsule/cylinder rows. The shape dimensions in the PDF follow the source scale 1.15: first family capsules measure 13.8/20.7 mm lengths with 6.9/10.35 mm diameters; second family 16.1/24.15 mm lengths with 5.75/9.2 mm diameters. Cylinder end ellipses have width 2.3 mm and matching diameters, and straight body endpoints give the corresponding full lengths. Thus all sixteen kind/length/diameter specimens are distinct.

No blocking semantic, count, geometry or blank-writing-surface defect found. Tables intentionally carry no specimen names or row numbers. Name/date fields from the supplemental checklist are absent, but were not an explicit acceptance requirement. Root owns final raster visual inspection and delivery destination. No implementation, generated output or runtime file changed in this audit.

## Retained Final PDF Reverification

Verified exact retained file C:/git/semio/output/pdf/Fingeruebung-Parametrisierung.pdf, 68,721 bytes, SHA-256 5dff42bab543ae3d6799faafb0dcf1df9b10bcba626e704a4913e3a7f19b50bd. Independent pypdf/pdfplumber rerun passes: five pages, de-DE language, zero characters in both five-column learner table regions, 28 matched geometric complete-graph edges on each graph page, 28 unique ledger pair labels per graph and zero text in all ledger writing areas. Specimen pages each retain 24 curves (eight specimens).

All fonts actually used by extracted glyphs are embedded: BitstreamVeraSans-Roman, BitstreamVeraSans-Bold and Anta-Regular. An unused Helvetica resource remains registered by ReportLab but no extracted glyph uses it; it therefore creates no substitution risk for the page content. Final font/page-spacing revision introduces no independently detected acceptance defect. Root completed every-page visual inspection. Final PDF left untouched.
