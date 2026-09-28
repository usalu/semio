# XML Document-Boundary Independent Audit

Date: 2026-09-28

Source-only audit of the XML/SVG document-boundary cut. No production file was changed and no Cargo/native command was run. The implementation report reviewed was 🔍️research/🧬xml-document-boundary-fidelity-implementation-2026-09-28.md.

## Findings

### P1 — PPTX public XML facets lose epilog and the doctype insertion position

PPTX Rust retains the canonical XML model, including both fields: [canonical import](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:14) and [PptxXmlPart](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:115). Its public schemas independently redefine an obsolete XML model:

- Snapshot TypeScript omits both fields in the type and decoder: [type](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts:26), [decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts:102).
- Snapshot JSON rejects both with additionalProperties false: [document](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json:174), [doctype](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔣️.json:332).
- Snapshot Proto omits doctype and epilog, and GraphQL has only declaration, prolog, root: [Proto](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🛰️.proto:9), [GraphQL](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔗️.graphql:7).
- Replacement diff TypeScript and JSON repeat the stale shape: [diff decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🟦️.ts:102), [diff JSON document](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json:167), [diff JSON doctype](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🔣️.json:325).
- The set-snapshot mutation JSON also embeds the stale shape: [document](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json:186), [doctype](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📸️set-snapshot/🧬️schema/🔣️.json:344).

A PPTX XML part with a doctype after a prolog comment/PI or with an epilog cannot cross these public boundaries. JSON rejects it and TypeScript discards the data. Replace these copied facets with the canonical XML public facet, or update every copy plus its validation. Test snapshot, XML-parts replacement diff, and set-snapshot transcoding with a positioned doctype and nonempty epilog. This is not the in-flight DOCX canonical XML-parts work.

### P1 — XML and SVG state decoders admit invalid document boundaries

Both XML state decoders construct XmlDocument without validate_xml_document_boundaries: [text decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs:35), [binary decoder](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs:63). The builder also accepts raw snapshots and returns them without a final gate: [ingress](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🦀️.rs:100), [build](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🦀️.rs:117). SVG repeats the two unchecked decoder paths: [text](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs:41), [binary](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔨️modules/🧬️mutation-support/🦀️.rs:69).

Thus a serialized state can contain non-misc prolog/epilog nodes or an out-of-range doctype position. XML DSL and pack ingress route directly to those decoders: [DSL](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:860), [pack](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:889). The checked exporter returns an error, but the compatibility writer panics on this state: [panic wrapper](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:321).

Validate every decoded/built snapshot and map the error to text, pack, or diagnostic results. Add XML and SVG DSL/binary cases for an illegal epilog node and an out-of-range doctype position. Final diff application is already checked.

### P2 — Grammar derivatives are stale and not covered

XML snapshot ANTLR/EBNF only permit declaration?, doctype?, element, misc*. They cannot describe a doctype between prolog nodes and their doctype form ends at the first greater-than sign: [ANTLR](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🅰️.g4:3), [EBNF](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/📝️text/🔤️.ebnf:2). SVG diff ANTLR/EBNF omit epilog=: [ANTLR](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📝️text/🅰️.g4:4), [EBNF](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/🔺️diff/📝️text/🔤️.ebnf:2). Conformance currently only parses COMPONENT_GRAMMAR_SEMIO values: [test](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/🧪️tests/🔬️unit/🦀️.rs:431).

Remove these non-authoritative derivatives or update and execute them. Include a positioned doctype with quoted greater-than plus SVG epilog=.

### P2 — The added independent quick-xml projection does not compare doctype content

The package now has quick-xml as a dev dependency: [manifest](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust/Cargo.toml:33), and compares source/exported boundary event sequences: [fixture law](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:40). This correctly removes the earlier self-parser-only coverage gap for event ordering, comments, PIs, and root name.

It converts every independent DocType event into an empty string: [projection](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs:19). The external oracle therefore cannot detect a changed doctype payload. Include the raw normalized quick-xml doctype payload in this tuple.

## Verified paths and validation status

The core XML parser/writer validate and preserve prolog, doctype insertion index, and epilog: [writer](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:326), [validation](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:374). XML/SVG diff application validates final state; XML canonical public TS/Proto/GraphQL/JSON, Semio conversion, and OPC preserve the fields. The coordinating agent corrected the invalid fixture PUBLIC identifier and added the package-local quick-xml law during this audit. It was not executed here.
