# GlTF Atomic Inference Freeze Report

## Boundary

- The old `geometric-analysis` GraphQL and Proto facets were removed. No glTF inference service, descriptor, artifact-definition row, or registry entry uses the old canonical aggregate ID or the removed `bounds` ID.
- The frozen public assembly is exactly `GltfInference { geometry: GltfGeometricInference }`. It delegates to `dag-assembly`; the inference root declares/reexports the leaf DAG and does not contain geometric calculations or public leaf-result construction.
- `geometry-core` contains reusable, non-public geometry/topology/statistics primitives. The 67 leaf folders own executable leaf functions, unavailable handling, descriptor/ID/version/cache metadata, typed result encoding, and facets.

## Static Census

| Check | Result |
| --- | ---: |
| Physical Rust leaf folders (`encode_result`) | 67 |
| Leaf descriptors/services | 67 |
| Unique canonical `s.stdio.gltf.inference.<slug>.v1` IDs in descriptor table | 67 |
| Unique canonical IDs in glTF artifact definition | 67 |
| Missing TypeScript/GraphQL/JSON Schema/Proto leaf facets | 0 |
| Leaves missing `infer` or `infer_pair` | 0 |
| Leaves missing `unavailable_measure` | 0 |
| Leaf folders with direct Rust test modules | 61/67 |
| `geometric-analysis` / `geometric_analysis` / `geometricAnalysis` public-source aliases | 0 (except one negative absence assertion) |

The six size leaves below have their executable/formula and unavailable paths but do not yet have a leaf-local Rust `#[cfg(test)]` module: `axis-aligned-bounds`, `oriented-bounds`, `bounding-box-dimensions`, `characteristic-length`, `footprint-area`, and `projected-area`. This is an explicit remaining test-coverage gap; they are not claimed runtime-verified independently.

The final static ban scan reports the frozen Rust field as `pub geometry: GltfGeometricInference`, zero stale camel/snake aliases, zero deleted aggregate facets, and exactly one `geometric-analysis` occurrence: the root parity test's explicit negative assertion that the forbidden ID is absent.

## Runtime Evidence

The following command passed before the later unrelated shared mutation-framework break:

```sh
SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/stdio-plugin:test-quick -- -- manifest_requires_exactly_one_fully_faceted_service_per_leaf
```

Result: 1/1 passed; it checked the 67 field IDs, 67 independently registered services, 67 artifact descriptors, canonical ID parity, algorithm/cache metadata, and absence of aggregate/bounds inference IDs.

The following targeted analytic command passed:

```sh
SEMIO_TEST_BUDGET_MS=120000 bun nx run @semio-tech/stdio-plugin:test-quick -- -- pair_geometry_preserves_contact_distance_and_box_overlap_leaf_semantics
```

Result: 1/1 passed. It verifies contact distance `0`, contact area `9` with deterministic-estimate quality, overlapping AABB volume/interference volume `12`, separated AABB volumes `0`, separated distance `2`, and exact zero-contact quality.

## Current Shared Build Boundary

A final rerun of the parity command was blocked after a concurrent shared build updated mutation framework inputs. The glTF inference compilation had already completed cleanly in the immediately preceding passing runs. The rerun fails only in out-of-scope Semio mutation derives:

- eight `E0425` errors: `::protocol::MutationOutcome` cannot be found;
- eight corresponding `E0061` errors: generated calls supply three arguments to a four-argument function.

Affected artifact mutation files are Semio `brep`, `drawing`, `mesh`, `text`, `table`, `graph`, `object`, and `kit`; no reported error originates under the glTF inference, glTF transport, or permitted glTF glue regions.

## Post-freeze ledger correction

The first structural gate after the source freeze rejected all 67 definition rows because they were marked with the unsupported `declared` state and `executable_registration: false`, despite the direct 67-service registration implemented in this wave. The combined-tree ledger was corrected to `status: implemented` and `executable_registration: true` for exactly those 67 inference rows.

```sh
bun 📜️script.ts stdio quick
```

Result after correction: pass — 36 artifacts, 40 dialects, and 6 codecs. The `verified` state remains intentionally unset: six leaves still lack a leaf-local Rust test module, and the umbrella per-leaf analytic/incremental/budget/metamorphic acceptance matrix has not yet run.

The same post-freeze audit compared each leaf JSON facet's target with the authoritative root member target and found all 67 leaf targets flattened to `geometry.overall.<slug>`. They were mechanically corrected to their exact typed paths such as `geometry.overall.size.axisAlignedBounds`, `geometry.overall.areaVolume.surfaceArea`, and `geometry.overall.topology.genus`. The root/leaf ID-to-target set difference is now zero, and the stdio quick gate remains green after the correction.

## Open multi-implementation and schema-parity defects

The frozen source boundary is not full umbrella acceptance. A deeper current-tree audit found:

- all 67 TypeScript leaves currently expose metadata descriptors only; they do not execute their leaf computations;
- all 67 leaf GraphQL and Proto result facets expose the measure as opaque `valueJson` / `bytes value_json` instead of the leaf's typed canonical measure value;
- the leaf JSON files currently carry `x-semio` descriptor metadata but do not yet define the complete result-value JSON Schema;
- the root GraphQL/Proto facets describe only the leaf DAG and do not mirror the full frozen `GltfInference { geometry }` result contract;
- six leaves have no leaf-local Rust test module, and the existing modules are primarily descriptor tests rather than the required analytic/golden, incremental, stale, budget, malformed-input, and metamorphic matrix per leaf.

These defects require a dedicated multi-implementation/schema/test remediation pass. The 67 Rust computations and independent native service registrations remain implemented, but the GLTF inference family must not be marked `verified` until these gaps are closed.
