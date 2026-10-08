# Artifact Identity And Custom Facet Audit Current

Read-only source audit,2026-10-08. No production edit, build, Cargo fetch/cache, Git mutation, runtime acceptance or whole-repository closure. Exact independent web-tree-sitter Rust item bodies, attached/ancestor cfg attributes, direct distinctive function calls and Office package imports are in [the input manifest](audit-artifact-identity-custom-facets-input-manifest-current.md). The successful AST census inspected9019 semantic artifact Rust paths, parsed1499 candidate files and captured35 items. An initial rg listing attempt exceeded Node’s default stdout buffer and did not produce a receipt; the successful run used a bounded32MB listing buffer. No compiler result follows from AST inspection.

## Custom Serde Facets: Four Actual Test-Only Items

Exactly four custom Serialize/Deserialize impl items manually invoking serde_json/Pack/encode/decode were found in this targeted Rust semantic-artifact census. Both Layout ChangeDataFields impls at18/20 have attached #[cfg(test)] at17/19. Both Forms FormDictionary impls at44/46 likewise have attached #[cfg(test)] at43/45. Independent AST sibling ownership and ancestor traversal recorded those attributes, rather than inferring test status from folder names or comments. They are test oracle facets, not production breaches. Root is relocating Layout’s test facets and retaining cfg(test); current source coordinates may shift during that work.

The first Board audit misclassified Layout’s two facets as production. It has been corrected in place with an explicit superseding paragraph. Production semantic FromValue/ToValue implementations are typed first-party admission/projection, not automatically codecs. Derived Serialize/Deserialize facets are a separate representation policy; they were not counted as custom codecs. Helper names deserialize_double_option in multiple diffs receive DslValue directly and are semantic nullable-state admission, not physical deserialize calls.

No production manual Serde bridge was established by this census. Macro-generated implementations, non-Rust twins and sources outside the stated semantic artifact path cut remain outside that conclusion.

## Office Address Identity Ownership

