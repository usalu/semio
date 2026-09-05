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

## Remaining Boundary

The next server-owned MAP slice must attach an encrypted provider credential lease and a durable, cancellable provider job journal to this frozen binding. It must not invoke the native service or expose proposal bytes until the stable drawing/value members and the durable parent-plus-child visibility/WAL primitive have their own green acceptance laws.
