# Program Contract Freeze

## Cutoff and scope

The normative cutoff is 2026-08-16. The catalog is exactly: binary, TXT, XML, DEFLATE, ZIP, JSON, CSV, Markdown, glTF, OBJ, STL, PLY, LAS, STEP, IFC, DWG, DXF, SVG, PNG, JPEG, GIF, BMP, TIFF, PDF, DOCX, PPTX, XLSX, BCF, Semio, MP4, AVI, MP3, WAV, EPW, TSV, and HTML.

An embedded codec is in scope when a normative field or registered code point of one of those artifacts selects it. Arbitrary resource payloads stored in a generic container are not recursively promoted to artifact families. Publicly specified historical and unsafe encodings remain readable and writable and emit typed security diagnostics. Undocumented structures remain byte-exact typed opaque extensions and never count as decoded support.

## Canonical identity grammar

- Artifact: `s.stdio.<artifact>`
- Standard: `s.stdio.<artifact>.standard.<revision>`
- Profile: `s.stdio.<artifact>.standard.<revision>.profile.<profile>`
- Source dialect: `s.stdio.<artifact>.standard.<revision>.dialect.<dialect>`
- Representation: `s.stdio.<artifact>.standard.<revision>.representation.<representation>`
- Codec: `s.stdio.<artifact>.standard.<revision>.codec.<codec>.<version>`
- Inference: `s.stdio.<artifact>.inference.<semantic-slug>.vN`
- Mutation: `s.stdio.<artifact>.mutation.<semantic-command>.vN`

IDs are normalized lowercase ASCII dot paths. Registration rejects any duplicate ID and any conflicting MIME type, extension, codec selector, or source dialect. Re-registering the identical object is allowed only when byte-for-byte descriptor identity and executable identity are both equal; it is idempotence, never last-write-wins.

## Artifact definition

Every catalog artifact owns one `ArtifactDefinition` whose plural collections cover:

1. standards and revisions;
2. profiles and conformance classes;
3. source dialects;
4. physical/logical representations;
5. artifact and embedded payload codecs;
6. semantic mutation commands;
7. atomic inference fields;
8. resources and external-reference policy;
9. English and German localization resources selected by an explicit locale;
10. conformance suites and support-ledger records.

The definition is authoritative. Registries, catalog counts, public descriptors, package exports, policy coverage, and conformance matrices derive from it. Roots assemble and re-export; they do not inspect inference result fields or mutation payload fields.

## Codec contract

`ArtifactCodec` and `PayloadCodec` are repository-owned interfaces and expose no third-party public type. Operations cover sniff, incremental/streaming decode, streaming encode, optional random access, cancellation, allocation/work/recursion budgets, exact source spans, typed diagnostics, resource resolution, and two explicit encode modes:

- lossless: untouched anchored syntax/opaque records reproduce identical bytes;
- canonical: deterministic, idempotent bytes independent of registration and map iteration order.

One authoritative snapshot owns stable semantic IDs plus anchored lexical and opaque records. Normalized projections, parsed indexes, topology, BVHs, statistics, and signal transforms are derived state.

## Inference contract

Each folder `🧬️schema/💡️inferences/<semantic-slug>/` defines exactly one public inference field and owns its Rust/TypeScript result type, computation adapter, dependencies, validity, quality, diagnostics, provenance, descriptor, and tests. Pure reusable algorithms live under named `🔨️modules/<kernel>/` and cannot construct public inference records. The inference root only re-exports leaves and assembles descriptors.

The cache key includes artifact ID, standard/dialect revision, event revision, generation, explicit policy, algorithm version, and dependency hashes. Cold and incremental results are equal. Apply, remote ingest, replay, undo, redo, reset, checkout, policy changes, and external-resource changes share one projection path. Revision/generation mismatches reject stale results.

## Mutation contract

Each folder `🧬️schema/🧬️mutations/<semantic-command>/{🦠️mutation,🔺️diff,↩️inverse}` defines exactly one command. The leaf owns payload validation, direct sparse planning, typed rejection, reference repair, touched paths, forward apply, and inverse reconstruction. Roots only assemble stable command IDs and wire tags.

There is no `NoMutation`, `SetSnapshot`, `CollectionMutation`, generic collection verb, generic `Set*`, hidden option bag, silent no-op, index clamping, or missing-target tolerance. Whole-document import/replacement uses reset/checkpoint outside semantic history. Shared persisted edits remain deterministic CQRS events; no CRUD or CRDT lane is introduced.

## Required facet and runtime parity

Rust and TypeScript are executable against shared vectors. GraphQL, JSON Schema, Protobuf, text grammar, and binary protocol exactly mirror the canonical schema. Native and WIT requests include artifact revision/generation, source dialect, explicit policy, budgets, cancellation identity, prior inference state, cache mode, typed diagnostics, and canonical payloads.

## Closure gates

An artifact/revision/profile/codec closes only when its definition-derived ledger reports no uncovered public entry and current-tree gates demonstrate byte-exact untouched roundtrip, canonical determinism/idempotence, schema/runtime parity, inference laws, mutation laws, official/public corpus provenance, differential validation, fuzz termination and hostile-input budgets, representative benchmarks, security diagnostics, and native/wasm/cross-platform package surfaces.

The umbrella ticket stays open until all 36 definitions and all transitive registered codecs satisfy those gates on the combined tree.
