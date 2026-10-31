# Current Schema Retrieval and Runtime Audit

Read-only audit of current Source and retained actual receipts. No compiler/fullgraph/test run, Source edit, alias, DTO or fallback was introduced. The accepted14URI vectors and prior physical-authority census were read first.

## Existing normative owner

All paths in this table are relative to `🧰️framework/🔨️modules/🧬️schema/`.

| Path | Actual contract or boundary |
| --- | --- |
| 📄️source/🔗️closure/🔣️.json | Production input requires id/path/baseUri/source; request requires dialect/root/resources/annotations; closure preserves documents, physical positions, URI resources and every reference edge. Limits explicitly cover source/document/coordinate bytes, depth, schema/resource/reference counts and bounded scheduling chunks. |
| 📄️source/🔗️closure/🟦️.ts | SchemaSourceInput, SchemaReferenceRequest and schemaReferenceClosure retain actual retrieval URI separately from authored resource $id. The first-party raw JSON syntax decoder rejects duplicate members; cancellation, progress and async yield are explicit. |
| 📄️source/🔗️closure/🧪️tests/🧫️fixtures/🔣️.json | Existing44case neutral corpus. Tests register each source with Ajv using its supplied baseUri and compare physical closure results. utf8-document-order already exercises reachable no-ID external resources; nested-relative-id distinguishes retrieval and authored resource scope. No new corpus authority is required. |
| ✅️validator/🦀️.rs:212–242 | OwnedJsonSchemaValidator compile/compile_with_documents/controlled variants/compile_value still accept bodies or values only. |
| ✅️validator/🔨️compile/🦀️.rs:13–30 | Scope retains root/base values and document/pattern indexes. index_documents requires every sibling's $id; no retrieval coordinate survives this API. |
| ✅️validator/🟦️.ts:181,285 | Prepared compileJsonSchemaSubset/validateJsonSchemaSubset still accept schema/root/documents values only. The TS source-closure implementation does not make this separate subset compiler source-aware. |
| 🧪️tests/🧩️module-compile/🦀️.rs:200–225 | Producer reads authentic physical path/body pairs, then indexes/selects body-only siblings by $id before compiling. This is the direct retrieval-authority loss boundary in the native inventory gate. |

The source-closure owner currently has only TypeScript source/package implementation, with no Rust source/package alongside it. The missing source-aware compiler bridge is therefore not exclusively a Rust URI-string bug: both subset compiler entry APIs lack retrieval context, while the separate TS closure API already models it.

## Repaired work that must not be repeated

schema-uri-reference-plan.md records actual native94805status0 at23:14:54Z,8tests including14reference vectors, followed by Bun and actual Node14/28 each and the prior131vector contract in both runtimes. Relative resolution against a declared $id, percent-decoded pointer fragments, original-root cycles and normalized unique sibling IDs are already implemented. No evidence here justifies another repair of those vectors.

Native schema library gate87337 closed0/null at01:05:18Z:30component tests plus6validator tests. That is actual Linux runtime acceptance for the body/$id subset, not anonymous retrieval context or whole-repository validity.

The last complete native inventory report schema-uri-reference-fixed.json is historical:6538documents,5965compiled,551Draft07failures including526ambiguity/23unresolved/2keyword-related cases,22other-dialect failures and159colliding IDs. Its command0 produced a report and passed its limited engine gates; it did not make those failures valid. No current full native inventory was rerun by this audit.

## Concrete context coverage gap still visible in current Source

Two historical native failure paths still contain genuine physical references today:

- `🧰️framework/🔨️modules/🧵️job/⏱️budget/🧬️schema/🔣️.json` references the actual retained-clone wire document via ../../../🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json. Both root and target have no$id.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/🧬️schema/🔣️.json` references its actual board schema through percent-encoded relative filenames. Both documents have no$id; target CameraDescriptor and NodeDescriptor pointers currently exist.

The body-only native constructor cannot index those anonymous targets, and its caller has discarded the real physical coordinates. This is a concrete API coverage gap consistent with the historical unresolved failures; current filesystem existence/pointer checks alone are not a fresh runtime verdict. No filename URI or semantic export name was registered as a substitute.

Other historical unresolved strings include absent absolute semantic owners, duplicate owners, stale exported pointers and dotted relative IDs. Retrieval context cannot be presumed to fix these. They need actual owner/resource identity and reference proof. Adding aliases would conceal the distinction.

SchemaExportRegistry also loses coordinates: 📇️registry/🦀️.rs SchemaLeafDocument stores actual path, but resolve returns only body and can concatenate multiple served bodies. ⚛️component/🦀️.rs structural_validator_in skips siblings without$id. A catalog scope/export key is not a physical retrieval URI; concatenated bodies cannot automatically be called one source document.

## Latest observed Linux failures are a different gate

Actual scoped architect public gate357357 ended1/null04:48:08Z with schema-export-incomplete findings across Rust/GraphQL/Protobuf; its independent81oracle tests passed. Actual schema-invariants gate366036 ended1/null05:02:58Z with four schema-catalog-stale findings,163other tests passing. Those are ownership/catalog/publication failures, not evidence of failed native retrieval.

The active discovery-boundaries canonical owner362816 is a separate authoring/discovery/resource observation and has not supplied a new native retrieval result. A source-aware Rust compiler is not established as the cause or prerequisite of those latest failing authoring checks. Ownership/catalog corrections and genuine post-correction gates remain their owners' work.

## Smallest meaningful next cross-language TDD step

When this runtime gap is selected for implementation, reuse the existing source-domain request and44case corpus rather than inventing a retrieval DTO or adding a filename fallback. Start with existing reachable anonymous-resource and nested-relative-id requests and an actual controlled source input from the two physical examples above. Preserve caller-issued id/path/baseUri/source exactly, keep retrieval and $id resource identities distinct, and compare closure positions/edges plus instance validation with Ajv registered at the supplied retrieval URIs.

A Rust implementation belongs under the existing 📄️source/🔗️closure domain, with its existing full refusal/control/extent contract—not a reduced alternate closure. The source-aware validator integration must retain that authority in both languages, and the native inventory must pass its already captured physical coordinates instead of reducing inputs to strings. Its tests must retain missing/ambiguous resource refusal, duplicate members, escaped paths, changed inputs, cancellation/progress and each finite budget. Keep existing body-only semantic subset behavior and honest unsupported-dialect refusals distinct from source closure traversal.

This is a bounded design/test recommendation, not implemented acceptance or justification to pause the currently owned authoring/resource gates.
