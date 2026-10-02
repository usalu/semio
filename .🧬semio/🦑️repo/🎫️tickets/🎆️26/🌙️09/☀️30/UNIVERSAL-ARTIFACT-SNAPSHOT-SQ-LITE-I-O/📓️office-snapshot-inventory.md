# Office Snapshot Authority Inventory

## Observed native authority

XLSX ECMA-376 base XlsxSnapshot owns exactly `schema: String`, `opc: OpcPackage`, and `xml_parts: Vec<XlsxXmlPart>`. Each XML part owns path, content type and a complete XmlDocument. XlsxWorkbook, XlsxSheet, XlsxCell and XlsxCellValue are semantic views built from those authoritative parts; they are not additional persisted snapshot fields. The logical XML graph must be individually queryable, with no serialized XML/ZIP snapshot carrier.

DOCX ECMA-376 base DocxSnapshot owns the same three categories: literal schema, OPC metadata/non-XML intrinsic parts, and authoritative typed XmlDocument parts. DocxDocument, blocks/tables/runs/styles are derived views and must not be invented as duplicate snapshot authority. Snapshot methods already distinguish XML-bearing parts from non-XML OPC payloads.

PPTX ECMA-376 base PptxSnapshot additionally owns its typed PptxPresentation. Presentation/slide/shape/run/transform entities therefore require their own explicit relational ownership beside OPC and XML parts, preserving all ordering, variants, integer widths and optional distinctions.

The actual current sources are under each stdio artifact `🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs`. ZIP OPC is owned at stdio ZIP `📦️opc/🦀️.rs`. Exact inherited subset/declaration publication, ZIP ISO21320 shape, and DWG remaining registration gaps still require inspection before implementation. No provider or verification result is claimed from this initial read-only inventory.

## Shared output control limitation

Current erased import output performs owner-bounded borrowed preflight and ordinary native encoding between outer checkpoints. Genuine interior cancellation during to-record/native output construction is still unfinished. Handwritten providers and controlled input parsing must not be described as resolving that separate expensive-operation boundary.

## Additional observed prerequisites

XLSX currently implements ArtifactDsl as hex of the native ZIP file and ArtifactPack as that native file, with no relational capability. Its new owner work must replace that persistence boundary with complete typed logical snapshot records and literal controlled parsing/construction, while ordinary XLSX file import/export remains its separate converter. The same boundary must be inspected individually for DOCX and PPTX rather than assumed.

OPC actual fields: ordered parts `(path, content_type, intrinsic bytes)`, ordered content-type default/override pairs, a BTreeMap from literal relationship-owner String to ordered relationships `(id, rel_type, target, target_mode)`, and literal package comment. XML-bearing parts already have their authoritative XmlDocument outside OPC part bytes. SQL must expose these metadata/relationship entities explicitly; intrinsic non-XML part octets may remain their own byte payload, not a carrier for the whole package or XML snapshot.

ZIP 2.0 currently explicitly declares both wildcard and `iso21320`, and its authored semantic validator delegates the named policy to `check_iso21320_conformance_controlled`. Its trait currently lacks the activated controlled native hook; previous erased successes are historical. DWG declares wildcard AC1018 and AC1024; its earlier native provider/subset work belongs to the TypeScript fleet worker, who confirms no concurrent ZIP/DWG edits and reports that central strict hook activation postdates most prior successes. This lane will own the live missing hook/declaration proofs after the current owners finish.


## Current Output Foundation And Revalidated Authority

The shared output limitation described above is now historical for the generic declared record helper: complete controlled metadata, typed construction, physical Text/Pack/hash/compression and envelope output passed all52 public native admission laws, with strict central owner dispatch active. Office owners still lack their explicit controlled hooks and relational providers, so their individual completion is not implied. Current raw source re-read confirms XLSX and DOCX retain exactly schema/OPC/typed XML part snapshot authority; PPTX additionally retains its presentation tree. This lane will preserve those canonical owner models and author a first-party multi-document XML relational boundary, without a duplicate compatibility model or whole XML/ZIP snapshot carrier.

## Actual XML/Intrinsic-Part Boundary Recheck

Current XLSX snapshot source explicitly documents and enforces that `OpcPackage.parts` carries **only non-XML payloads**; every XML-bearing content part has one authoritative `XlsxXmlPart.document: XmlDocument`. Its authority check rejects XML content types/paths inside the binary part channel, duplicate part paths, content-type disagreement and unresolved relationship ownership. This confirms the planned SQL boundary can expose XML node/attribute/order entities without retaining an XML-text or complete ZIP carrier. The derived workbook/cell semantic views remain outside persisted snapshot authority.

