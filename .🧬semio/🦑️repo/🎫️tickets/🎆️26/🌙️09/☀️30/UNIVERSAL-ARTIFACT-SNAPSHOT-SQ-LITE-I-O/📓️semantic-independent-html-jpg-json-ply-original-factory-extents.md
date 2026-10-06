# HTML, JPG, JSON and PLY Original Factory Extents

Independent Bun SQLite measurements are retained in [the complete per-table input](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/html-jpg-json-ply-original-independent-null-zero-extents.json). INTEGER/REAL cells cost eight bytes, TEXT costs UTF-8 bytes, BLOB costs its length and NULL costs zero. All eight databases passed integrity and foreign-key checks. These are Source projection measurements, with no Native compilation or runtime qualification.

| Owner | Full rows/bytes | Metadata-retaining empty rows/bytes | Canonical schema bytes | Tables / maximum columns |
|---|---:|---:|---:|---:|
| HTML | 38 / 864 | 4 / 87 | 1245 | 8 / 5 |
| JPG | 182 / 5992 | 179 / 5832 | 4750 | 14 / 13 |
| JSON | 20 / 552 | 2 / 40 | 1397 | 4 / 6 |
| PLY | 62 / 1893 | 3 / 122 | 3213 | 8 / 8 |

HTML emptiness clears root children while preserving the mandatory root, attributes, doctype and schema. A null root is not a supported empty Snapshot. JPG clears pixels and JFIF thumbnail RGB data while preserving dimensions, headers, quantization/Huffman tables and other segments. JSON uses an empty object root. PLY clears elements while retaining schema, comments and format.

HTML's original Native factory parses fixture htmlText whereas Source constructs an explicit tree. JSON's Native factory parses fixture jsonText whereas Source constructs an explicit value tree. Their measured Source constants must not be assigned to those Native factories without matching their actual parser results. JPG and PLY use the same adjacent JSON fields in both implementations; PLY Source explicitly converts binary32/binary64 words and BigInt counts. Floating cases require word-preserving SQL reprojection rather than ordinary NaN equality.

HTML currently has a domain-owned controlled markup reader, paid arena/frontiers, and normalized_domain semantic row/byte admission before final HtmlNode construction. It is not a missing generic DslRecord census: intermediate Content owns paid strings already, so a complete earlier borrowed admission would require the actual markup parser sink. Its canonical output schema is stdio.html, another reason not to infer parity with the handcrafted Source factory. JSON delegates native decoding to its owned_pack module; that actual custom boundary needs separate inspection, not an assumption that a generic derived binder applies.

Actual Snapshot authorities:

- [HTML SQLite provider](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:50) and [controlled markup boundary](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:76).
- [JPG original Native laws](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:4).
- [JSON owned boundary](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs:49).
- [PLY original Native fixture](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:16).

Registered owning commands are @semio-tech/stdio-html-rs, @semio-tech/stdio-jpg-rs, @semio-tech/stdio-json-rs and @semio-tech/stdio-ply-rs with test-snapshot-sqlite-source or test-snapshot-sqlite-native. Their existing package 📜️script.ts routers preserve Source selectors: HTML one Snapshot path; JPG producer plus Snapshot; JSON GeoJSON, I-JSON and base Snapshot; PLY one Snapshot path. HTML/JPG/JSON retain the original component-app-assembly test feature and JSON retains its separate owned native-schema-check command. No filter, grant or deadline change is proposed here.
