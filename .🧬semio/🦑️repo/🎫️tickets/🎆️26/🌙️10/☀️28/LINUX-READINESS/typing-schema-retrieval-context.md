# Schema Validator Retrieval Context Census

Read-only census for Root. No schema/helper/URI/source edits or tests. Registered rg is unavailable, so registered Node scanned 18,134 framework Rust/TypeScript files excluding node_modules, target, cache and Git. Raw match inventories: 🗑️generated/typing-schema-context-census.json and typing-schema-retrieval-contexts.json. Findings below were read before boot 11220615; the report write interrupted by reboot is recovered here.

## Existing Normative Source Owner

The existing first-party contract is 🧰️framework/🔨️modules/🧬️schema/📄️source/🔗️closure/🔣️.json#/$defs/input: a closed object requiring id, path, baseUri, source. The sibling TypeScript interface SchemaSourceInput has exactly these fields (🟦️.ts:5); SchemaReferenceRequest has root/resources (:7), and SchemaReferenceClosure preserves root plus document inputs (:21). Package: @semio-tech/schema-source-closure-ts.

Its implementation validates an absolute fragment-free retrieval baseUri (:155–157), starts document traversal at this URI (:177–178), registers the actual retrieval URI separately (:190), and resolves authored $id against the inherited scope (:188–191). Anonymous documents already have real retrieval context. This branch currently has TS source/package only; no Rust implementation/package was present.

This is the existing domain owner to consult for document-source input. The validator prepared-snapshot schema at ✅️validator/🏗️compilation/🧬️schema/🔣️.json describes test corpus schema/documents/before/mutations/after, not a production source API. The schema module root 🔣️.json defines FacetLeaves as five body strings and SchemaExportEntry as scope/export/format. Neither carries retrieval context. Semantic catalog/export ids are not physical filename authority.

## Current Compiler APIs

Paths here are relative to 🧰️framework/🔨️modules/🧬️schema/.

| Source | Actual shape | Context gap |
| --- | --- | --- |
| ✅️validator/🦀️.rs:212–233 | compile(schema_json: &str), controlled variant, compile_with_documents(schema_json: &str, documents: &[&str]), controlled variant | Bodies only; no retrieval URI. |
| ✅️validator/🦀️.rs:237–242 | compile_value(schema: &Value) | Semantic value only. |
| ✅️validator/🦀️.rs:325–340 | compile_intrinsic_with_documents(schema: &DslValue, documents: &[DslValue]), controlled variant | Siblings must contain $id; no physical source. |
| ✅️validator/🔨️compile/🦀️.rs:12–29 | Scope has root/base Value references plus document/pattern indexes. index_documents parses body strings, requires sibling $id and rejects duplicates. | A URI-normalization helper cannot recover discarded retrieval authority. |
| ✅️validator/🟦️.ts:107–130 | Scope has root and Map<string,unknown>; siblings require unique $id. | No retrieval field. |
| ✅️validator/🟦️.ts:156–158,260–261 | compileJsonSchemaSubset(schema, root = schema, documents = []); validateJsonSchemaSubset(schema,value,root = schema,documents = []) | Owns a snapshot of values but not physical coordinates. |
| ✅️validator/📦️packages/🟦️typescript/🟦️.ts | Explicit compiler/validator and CompiledJsonSchemaSubset reexports | Adds no retrieval parameter. |

No Rust SchemaSourceInput/retrieval_uri/retrievalUri/base_uri carrier appeared in the bounded framework source census. This concerns those source vocabularies and inspected APIs; it is not proof against all generic URI types elsewhere.

## Production Rust Constructors

| Actual caller | Data supplied |
| --- | --- |
| framework/schema/⚛️component/🦀️.rs:54 | SchemaCatalog::register_json(id,schema) calls compile_value. id is a semantic catalog key. |
| framework/schema/⚛️component/🦀️.rs:286–298 | structural_validator_in resolves selected body, scans JSON-schema export bodies, skips siblings without $id, deduplicates by declared $id, then calls compile_with_documents with bodies. |
| os/🔨️modules/🌉️mcp/🧬️schema/🧾️scope-exports-and-validation/🦀️.rs:107–109 | compile_validator serializes serde_json then calls compile. |
| os/🔨️modules/📇️directory/🔌️client/🌐️document-http/🦀️.rs:21,30–34 | Compiles included declaration schema and input_schema/output_schema strings; dynamic strings require owner-prefixed $id but contain no retrieval field. |
| os/🔨️modules/🔌️plugin/⏪️time-travel/🗄️store-state/🦀️.rs:350–364 | Mutation input_schema yields text; time_travel_schema_documents returns strings; compile_with_documents receives only bodies. |
| os/🔨️modules/🔌️plugin/🧪️testing/⚖️declared-verb-verdicts/🦀️.rs:13 | Serializes semantic argument schema, then compiles. |
| print/🚪️io/📝️text/💡️inferences/🦀️.rs:170 | Compiles intrinsic contract with no siblings. |
| print/🧬️schema/💡️inferences/🦀️.rs:10 | Root and two siblings come from actual owned_json_file! paths, but the API receives DslValue results, not those paths. |

framework/schema means the schema module above; os means 🧰️framework/🛍️products/💻️os; print means 🧰️framework/🛍️products/📓️print. Other census matches are constructor tests, declarations or docs. Important test callers are manifest mutation-inputs, schema component/unit/module-compile laws, plugin history-edit schema resolution and print chart mutation laws; raw JSON inventories contain exact paths/lines.

## Real Physical Authority Before Loss

📇️registry/🦀️.rs:156–165 already defines SchemaLeafDocument {scope,export,format,path} and SchemaLeafReader::read(path) -> body. FileSystemSchemaLeaves holds the actual repository root (:222–241); PackageAssetSchemaLeaves reads exact asset paths. These are physical coordinates, not a retrieval-URI carrier.

resolved_leaf_body (:188–218) and SchemaExportRegistry::resolve (:493) return only body strings. Served paths are behind the registry interface; embedded FacetLeaves are already strings. Preserving retrieval context requires an explicit owner interface, not reconstruction from scope/export/$id. Multiple served documents may be concatenated by the existing resolver, which is another reason a body is not automatically one physical document.

🧪️tests/🧩️module-compile/🦀️.rs:200–225 already reads (actual repository-relative path, body) pairs. The by_id/referenced-document selection then passes body strings only. This is the direct boundary for the reported no-ID physical relative references: preserve actual source coordinates rather than manufacture filename aliases. The authored-schema test (:389–400) similarly reads paths then reduces siblings to strings and constructs a new intrinsic root from definitions.

A body-only constructor cannot recover real retrieval context. Preserve explicit source input where physical authority exists, keep retrieval URI distinct from authored resource $id, and consult the existing source-domain normative input across implementations. No synthetic URI scheme, injected $id, alias, filesystem discovery, or production API change is proposed as an implemented result by this census.

## Root Contract Confirmation

Root independently read the current Source closure schema and TypeScript implementation. The canonical input is exactly id/path/baseUri/source, the request has dialect/root/resources/annotations, and closure output preserves physical input identity separately from resource URIs and reference edges. Controls include explicit source/document/coordinate byte extents, depth, schemas, resources, references, chunked scheduling, cancellation and progress. Current TypeScript implementation uses the first-party raw JSON syntax decoder and duplicate-member policy. No Rust retrieval DTO, weaker raw-JSON parser, filename alias or partial alternate closure was introduced by this inspection. A future native integration must preserve this existing authority and budget contract rather than treating native validator body/$id indexing as equivalent.