Exact owners below are in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/<kind>/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base`.

DOCX `🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs`:

- hash_bytes155 is FNV1a; hash_field162 emits length u64 little-endian followed by string UTF8; hash_node167 writes typed variant tags, lexical names/attributes/counts and recursive children; hash_shallow_node202 omits descendant content while retaining child count. address_revision234 includes shallow ancestor/sibling facts and node-path indices, then complete selected subtree, and outputs16 lowercase hex. docx_xml_subtree_revision252 hashes only a subtree. These are different commitments and must not be merged accidentally.
- docx_xml_address259 calls address_from_root34 and its lineage commitment; resolve_docx_xml_address278 recomputes address_revision at294. revision_after_replacement301 computes the hypothetical lineage commitment against a replacement. Mutation root:251/268 uses subtree commitment for insertion/removal, while xml-address reducers468/823 resolve lineage commitments. The typed witnesses must explicitly distinguish SubtreeRevision from LineageRevision.
- All XmlNode data is first-party XML schema. `materialized_document`27 delegates to DocxXmlPart::materialize_document_exact (snapshot190), which rebuilds retained typed XML nodes; this is not physical XML text decoding. Moving identity encoding does not justify moving/replacing retained XML storage or retirement mechanics.
- Text and binary mutation IO presently publicly forward the address and subtree helper set from the semantic module (text30/binary29). Canonical identity IO must be called directly, without preserving physical helper forwarders at semantic roots.

XLSX `🧬️schema/🧬️mutations/🧭️cell-address/🦀️.rs`:

- hash_bytes100/hash_field107/hash_node112 emit FNV typed-node variant/count/string preimages. address_revision171 includes shallow ancestor and sibling facts, path indices, selected recursive subtree AND referenced shared-string entry. The shared-string relationship dependency is part of the accepted identity, not just the cell XML subtree.
- xlsx_cell_address311 and worksheet/vacancy constructors admit typed part/node-path/cell coordinates plus a hex revision. resolve_xlsx_cell_address367 checks it; canonical-edit247/352 calls the resolver before constructing actual cell deltas. Mutation root174/185 constructs addresses for net SetCell/RemoveCell. Base editor302 and main window87 construct them from an actual &XlsxSnapshot; Strict/Transitional editor imports directly select the base helper. These are statically resolved free function calls, not ambiguous method names.

PPTX `🧬️schema/🧬️mutations/🧭️xml-address/🦀️.rs`:

- hash_bytes82/hash_node89 encode XmlNode using byte markers E/A//E/T/C/M/P and lexical attribute/text data. pptx_xml_subtree_revision125 emits16 hex. Unlike DOCX/XLSX it does not length-delimit every field or add lineage/sibling/shared-string facts. Adjacent text fields therefore need an independent unambiguous canonical preimage witness, rather than simply declaring this encoding shared with the other artifacts.
- address160/166 and hypothetical_address500/505 publish subtree commitment; resolve_pptx_xml_address174/181 recomputes it. Shape373, container511, replacement646 and text/position666/688 plans use this resolver. Mutation root129 creates ReplaceXmlNode address from actual &PptxSnapshot. No physical ZIP/Pack decode is involved in these typed XML plans.

Direct package imports are first-party `semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr,XmlNode}`, XML schema diff and `semio_s_artifact_stdio_zip::opc::resolve_relationship_target` plus own schema vocabulary. This is a domain-model dependency, not evidence of a third-party physical codec. Preserve explicit first-party type ownership.

Recommended schema-first completion for each: an ArtifactXmlAddress with typed part identity, typed node index path, typed expanded name and typed RevisionIdentity octets; separate typed commitment scopes for subtree versus lineage versus shared-string dependencies. Controlled `🚪️io/💾️binary/🪪️identity` owns preimage traversal/lengths/order/tag bytes/hash; `🚪️io/📝️text/🪪️identity` owns hex spelling. A first-party admitted address witness binds revision to the exact typed snapshot generation and commitment scope. Reducers consume this admitted witness and typed resolved target, perform semantic stale checks, and return typed delta/inverse facts. Construction, validation and hypothetical inverse revisions must all use the same explicit identity admission service; relocating a function while leaving pure reducers calling it indirectly is incomplete. No alias/backwards adapter/export forwarding is recommended.

## Additional Actual Hash And Identity Owners

All are production AST items except the stated test witnesses. Exact full paths/bodies and distinct caller rows are in the manifest.

| Family | Actual owner | Resolved consumer/purpose | Recommended ownership |
|---|---|---|---|
| Draw | drawing schema root11 drawing_id_hex;18 create_drawing_id | path construction25, layer/default constructors and clone IDs use DefaultHasher over byte material then hex | typed DrawingIdentity; controlled drawing binary identity preimage/hash and text hex IO; constructors take admitted IDs |
| glTF | geometry-core135 fingerprint,151 byte_fingerprint | raw-part projection308 and DAG-assembly134 hash decoded points/triangles; binary decoded-accessors16 calls byte_fingerprint on raw buffer slices | typed GeometryIdentity/BufferIdentity; binary identity IO provides admitted provenance; typed inference receives it |
| PNG | schema operations26 png_revision; validation10 revision | patch-pixel editor command161 obtains revision; semantic require_revision and native paint validation compare hashed samples/metadata | typed PngRevision witness; controlled binary identity cursor + text spelling IO; preserve validation progress/cancellation |
| BMP | schema operations25 bmp_revision;49 require_revision; validation7 revision | native paint admission uses matching revision over metadata/palette/samples/opaque gap/trailer | typed BmpRevision admission + binary identity IO; no raw image codec required in semantic paint |
| TIFF | paint-region/samples9 tiff_revision | editor paint command101 publishes revision; samples62 recomputes before typed paint runs; mutation default64 uses fixture revision | typed TiffRevision; binary identity IO; semantic paint receives admitted snapshot revision, retains logical sample edits |
| Raw Binary | extent25 compute_binary_extent | inference publishes count and DefaultHasher+hex content_digest of intrinsic raw bytes | count stays semantic; typed digest provided by binary identity IO; raw byte splice operations remain legitimate |
| Deflate | window41 compute_deflate_window | inference publishes decoded payload length/window/dictionary facts plus DefaultHasher+hex content digest | logical counts stay semantic; typed payload commitment provided by identity IO; do not reclassify decompressed payload as opaque compressed stream |
| ZIP | entries28 compute_zip_entries | inference hashes every decoded entry name+data in order, sums actual decompressed sizes | typed entry census stays semantic; typed archive-content identity admitted by IO |
| Home | digest15 compute_content_digest | hashes schema/catalog_generation and outputs hex | typed home identity facts; binary commitment + text hex IO |
| Process3D | inference432 hash_intrinsic,449 hash_value,456 prefix_signature | replay_process199/209/221/238 uses u64 signatures as memo keys for typed stock and enabled typed step prefixes | internal cache keys differ from persisted identity: use exact typed key/equality, or injected first-party memo-identity service; no artifact Pack replay |

Process3D’s custom DslValue hash is a typed structural cache optimization. It does not serialize a snapshot, but under the user’s strict hash-owner rule its custom hash execution should leave semantic inference. A collision-free typed key avoids imposing physical identity IO on every cache lookup. Std Hash implementations used only by a typed HashMap are not automatically physical codecs; the actionable issue is manually computed/exported identity or manually encoded cache preimage.

CAS internals `➗️equation/.../🧬️schema/💡️inferences/🌿️cas-internals/🦀️.rs:323` hash_kind emits tags, decimal integer/rational text and child hashes/indices/assumption masks in little-endian. It is an expression hash-consing key, not artifact XML/Pack identity. Native Hash/Eq delegates at204/499 compare cached semantic hashes. Keep algebra/equality in the CAS semantic owner; replace manually encoded fingerprint keys with first-party structural typed keys, or inject a narrowly scoped cache identity service. Do not confuse the FNV byte preimage with expression text/DSL parsing. Exact source excerpt is appended to the manifest.

Unused Energy entries imports mention DefaultHasher/Hash/Hasher, but its current source contains only EnergyModelEntries fields and test-module declaration; no hashing function is present. This is not a runtime breach. DWG’s Sha256 debug fixture at4198 is inside test ownership, not production identity proof.

## Additional Draw Color Text Codec Boundary

Draw schema root844 hex_to_rgba trims #, expands three-character spelling, parses substring byte components;851 rgba_to_hex emits #rrggbb. Actual production mutation root104/114 parses field-edit fillColor/strokeColor text; typed FillEdit::Color is itself a String and fill reducer93 parses it again. Properties panel55/66/243 formats typed color for browser input. Those concrete imports/calls remain despite earlier Draw runtime receipts.

Canonical text color IO should admit #RGB/#RRGGBB under controlled refusal and deliver typed RGBA components; FillEdit::Color must carry typed components. Browser/property publication invokes color IO directly. Current root decoder slices [start..start+2] unconditionally: invalid shorter/non-ASCII text can panic before its unwrap_or fallback. This is source-demonstrated control flow risk; no new runtime failure is claimed. Add neutral malformed/color spelling witnesses and independent color admission test when fixing. Semantic strokeDash text parsing in the same field-update branch is another adjacent typed-input candidate, not audited as complete here.

## Scope Limits

Physical byte replay, third-party derive facets and text admissions outside these exact owners remain separately unfinished work. Intrinsic raw bytes, decoded pixels/samples, first-party retained XmlNode materialization and opaque compressed audio data must not be collapsed into one codec category. This report establishes concrete remaining owners and type-resolved static call chains; it does not assert every indirect runtime target is covered.