The shared OPC owner itself declares `OpcPart { path, content_type, bytes }`, ordered content-type pairs and literal relationship fields. OPC's generic native ZIP/XML serialization functions are ordinary format converters; they are not a controlled complete owned-snapshot persistence authority. The new Office snapshot codecs must emit complete literal logical records under the verified genuine controlled producer rather than call those wire serializers. Actual named strict/transitional subset policies must remain separately authored and tested against exact declared coordinates. This source inspection adds no provider or runtime completion claim.

## Controlled Subset Validation Boundary

The observed XLSX `validate_authority()` helper allocates path sets and calls `project_workbook()`; it is therefore an ordinary format/model analysis helper, not an interior-controlled owned subset validator. The named SQLite subset implementation must not invoke it blindly or lower the model into native ZIP/XML. Its explicit semantic policies should inspect the authored relational rows or borrowed typed fields under admission/cancellation, including part ownership, content-type agreement, relationship roles and exact Strict/Transitional namespaces. Wildcard complete owned-state fidelity remains separate from wire representability. This keeps arbitrary literal schema/field presence intact while refusing a genuinely nonconformant exact named profile.

## Explicit multi-document XML boundary and expensive copies

The current first-party XML relational module exposes thirteen typed table names and `project_xml_document`/`reconstruct_xml_document`, but the implementation currently assumes one document with ID 1. Office requires an explicit multi-document root boundary and stable document ownership across node/root/misc/declaration/doctype/entity rows; transforming an already projected single-document database by guessed column positions is rejected as an architecture. The existing XML helper also copies text using private `.into()` calls with only checkpoints before large operations, bypassing shared Projection/Reconstruction. Genuine interior-controlled copying must be established before Office composition can claim those guarantees. Rust owner has been contacted for a narrow XML graph ownership partition; no XML source edits or runtime claims yet. XLSX schema authority remains only schema, non-XML OPC package, and ordered logical XML parts.

Rust owner explicitly delegated the XML typed graph controlled-copy and multi-document boundary to this lane after Note/PDF, retaining his native typed guards and handwritten XML semantics. No overlapping XML edits remain.
# XLSX Schema-First Draft

An individually authored twenty-table XLSX schema now lives beside its actual base snapshot at `📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql`. It represents the snapshot's literal schema, package comment, ordered intrinsic binary parts, ordered default/override metadata, all relationship-owner map entries including empty owners, ordered relationships, ordered XML parts and each XML document's explicit node/component/attribute/declaration/DTD relationships. Nodes carry the exact owning XML document identity. Derived workbook/cell views are not additional snapshot authority.

This is a schema-first draft only: no XLSX provider, native capability, source/native runtime proof or completeness claim follows from its presence. Multi-document typed XML projection and reconstruction must be authored against those literal relationships; parsing a single-document database and guessing column rewrites is not an accepted bridge.

The inspected OPC owner at `🎒️zip/📦️opc/🦀️.rs` has literal `OpcPart { path, content_type, bytes }`, ordered default/override `(String,String)` pairs, a `BTreeMap<String,Vec<OpcRelationship>>` whose keys include the package-root empty string, and the package comment. The relationship fields are identity, type, target and an internal/external enum. The schema's binary-part BLOB represents that intrinsic owned byte field, never the complete XLSX/OPC archive. XML-bearing parts are separate typed documents and receive distinct surrogate document identities; repeated lexical element names and equal root nodes must remain distinct owned occurrences.

SQL foreign keys alone cannot establish same-document XML ownership across a child edge or root/misc reference. Reconstruction must check exact owner identity on every node relationship, distinct roots, unique ownership and contiguous collection ordinals before creating the native tree. The single-document provider currently assumes document identifier 1 and must not be treated as this multi-document implementation.

Ordinary `XlsxSnapshot::validate_authority` constructs identity sets, resolves relationship ownership and derives workbook state. It is a separately observed ordinary wire policy, not a controlled wildcard snapshot validator. Named strict/transitional registrations require explicit owned namespace/domain checks in both directions, while wildcard projection must preserve the complete admitted typed state. Native Text/Pack also require an explicit logical snapshot record, controlled XML/OPC field construction and partial-tree retirement; ordinary XLSX archive serialization does not preserve all owned intermediate fields.
