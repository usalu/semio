# Preserving Text When Changing PDF Fonts

Changing a selected font previously left the source glyph codes attached to the new font. The reproduction uses a Helvetica WinAnsi alias that retains byte 233 and a Times-Roman MacRoman face that needs byte 142 for the same character. The requested face change must preserve both accented letters, the signed spacing adjustment and the unchanged next line.

The staged page action resolves the original character codes through the former font and confirms the destination can represent each Unicode operand. It keeps the original operator and numeric spacing. It reuses a matching admitted font or adds a named standard PDF face. Insertion searches admitted fonts for one that can represent each line, then tests the standard fallback. Direct selected-text edits refuse unsupported characters before publishing a mutation.

The isolated native harness executed the staged page action, the actual font codec and the production content printer. It verified that both retained codes changed to the correct destination mapping, the adjustment remained, and unsupported text and unrecognized faces were refused. The raw Type1C subset probe also refuses a Unicode character absent from the actual glyph program. MuPDF rendered the resulting stream with identical text, the requested face for the first line, and an unchanged second line. The staged registered regression opens the same file, invokes the normal action and checks a fresh reopen of the saved bytes.

The font edit is staged with the raw CFF codec and reserved source captures. The repository reader measured 3,988 test-active lines for the page package and 2,269 for the font package with this overlay. The whole-page text projection uses its existing separate action and is outside this targeted fix.

- [Original and edited MuPDF witness](📜️font-reassignment-oracle.py)
- [Native page action, font and print probe](📜️font-reassignment-probe.py)
- [Staged font and page implementation](📜️font-reassignment-stage.py)
- [Registered action and reopen regression](📜️font-reassignment-editor-stage.py)
