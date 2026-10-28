# Named Nonzero Delta Delivery

Six ledger prefixes in each localized content file now use named, semicolon-separated parameter changes. Positive changes read L+1; B+1; H+1; followed by the existing form change; negatives read L-1; B-1; H-1;. The two transitions with unchanged dimensions show only the form change. Exact reverse-byte comparison verified that every other localized content byte, including categorical phrases and arrows, was preserved.

The schema-first v16 source regression rejected the tuple baseline and passed both updated locales. The independent native PDF regression rejected the old zero tuple and passed all six new values, pair mappings, signs, absent zero terms, visible semicolon spacing, 9 pt font, 6.3 mm pitch and 64.6 mm rules. Maximum rendered value width is 59.772256 mm. Independent Pillow FreeType and ReportLab width measurements also passed for both languages; no styling change or font reduction was required. English native compilation was unnecessary for this focused text-only update.

Native chip, worked-matrix and task-matrix checks passed. All three current German pages were rendered and visually reviewed. The native cylinder oracle was updated precisely for concurrent authored hidden-line work: two solid curves and one dashed hidden rim, preserving strict geometry, dimensions and centering checks. That work was retained.

Output: C:/git/semio/output/pdf/Fingeruebung-Parametrisierung.pdf

Three A4 pages; 125852 bytes.

SHA-256: 3ff0b77d9cfb3f27c524958cd21f0e4d0c3d55c3d111d12a23819de765131226

Detailed retained evidence: delta-content.md, delta-style.md and delta-audit.md. Temporary generated files were removed after all worker processes exited. The compiler image junction was removed first; original logo hashes/link counts and the final PDF hash were verified after cleanup.
