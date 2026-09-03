# Stdio Catalog-Root Final Audit

Read-only final source audit, 2026-09-03. I did not run a build, test, or runtime command. Reported gate outcomes below are attributed to the implementation lane.

## Verdict

**REJECT.** The current packet closes several earlier structural gaps and safely keeps hub codec admission empty, but it does not yet provide the requested independent, reader-visible catalog authority. The two acceptance blockers are:

1. The reader path requires a freshly read owner JSON/Pack pair as `StrictCatalogDescriptor`; therefore owner files remain authoritative input, not non-authoritative derivatives of the completion marker.
2. `PluginRegistryEntry` carries only a stripped `pluginId`; its generic projection reconstructs `packageId` as `semio:${pluginId}`. That is an inferred fallback rather than an explicit, strict Cargo component-contract identity carried through every projection.

The final monolithic stdio Rust gate is also red, so no source-level repair may be treated as a fully compiled stdio packet.

| Boundary | Decision | Evidence and qualification |
| --- | --- | --- |
| Guest `PackageDescriptor` construction and first-party structural codec | **ACCEPT, source only** | `PackageDescriptor` is now required `package_id` with direct `ToValue`/`FromValue` and `deny_unknown_fields` in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4901`. The stdio guest obtains it from its bounded, duplicate-section/key-rejecting Cargo component parser in `✏️s/🔌️plugins/🗄️stdio/🦀️.rs:192`, and both describe constructors use the required field in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs:141,199`. The test's serde use is an oracle only, not a production conversion route. This row is not a compilation claim because the final Rust gate is red. |
| Cargo identity through registry/source projection | **REJECT** | `parsePluginCargo` reduces `[package.metadata.component].package` to `pluginId` with a regex at `…/📇️registry/📜️script.ts:231-280`; it neither preserves `packageId` nor applies the stdio parser's duplicate-section/key and grammar rules. `validateCatalogDescriptorPair` then synthesizes `semio:${entry.pluginId}` at `:2361`. The stdio producer does compare its descriptor to `componentPackageId()` before publication (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:406,452`), but `catalog-complete`/the reusable verifier do not retain that exact Cargo-contract value. |
| Marker-last staging, bounds, cancellation, and tamper checks | **ACCEPT as a producer-local check; not an authority acceptance** | `CatalogRootScript` requires an absolute empty, dedicated root; stages raw/core/descriptor, then publishes the row, mirrors owner files, regenerates/audits, and writes the marker last (`…/📦️packages/🦀️rust/📜️script.ts:360-453`). Copies and marker writes have 64 KiB I/O, 64 MiB artifact / 64 KiB marker-descriptor limits, cancellation/deadline checkpoints and file `fsync`. The marker carries exact path, size and SHA-256 receipts for raw, core, staged descriptor and owner Pack/JSON (`…/📇️registry/📜️script.ts:2094-2111`); it is written to a temporary file, synced, and renamed. No power-loss/directory-durability run was supplied, so this is not a crash-recovery runtime claim. |
| Reader-visible immutable completion authority | **REJECT** | `FreshCatalogBuildVerifier.verify(source)` requires caller-supplied `StrictCatalogDescriptor`, rereads the owner JSON/Pack before marker comparison and again at final validation (`…/📇️registry/📜️script.ts:2597-2627`). It therefore cannot consume a marker plus staged row on its own. Deleting or mutating the owner pair blocks a marker that otherwise has intact staged raw/core/descriptor bytes. This is fail-closed, but it contradicts the requested boundary that owner files be non-authoritative derivatives. The hub has no marker consumer. |
| Read-after-verify mutation law | **ACCEPT, source plus attributed focused test** | `readVerifiedCatalogArtifact` retains bytes read through one descriptor, checks size before/after, applies bounds/cancel/progress, and `CatalogArtifactReceipt` returns those bytes (`…/📇️registry/📜️script.ts:2113-2131,2501-2528,2619-2637`). The consumer/oracle hashes the retained bytes, not reopened paths (`…/📦️packages/🦀️rust/📜️script.ts:454-457`). `catalog-complete.test.ts:192-209` deterministically rewrites raw after it is captured and proves only the retained original bytes are returned; later descriptor mutation rejects. The lane attributes `NX_SKIP_NX_CACHE=true bun nx run @semio-tech/plugin-registry:test-quick -- catalog-complete` as green: 1 file, 5 tests, 0 failures, 9.90 s. I did not run it. |
| Current codec admission / glTF negative | **ACCEPT: intentionally empty and fail-closed** | `native_codec_factory_receipts()` rejects the current `6` schema codec rows / `0` executable registrations rather than promoting the 26 runtime candidates (`✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:981-988,1110-1124`). Its prospective receipt checks private factory id, descriptor codec id, runtime capability, kind, schema, extension and non-zero hash. The glTF law records zero factory hash and extension/claim mismatch (`:1127-1145`). Hub `linked_native_codec_bindings()` remains `Vec::new()` at `🌎️hub/📦️packages/🦀️rust/📦️bin.rs:366-368`; this is the correct current safe state. |
| Non-empty codec authority | **REJECT / deferred** | The trusted bundle exposes only `{artifactKind, artifactSchema, packSchemaHash}` and hub binding only plugin/package/kind/`ArtifactCodec` (`🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:58-62,111-122`). It cannot attest the receipt's `factoryId`, `descriptorCodecId`, `runtimeCapabilityId`, extension, or representation/format mapping. A non-empty binding would therefore be an unsafe inference even though zero-hash is rejected today. |
| Whole-packet gates | **REJECT** | The implementation lane reports the final monolithic stdio session `74967` red after about 13 minutes, with 1,852 residual owner serde `E0277` diagnostics and an MP4-owned `E0502` subsequently patched. It explicitly reports no PackageDescriptor identity/receipt diagnostic, but no final stdio Rust green. Hub strict-projection and framework-codec sessions had no final outcome supplied to this audit. |

