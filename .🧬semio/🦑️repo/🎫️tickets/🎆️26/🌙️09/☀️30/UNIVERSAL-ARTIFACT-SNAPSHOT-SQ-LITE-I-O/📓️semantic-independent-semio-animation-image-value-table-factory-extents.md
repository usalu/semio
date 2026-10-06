# Semio Animation, Image, Value and Table Factory Extents

[Source per-table observations](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/semio-animation-image-value-table-original-independent-null-zero-extents.json) and [animation/image original Native-matched observations](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📥️inputs/semio-animation-image-native-matched-and-value-table-source-original-independent-null-zero-extents.json) were measured through actual Source providers and independent Bun SQLite, with NULL zero. All databases passed integrity and foreign-key checks. No Native test ran.

| Subset | Source full rows/bytes | Original Native-matched full | Retained empty rows/bytes | Schema | Tables / columns |
|---|---:|---:|---:|---:|---:|
| animation | 61 / 3260 | 184 / 10080 | 1 / 31 | 2217 | 8 / 13 |
| image | 21 / 835 | 21 / 835 | 3 / 131 | 1205 | 4 / 7 |
| value | 20 / 542 | not established | 2 / 45 | 1331 | 5 / 8 |
| table | 39 / 1057 | not established | 7 / 233 | 1683 | 7 / 8 |

Animation's original Native constructor repeats all five channel kinds and five keyframe variants in every original timeline. The Source constructor has a populated first timeline, empty second timeline and one special last timeline. The Native-matched Source measurement hand-reproduces the actual constructor including timeOrder modulo three; the 3260 Source constant cannot qualify that 10080-byte original Native fixture.

Image's Native constructor spells raw frames, delays and ICC explicitly; independent matched projection has identical extents, but equal total bytes alone does not prove carrier identity. Preserve exact frame octets and ICC presence independently. Image empty clears frames only, retaining dimensions, colorspace, bitDepth, ICC and duplicate metadata. Animation empty clears timelines. Value empty replaces root with Null and clears nodes. Table empty clears rows while retaining column declarations; do not substitute zero columns or a differently named schema for this metadata-retaining case.

Table's original Native fixture differs from Source: it has one row, Bool true and the fixture column descriptors; Source has two rows with nested list/map/ref values and Bool false in the first row. Its 39/1057 Source authority therefore requires a new matched Native case. Value's original Native constructor requires additional direct readback before assigning Source constants. Duplicate map keys, literal integer/float lexemes and unresolved semantic Ref identities remain original domain data and must not be normalized by a generic ownership forecast.

Actual original Native factory authorities: [animation](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:3), [image](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:3), [table](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🧬️schema/📸️snapshot/🧪️tests/🪶️sqlite/🦀️.rs:10).

All four belong to the actual stdio-semio-rs owning package. Original per-subset typed public IO, native construction, deep retirement and controlled cancellation laws remain required alongside new full relational cell/metadata limits. These measurements supply closed expected values, not runtime qualification or a blanket admission-provider audit.
