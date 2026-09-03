# Hub Trusted Catalog Loader

## Outcome

Implemented the bounded P2 catalog-verification slice without advertising the repository's incomplete 59-row generated registry as a deployable catalog. The hub now has a schema-first immutable bundle contract and an opt-in startup loader that retains the selected dependency closure as exact component bytes, component SHA-256, component BLAKE3/`PackageRef`, raw descriptor bytes and SHA-256, the decoded existing `PackageDescriptor`, independent plugin/package identities, and exact hub-native artifact codec identities.

The loader validates the complete selected closure before one registry assembly commit. It does not introduce a second semantic descriptor. Descriptor inputs remain the existing `🛂️.descriptor.semio` pack bytes decoded into `semio_framework::PackageDescriptor`.

## Production Contract

- The JSON Schema fixes bundle version, row shapes, identity/path limits, per-component and per-descriptor byte ceilings, dependency/profile/package/codec count ceilings, lowercase 32-byte digest encoding, and nonzero codec schema hashes.
- Bundle and file reads are bounded before allocation. Component hashing uses one retained byte buffer for SHA-256 and BLAKE3, yields every 64 KiB, and observes cancellation/deadline checkpoints. The selected component closure is capped at 512 MiB and its descriptor closure at 64 MiB.
- Canonicalized component and descriptor paths must stay below the canonical bundle directory. Lexical traversal, absolute paths, symlink escapes, path reuse, canonical aliases, and reuse of the bundle path are rejected.
- Plugin id and package id remain distinct fields throughout the bundle, verified package, `PackageRef`, codec binding, and authority identity. Duplicate plugin ids, duplicate package ids, conflicting exact roots/dependencies, missing rows, self-dependencies, and cycles fail before activation.
- Every descriptor agrees with the trust record on descriptor version, role, plugin id, version, component SHA-256, dependency compatibility, and artifact kind/schema set. `PackageDescriptor` has no package-id field, so package id is independently attested by the bundle rather than fabricated from plugin id.
- Every selected artifact kind requires one explicit `NativeCodecBinding` for the exact plugin id, package id, artifact kind, and schema. Missing, extra, duplicate, zero-hash, and hash/schema-mismatched bindings fail before activation.
- Codec activation uses the store's assembly preflight followed by its batch registration. All filesystem, hash, descriptor, closure, and binding checks happen before that commit; there is no fallible cancellation/progress check after the commit.
- `OS_HUB_TRUSTED_CATALOG_BUNDLE` and `OS_HUB_TRUSTED_CATALOG_PROFILE` are opt-in paired startup settings. Neither preserves existing startup behavior; only one fails closed. A successfully verified catalog is wrapped in `ValidatingCanonicalArtifactAuthority` and retained in `HubState` before routes open.
- The production `linked_native_codec_bindings` provider is intentionally empty today. Therefore setting the bundle/profile cannot manufacture a usable catalog from static declarations. Startup will reject any selected artifact kind until genuine hub-linked native codecs are supplied.

## Tests and Independent Oracles

The language-neutral two-package fixture intentionally lists the dependent package before its prerequisite. Rust proves deterministic dependency-first resolution, byte/hash retention, independent plugin/package identity, existing binary descriptor decode, explicit native binding, progress bounds, cancellation, and no registry activation on component-bit, descriptor-bit, missing-binding, lossy-package-id, zero-hash, or mismatched-hash failures.

The same fixture is consumed by a permanent TypeScript test. AJV 2020 independently validates the schema and exact max/max+1 vectors; Node crypto recomputes component and descriptor SHA-256; and a separate TypeScript closure walker independently obtains `fixture.base` before `fixture.editor` while rejecting missing/conflicting identity rows. The Rust `blake3` dev crate independently matches the repository BLAKE3 implementation against the pinned vector.

Evidence on 2026-09-03:

- `CARGO_TARGET_DIR=…/🗑️generated/hub-trusted-catalog-target bun nx run os-hub:test -- --lib trusted_catalog`: **4 passed, 31 skipped** after the final loader validation changes.
- `CARGO_TARGET_DIR=…/🗑️generated/hub-trusted-catalog-target bun nx run os-hub:test -- --bin os-hub trusted_catalog_startup_is_opt_in`: **1 passed, 29 skipped** after the peer-owned test-helper correction.
- `bun nx run os-hub-ts:test -- -t 'immutable trusted-catalog'`: **1 passed, 6 skipped**.
- First red run compiled successfully and executed four tests; it exposed a malformed pinned descriptor digest and the pack decoder's integral values arriving through the serde bridge as exact integral floats. The fixture was corrected and the existing descriptor decoder now normalizes only finite, integral values inside the shared JSON-safe integer range before `PackageDescriptor` hydration.
- The focused bin startup filter initially reached an unrelated concurrent P2-C `Semaphore: Default` compile error. Its owner corrected the four-field test helper, after which the warmed startup filter passed.

## Files

- `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🧬️schema/🔣️bundle.schema.json`
- `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🧪️fixtures/🧬️two-package/🔣️.json`
- `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/Cargo.toml`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- `🌎️hub/📦️packages/🟦️typescript/🧪️index.test.ts`

## Exact Residuals

1. Verified component bytes are not yet attested to a compiled/instantiated Wasmtime handle. The current plugin host drops package identity from `CompiledHandle` and the audited `WasmtimeNodeHost` path is not a genuine hub-native executable catalog. This slice therefore names and retains verified bytes plus explicit native `ArtifactCodec` function bindings, but does not claim the component itself was compiled or loaded.
2. No valid complete production bundle/profile exists for the audited 59-row registry: 19 rows lack committed descriptors/metadata, only one expected component exists, and its `stdio` closure is absent. No fake roots or fallback identities were added.
3. The authority's 64 MiB pair ceiling remains larger than the current database artifact blob adapter's 496 KiB per-blob ceiling. This loader does not imply large checkpoint publication support.
4. This slice composes the verified catalog with `ValidatingCanonicalArtifactAuthority`; durable blob staging and verified publication remain the already-existing downstream seams and are not exposed through a new route here.
