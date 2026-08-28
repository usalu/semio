# 📌️ Status — subset-scoped external-oracle mutation testing

Baseline `a8d1caf41f68204e73ff5e47ce40c5f543ed442d`. Harness **94/94**. TS/Rust/Go/.NET/Python hosts build.

## Measured, not claimed

| | |
| --- | --- |
| Mutation manifests | **20** owners, **403** mutations under the v2 contract |
| External-oracle coverage | **194/403 (48.1%)** — a qualifying oracle is REGISTERED |
| Subset ownership | **382/403 (94.8%)** — 21 still wildcard-owned (pdf, jpg) |
| Fixture provenance | **348/348 (100%)** |
| Fixture reproducibility | **348/348 (100%)** |
| Third-party-generated fixtures | **121** STEP cc6 + **72** BRep kernel (brepjs/OCCT) + **65** mesh (manifold-3d) |
| Leaf descriptors | 657 · payload schemas 1384 · fully-described owners 31 |
| Runtime inventory | **0/8** — blocked, see below |

`fixtureReproducibilityCoverage` 1.65% and the three 0.00% rows (`expectedOutcome`, `inverse`,
`metamorphic`, `determinism`) are honest empty denominators: the coordinates are enumerated and
report `missing`, which under this protocol fails rather than reading as 100%.

## What is genuinely externally oracled today

`step@ap214/cc6` is the worked example and it gates for real: **brepjs (OpenCASCADE 8.0 WASM)**
produces the STEP, **ruststep** re-reads it in a different engine family, **manifold-3d** and
**three-mesh-bvh** measure volume/area and symmetric Hausdorff. Meshes are compared in *tessellation
tolerances*, not a fixed constant, so a legitimately different tessellation passes and a wrong solid
does not. Plus `gltf` (120), `png` (15), `jpg` (10), `tiff` (6), `pdf` (5), `bmp` (5), and now
`sequence` (4 of 8), and now **`semio@v1/mesh` — all 17 of 17**, the first owner where every mutation
of the subset is externally oracled. `three` reads the STL/OBJ/PLY/glTF carriers, `manifold-3d`
measures in a different engine family. One mesh through all four readers agrees to 0.000e+00 while a
one-millimetre bore error separates at 1.07e-01. Details and the tolerance assumption it overturned:
`📓️mesh-pilot.md`.

## The finding that reorders the remaining work

**163 serializers do not write the format they declare**: 97 emit `print_dsl` text under a standard
extension, 33 reinterpret their own pack bytes as the target type, 19 coerce through serde into an
empty document, and 14 never read their input at all. The gate shipped finding 80 — four
false-negative classes and one false-POSITIVE class were closed, so 83 exporters had been counted as
real carriers and 2 real ones as stubs. Full breakdown and the reachability map: `📓️reachability.md`. A third-party reader of a standard format
cannot verify a mutation when the artifact writes its own DSL text instead: the parser either fails
outright or, worse, a lenient one accepts garbage. This is now the `stub-serializer` gate.

Eight owners were investigated by reading every `serialize_bytes` body rather than the directory tree,
which lies in both directions. Seven are blocked by the carrier, **not** by any absence of a library —
`tobj 4.0.5`, `stl_io 0.11.0`, `png 0.18.1`, `zip 9.0.0`, `python-pptx 1.0.2` all exist and would work
the day the export writes real bytes. `en1998` is the separate case: verified against npm, PyPI and
crates.io, no Eurocode 8 implementation is published anywhere, and all 49 of its mutations are scalar
field-sets with nothing to recompute.

So the ordering is the reverse of what the goal's phrasing implies. This is not primarily
oracle-research; it is **export correctness**. Details and per-owner verdicts:
`📓️oracle-research-findings.md`.

## Two gate bugs found and fixed this session

* A carrier oracle covered mutations its carrier provably cannot encode, because the gate checked only
  that *some* qualifying oracle declared the capability — capability-level checking standing in for
  per-mutation checking, the exact substitution this protocol exists to forbid. A requirement that
  names an oracle is now discharged only by *that* oracle. Locked by two harness checks.
* `optional: true` previously excused a QUALIFIED probe's hard failure.

## Blocked — and the blocker moved

`semio-framework` **now compiles**; that blocker is gone. Runtime inventory is still 0/9 because
`semio-s-plugin-stdio` does not build: commit `d394744295` (17:14, after this ticket's 11:04 baseline)
ADDED a new `aggregate source is not the taxonomy canonical mutation primary` check to the `Mutations`
derive, and the aggregate files it now demands have not been renamed yet. That is a peer's in-flight
refactor, established by diffing the derive against the baseline rather than inferred, and per repo
rules it is not chased.

## Next

1. Real exporters for the 53 binary stub serializers — this unlocks the carrier oracles already identified.
2. Runtime inventories once the workspace builds.
3. Split the 21 wildcard-owned pdf/jpg/tiff mutations onto real subsets.
