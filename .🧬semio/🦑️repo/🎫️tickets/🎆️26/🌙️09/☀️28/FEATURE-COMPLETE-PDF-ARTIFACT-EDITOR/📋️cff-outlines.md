# Embedded OpenType CFF Outlines

The PDF font codec now uses a first-party CFF 1 reader for embedded OpenType outlines. It borrows the CFF header, font dictionaries, CharStrings INDEX and subroutine indexes. It resolves local subroutines for ordinary name-keyed fonts and for CFF CID fonts using FDSelect formats 0 and 3. Type 2 line, cubic, flex and subroutine operators emit native closed contours. Operand count, subroutine depth, instruction count and emitted segments have explicit limits. This probe renders vector outlines without hinting.

FontTools 4.55.0 matched every emitted command and control point in 31 glyphs from three repository Libertinus fonts and the CID-keyed Noto Sans CJK SC font. The independent comparison also matched every CID-to-glyph entry in the authored CFF charset. The shared fixture covers Latin curves, accented forms, a combining mark, CJK ideographs and spaces.

A separate MuPDF run opened a 4,177-byte PDF embedding a subset OpenType CFF font. MuPDF recovered all four Unicode characters at the declared baseline positions and confirmed visible ink in each glyph region.

The PDF codec uses authored WinAnsi or Standard glyph bindings for qualifying nonsymbolic simple fonts and the CFF charset for qualifying CID-keyed programs without an explicit CID-to-glyph map. The page painter and text clipping use the same outline. The live production reader passed the independent outline and charset comparison again. A source-only production codec probe also opened the fixture, resolved the embedded font stream, decoded and re-encoded all four WinAnsi characters, verified the PDF-declared 500-unit widths and matched four distinct mapped glyphs with cubic outlines. The package closures measure 3,493 active lines in the PDF model, 1,936 in the font codec and 3,965 in the page editor under test configurations. Raw Type1C, raw CIDFontType0C, CFF2, vertical positioning, legacy composite charstrings and nondeterministic charstring operations remain separate paths. Runtime editing, Save and mounted renderer verification remain in progress.

- [Language-neutral glyph fixture](🧫️cff-outlines.json)
- [FontTools comparison](📜️cff-outline-oracle.py)
- [MuPDF embedded-font fixture](📜️cff-pdf-oracle.py)
- [Production PDF font codec probe](📜️cff-file-codec-probe.py)
- [Production reader](../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules/🔤️fonts/✒️cff/🦀️.rs)
- [Adobe CFF specification](https://adobe-type-tools.github.io/font-tech-notes/pdfs/5176.CFF.pdf)
- [Adobe Type 2 charstring specification](https://adobe-type-tools.github.io/font-tech-notes/pdfs/5177.Type2.pdf)

## Raw CID-keyed CFF

The staged follow-up reads a horizontal raw CIDFontType0C program directly. It maps CIDs through the authored CFF charset, applies the Top DICT and selected Font DICT matrices in PDF font space, and uses the existing vector painter. The 2,972-byte fixture shows four CJK characters at distinct positions using sparse CIDs. MuPDF checked the text, four baseline positions and visible glyphs. The staged native codec matched FontTools’ exact sparse CID-to-glyph mapping and every normalized path command and control point. The original 31 OpenType outline and full CJK charset comparison still passes. The raw decoder and painter are staged while the registered and mounted builds run.

- [Raw CID CFF file and MuPDF comparison](📜️raw-cff-pdf-oracle.py)
- [Native raw font codec probe](📜️raw-cff-codec-probe.py)
- [Staged native decoder and painter](📜️raw-cff-stage.py)
- [FreeType CID font matrix implementation](https://raw.githubusercontent.com/freetype/freetype/master/src/cff/cffobjs.c)

## Raw Name-keyed CFF

The second staged path resolves Type1C glyphs by original CFF name. It reads predefined and explicit charsets, the CFF string index, standard or authored built-in encoding and PDF Differences. The two authored files show the same four characters and produce the same independent MuPDF ink counts. The native codec matched the FontTools glyph IDs and declared 500-unit widths. The authored range-and-supplement contract also covers an affine FontMatrix, a custom name from the String INDEX and two character codes for a single outline. FontTools matched the names, range and matrix. MuPDF independently rendered the supplemental code as a pixel-identical alias. The staged ExpertEncoding table follows all 256 Appendix B entries. The neutral CFF contract also verifies expert charset names, including the published code 175 to SID 313 mapping.

- [Type1C MuPDF fixtures](📜️simple-cff-pdf-oracle.py)
- [Native Type1C file codec probe](📜️simple-cff-codec-probe.py)
- [Staged name and encoding resolution](📜️simple-cff-stage.py)
- [Neutral CFF schema test and independent oracles](📜️cff-contract-stage.py)
