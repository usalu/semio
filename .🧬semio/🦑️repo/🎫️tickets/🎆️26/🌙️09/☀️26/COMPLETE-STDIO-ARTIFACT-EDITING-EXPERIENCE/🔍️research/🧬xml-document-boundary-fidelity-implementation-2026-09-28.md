# XML Document Boundary Fidelity Implementation

## Frozen authority

- `XmlDocument.epilog: Vec<XmlNode>` is the only persisted representation for logical miscellaneous nodes after the root element.
- `XmlDoctype.prolog_position: usize` is the only persisted authority for the doctype's position among `XmlDocument.prolog`. It counts the prolog nodes preceding the doctype.
- Prolog and epilog accept comments and processing instructions only.
- Checked publication rejects a doctype position greater than `prolog.len()` and rejects non-miscellaneous boundary nodes. No writer clamps or normalizes invalid authored positions.
- Raw XML source is not retained as a second document authority.

## Implemented production surfaces

- XML parsing retains epilog comments and processing instructions, rejects duplicate and trailing doctypes, and scans doctype delimiters without treating quoted `]` or `>` as structural closure.
- XML writing interleaves the typed doctype at `prolog_position`, writes the complete epilog, and exposes `xml_document_to_text_checked` for fallible publication.
- XML snapshot and diff text/binary codecs carry epilog and doctype position. Diff apply validates the resulting document before returning it.
- XML and SVG snapshot/diff grammar and protocol descriptions were updated for the added epilog field, doctype position, and diff presence bit.
- SVG snapshot export uses checked XML publication. SVG diff, snapshot state codecs, mutations, editor boundary checks, and public diff facets carry epilog and doctype position.
- Public XML JSON Schema, TypeScript, Protocol Buffers, and GraphQL snapshot/diff facets expose both fields. The TypeScript decoder enforces non-negative integer positions, the prolog bound, and miscellaneous-only boundaries.
- XML-valid and SVG basic/tiny set-snapshot schemas, plus XML/SVG set-doctype schemas, expose the same persisted fields.
- Semio value/XML conversion preserves epilog and doctype position in both directions.
- OPC compact XML output appends epilog, validates document boundaries, and uses the checked XML writer when a doctype is present. OPC archive publication maps boundary failures to a part-specific XML error.
- Narrow non-DOCX `XmlDocument` producers were updated across XML/SVG tests, Semio drawing/value conversion, BCF, XLSX, PPTX, and OPC. DOCX remains assigned to the text/office integration owner to avoid overlapping its active canonical `xml_parts` migration.

## Independent oracle and fixtures

- The neutral `document-boundaries` fixture covers doctypes before, between, and after prolog nodes; epilog comments/PIs; quoted brackets and greater-than characters in external identifiers and entity values; duplicate/trailing doctypes; invalid authored positions; and non-miscellaneous epilog nodes.
- The independent `quick-xml` XML oracle and standalone generator now retain epilog, reject duplicate/trailing doctypes, project doctype position, and write the doctype at its parsed position.
- The independent SVG oracle and standalone generator now retain prolog/epilog and doctype position rather than dropping boundary nodes or moving the doctype.
- XML snapshot tests consume the neutral fixture. The XML field sweep now changes prolog and epilog and verifies their diff replay. OPC has a focused compact-output and invalid-position publication test.

## Source-only validation completed

- `[DEBUG] parsed 155 XML/SVG JSON documents` using Bun `JSON.parse`; no syntax failures.
- `[DEBUG] imported XML/SVG snapshot and diff TypeScript facets` using Bun; no module parse/import failures.
- `[DEBUG] public TypeScript boundary guard accepted valid ordering and rejected invalid authored states` using valid and invalid in-memory public values.
- `[DEBUG] rustfmt source parse exit=0` for the XML/SVG snapshot, diff, oracle, generator, OPC, and Semio value conversion production sources.
- `git diff --check` completed without whitespace errors for the XML/SVG/OPC/Semio/XLSX/PPTX/BCF paths.

## Pending native gate

No Cargo command or native Rust test was run in this task because the coordinating root agent owns the active native sessions and explicitly held new artifact/full-catalog retries until this interface checkpoint. The DOCX owner must finish its narrow literals/copies before the coordinated native check. Generated fixture bytes were not regenerated in this source-only checkpoint.

