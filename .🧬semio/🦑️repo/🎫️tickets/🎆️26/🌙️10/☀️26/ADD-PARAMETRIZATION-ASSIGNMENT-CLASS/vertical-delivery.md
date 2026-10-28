# Vertically Centered Matrix Text Delivery

The shared filled-matrix cell builder now centers each content box using its measured height and depth. It retains the symmetric six-millimeter writing strut. Both label and value cells use the same helper. Header icons, blank rows, frame geometry, automatic right chips, content, graphs and ledgers are unchanged.

Test-first validation: the four language-neutral style contracts failed before the fix and passed afterward. The schema-first v15 parametrization verifier passed, as did the 19 generic assignment tests with 78 expectations. An independent PDFium/Pillow oracle measured actual rendered ink rather than font ascent/descent boxes. All 16 original filled cells failed the 0.25 mm contract; their maximum error was 1.3899 mm. After the fix, the maximum error is 0.0459 mm in German and 0.0512 mm in English. Native worked/task matrix and chip checks passed. All three pages of each language were rendered and visually reviewed without clipping or overlap.

The unchanged localized content hashes are recorded in vertical-content.md; source contracts and detailed native measurements are in vertical-style.md and vertical-audit.md.

Output: C:/git/semio/output/pdf/Fingeruebung-Parametrisierung.pdf

Three A4 pages; 127105 bytes.

SHA-256: 026e4568763835a5694e66395528af406fb69287b898c5c4d12950996d0d4317

English validation preview: 127103 bytes, SHA-256 97ff1b2b5f7dc50fbd6134757d82a6a4b351a8e875176ed5fc33dd62eb64c325.

All worker processes exited before temporary generated files were removed. The compiler image junction was removed first; both original logos retain their recorded hashes and link counts. The published PDF hash was checked after cleanup.
