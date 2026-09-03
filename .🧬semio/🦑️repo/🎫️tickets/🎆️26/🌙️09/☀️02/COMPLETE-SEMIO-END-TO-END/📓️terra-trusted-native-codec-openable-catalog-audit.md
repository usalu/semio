# Trusted Native Codec And Openable Document Catalog Audit

Date: 2026-09-03  
Scope: production `linked_native_codec_bindings` / `OpenableDocumentCatalog` required by trusted hub startup, checkpoint materialization, and server-issued open plans. Read-only source audit; no source/test change or build was run.

## Decision

Add a schema-first, generated-and-statically-linked **`NativeOpenableCatalogProviderV1`**. It is compiled from the same strict descriptor/build receipts as the selected trusted bundle, exposes native Rust codec/factory pointers only inside the hub, and publishes a bounded immutable query snapshot only after the whole bundle closure, native factories, document codecs, inference services, mutation services, and opening surfaces preflight together.

Do not hand-write 59 rows in the hub, infer package identity from a plugin ID, treat a generated WASM filename as a Rust factory, derive authorization from a client-selected app/plugin, or load a new catalog generation into the current process-global codec registry. A runtime WASM component is attested by the selected bundle's exact retained bytes and hashes; a Rust factory is attested separately by code generation and static linking to the exact descriptor/build receipt. Both attestations are required.

## First Deterministic Blocker

**High — trusted startup has no linked native codec provider.** [`linked_native_codec_bindings()`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:165) returns `Vec::new()`. Any selected bundle that declares a native artifact codec fails with “no explicit native codec binding”; a selected closure with none fails because the loader rejects an empty executable codec set ([`trusted-catalog/🦀️.rs:296`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:296), [`:340`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:340)). When the bundle/profile variables are omitted, the hub silently retains no artifact authority ([`bin.rs:152`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:152)); that is not a usable trusted startup.

## Existing Assets And Exact Limits

| Area | Evidence | What is reusable | What is missing |
|---|---|---|---|
| Trusted bundle loading | [`trusted-catalog/🦀️.rs:250`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:250) canonicalizes paths, bounds and hashes component/descriptor bytes, decodes the packed descriptor, and resolves dependency-first. | Exact component SHA-256+BLAKE3, descriptor SHA-256, descriptor retention, containment checks, bounded async IO, cancellation/progress. | Native factory source identity, opening surfaces, inference/mutation factory linkage, and a query provider. |
| Existing native binding | [`NativeCodecBinding`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:109) keys plugin/package/artifact kind to an `ArtifactCodec`; loader additionally compares schema/hash ([`:296`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:296)). | The codec registration barrier is atomic after bundle verification ([`:345`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:345)). | Binding carries neither descriptor/component identity nor a factory provenance lock, apps/surfaces, inference, mutation, or viewer/editor metadata. |
| Package vs plugin identity | `VerifiedTrustedPackage` keeps `plugin_id` separate from `PackageRef { package, hash }` ([`:125`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:125)); the loader checks binding keys separately. | This separation must be retained. | `PackageDescriptor` has no `packageId` field ([`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4882`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4882)); a registry `packageName` is not a durable package identity. |
| Executable codec registry | [`ArtifactCodec`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9250) has typed pack/spr print/apply functions and a structural hash. The global registry rejects conflicts before publication ([`:9464`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9464)). | Codec functions and cross-registry assembly lock. | Process global has no removal/generation replacement; a hot reload cannot be made safe by overwriting it. |
| Live plugin-host adapter | [`PluginHostTrustedArtifactCatalog::load`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs:104) validates loaded manifests plus registry codecs and builds exact authority identities. | Its duplicate/codec/schema checks and `LivePluginPackageBinding` shape. | Hub creates no `PluginGraph` or live bindings; this snapshot does not retain descriptors/apps/surfaces and does not prove bytes of a selected WASM bundle. |
| Generated registry | [`PluginRegistryEntry`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:37) includes crate path, package name, WASM output, deps and optional descriptor hashes. | Single discovery source, registry freshness check, dependency ordering, strict fresh-build verification. | Generated Rust output is only `(pluginId, wasmOut)` at [`:935`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:935); it cannot link a codec or select an app. |
| Strict source/build gate | [`catalog-complete`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2394) insists on an absolute fresh build root; its descriptor validation checks JSON/packed descriptors, canonical pack, and raw/core/descriptor hashes ([`:2218`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:2218)). | Receipt/hash authority, 256-node/128-dependency/64-MiB artifact budgets, ordered progress/cancellation plan. | It does not emit a hub-linkable factory projection or a signed/immutable production bundle. Local `target/` is not a deployment receipt. |
| App and artifact declarations | Descriptor `PluginManifest` has plugin-level kinds, apps and contributions ([`manifest/🦀️.rs:4059`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4059)); `AppDefinition` binds a role/dialect ([`:3414`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3414)); apps may also carry `artifactKinds` as the GIS descriptor does ([`✏️s/🔌️plugins/🌍️gis/🔣️.json:169`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🔣️.json:169)). | Rich source-level publication fields and EN/DE labels. | The trusted loader considers only `manifest.artifact_kinds` ([`trusted-catalog/🦀️.rs:539`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:539)); it cannot catalog GIS's app-scoped schemas/surfaces. |
| Inference/mutation contracts | Manifest declares contributions/inferences ([`manifest/🦀️.rs:4012`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4012)); plugin host has a bounded native inference registry with preflight/register separation ([`🔌️plugin/🦀️.rs:1382`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:1382)). | Native service metadata and collision detection. | They are not part of hub startup/catalog identity or open-plan policy. |