## Root Independent Oracle Integration

An independent Python minidom/Expat run found the original doctype-between-prolog fixture’s PUBLIC identifier contained illegal `]`/`>` characters. Root corrected that PUBLIC literal to `public-id`, retaining the quoted delimiter coverage in its SYSTEM identifier. The rerun independently accepted all three valid sources with the exact neutral doctype position/prolog/epilog projection and rejected both invalid sources; generated evidence is `xml-boundary-expat-reference-current-1.log`.

The actual XML native package now has dev-only quick-xml0.39.4 and its boundary round-trip law compares the independent parser’s document-boundary sequence and comment/PI payloads for original versus exported XML. This executable assertion is authored but has not run. The already-running XML4 Cargo invocation predates this dev dependency and is not assumed to cover the new law.

## Independent audit repair

- XML and SVG structured-text and binary snapshot decoders now validate the decoded `XmlDocument` before returning it. Both builders run the same validator at their final boundary and return a diagnostic instead of permitting state that an infallible compatibility writer would later reject.
- XML and SVG source tests encode invalid authored state and prove that text decoding, binary decoding, and builder publication reject an out-of-range doctype position. They also cover a non-miscellaneous epilog node.
- The XML snapshot ANTLR/EBNF derivatives now admit miscellaneous nodes on either side of a single semantic doctype position before the root and scan quoted greater-than characters plus internal subsets. The SVG diff ANTLR/EBNF derivatives now include `epilog=`.
- The quick-xml event projection now includes the raw doctype payload after quote-aware whitespace normalization. The existing original/exported comparison therefore checks doctype semantics and spelling content in addition to boundary order, comments, PIs, and the root name.

## PPTX copied XML public facets

- The PPTX snapshot TypeScript, JSON Schema, Protocol Buffers, and GraphQL facets carry `XmlDocument.epilog`, `XmlDoctype.prologPosition`, and the pre-existing canonical `XmlDeclaration.quote`.
- A single TypeScript snapshot parser is now the authority used by `parsePptxArtifact`, replacement `xmlParts` diff parsing, and set-snapshot mutation parsing. It preserves all three fields, validates miscellaneous-only prolog/epilog state, and rejects a doctype position greater than the prolog length.
- The PPTX replacement-diff and set-snapshot JSON schemas carry the same fields. The set-snapshot mutation leaf delegates to the shared parser.
- Neutral fixture `📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧫️fixtures/🧭️xml-document-boundaries/🔣️.json` contains snapshot, XML-parts replacement diff, and set-snapshot values with a single-quoted declaration, a doctype after the first of two prolog nodes, and a nonempty epilog.

## Audit-repair validation

- `[DEBUG] PPTX snapshot/diff/setSnapshot Ajv and TypeScript guards retained declaration quote, doctype position, and epilog; invalid authored position refused` using Ajv 8.20.0 and the first-party public parsers.
- `[DEBUG] parsed 243 XML/SVG/PPTX JSON files`; no JSON syntax failure.
- Exact strict TypeScript compiler invocations used by the artifact package check passed for the XML and PPTX package roots with exit code 0. This confirms removal of the accidental `XmlAttr.prologPosition` field and the typed PPTX artifact parser.
- `rustfmt --edition 2021 --check` passed for the repaired XML/SVG decoder and test sources.
- `bun nx run @semio-tech/stdio-pptx:check` passed after the shared graph lock cleared. The first Nx XML package check was interrupted after extended project-graph contention and produced no compiler result; its exact strict TypeScript invocation passed independently.
- No Cargo/native test was started. The coordinator owns XML5 and the coherent Office/native gates.

## Delta audit raw-ingress repair

