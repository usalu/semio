# Canonical IDML Native Artifact

## Implemented Primary Model

IdmlSnapshot owns the canonical intrinsic wire {domVersion, parts:[{path,bytes}]}. It reconstructs and validates a real IdmlPackage before admission or external binary output. Both ordinary and controlled FromValue routes use the same ZIP/XML package invariants; controlled construction uses the framework wire decoder and cancellation checkpoints. IdmlPackage DOM version is now immutable to external callers and available through dom_version(). Existing decode, encode, from_archive, into_archive, parts, part and xml_part methods remain.

IdmlMutation owns two semantic operations: replace-package for whole-package import replacement and replace-part for an existing member. The latter rederives DOMVersion from an edited designmap.xml. Both decide validated sparse diffs. IdmlDiff is an ordered sequence of patches over optional DOM version, optional complete member order, and per-path byte insert/replace/removal. The central capability-gated applier stages every step and validates the resulting package before atomic installation. Absorb retains sequential steps so a rejected intermediate graph cannot be hidden by later changes. Diff inverses and semantic inverse operations retain exact prior content and ordering.

## Native Representations and Registration

The snapshot implements ArtifactDsl, ArtifactPack and ArtifactStoreSnapshot. Native text wraps canonical intrinsic JSON in the stdio.idml DSL envelope. Binary snapshots wrap the first-party intrinsic value pack in the verified stdio.idml pack envelope. This encoding carries IDML fields, not a ZIP artifact snapshot. Mutation text uses semantic opcodes and payloads; mutation binary has an explicit version/tag and first-party pack payload. Diff text and binary also use their owned native types.

The exact encoded record is one authored value field (id 1, key value, intrinsic Value shape). native_record_spec() exposes that real metadata. schema_hash() delegates to the first-party canonical schema hash. A native test requires hash 32afdfb0a01bab498e3b3e63cc557cb8e1b9e5619be657d6bfdbd0993997757e; the main agent independently computed it from canonical bytes [1,1,1,5,118,97,108,117,101,0,14] with both first-party TypeScript BLAKE3 and @noble/hashes.

The main agent added the twenty language facets, descriptor, definition manifest, actual native codec factory, headless declaration, contribution and stdio artifact collection membership. The primary agent added native laws for definition validation, executable runtime assembly, exact IDML MIME/extension, factory schema/hash, verified native factory receipt, mutation metadata and missing-dependency registry refusal. Factory source audit found that artifact_codec additionally requires SQLite capability, so the main agent was directed to use bare_artifact_codec for this explicitly capability-based native Store owner. No SQLite provider is fabricated.

## Executed Evidence

- Eight language-neutral semantic cases were written before the production native model/traits and then expanded to nine with atomic DOMVersion advancement. The persistent independent oracle returned { cases: 9, archives: 9, xmlParts: 10 }. It independently decides mutation outcomes using fast-xml-parser 5.11.2, uses fast-json-patch 3.1.1 for exact RFC 6902 after-documents, and uses fflate 0.8.3 to produce/read actual ZIP archives with matching member order/payload and a stored first mimetype. The XML library is supplied behind a test-only oracle interface; production imports none of these libraries.
- Ajv 8.20.0 (2020-12 dialect) validated all nine mutation fixture payloads against the final owned tagged mutation schemas, resolving the package schema by its real id.
- Installed web-tree-sitter 0.20.8 with the Rust grammar parsed all ten inspected primary/declaration/descriptor/mount/test native files with no syntax errors.
- Every primary Cargo path inspected resolved to a real Cargo.toml. New source paths stay below the workspace 256-character limit.
- All native include_str payload/fixture references inspected resolve.

## Native Tests Authored

Mounted tests cover exact mutation after-documents, immutable inputs, explained refusals, sparse diff transport, diff inverse, semantic inverse, snapshot DSL/pack roundtrips, operation text/binary roundtrips, atomic hostile-diff refusal, sequential composition and preservation of refused intermediate steps, schema-hash identity, controlled valid/invalid XML admission, cancellation and declaration/factory/receipt identity.

## Unverified Checks and Remaining Runtime Risks

Native Rust compilation/type checking and native test execution remain unverified. The available repo runner was already known to be stale and silent; source parsing and independent oracle execution are not a native pass. The current framework native Store may require original retained mutation preparation providers beyond the manual Mutation metadata and cold reference trait implementations. Actual native Store retained replay/preparation must be exercised before claiming that path works.

The first-party XML parser called by package admission is synchronous and exposes no controlled parsing API. Wire construction is controlled and cancellation is checked before/after package admission, but cancellation inside one XML parse and accounting of its temporary XML DOM allocations remain unverified. IDML package graph admission covers container/path/XML syntax/DOMVersion invariants; it does not claim complete Adobe IDML semantic conformance.

## Primary Files

- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/📦️packages/🦀️rust/Cargo.toml
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/📸️snapshot/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🧬️mutations/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🚪️io/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧪️tests/native.rs
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧪️tests/mutations.json
- ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧪️tests/oracle.mjs

## Additional Main-Agent Files

- IDML schema root/snapshot/diff/mutation TypeScript, JSON Schema, GraphQL and Protobuf facets.
- IDML schema/📇️descriptor/🦀️.rs.
- IDML 📜️artifact-definition.json and 🧾️artifact-declaration/🦀️.rs.
- IDML direct leaf metadata and payload schemas under 🧬️schema/🧬️mutations/📦️replace-package and 📄️replace-part.
- stdio artifact collection metadata and associated declaration dependencies.
