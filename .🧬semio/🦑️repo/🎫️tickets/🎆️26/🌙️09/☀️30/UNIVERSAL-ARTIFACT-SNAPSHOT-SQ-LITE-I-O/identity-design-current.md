# Current Native Identity Design

Read-only source audit on 2026-10-09. No implementation or execution claimed. Counts remain separate: three retained catalogs contain33 rows,32 kinds,33 authored factory identities; Stdio's original declaration census contains89 coordinates. No exact global public binding denominator is established by summing them.

## Schema-First Identity

Author the neutral binding schema before native code: exact registration channel, original owner/plugin/app, optional actual dialect, document schema, identity classification, optional authored factory, SQL presence/schema, original canonical source/type label. Retain duplicate rows before enforcing unique channel+coordinate keys. Stable fixture serializes labels/provenance, never process-local TypeId or function addresses.

First-party runtime identity should be a closed enum: `Typed { snapshot_type: TypeId }`, `Guest { plugin_id, component_package_hash: [u8;32], artifact_schema }`, and an explicitly named `ErasedFixture { owner }` for the three actual erased test codecs. Guest package hash is the current compiled component's content identity, not the independent pack shape hash. Do not create placeholder Guest, infer Guest from None, or give Guest a fake host Snapshot TypeId. If package identity and compiled component hash become separate real authored values later, retain both then; do not invent a second hash now.

Place identity on ArtifactCodec independently of optional SQL. `bare<P,Mutation>` populates Typed directly from P even when `P::sqlite_snapshot_codec()` is None; `of` preserves it. SQL provider's optional TypeId stays a separate capability claim and must agree with Typed when present. Guest SQL capability with no host TypeId remains valid only under real Guest identity. Test erased classification must not qualify the production census.

## Exact Construction Changes

Store `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`:12329 adds identity; its canonical bare `Self` literal near12760 sets actual P TypeId. The `of` wrapper near12592 inherits identity. Full manual literals are exactly four current files: hub artifact-authority unit:181 (fixture.number), hub trusted-catalog unit:1080 (fixture family), OS IO `🧪️tests/🪶️transfer/🦀️.rs`:20 (erased relational transfer fixture), and production MCP `🏠️workspace/🦀️.rs`:3264 (actual Guest). All three test literals need explicit ErasedFixture owner labels or real typed fixture owners if deliberately redesigned; never blanket Guest. Store unit:7267 struct update inherits identity and needs conflict coverage, not an additional new identity. Other `fn codec() -> ArtifactCodec` search hits and PluginHostArtifactCodec/VerifiedNativeArtifactCodec wrappers are not constructing literals.

Production guest constructor already receives plugin_id, artifact_schema and component bytes, compiles the actual component, and checks the pack hash independently. `GuestCodecRoute`:3044 retains plugin_id, actual dialect, schema, and compiled handle. At3261 current idempotence checks compiled.package_hash only; require matching plugin id/schema/dialect plus new Guest identity. Set Guest identity at3264 from these existing originals. No external runtime type escapes the first-party enum.

## Equality And Admission

Store `same_document_codec`:12805 must compare identity before optional provider and existing function/hash equality. Kernel IO `propose_native_snapshots`:2186 checks original codec identity along with extension/pack/provider, so two Guest components sharing the generic function table cannot alias. Typed SQL provider identity must be checked against the independent codec owner; None SQL preserves Typed and remains a missing capability row. Current typed route resolver IO:2530 should assert codec Typed(P) as well as provider TypeId and schema. Registry conflict tests cover same schema/dialect but different actual typed owners and same guest routing thunks but different authentic component packages.

## Complete Prefilter Bindings

Plugin DocumentCodecSpec:3566 owns schema/dialect/app_id/foreign and typed constructor thunk. Public flat `document_codec_bindings`:3718 drops app/channel metadata. Flat plan:4342 traverses declaration+hosted,4350 pushes codec then4351 filters SQL; foreign rows4356–4358 do the same. Capture immutable binding descriptor from each original spec before this filter, retaining channel and real app/plugin context.

Tree original public standards/subsets:39883 contain actual subset dialect/native codec. Preflight around39990 and commit around40100 must collect the same descriptors before `from_capability`. Runtime `into_runtime`:4419 currently separates schema-only document rows from admitted snapshots into Kernel plan; `PluginRuntimeRegistry`:4425 retains no complete binding ledger. Retain complete descriptors there and in atomic publication, then expose fallible read-only installed enumeration. Do not return empty on poisoned registry, collapse duplicates before census, or recover missing dialects from schema later.

Direct production entry channels must be explicit: Kernel register_native_document_codec:1899, register_native_snapshot_codec:1892, legacy actual app publication Plugin:43021, authentic MCP Guest:3277, and actual standalone document probes. Newly authored Socket coordinate is a genuine current native binding; its factory remains absent unless a catalog actually authors one. Internal Store prerequisite owners remain a distinct ledger, not public factory denominator.

## Exact Original Census Law

Closed neutral fixture records all actual channel/schema/coordinate/type labels and real optional factories. Native law invokes every original composition initializer and collects complete prefilter descriptors before capability admission, rejects duplicate exact keys, and compares full cardinality/keyset to fixture. Assert process-local P TypeIds through real constructors even for owners missing SQL; assert actual Guest plugin/package/schema provenance through compiled owner receipts. Compare immutable original descriptors to installed registry rows and routes, not only membership of successful providers.

For each admitted original row require both actual route directions and explicit Text/Binary/provider/control/edit receipts. For each real absent-SQL row invoke public refusal at the unchanged original coordinate/schema and prove original snapshot/carrier preserved. Do not remove SQL from copied positive codecs to stand in for genuine missing owners. Join actual catalog factory rows separately; document-only channels keep optional coordinate/factory absent. Every unmatched row is explicitly unqualified rather than excluded from the denominator.