- The single XML document validator now checks declaration quote safety, XML `VersionNum`, XML `EncName`, and the artifact's UTF-8 byte contract in addition to prolog/epilog node kinds and doctype position. A declared encoding is accepted case-insensitively only when it is `UTF-8`; conflicting labels are refused before publication.
- Native XML parsing calls the same validator and now rejects a standalone pseudo-attribute other than `yes` or `no`. XML/SVG structured state decoders, pack decoders, builders, diff application, raw artifact conversion, snapshot replacement, and checked writers all converge on that validator.
- `XmlArtifact::from_snapshot` / `SvgArtifact::from_snapshot` and both `set_snapshot` methods are fallible. Snapshot replacement validates before assignment, so a refusal leaves the artifact unchanged.
- XML editor source rendering and every base/basic/tiny SVG editor/viewer image route use checked publication and return `PluginAssemblyError` for invalid state. They no longer reach the infallible compatibility writer on a public snapshot path.
- The neutral XML boundary fixture adds a quote-bearing declaration encoding and an `ISO-8859-1` declaration over non-ASCII `Grüße` content. XML and SVG laws feed both cases through structured text, pack, builder, raw conversion, atomic replacement, diff application, and writer boundaries. Direct XML editor and SVG viewer laws assert refusal instead of panic.
- The old declaration-mutation fixture's `UTF-16` claim conflicted with the artifact's always-UTF-8 bytes. Its valid field-change case now uses the legal case-insensitive `utf-8` spelling while still changing standalone state.

## Delta audit public-facet repair

- PPTX snapshot, XML-parts replacement diff, and set-snapshot JSON Schema no longer advertise `XmlAttr.prologPosition`. Their shared TypeScript XML attribute guard rejects that doctype-only field instead of silently dropping it.
- The set-snapshot leaf uses the exported `pptxGuardObject` authority and the package guard refusal; the leaf-local raw `Record` cast and `TypeError` path were removed.
- XML and PPTX TypeScript declaration guards enforce the same VersionNum, EncName, quote, and UTF-8 transport rules. SVG artifact and snapshot guards now delegate their embedded document to the canonical XML parser.
- The SVG diff TypeScript facet no longer redeclares permissive raw XML node/declaration types. It imports the canonical XML guards, restores its missing recursive `parseSvgNodeDiff`, and preserves omitted sparse triple arrays as omitted.
- The neutral PPTX boundary fixture adds `invalidXmlAttribute` for snapshot/diff/set-snapshot Ajv and TypeScript negative routes.

## Delta audit source evidence

- Direct strict TypeScript compilation with `--skipLibCheck` passed for the XML snapshot, SVG artifact/snapshot/diff, and PPTX artifact/snapshot/diff/mutations/set-snapshot facets; log `🗑️generated/xml-svg-pptx-boundary-tsc-2.log` is empty and the command exited 0. The earlier invocation without `--skipLibCheck` reached only unrelated installed declaration failures in markdown-it, MDX, and bun-types; it is not counted as a product pass.
- `[DEBUG] XML/SVG UTF-8 declaration guards and PPTX XmlAttr/setSnapshot guards refused every invalid public ingress` was observed from the Bun runtime proof in `🗑️generated/xml-svg-pptx-boundary-runtime.log`.
- `[DEBUG] PPTX snapshot/diff/setSnapshot TS guards reject XmlAttr.prologPosition` was observed from the three-route Bun proof in `🗑️generated/pptx-xml-attr-runtime.log`.
- `rustfmt --edition 2021` parsed and formatted every touched XML/SVG Rust source without error. Both XML snapshot neutral-fixture includes and the SVG cross-artifact include resolve on disk.
- XML native5 ran for 20 minutes 27 seconds but stopped at compile errors before any assertion. Two stale fixture include paths are fixed here; the coordinator fixed the two root-owned editor-test type errors. No native pass is claimed. XML native6 remains the first executable gate for this frozen raw-ingress delta.

## Raw-ingress source freeze

- Production interfaces are frozen after the delta repair: the persisted model remains `XmlDocument.epilog` plus `XmlDoctype.prolog_position`; raw XML/SVG conversion and replacement are fallible; XML/SVG render routes return checked publication errors.
- A final call-site audit found every `write_svg_xml` caller handles its `Result`; the remaining `XmlArtifact::from_snapshot` and `SvgArtifact::from_snapshot` call sites are the direct refusal/valid-publication laws. The binary SVG protocol description now reflects the checked writer result.
- `git diff --check` passed for the XML, SVG, PPTX, and implementation-report paths.
- The coordinator's Office TypeScript gate 14 passed 37/37 targets in 1 minute 8 seconds. Its runtime evidence covered 6 snapshots, 5 diffs, 9 public artifact facets, 4 optional clears, 5 XML property diffs, 6/6 sparse XML cases, 3 boundary routes, and all 3 invalid PPTX attribute refusal routes.
- No implementation item from the independent delta audit remains open in this source freeze. XML native6 is still required; no native pass is claimed here.
