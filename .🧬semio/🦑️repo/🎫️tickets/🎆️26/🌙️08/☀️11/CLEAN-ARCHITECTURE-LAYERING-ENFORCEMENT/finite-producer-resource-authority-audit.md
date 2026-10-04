# Finite Producer Resource Authority Audit

Read-only current-source inspection. No Rust, Cargo, compiler expansion, or registered tests were executed. Full bounded inputs below were captured without editing production.

## Current failure mechanisms and callers

OS derive `mutation_leaf_referenced_documents` (707) infers a search root (727), obtains a process-thread cached index (735), then silently continues on an absent referenced id (717). The directory walker returns on read_dir failure, flattens entry errors, skips file_type failures, drops read/parse/id failures, and uses `index.entry(id).or_insert(path)` (751): duplicate identity is first filesystem encounter wins. Cache identity is only root path, not captured bytes or inventory identity. `mutation_schema_document_references` (768) does read/parse errors explicitly, but traverses every JSON object/array, including annotation data, strips fragments, ignores local refs, and does not resolve relative/nested `$id` bases. These are actual implementation behaviors, not acceptable authority semantics.

Leaf generation (575,616) embeds only found sibling raw documents. Aggregate generation (2161,2249) forwards each leaf slice. Application controller plugin source (24753–24754) and time-travel (878–879) publish those slices via `register_referenced_schema_documents`. The registry (625) takes raw static strings, returns unit, and deduplicates by raw equality alone; it preserves no physical identity, digest, URI provenance, or closure witness. Separate plugin shared-document declarations pass owner/direct-dependency validation in builder (712), and publish exact scope exports through `publish_declared_catalogs` (4317), which returns registration errors. This existing explicit owner declaration is the right authority input; global registration timing is not proof that a producer dependency was covered.

`structural_validator_in` (290) enumerates registered JSON Schema leaves, silently skips failed sibling resolves and failed parse/nonstring `$id`, and accepts same-id equal raw bodies from distinct physical origins. It refuses same-id different raw bodies. This runtime snapshot path is distinct from producer compile dependency closure: both need strict input validation, but neither may be substituted as evidence for the other.

## Proposed closed input and output contract

A producer authority is an immutable explicit finite list of resource records, not a root directory. Each record supplies owned `sourceId`, exact portable physical path, raw UTF-8 source, raw-byte SHA256, explicit retrieval base, declared dialect and permitted annotation schema positions. The root is identified by sourceId. A record's URI identity is derived from strict decoded-member JSON plus resource resolution; a claimed id must be checked, never trusted as a lookup override. Physical path authority is supplied and validated by the Repo capture layer; the neutral closure cannot infer filesystem existence from strings.

The producer request also supplies exact consumer physical source/digest, bound macro export and attribute/derive span, exact descriptor/taxonomy records, and the sealed producer slot witness described in `producer-origin-registry-enforcement-contract.md`. Map neutral closure `documents` back by sourceId to supplied physical records; reject absent mapping, ambiguous physical identity, duplicate root/nested `$id`, malformed sources, unknown references, unsupported dynamic refs, and malformed pointer/anchor resolution. Admit all records strictly before success; irrelevant malformed supplied resources cannot quietly disappear. Strict decoded duplicate names must fail before platform JSON parsing. Preserve raw source without normalization; output sorted unique physical sibling closure excludes root, while origin edges retain repeated/cyclic structural references and their source/pointer/target identities.

Use explicit production annotation authority for schema-containing extension positions only. Existing selected annotations `x-semio-state`, `x-semio-ui`, `x-semio-formats` are data unless their schema contract explicitly declares child-schema positions. Never treat every `$ref` string inside const/default/examples/enum or vendor annotations as a schema dependency. Preserve original payload/taxonomy/descriptor raw bytes and source includes. Introduce no numeric public value interface to perform this operation.

Control is owned and mandatory: cancellation/progress and source/document/coordinate/depth/schema/resource/reference budgets passed to strict decoder and closure, with bounded chunks and cancellation checks during admission, traversal and result construction. A cache, if any, must key the entire sealed input identity (all records, raw digests, bases, dialect/annotation policy and root), validate the snapshot on reuse, and never mask changed source. The simplest initial implementation uses no producer cache.

## Scope and independent evidence obligations

The earlier 797 captured inputs are a declared bounded authority selected from eight inspected source roots. They contain 776 selected schema/descriptor files and 389 selected indexed schemas; they are not whole-consumer coverage or permission to accept the broad 4463-file directory tree. The broader root census has duplicate id groups; feeding it directly would require refusal, not first-wins recovery. Explicit shared framework resources must be supplied as additional records when referenced; unknown references must fail rather than assume later runtime registry publication. Coverage must be recorded per actual consumer invocation/configuration, not per provider family or a passing sample.

Exact-source registration can prove that each proposed emitted physical dependency has captured raw source and a sealed producer slot/consumer origin. It cannot prove that rustc expanded that export, included those resources, used the active Cargo alias/configuration, or that an incremental rebuild observes edits. Independent real compiler expansion and actual escaped `.d` verification remain required for every active producer variant/configuration and each participating consumer cohort. Compare actual emitted includes/dep-info against registered slot ordinal counts and sorted physical closure; alter a sibling resource and prove invalidation; test unknown/malformed/collision/cycle/local/relative/nested-base cases without weakening original laws. Preserve taxonomy/descriptor/payload singleton obligations and aggregate taxonomy singleton; repeated closure references produce unique physical includes but retain edge provenance.

Run original complete derive tests (mutation-leaf-derive/json/source-authority, mutation-aggregate-source-authority, attrs, mandatory-mutations, composite attrs/timestamp, macro-exports, mandatory descriptor), registered exports/source-authority checks, and full Repo rust-source-direction syntax/native cohorts after integration. Source audit and neutral TS closure tests cannot replace these native obligations. Do not claim execution here.

## Captured Inputs

| Source | Bytes | SHA256 |
|---|---:|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs` | 166451 | `d9f28be7e7f71db1cc63dec03fa5a8d2bfb419b3de5def1ebfad94204e732005` |
| `🧰️framework/🔨️modules/🧬️schema/📇️registry/🦀️.rs` | 31642 | `a6623d265ff7c3922ba58fd39f85f04dc2717a4c8bf557399e2341d28e35614e` |
| `🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs` | 18467 | `4b2d01c82f1b0b4e8c179951c7dcb15743f17d7a468389e07a5cec99a7e5ad03` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs` | 45425 | `4e610bb4a332998d1a03456392cfd0b44b6eb91443496ef59a1c3d964ad1cb78` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | 2922208 | `be6f5fe715a68ec712404e4f040bd5c88bcf84468fb0854861a84dd0beeadae8` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🦀️.rs` | 232013 | `e5ec2b0b6560956b385a79b2e95d599cffb5a9d6c9c4202b26563578cf3cdbd0` |
