# GIS Map Executable Inference Service

## Scope

Implemented the missing native execution boundary for the already-declared `s.gis.gismap.inference` schema. The artifact declaration now carries one whole-map `ArtifactInferenceService`; no leaf services or external runtime dependencies were introduced.

## Contract

- Owner: `gis`
- Artifact kind: `s.gismap` (the artifact declaration identity)
- Artifact schema: `s.gis.gismap`
- Document schema: `gis.map`, version 1
- Inference schema: `s.gis.gismap.inference`, version 1
- Algorithm/policy versions: 1/1
- Input: `<GisMapSnapshot as store::ArtifactPack>::decode_pack(request.canonical_payload)`
- Output: `GisMapInference` converted through its first-party `ToValue` implementation and encoded with `pack_rt::encode_wire_value`
- Validity/quality/completion: `valid` / `exact` / `true`
- Cache behavior: cold and bypass are honored; incremental is rejected because this inference schema has no incremental algorithm or inferred-field cache
- Bounds: request bytes are admitted before snapshot decoding; every visited `DslValue` consumes one work unit and is checked against the recursion budget; encoded output is checked against the allocation budget
- Cancellation: the service requires a non-empty cancellation identity. Active pre/during/post cancellation observation remains at the framework wire wrapper, because `ArtifactInferenceExecutionRequest` exposes an identity but no cancellation probe to a plugin fn pointer.

## Test Design

The language-neutral vector `🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/infer-gismap-1/🧫️fixtures/🔣️.json` pins schema/version, empty-map behavior, mixed position/route/region counts, and geographic bounds.

The subject executes through the real `ArtifactInferenceService` and decodes the emitted typed wire value. The independent third-party oracle uses `geo 0.31.0`'s `BoundingRect` over the vector's explicit oracle coordinates, then compares its rectangle both with the subject and the committed expected value. `geo` is dev-only with default features disabled.

Focused Rust coverage verifies:

- service metadata, local registry insertion, real declaration construction, and full GIS plugin assembly;
- deterministic byte-identical output and typed decode;
- empty and mixed language-neutral vectors against `geo::BoundingRect`;
- structured malformed-snapshot failure;
- allocation/work/recursion admission and cancellation identity;
- truthful incremental rejection and cache-mode preservation.

## Verification Evidence

1. `bun nx run @semio-tech/gis-plugin:test-quick --skip-nx-cache`
   - Initial test-first run produced only the Nx/script headers, then slept on `/Users/ueli/Documents/semio/target/debug/.cargo-build-lock` with no compiler child.
   - It was interrupted and its two orphaned, task-owned Cargo processes were terminated explicitly.
   - Result: exit 130; no compiler/test result claimed.
2. `CARGO_TARGET_DIR='<ticket>/🗑️generated/gis-inference-cargo-target' bun nx run @semio-tech/gis-plugin:test-quick --skip-nx-cache`
   - Reached compilation independently of the shared target.
   - Result: exit 1 before the GIS crate; shared dependency `semio-framework` failed with two `E0277` errors because `kernel::CapabilityGrant` did not satisfy `ToValue`/`FromValue` in the concurrent tree.
3. Same isolated command, warm retry.
   - Result: exit 1 before the GIS crate; the prior framework error had moved, but shared dependency `semio-framework-os-infinite` failed with five `E0283` type-annotation errors and 578 warnings.
4. Same isolated command after correcting the service artifact-kind identity and adding full plugin-assembly coverage.
   - Result: exit 1 before the GIS crate; shared dependency `semio-framework-os-infinite` again failed with five `E0283` type-annotation errors and 578 warnings.
5. `jq -e . '<fixture>' >/dev/null`
   - Result: exit 0; the language-neutral vector is valid JSON.
6. `realpath '<fixture>/../../../🧬️schema/💡️inferences/🔣️.json'`
   - Result: exit 0 and resolved to the checked-in `gismap` inference JSON Schema.
7. `git diff --check -- '<scoped files>'`
   - Result: exit 0.

No GIS test pass is claimed until a run reaches and executes the GIS crate.

## Changed Files

- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/infer-gismap-1/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml`
- `Cargo.lock` (the `geo` dev-oracle resolution is this lane's portion; the file also contains concurrent unrelated changes)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-gis-inference-service.md`

## Residual Blocker

The current shared tree does not compile far enough to type-check or run GIS because independently modified `semio-framework-os-infinite` fails first. The ticket-local Cargo target used to avoid unrelated shared-lock contention was deleted after the final retry.