## Exact retained-byte law

For a marker accepted by the current verifier, each returned artifact byte array is the exact byte sequence hashed from the same open descriptor. A replacement after that read may make a later verification fail, but it cannot cause that invocation to consume newly reopened bytes. This is materially stronger than an end-of-path rehash and is the part of the race repair supported by the attributed focused gate.

It does **not** make the marker an independent authority: the same invocation still reopens the owner JSON/Pack to construct and revalidate `source`.

## Smallest next packet

1. Add required `packageId` to `PluginRegistryEntry`, parse the complete Cargo value once with the same bounded duplicate/key/grammar law as the stdio parser, and compare it directly at descriptor audit, marker creation, generated registry projection, and trusted bundle record. Remove every `semio:${pluginId}` reconstruction from authority paths.
2. Define one marker-owned canonical receipt payload: package identity, raw/core/descriptor bytes' exact hash+size, canonical staged descriptor Pack hash/self-hash, and the future codec-receipt collection (currently empty). Implement a reader that decodes and validates only the marker and staged row, returning retained bytes. Owner Pack/JSON may be checked while producing a marker, but must not be reopened as a reader prerequisite.
3. Keep `linked_native_codec_bindings()` empty. Before a non-empty change, extend the schema/bundle/hub binding tuple with `pluginId, packageId, descriptorCodecId, factoryId, runtimeCapabilityId, artifactKind, documentSchema, extension, representationId/mime/extensions, packSchemaHash`; generate it from a non-empty verified receipt only and reject zero hash, extension or capability mismatches. Add neutral vectors and a Rust factory oracle for that exact tuple.
4. Obtain clean, scoped final Rust stdio, hub strict-projection, and framework value-codec gates. Attribute logs in the implementation report; do not fold unrelated serde diagnostics into this packet.

## Older-audit deltas

The earlier missing `PackageDescriptor` codec, missing stdio owner pair/root marker, and post-read artifact mutation gap are no longer accurate descriptions of the live source. The remaining blockers are narrower: authority still depends on owner files, registry identity still reconstructs the package id, and non-empty native codec admission has no full cross-layer tuple.

## Files reviewed

- `✏️s/🔌️plugins/🗄️stdio/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛂️describe/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/{📜️script.ts,🧪️catalog-complete.test.ts}`
- `🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs`
- `🌎️hub/📦️packages/🦀️rust/📦️bin.rs`
- Prior ticket reports `📓️terra-stdio-catalog-root-completion-audit.md`, `📓️terra-stdio-native-codec-receipt-post-implementation-audit.md`, and `📓️terra-package-descriptor-value-codec-audit.md`.