The loader's fixed limits are sound starting constraints: bundle 4 MiB, descriptor 4 MiB, component 64 MiB, closure components 512 MiB, descriptors 64 MiB, 256 dependencies and profiles ([`trusted-catalog/🦀️.rs:16`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:16)). They do **not** mean checkpoint materialization can support a 64-MiB pair today: the current DB CAS caps an individual blob at 496 KiB ([`adapters/🦀️.rs:20`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔌️adapters/🦀️.rs:20)). P2-D chunk-CAS remains a separate prerequisite for large document pairs.

## Representative Catalog Reality

* The generated artifact map lists **59** plugin/extension WASM names, including `gis`, `stdio`, `draw`, `layout`, and `energy` ([`🦀️artifacts.rs`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🦀️artifacts.rs:1)). It contains neither package IDs, descriptor locations/hashes, artifact schemas, nor native function paths.
* GIS has a committed descriptor and editor/viewer IDs for `s.gis.gismap` and `s.gis.gisterrain`, but its top-level `manifest.artifactKinds` is empty while app-level declarations include `2d.map` / `gis.map`. Current trusted-catalog validation therefore has no authoritative way to join those surfaces to a native document codec. Its `documents.write` capability request is a host privilege request—not user document write authority ([`✏️s/🔌️plugins/🌍️gis/🔣️.json:10`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🔣️.json:10)).
* `stdio` has extensive source-native `document_codec_bare` declarations, for example XLSX ([`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs:19`](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs:19)), but no root committed descriptor was found at the generated registry owner location. It must remain unavailable to a trusted profile until it supplies a canonical JSON+packed descriptor pair and a native binding declaration; source code alone is not catalog authority.
* The headless runner has a useful contrast: it maps generated WASM names and reads committed descriptors ([`🏃️run/📦️bin.rs:88`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs:88), [`:123`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️bin.rs:123)), but missing descriptors are merely an honest per-plugin failure. Trusted hub authority must fail the selected profile atomically, not proceed with a partial openable catalog.

## Provider Contract: NativeOpenableCatalogProviderV1

Define a language-neutral schema before Rust/TypeScript generation. Its **private construction input** contains a bounded maximum of 256 package declarations and 16,384 artifact/surface rows; its **public verified query** is a redacted projection. Every identifier is bounded at 256 UTF-8 bytes, every finite list is duplicate-free and canonically byte-sorted, and no relative filesystem path, Rust symbol, factory pointer, raw descriptor/component byte, arbitrary capability request, or untrusted client selection is serializable to the public query.

