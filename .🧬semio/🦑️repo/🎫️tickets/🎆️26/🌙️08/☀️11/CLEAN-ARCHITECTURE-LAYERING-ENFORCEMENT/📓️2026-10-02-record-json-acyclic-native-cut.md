# Record JSON Acyclic Native Cut

Read-only current source design map, no source edits/Nx/native execution. Full source/hash capture: `🗑️generated/record-json-cut-readonly/captured-inputs.json`,63 inputs,zero changed on immediate reread,16 Rust test files. This is bounded per-file capture, not a whole coherent compiler epoch. Concurrent lower lexical extraction is already visible; all paths below describe the observed snapshot.

## Minimal Ownership Floor

| Owner | Required public surface | Dependencies |
| --- | --- | --- |
| Neutral Value | DslValue/Number, native encode/decode controls and retirement | existing Value/Base64 |
| Neutral record model under DSL/schema | RecordLayout,Shape,FieldSpec,RecordSpec,GrammarSpec,FieldValue,RecordValue,WireNode/WireEdgeLabel/WireValue,ExprOp/ExprValue,RecordSpecProducer/NativeSchemaControl | Value, owned standard collections only |
| Neutral DSL record text | ParseOptions/SourceMode,Cst,JoinMode/Writer,parse/parse_exact/parse_tokens,wire/expression parser/printer,controlled parsing/printing,LanguageService | record model plus current lower lexical DSL |
| Neutral DSL binding/derive | DslField,DslVariants,owned binding runtime; DslRecord/DslScalar/DslEnum and generic record emission | record model,Value,text; no kernel |
| Neutral JSON primitives | current Pack JSON Number/Object/Value/JsonError,lexer,parse/parse_bytes,writer,Value conversion | Value only |
| Neutral record JSON-schema projection | shape_json_schema/record_spec_json_schema | record model plus JSON primitives |
| Neutral Pack record/document codec | EncodeOptions/DecodeOptions,DecodeReport,PackSchemaGraph,record/document/value encode/decode,controlled variants and retained cursors | record model,text,JSON primitives,existing Pack codec/source/format floor |

Keep JSON primitive ownership independently importable from the full Pack record facade: either a distinct JSON package under the existing IO taxonomy or an independent Pack JSON package. Do not make record model or lexical DSL depend on the facade that also depends on record model. Existing neutral Pack may remain a facade over independent primitives and codec packages; model consumers import the model directly. ToolRun then depends directly on neutral record/binding/codec, removing its actual Kernel imports.

## Exact Source Witnesses

OS schema `🦀️.rs:8–16` now imports lexical facilities from `semio_framework_dsl`; neutral lexical package exposes repository-owned diagnostics/span/token/lexer/grammar/idiom (actual package glue captured). Schema model lives at lines33–221 and364–489. JSON-schema functions at238–344 currently return `crate::os_pack::json::Value`; move them apart from the model. The same file mixes ordinary parsing from586, expression text at763/780,wire text1202 and print machinery1545 onward. Preserve the owned expression AST with the model; put text functions in text owner so binary codec's canonical expression text is an explicit one-way dependency.

Controlled schema decode imports lower lexical language/limits (lines3–4) but Kernel control via `crate::os_dsl` (line5); change control origin to Value. Controlled encode imports model/text and Value control via protocol (lines2–3). Producer imports schema plus `crate::os_dsl` controls; its function-pointer factories must use the same actual neutral Value controls, retaining scoped-depth/workload/charge/allocation semantics. Neutral JSON imports protocol Value (lines28–29); these are owned repository types but direct Value import removes unnecessary facade closure. JSON public Number/Object/Value are authored owned types, distinct from DslValue's Number; keep exact conversions at540/558 rather than replacing one with the other.

OS Pack value line13 imports schema; line424 canonically prints Expr text. Its schema graph/hash/retained record codecs belong to the codec owner above. OS CLI and artifact-specific scene adapters remain higher consumers. Do not pull ArtifactStore, ArtifactDsl,Semio envelope classifiers,built-in family registry or artifact dialect detection into generic binding/model solely because they share the old facade.

## Derive Consumer Authority and External Type Closure

Generic derive emitters currently hardcode `::dsl::DslField`, `::dsl::schema::producer`, `::dsl::NativeSchemaControl` and RecordSpec at owner lines1426–1428,1485,1510,1582,1727–1745,1790–1805. Existing callers alias Kernel as `dsl`; moving definitions without changing aliases leaves product dependencies. Add explicit absolute neutral binding authority in schema-first derive attributes, verify actual Cargo provider and migrate every generic consumer to it; do not keep Kernel forwarding as compatibility. DslArtifact/store-specific derivation is a separate product extension, not part of generic binding cut.

ArtifactChild rule: original binding test line119–123 explicitly binds OS `ArtifactChild` plus ArtifactRef. Its DslField implementation belongs beside those product-owned types, using the lower trait. Keep that exact law under the higher artifact contract integration target; move the other eight generic binding laws to lower DSL/Pack owners. The two pure native decode-control laws are now lower source-ready per coordinator; this audit does not assert their queued runtime outcome.

Public model/binding/codec signatures must close over owned types,standard library and explicit lifetime/control parameters. Existing repository-owned protocol exports are not third-party types, but direct lower imports give correct ownership. Keep serde traits/JSON validator/image/hash libraries as behind-interface test references where appropriate; never expose serde_json::Value or foreign error classes as the new record/JSON/codec API. Existing authored JSON Value remains valid; do not conflate facade reexports with owning its definition.

## Original Law Preservation Roster

Preserve and register actual schema unit,json-schema,intrinsic-bytes,record-list,borrowed-object,controlled decoding and producer native files plus every authored fixture/schema in their source trees. Current schema owner mounts unit/json-schema/intrinsic-bytes/record-list (lines346–348,2260–2270); controlled decode and producer suites are also mounted by Kernel snapshot-native-admission. Pure file presence does not grant native discovery.

Preserve JSON native unit laws mounted by current lower JSON owner lines1680–1683. Preserve OS Pack schema-hash integration target, record-codec-laws and unit/corruption/controlled/retained/schema-graph laws with original selected feature profile, expected names and schema fixture bytes; their actual package registrations are higher today. The capture covers Pack value subtree but not the entire Pack law family, so no complete Pack test count is inferred from16captured Rust test files.

Preserve neutral lower DSL ownership/selection and literal-protocol TypeScript laws; schema record-list already runs independent Ajv and Bun SQLite assertions inside its Rust law. Existing neutral Value binding fixtures are language-neutral corpus; current portable decode tests and stage oracle remain required. JSON's existing independent serde_json/native comparison stays with JSON; record/schema tests' independent AJV/SQLite/blake3 comparisons stay with their respective cases. No new mirror test may substitute for an original scenario.

Before claiming the cut complete, capture every actual Cargo mount/feature/test roster for model,text,binding,JSON,Pack record and higher artifact scenario; run original native+portable suites, compare exact original law discovery and execute a fresh products/S/Hub-absent lower workspace with direct dependency closure. Product-specific classifier/built-in registry and ArtifactChild scenarios require their original higher runs. All such executions remain proposed or queued, not performed by this audit.
