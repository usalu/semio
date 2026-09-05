# GIS Map Frozen Verified Binding

## Outcome

The hub now retains the concrete `Arc<VerifiedTrustedCatalog>` alongside its erased document-open catalog and derives an optional process-lifetime `VerifiedGisMapArtifactBindingV1` before readiness publication. The binding is present only for the sole verified GIS Map editor target with read/write/observe grant and pins the catalog generation, package/version, component SHA-256 and BLAKE3, raw descriptor-byte SHA-256, artifact and pack schema, full parent dialect, selected surface and renderer, declared inference service versions, and the literal non-capturing GIS executable.

This packet deliberately adds no public route, provider credentials, client payload authority, inference execution, approval, or artifact publication.

## Owned Source

- `🌎️hub/🧪️fixtures/🗺️gis-map-frozen-binding-v1/{🧬️.schema.json,🔣️.json}`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs`
- `🌎️hub/💡️inference/{🦀️.rs,📇️catalog/🦀️.rs}`
- `🌎️hub/📦️packages/🦀️rust/{Cargo.toml,🚀️bin.rs,📜️script.ts,📋️project.json}`
- `.vscode/🧩️launch.seed.jsonc`
- generated `.vscode/launch.json`

## Invariants

- The binding owns the verified catalog `Arc`; it is never reconstructed from `DocumentOpenPlanV1` or client bytes.
- The catalog contributes exactly one retained open selection. Non-GIS profiles and read-only GIS viewer profiles produce no write binding.
- Every package hash role remains distinct and is compared to its independently verified package fact.
- The selected target must be `s.gis.gismap` / `gis.map` / `s.gis.gismap@1/*#editor` / `gis2d-main` with the complete `s.gis.gismap@1/*` parent dialect.
- The declared contribution must be the unique `s.gis.gismap.inference` service owned and contributed by GIS with all five versions equal to 1 and no dependencies.
- The actual `ArtifactInferenceService` metadata and process-local executable identity must equal a fresh literal `gis_map_inference_service()` capability.
- The canonical binding digest is SHA-256 over a NUL-separated v1 domain and the exact neutral projection.

## TDD Evidence

The registered neutral/AJV source gate was run before the retained binding existed and failed with:

```text
hub does not retain the exact verified GIS Map catalog and executable binding
```

After implementation, the direct and Nx source gates passed:

```text
gis-map-frozen-binding-check: checks=44 clean; no route, provider, inference execution, or publication claim
```

The exact native law `inference::catalog::tests::gis_map_verified_binding_freezes_catalog_selection_and_native_executable` is registered with a ticket-owned receipt and target directory. Native compilation is still active at this checkpoint; its final receipt or exact external blocker will be appended here.

## September 5 Verification Continuation

The production `local-stdio-gis-open-v1` profile previously selected a viewer, so startup could never produce this editor binding. Its neutral corpus, schema, native validation, materializer, and rotation identities now select the exact Map editor with read/write/observe and `wasm` renderer. The independently recomputed profile generation is `7cf0515d5cb12f9404020fef548b46f6b0b3dca140d4e1f2b58329174a40ebef`. The registered `os-hub:trusted-stdio-gis-bundle-check -- --source` passed (2 packages, 28 codecs, 19 hostile cases, 8 cancellation cases, 4 descriptor pairs, 1 stale plan); this does not establish materialization or hub startup.

The frozen projection corpus now uses that reachable `wasm` renderer and covers substitution of all 34 leaf fields, including every service version and contributor. The digest is `435e02065db2d2e0694b6b1c468ee24ae8fe8711432f112f510aaa103a546386`. A source checker now rejects missing, duplicate, or unknown field coverage. The registered `os-hub:gis-map-frozen-binding-source-check` passed with 52 checks after these changes. The catalog constructor also clones its retained selection before moving the catalog owner, avoiding a borrow-after-move error.

The first native binding build finished **RED before any selected law ran**, with an unresolved `DirectorySpaceDetailV1` import in the shared directory client. That production import had already been replaced by concurrent administration work when the result was inspected. A cached-target rerun is now active; neither constructor acceptance nor process readiness is claimed.

The `exact-cargo-laws-dFj7qf/00` rerun also finished **RED before any selected law**, on a stale BMP `replace-pixel-data` include in Stdio (`🧮️` in the captured source, now `🔲️` in the mount). The current destination and matching descriptor both exist, verified read-only after the failure. No old path was restored. A further exact hub-library rerun is queued on the same cached target as the Home process binary build; native acceptance is still unclaimed.

A second registered native law, `artifact_authority::trusted_catalog::tests::gis_map_binding_constructs_from_loaded_catalog_and_retains_verified_bytes`, now exercises the actual GIS plugin assembly/descriptor emission and linked private native codec receipts through `TrustedCatalogLoader::load`. It covers an editor binding, a viewer without a write binding, denial of a foreign service contributor, backing-file tampering, and retained catalog/byte ownership. Its component bytes are explicitly synthetic; this is catalog/binding acceptance, never a Wasm execution test. The law is not yet run. The cached-target rerun selects this acceptance law alongside the original projection/function-identity law.

## Remaining Runtime Boundary

The next server-owned MAP slice must attach an encrypted provider credential lease and a durable, cancellable provider job journal to this frozen binding. It must not invoke the native service or expose proposal bytes until the stable drawing/value members and the durable parent-plus-child visibility/WAL primitive have their own green acceptance laws.