```text
CatalogGenerationV1
  generationId = SHA-256("semio/hub/native-openable-catalog/v1\\0" ||
                        selected-profile-id || canonical sorted package attestations)
  package attestation:
    pluginId, packageId, version,
    componentSha256, componentBlake3, descriptorByteSha256,
    descriptorDeclaredSha256, descriptorVersion, role
  artifact declaration:
    artifactKind, artifactSchema, packSchemaHash, dialect,
    codecFactory, requiredDescriptorDigest
  opening target:
    appId, windowKindId, appRole(viewer|editor), rendererTargets,
    operationKinds(read, write, observe, inference[], mutation[])
  executable service declaration:
    identity tuple + inference/mutation schema/version/algorithm/policy,
    factory pointer, bounded execution budget
```

`packageId` must become an explicit canonical `PackageDescriptor` field (or an equivalent required packed descriptor field) and be asserted against the generator's Cargo/package projection, bundle record, `PackageRef`, and static factory receipt. It cannot be reconstructed from `pluginId`, Cargo `packageName`, crate path, or a client Hello. Preserve both the descriptor's self-hash and the exact packed descriptor-byte SHA-256; they answer different integrity questions.

The generator reads only strict descriptor pairs and fresh-build receipts, then emits two products from one sorted `CatalogGenerationV1` input:

1. an immutable trusted bundle/profile carrying exact component/descriptor hashes and declared artifact rows; and
2. a generated Rust composition projection, e.g. `native-openable-catalog.rs`, which calls each source package's explicitly exported `native_catalog_v1()` factory and supplies the expected identity constants.

The composition crate's dependency/feature projection is generated from exactly the same discovery output, so the hub does not own a handwritten 59-plugin match. Static Rust linking is unavoidable for native `ArtifactCodec` function pointers; it must be visible in the composition crate build graph, be code-generated/checkable, and use only workspace code—no runtime downloader, `inventory`-style ambient registration, dynamic library scan, or external runtime dependency. A plugin with no safe host-native factory simply has no row and cannot be selected in a native trusted profile.

At startup, `TrustedCatalogLoader` verifies retained bundle bytes first, decodes and validates the exact descriptor, then joins the generated factory only when **all** package, descriptor, component, artifact kind/schema, and pack-schema-hash fields match. It preflights codecs, inference services, mutations, and open targets under `begin_artifact_assembly`; only after the entire selected closure is conflict-free does it register codecs and publish `Arc<VerifiedOpenableDocumentCatalog>`. Duplicate plugin/package, duplicate `(package, kind, schema)`, duplicate opening target, two codecs for one schema, a factory outside the selected closure, an artifact lacking a factory, an app target not owned by the package/dialect, an inference/mutation not declared by the descriptor, or an ambiguous editor/viewer pairing is a fatal catalog error.

### Runtime-WASM Attestation Is Not Rust Linkage

The selected bundle's 64-KiB-chunk hashing/path checks establish that the runtime component bytes match `componentSha256`/`componentBlake3`; the decoder binds descriptor manifest and WASM hash ([`trusted-catalog/🦀️.rs:275`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:275), [`:532`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:532)). That does not prove that a native `fn` pointer was compiled from the corresponding plugin source.

The generated static factory projection must carry the same fresh receipt identity and the hub must compare it before accepting the pointer. Conversely, the Rust linkage receipt is not permission to instantiate/execute arbitrary WASM. WASM/node runtime handles remain client/host runtime attestations under their own `PackageRef`/component digest checks. The hub's native authority executes only verified native codecs and never uses a client's reported plugin/module identity as a substitute for either proof.

## Verified Queries And Open-Plan Boundary

`OpenableDocumentCatalog` is an immutable read-side port backed solely by the verified generation:

