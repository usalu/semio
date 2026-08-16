# Wave 0 R1 Luna Audit

## Verdict

**FAIL.** The schema-derived 36-artifact catalog and structural quick gates pass, but Wave 0 does not satisfy canonical capability identity, schema-to-runtime binding, transactional registration, runtime security, or combined-tree compilation requirements.

The tree was frozen for this read-only pass. Three independent Luna auditors inspected standards/ledger truth, architecture/OCP/schema parity, and runtime/CQRS/security honesty.

## Current Evidence

- `bun ./📜️script.ts stdio quick`: passed; 36 artifacts, 40 dialects, 6 declared codecs.
- `bun nx run workspace:stdio-quick`: passed.
- `bun nx run @semio-tech/stdio-plugin:test-quick`: failed because declaration assembly still calls removed `primary_mime()` and `primary_extension()` helpers.
- `cargo check --workspace`: failed where store-sync still treats typed `document_codec()` results as `Option`.
- `cargo test -p semio-framework-os-kernel --lib`: 890 passed and 3 failed: a stale IO conflict assertion, a fixture sweep with no usable pairs, and a dangling-alternative fixture that now reaches the fallible constructor.

## P0 Findings

### Canonical capability identities

- All six GLTF codec IDs violate `s.stdio.<artifact>.standard.<revision>.codec.<codec>.vN`.
- All 28 GLTF mutation IDs and all 15 GLTF inference IDs omit `.vN`.
- The mutation list still contains forbidden `no-mutation`, `set-snapshot`, and generic `set-*` vocabulary.
- The TypeScript validator applies only a generic path regex instead of category-specific identity grammar.

### Definition-to-runtime authority

- The Rust stdio registry rejects every nonempty codec, mutation, and inference row as lacking a typed executable mapping.
- Its `build()` path omits codecs, mutations, and inferences.
- Runtime declaration methods append synthesized, hash-suffixed capabilities instead of validating exact registrations against immutable schema-owned IDs.
- Declared and verified counts are conflated. Declared: 6 codecs, 28 mutations, 15 inferences. Verified: zero for all three.

### Runtime compilation and transactionality

- Plural format and fallible document-codec caller migrations are incomplete.
- The fallible `ArtifactStore::new` contract still has 66 external callers.
- Multi-registry plugin commit is sequential and can leave partial state.
- Remote ingest can partially mutate before validation and accepts duplicate identity without payload equivalence.
- Snapshot merge skips known operation identities without equivalence validation.
- Host IO route registration overwrites and is not atomic.

### Inference runtime enforcement

- Guest inference ignores actual budget consumption, cancellation registration, policy, dependencies, previous state, and requested cache mode.
- It always executes cold and hardcodes valid/complete/cold metadata.
- Host result validation checks only a subset of echoed request/result metadata.

## P1 Findings

- IFC 4, DWG AC1024, PDF 1.7, and GIF 89a lack their own representation.
- The stdio policy owner list is empty, making its IO matrix checks vacuous.
- Runtime binding is a positional 36-function array, so catalog changes require central dispatch edits.
- Codec version construction accepts arbitrary text while mutation and inference IDs validate `vN`.
- Owner validation skips malformed kinds when parsing fails.
- The build-local definition registry is discarded instead of becoming authoritative runtime state.
- Resource resolution can bypass bounded wrappers; wire JSON and dialect interning are unbounded.
- Fallback IO bypasses local subset validation; format filters silently ignore unknown IDs.
- Checkpoint pins are not validated and do not rederive checkpoint identity as promised.

## Preserved Positive Seams

- Exactly 36 schema-owned definition leaves and 36 nonempty TypeScript facades exist.
- Plural format claims are schema-derived; EPW has no fabricated MIME.
- All 40 standards remain honestly unverified.
- `ArtifactStore::new` is fallible; replay validates before apply; projections reject stale stamps.
- Runtime, fuzz, cross-platform, and standards-coverage gates fail explicitly while support is unimplemented.

## R1 Remediation Ownership

1. Plugin/definition/WIT: plural binding, immutable exact runtime validation, strict IDs, transactional assembly, inference enforcement, and host registry conflicts.
2. IO/store/CQRS: remote and snapshot atomicity/equivalence, checkpoint pins, bounded resources/wires, fallback validation, typed queries, and caller propagation.
3. Stdio schema/runtime: canonical executable leaves and mappings, declared-versus-verified counts, per-standard representations, non-positional assembly, and stdio parity.

No Wave 1 implementation begins until these P0 failures pass another frozen-tree audit.
