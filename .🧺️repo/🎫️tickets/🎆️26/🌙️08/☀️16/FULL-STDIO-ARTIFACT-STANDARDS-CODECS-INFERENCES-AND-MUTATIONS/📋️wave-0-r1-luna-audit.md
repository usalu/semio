# Wave 0 R1 Luna Audit

## Verdict

**FAIL.** The schema-derived 36-artifact catalog and structural quick gates pass, but Wave 0 does not satisfy canonical capability identity, schema-to-runtime binding, transactional registration, runtime security, or combined-tree compilation requirements.

The tree was frozen for this read-only pass. Three independent Luna auditors inspected standards/ledger truth, architecture/OCP/schema parity, and runtime/CQRS/security honesty.

## Current Evidence

- `bun ./📜️script.ts stdio quick`: passed; 36 artifacts, 40 dialects, 6 declared codecs.
- `bun nx run workspace:stdio-quick`: passed.
- `bun nx run @semio-tech/stdio-plugin:test-quick`: failed in the plugin library because the plural `FormatDescriptor` contract removed `primary_mime()` and `primary_extension()` while declaration assembly still calls them.
- `cargo check --workspace`: failed in store-sync callers which still treat the typed `document_codec()` result as `Option`.
- `cargo test -p semio-framework-os-kernel --lib`: compiled and ran 893 tests; 890 passed and 3 failed. The failures cover a stale IO conflict assertion, a fixture sweep with no usable pairs, and a dangling-alternative fixture that now reaches the fallible constructor.

## P0 Findings

### Canonical capability identities

- All six GLTF codec IDs violate `s.stdio.<artifact>.standard.<revision>.codec.<codec>.vN`.
- All 28 GLTF mutation IDs and all 15 GLTF inference IDs omit `.vN`.
- The mutation list still contains forbidden `no-mutation`, `set-snapshot`, and generic `set-*` vocabulary.
- The TypeScript validator applies only a generic path regex instead of category-specific identity grammar.

### Definition-to-runtime authority

- The Rust stdio registry rejects every nonempty codec, mutation, and inference row as lacking a typed executable mapping.
- The registry's `build()` path materializes standards, profiles, dialects, representations, resources, localization, and conformance, but omits codecs, mutations, and inferences.
- Runtime declaration methods append synthesized, hash-suffixed capabilities to the definition instead of validating exact executable registrations against immutable schema-owned capability IDs.
- Registered counts and verified counts are conflated. Current declared counts are 6 codecs, 28 mutations, and 15 inferences; verified counts are zero for all three.

### Runtime compilation

- `ArtifactDeclarationBuilder::formats` still calls removed singular format helpers.
- Store-sync callers have not propagated the fallible `document_codec()` contract.
- The fallible `ArtifactStore::new` contract still has 66 external callers outside its implementation.

### Transactionality and remote integrity

- Multi-registry plugin commit mutates independent global registries sequentially; a later failure can leave earlier commits installed.
- Remote ingest can mutate DAG/history before all validation completes and accepts duplicate edit identity without payload equivalence.
- Snapshot merge skips known operation identities without equivalence validation.
- Host IO route registration overwrites existing routes and does not commit atomically.

### Inference runtime enforcement

- Guest inference execution validates only nonzero budgets and a nonempty cancellation identity, then ignores policy, actual budget consumption, cancellation registration, dependencies, previous state, and requested cache mode.
- It always executes cold and hardcodes valid/complete/cold metadata.
- Host validation checks only a small subset of echoed request/result metadata.

## P1 Findings

- Four declared standards lack their own representation: IFC 4, DWG AC1024, PDF 1.7, and GIF 89a.
- The stdio policy owner table returns an empty owner list, making its migrated/legacy IO matrix checks vacuous.
- Runtime binding remains a positional 36-function factory array, so catalog insertion requires central dispatcher edits.
- Codec version construction accepts arbitrary text while mutation and inference IDs validate `vN`.
- Artifact owner validation silently skips malformed kinds when parsing fails.
- The local definition registry used during plugin build is dropped after validation instead of becoming the authoritative runtime registry.
- Resource resolution returns raw sources that can bypass bounded wrappers; wire JSON has no size budget; dialect interning can retain unbounded input.
- Fallback IO dispatch bypasses local subset validation; format accept filtering silently ignores unknown IDs.
- Checkpoint pin mutation neither validates pins nor rederives the checkpoint identity promised by its contract.
- Plugin registration poison and partial-failure behavior is not uniformly typed across host and runtime registries.

## Preserved Positive Seams

- Catalog discovery matches exactly 36 schema-owned definition leaves.
- All 36 TypeScript roots export their schema definitions.
- Definition roots and basic plural format claims are schema-derived; EPW has no fabricated MIME.
- Provenance is honest: the 40 standards remain unverified and no public support is claimed as complete.
- `ArtifactStore::new` is fallible; replay validates before apply; projection generation invalidates and rejects stale stamps.
- Format registration itself validates claims before insertion.
- Runtime, fuzz, cross-platform, and standards-coverage gates fail explicitly while support remains unimplemented.

## R1 Remediation Ownership

1. **Plugin/definition/WIT lane** owns plural format binding, immutable exact definition-to-runtime validation, category-specific IDs, transactional registry assembly, inference request/result enforcement, and host registry conflicts.
2. **IO/store/CQRS lane** owns remote/snapshot atomic validation, equivalence checks, checkpoint pins, bounded resource/wire paths, fallback validation, typed query behavior, and framework/store caller propagation.
3. **Stdio schema/runtime lane** owns canonical executable capability leaves and mappings, declared-versus-verified ledger counts, per-standard representations, non-positional assembly, and current stdio gate parity.

No Wave 1 primitive implementation begins until these P0 failures pass another frozen-tree audit.