* `resolve_existing(DocumentDescriptor)` takes the durable descriptor owner tuple, kind/schema/pack hash, and optional pinned generation; it returns a single exact artifact/opening row or `NotAvailable`. No fallback by plugin ID, schema, app label, or newest version.
* `list_creatable(scope, actor)` first uses hub-derived membership/space policy, then returns only catalog rows allowed by the server's creation policy. The client may supply a display preference, never the authoritative owner/package/app/surface.
* `select_open_plan(actor, document, preferredSurface?)` resolves actor/session/scope and durable descriptor first, chooses a verified compatible view/editor target, intersects **server** read/write/share policy with the target's declared operation kinds, and returns an immutable plan with exact package/component/descriptor identities and redacted UI labels/EN+DE metadata.
* `resolve_execution(identity)` is internal to checkpoint validation/inference/mutation paths. It exposes a native factory only after exact identity resolution; it is never an HTTP/MCP general catalogue dump.

`AppRole::Editor` and `WindowKindDefinition.surface_kind` describe a declared UI's functional shape ([`manifest/🦀️.rs:3414`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3414), [`:3190`](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:3190)); they do not authorize a member to write. A public share read-only policy may yield only viewer/read/observe operations even where an editor target exists. Inference/mutation availability must additionally meet descriptor declaration, native factory presence, document frontier/generation binding, and caller authority. Package capability requests are never copied into the user authorization result.

The persisted `DocumentDescriptor` already records plugin ID, package ID, version, component hash, kind/schema and pack hash in the SQLite projection ([`🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:434`](/Users/ueli/Documents/semio/🌎️hub/📇️directory/🪶️sqlite/🦀️.rs:434)). It should be validated against the provider at create/announce and read at open/rebootstrap. Add the descriptor-byte SHA and catalog generation only if the current tuple cannot distinguish future aliases; do not replace the immutable owner identity with an app choice.

## Generation, Progress, Cancellation, And Invalidations

Use the present `OperationContext` checkpoints in every file read/hash/package/factory/app/service stage. Keep the current 30-second hub-start deadline only as a launcher-level upper bound ([`bin.rs:158`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:158)); replace `StartupCatalogControl::is_cancelled() == false` ([`bin.rs:139`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:139)) with real startup cancellation. Report fixed stage totals: `bundle`, `packageBytes`, `descriptor`, `nativeFactory`, `codec`, `openTarget`, `service`, and `published`; diagnostics stay bounded as the existing authority adapter does.

There is no safe in-process catalog replacement today: `DOCUMENT_CODEC_REGISTRY` is a process-global `OnceLock<RwLock<...>>` ([`store/🦀️.rs:9403`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9403)) and permits no unregister. Catalog generation is therefore startup-only. If an input/receipt changes, mark that launcher generation stale, stop admitting new opens/creates, drain or restart the hub, build a fresh process, and make clients re-request server open plans. Existing documents are never rebound to a newer package because a plugin label matches. This is the safe invalidation semantics until registries become instance-scoped.

## Ordered Implementation Packet

