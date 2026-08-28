# 📓️ Wave 4 — the BRep fixture corpus

**121 bundles, all five outcome classes, all three fixture classes, 0 contract problems.**
`bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts fixture audit`

| Class | Count | Licence | Reproducible | Producer |
| --- | --- | --- | --- | --- |
| `third-party-generated` | 119 | Apache-2.0 | **false** (measured) | `brepjs-occt` / opencascade |
| `real-world` | 1 | proprietary-internal | true | Rhino 8.31 / ST-Developer v19.2 |
| `handcrafted` | 1 | LGPL-3.0-or-later | true | nobody — 103 entities by hand |

| Family | Recipes |
| --- | --- |
| `robustness` | 36 |
| `spatial-relationship` | 33 |
| `shape-complexity` | 22 |
| `mechanical` | 16 |
| `failure` | 12 |

All FIVE outcome classes are represented, which is what makes the declared-outcome rule enforceable
rather than aspirational: `applied` 79 · `disjoint` 18 · `empty` 10 · `no-op` 9 · `rejected` 5.

The corpus is assembled from one module per family under `🏭️generator/🧪️<family>/📜️script.ts`. That split
is not cosmetic: the family is the sharding key CI uses and the axis the exhaustive Boolean matrix is
organised by, so it is the unit somebody extends, reviews or runs in isolation — and it let four agents
grow four families at once without touching one file.

## Two defects in the generator, found by using it

**`generate --only <id>` overwrote the manifest index** with just that one entry instead of merging, so
the natural way to develop a recipe — a sequence of narrowed runs — silently destroyed every other
fixture's record while leaving its files on disk. The command reported success for exactly the fixture
asked for, which is why it went unnoticed. It merges now.

**The tessellation tolerance was a fixed absolute `1e-3`**, which is precisely the mistake this protocol
exists to prevent. On a part translated to 1e6 units that is a RELATIVE tolerance of 5e-11, and the
meshing stage did not merely produce a large mesh: it ran over twelve minutes and climbed past 2.4 GB
before being killed, while the underlying exact Boolean had finished in under a second. The measuring
tool was consumed by the boundary it existed to measure. It is now
`max(1e-6, 3e-5 × bounding-box diagonal)` — the same `max(absolute, relative × reference)` rule every
other dimensional tolerance here uses — and every fixture records the tolerance it was actually built at.

## Why three classes and not one

Twenty-four of these were produced by the same OpenCASCADE kernel that will read them back. That is
the right primary source — it is a genuine third-party reference, and it is what makes the corpus
possible at all — but a corpus consisting only of that is self-referential in one specific way: a
kernel defect appears identically in the expectation and in the measurement, and nothing in the
comparison can see it. The other two exist to break that symmetry from opposite ends.

### ✍️ `handcrafted-tetrahedron` — the expectation nothing computed

103 hand-authored AP214 entities: a tetrahedron at (0,0,0), (10,0,0), (0,10,0), (0,0,10) — 4
`VERTEX_POINT`, 6 `EDGE_CURVE` over `LINE`, 4 `ADVANCED_FACE` over `PLANE` with `EDGE_LOOP` bounds,
one `CLOSED_SHELL`, one `MANIFOLD_SOLID_BREP`, every face oriented outward by hand.

Its expected answer is a **closed form**, not an oracle's opinion:

| Quantity | Closed form | Measured by the external reader | Relative error |
| --- | --- | --- | --- |
| Volume | 10³/6 = 166.666… mm³ | 166.66666666666663 | **1.71e-16** |
| Area | 150 + 50√3 = 236.6025… mm² | 236.60254037844393 | **3.60e-16** |

So it pins the fixture and the kernel at once — and it is byte-reproducible, because its `FILE_NAME`
timestamp is a fixed constant rather than a clock reading, which is exactly what the 24 generated
bundles cannot claim.

**A first attempt was rejected and deleted.** It carried a complete `PRODUCT` /
`PRODUCT_DEFINITION_FORMATION` / `PRODUCT_DEFINITION` chain and an *empty*
`ADVANCED_BREP_SHAPE_REPRESENTATION` — structurally the entities `set-file-schema` and
`set-product-identity` address. The external reader returned `ok: false`, so it was removed rather
than registered: a handcrafted fixture that nothing can validate is worth less than none, and
registering it would have added a bundle to the count while adding no evidence.

### 🏗️ `real-world-hexagonal-cut-concrete-forest-left` — the artefact nobody wrote for a test

A real Rhino 8.31 / ST-Developer v19.2 BIM export, already committed in the repository. Measured
through the independent reader: **1 solid, 1 shell, 57 faces, 126 edges, 71 vertices, valid**, volume
1.40998e10 mm³, bounding box 10.8 m × 4.68 m × 3.0 m.

Its bounding-box diagonal of 12 145 mm is why it carries `real-world-import` rather than
`mechanical-standard`: at that scale the absolute term is meaningless and only the relative one says
anything, which is the whole reason the tolerance model resolves `max(absolute, relative × reference)`.

Its one edit is disclosed in its own file header, and it is why `class` is `real-world` while
`provenance.source` is `vendored` rather than `downloaded`: the DATA section is byte-for-byte the
source export's — every entity id, coordinate, B-spline, topology relationship and `PRODUCT` record
untouched — and the single change is the `FILE_SCHEMA` line, from the source's real
`AP242_MANAGED_MODEL_BASED_3D_ENGINEERING_MIM_LF` to `AUTOMOTIVE_DESIGN`. That edit is defensible
precisely because it changes nothing the geometry depends on: the entity graph is drawn entirely from
ISO 10303-214's own common-resource geometry/topology/product schemas, with zero AP242-only
PMI/GD&T/kinematics entities anywhere in the source.

## The contact bracket, measured

Three bundles bracket exact tangency, and the measurements are what set their declared outcomes —
two of which had to be corrected against what the kernel actually did:

| Fixture | Faces | Edges | Volume Δ from 8000 | Declared outcome |
| --- | --- | --- | --- | --- |
| `cut-tangent-cylinder-epsilon-below` | 6 | 12 | −1.8e-12 | `no-op` |
| `cut-tangent-cylinder-exact` | **7** | **15** | −1.8e-12 | `applied` |
| `cut-tangent-cylinder-epsilon-above` | **9** | **21** | −8.4e-08 | `applied` |

The middle rung removes **zero volume** but IMPRINTS the tangent line. It was first declared `no-op` on
reasoning, and the measurement corrected it: a volume-only comparison cannot tell it from the rung
below, and the rung below genuinely leaves 6 faces untouched. Likewise `fuse-edge-touching-boxes` was
declared `applied` and measured as **two solids** — so its class is `disjoint`. Total volume is
identical either way; only the component count separates the two answers, which is why
`connectedComponentsEqual` is a gating assertion and face count is not.
