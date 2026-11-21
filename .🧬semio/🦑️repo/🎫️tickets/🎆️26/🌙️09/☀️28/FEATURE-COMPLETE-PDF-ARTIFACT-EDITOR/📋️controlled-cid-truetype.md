# Controlled CID TrueType PDF Admission

A 49,719-byte raw and 49,418-byte Flate PDF share one embedded TrueType program across 48 pages. The language-neutral input defines four physical glyph IDs and their widths for `AgRé`. An independent FontTools build checked the glyph IDs and font metrics. PyMuPDF 1.24.14 verified the original program bytes, both text runs on every page, and each full-page RGB raster. The existing controlled lift refused the same genuine file with `Deferred(Resources)` after 28 semantic work units.

The staged change extends the existing retained font owner and direct ToUnicode parser for one shared horizontal `Type0` font. It qualifies one indirect `CIDFontType2` descendant using `Identity-H`, a default width and one bounded optional explicit width run, and a direct two-byte ToUnicode CMap. It borrows the descriptor, program and Unicode map. Physical two-byte text codes and `TJ` adjustments survive in typed operators. The snapshot records the direct program and map roles, and each page's original program role carries all four additional font object identities.

The exact source harness admitted both PDFs through the product graph and document cursors under one-work-unit turns and a 32 KiB copy ceiling. The raw fixture used 147,353 graph and 356,567 semantic work units. The Flate fixture used 147,551 graph and 356,567 semantic work units. Cancellation retired the private font allocation during program hashing. The original OpenType, JPEG, sampled image and Standard 14 font regressions passed in the same harness.

The current content codec and native raw writer produced a PDF from the admitted physical text operators. MuPDF reopened it with an identical embedded program, identical Unicode and identical RGB raster hashes for all 48 pages. The repository source reader passed macOS, Linux, Windows and WASI cfg measurements. The largest line closure was the lift test at 3,952 active lines. The inferences test remained exactly 32 source files.

The change is staged while the approved product epoch is compiling. Mounted Open, edit, Save and reopen for this additional profile await its own product release. Vertical maps, indirect or nonidentity CID-to-GID maps, multiple disjoint width runs and non-Identity CMaps are outside this qualified path.

- [Neutral CID fixture](🔣️controlled-cid-truetype.json)
- [Isolated source stage](📜️controlled-cid-truetype-stage.py)
- [Graph, lift, and writer regression](📜️controlled-cid-truetype-test.py)
- [Independent FontTools and MuPDF oracle](📜️controlled-cid-truetype-oracle.py)

## Released Candidate

The reviewed five-path roster was applied at 22:39 UTC. All live SHA-256 identities matched their staged postimages, including the neutral raw/Flate unit fixture and the shared font-law extraction. The final font test closure measures 12 files, 3,198 active lines and 174,135 bytes. The product's real browser and native test captures include the typed font extension. The mounted test now recognizes outlines using line, quadratic and cubic path segments and can run against either the OpenType or CID TrueType fixture directory.