1. **Schema and identity closure (H1).** Add `NativeOpenableCatalogProviderV1`/public query schemas and neutral fixtures. Add explicit `packageId`, exact artifact-to-app/window opening declarations, renderer availability, and self-owned/contributed inference/mutation identity to packed descriptors. Define canonical package/generation digest format and fixed bounds before code generation.
2. **Plugin native factory ABI (H2).** Give each eligible plugin package an explicit host-native `native_catalog_v1()` factory that returns typed codecs plus declared opening/service metadata. Migrate a minimal valid cohort first (one app/editor/viewer artifact and one headless codec); un-migrated packages, including current `stdio`, are unavailable rather than guessed.
3. **Registry generator projection (H3).** Extend [`📜️script.ts`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts:1805) to emit the strict bundle manifest, factory composition projection, Cargo dependency/feature projection, generation lock, and launch/profile metadata from one source audit. Make `check` fail an incomplete selected production profile; retain ordinary development warnings only outside the trusted profile gate. Never hand-edit generated output.
4. **Provider and loader assembly (H4).** Extend [`NativeCodecBinding`](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🗂️trusted-catalog/🦀️.rs:109) or replace it with an exact factory binding; retain verified descriptors in `VerifiedTrustedCatalog`, build an immutable openable index, preflight codecs/services/openers as one transaction, then publish it beside `ValidatingCanonicalArtifactAuthority`.
5. **Hub composition and authority queries (H5).** Replace the empty [`linked_native_codec_bindings`](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs:165) with the generated composition crate. Fail startup if a configured profile cannot produce one complete verified provider. Thread only verified catalog queries into document announce/create, checkpoint/rebootstrap materialization, and future server-issued open-plan CQRS handlers.
6. **Open-plan and client consumption (M1).** Server derives actor/session/scope/document; React/native only render the returned immutable plan. Enforce viewer/editor/mutation/inference policy server-side and include generation/frontier in stale-result rejection. This can proceed in parallel with P2-D, but no test may claim 64-MiB checkpoint materialization before chunk-CAS lands.
7. **Generation lifecycle (M2).** Wire readiness to expose only redacted generation state, add stale/restart handling, and make launch scripts build/check the exact factory/bundle pair before the hub starts.

## Neutral Fixtures And Independent Oracles

Create a small fixture set—not a copied 59-row catalog—with one valid app-owned artifact (GIS-shaped editor/viewer), one valid headless artifact (stdio-shaped), one declared inference/mutation, and these hostile cases: wrong package ID; descriptor self-hash vs descriptor-byte hash confusion; WASM SHA/BLAKE mismatch; stale generated static receipt; duplicate schema/kind/open target; app dialect that does not own the artifact; missing codec/factory; factory outside closure; app editor reported as public-write; unlinked inference/mutation; cancelled/expired generation; and a re-open after generation replacement.

* The language-neutral oracle reads the JSON schema/fixtures with Node built-in `crypto` and the existing canonical pack encoder, recomputes descriptor/generation digests, verifies sort/duplicate/ownership rules, and expects the same accepted/rejected public query result.
* Rust independently decodes the packed descriptor and fixture, verifies actual `ArtifactCodec` hashes/function registration, and compares the public projection byte-for-byte with the fixture. Factory pointer behavior is Rust-only, but its identity decision must match the neutral oracle.
* A direct process test starts the hub with a selected generated fixture profile, confirms `readyz` reports a verified catalog generation, creates/opens a document with no client-supplied plugin authority, validates a checkpoint/rebootstrap codec resolution, rejects a mismatched descriptor, and proves a read-only/share session cannot select mutation/editor-write. Capture logs and ensure no raw bundle path/factory symbol leaks.
* Build/runtime tests must use a fresh declared build root, not pre-existing `target/` residue; run the process oracle on Windows, macOS, Linux, and devcontainer once platform launch/secure session prerequisites exist.

No commands were run here. Focused commands after implementation are:

```sh
bun nx run @semio-tech/plugin-registry:check
bun nx run @semio-tech/plugin-registry:catalog-complete -- --build-root <absolute-fresh-build-root>
bun nx run os-hub:test-quick
bun nx run os-hub-ts:test
bun nx run @semio-tech/framework-os-dev:build -- gis
```

## Blocker Order

1. **High:** Empty `linked_native_codec_bindings` makes every nonempty trusted profile fail; no `OpenableDocumentCatalog` exists.
2. **High:** Descriptors do not carry canonical package IDs or exact artifact→app/window opening declarations; current trusted validation misses app-scoped GIS kinds.
3. **High:** Registry generation emits only WASM filenames and cannot statically link/attest native factories; source/native codecs are not deployment evidence.
4. **High:** `stdio` and other incomplete descriptor/factory sources cannot join a trusted profile despite source codecs; do not generate omissions as valid rows.
5. **Medium:** Global codec/service registries preclude safe in-process catalog generation swap; require restart semantics now.
6. **Medium:** P2-D chunk-manifest CAS/retention remains necessary for authority materialization of pairs larger than 496 KiB.

