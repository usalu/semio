# SVG, XML and TXT Original Factory Extents

[Independent per-table Bun SQLite observations](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/svg-xml-txt-original-independent-null-zero-extents.json) preserve NULL zero, INTEGER/REAL eight, UTF-8 TEXT and actual BLOB lengths. All six measured databases passed integrity and foreign-key checks. No owning test or Native gate ran.

| Owner | Original Source full rows/bytes | Retained metadata empty rows/bytes | Canonical schema bytes | Tables / maximum columns |
|---|---:|---:|---:|---:|
| SVG | 41 / 1030 | 14 / 393 | 2637 | 13 / 7 |
| XML | 40 / 1040 | 21 / 625 | 2637 | 13 / 7 |
| TXT | 4 / 132 | 1 / 29 | 437 | 2 / 4 |

SVG and XML clear only root children: root name/attributes, declaration, doctype including original prolog position/entities, prolog and epilog survive. SVG requires an svg root; an absent root is rejected by its Source projection and is not an equivalent empty case. XML's more permissive logical SQL supports absent roots, but deleting prolog while retaining doctype position is not an established natural-wire parity case. The retained-root recipe avoids that invented equivalence. TXT clears lines and trailingNewline, preserving schema and lineEnding.

SVG and XML original Native factories import fixture svgText/xmlText as UTF-8 documents, while Source tests handcraft typed trees. Therefore the Source counts above require a matched handconstructed Native tree, or a separately measured parsed Source carrier, before use as Native constants. TXT's original adjacent JSON has direct schema, lines, trailingNewline and lineEnding fields and can supply the same literal owner in both implementations.

Actual authorities:

- [SVG Source factory](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts:16) and [Native parsed factory](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:76).
- [XML Source factory](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts:41) and [Native parsed factory](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:4).
- [TXT Source literal owner](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🟦️.ts:62) and [original Native literal fixture](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:158).

Existing owning commands use @semio-tech/stdio-svg-rs, @semio-tech/stdio-xml-rs and @semio-tech/stdio-txt-rs with test-snapshot-sqlite-source/native. Preserve their package 📜️script.ts selectors and all original controls. XML also has original custom native document/node cursor, deep stack, cumulative multi-document and bounded text laws: a complete borrowed semantic sink must retain those caller controls and stream offsets, rather than replace them with generic derived Record admission. TXT already has exact native limit/cancellation and arbitrary literal-schema laws; new complete SQL constants supplement these rather than replace their original byte and line semantics.
