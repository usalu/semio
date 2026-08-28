# 📓️ v3 — Independent verification of the cc6 BRep fixture corpus

Independent-verifier pass over the 24-bundle third-party-generated BRep fixture corpus at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🧫️fixtures/`. Every number below
comes from RUNNING `.../✳️cc6/🔬️probes/📜️script.ts` (brepjs / OCCT 8.0 WASM) fresh in this session —
`measure`, `topology`, `brep-validity`, `reimport-compare` — never from reading `expected.metrics.json`
as ground truth. Digest recomputation used plain `hashlib.sha256`. All commands and scratch STEP files
are under this ticket's scratchpad; nothing in the fixture corpus, generator, probes, or oracle registry
was modified.

## 1. Declared outcome vs measured reality (24/24)

| Fixture | Family | Declared outcome | Measured solids | Measured volume (mm³) | Verdict |
|---|---|---|---:|---:|---|
| cut-bored-box-through | spatial-relationship | applied | 1 | 6429.20367320505 | CONFIRMED — differs from operand-a (8000, 6 faces) |
| cut-disjoint-operands | spatial-relationship | no-op | 1 | 999.9999999999998 | CONFIRMED — identical to operand-a (topology + `reimport-compare` both 0 error) |
| cut-contained-operand | spatial-relationship | applied | 1 | 7783.999999999998 | CONFIRMED — 6→12 faces, cavity present |
| cut-full-subtraction | spatial-relationship | empty | 0 (no `expected.step`) | — | CONFIRMED — operand-a bbox [5,10]³ ⊂ operand-b bbox [0,20]³, tool geometrically swallows base |
| fuse-face-touching-boxes | spatial-relationship | applied | 1 | 1999.9999999999993 | CONFIRMED — single fused solid, ≠ either 1000-vol operand |
| fuse-edge-touching-boxes | robustness | disjoint | 2 | 1999.999999999999 | CONFIRMED outcome, but **metrics.json is wrong** — see Finding 1 |
| intersect-overlapping-boxes | spatial-relationship | applied | 1 | 124.99999999999997 | CONFIRMED |
| intersect-disjoint-operands | failure | empty | 0 (no `expected.step`) | — | CONFIRMED — operand-a x∈[0,10], operand-b x∈[50,60], bboxes disjoint |
| cut-tangent-cylinder-epsilon-below | robustness | no-op | 1 | 7999.999999999998 | CONFIRMED — bit-for-bit same volume as operand-a, 6/12 faces/edges unchanged |
| cut-tangent-cylinder-exact | robustness | applied | 1 | 7999.999999999998 | CONFIRMED — volume unchanged, but 6→7 faces / 12→15 edges (imprint) |
| cut-tangent-cylinder-epsilon-above | robustness | applied | 1 | 7999.99999991569 | CONFIRMED — sliver of 8.430834e-08 removed, 6→9 faces / 12→21 edges |
| cut-coplanar-face-cutter | robustness | applied | 1 | 3999.999999999999 | CONFIRMED — exactly half of 8000 |
| cut-identical-operands | failure | empty | 0 (no `expected.step`) | — | CONFIRMED — operand-a/b bboxes bit-identical [0,12]³, `reimport-compare(a,b)` = 0 error on every term |
| cut-skewed-bore | shape-complexity | applied | 1 | 7998.615352390601 | CONFIRMED |
| cut-sphere-from-box | shape-complexity | applied | 1 | 7281.622479879134 | CONFIRMED |
| fuse-cylinder-cross | shape-complexity | applied | 1 | 5616.518640503333 | CONFIRMED |
| cut-thin-wall-shell | robustness | applied | 1 | 470.46399999999585 | CONFIRMED |
| cut-micro-scale-bore | robustness | applied | 1 | 6.429203673205049e-06 | CONFIRMED — same shape as cut-bored-box-through scaled ×1e-3 |
| cut-large-coordinate-bore | robustness | applied | 1 | 6429.2036732026445 | CONFIRMED — matches cut-bored-box-through to 3.8e-13 relative (see §"could not break") |
| cut-disconnected-result | spatial-relationship | applied | 2 | 3400.000000000001 | CONFIRMED, but outcome taxonomy is ambiguous — see Finding 4 |
| mechanical-fixture-plate | mechanical | applied | 1 | 17035.681469282023 | CONFIRMED |
| mechanical-pipe-manifold | mechanical | applied | 1 | 40771.91920749615 | CONFIRMED |
| mechanical-ribbed-enclosure | mechanical | applied | 1 | 10584.000000000004 | CONFIRMED |
| mechanical-filleted-bracket | mechanical | applied | 1 | 24948.07539609445 | CONFIRMED geometrically, but **fixture files under-declare the recipe** — see Finding 2 |

**24/24 declared outcomes are geometrically consistent with independently measured reality.** No
`no-op`/`empty`/`disjoint`/`applied` mislabel was found. `brep-validity` returned `valid: true` for
every one of the 66 STEP files probed (all operands + all `expected.step`), with zero exceptions.

SHA-256 recomputed for all 112 manifest file entries across the 24 bundles: **0 hash mismatches, 0 size
mismatches, 0 unresolved paths.**

## 2. Closed-form cross-checks

| Fixture | Analytic value | Measured value | Relative error |
|---|---:|---:|---:|
| cut-bored-box-through = 20³ − π·5²·20 | 6429.203673205104 | 6429.203673205047 | 8.77e-15 |
| intersect-overlapping-boxes = 5³ | 125 | 124.99999999999997 | 2.27e-16 |
| fuse-face-touching-boxes = 2·10³ | 2000 | 1999.9999999999993 | 3.41e-16 |
| cut-thin-wall-shell = 20³ − 19.6³ | 470.4639999999981 | 470.46399999999585 | 4.83e-15 |
| cut-coplanar-face-cutter = 20³/2 | 4000 | 3999.999999999999 | 2.27e-16 |
| cut-contained-operand = 20³ − 6³ | 7784 | 7783.999999999998 | 2.34e-16 |

All six are correct at double-precision machine-epsilon level, three to five orders of magnitude better
than the 1e-12 bar in the brief. **No closed-form violation found.**

## 3. The contact bracket (epsilon-below / exact / epsilon-above)

| Rung | Measured volume | Δ vs operand-a (8000 mm³) | Faces | Edges |
|---|---:|---:|---:|---:|
| epsilon-below (misses by 1e-6 mm) | 7999.999999999998 | 0 (bit-identical) | 6 | 12 |
| exact tangency | 7999.999999999998 | 0 (bit-identical) | **7** | **15** |
| epsilon-above (bites 1e-6 mm) | 7999.99999991569 | **8.430834e-08** | **9** | **21** |

This matches the on-record claim exactly: epsilon-below is a true volume-and-topology no-op, exact
tangency imprints the contact line onto the surface (topology changes, volume does not), and
epsilon-above removes a measurable sliver and imprints further. The three rungs are genuinely,
independently distinguishable by the kernel — **the bracket is not degenerate.** (Note: the generator's
own source comment at `🏭️generator/📜️script.ts` says "a sliver of 8.44e-8"; the freshly measured value is
8.430834e-08 — a ~0.1% stale-comment drift, not a fixture defect; the number that matters, the one in
`expected.step`, is correct.)

## 4. Findings, ranked by severity

**Finding 1 (HIGH) — `fuse-edge-touching-boxes/expected.metrics.json` is internally wrong.**
Committed file declares `"edges": 23, "vertices": 14`. Re-running `topology` against the exact same
`expected.step` (SHA-256 verified to match the manifest, so this is not a stale-file problem) measures
`edges: 24, vertices: 16` — confirmed in two independent invocations. Volume, area, faces and solids all
agree with the committed file; only edges/vertices are wrong, by a physically implausible count (an
edge-touching pair of boxes cannot close at 23 edges — 12+12 faces meeting with zero edge merging is
24, and any partial edge-sharing would drop by more than 1). This is exactly the class of error the
brief asked verifiers to assume exists. It happens to not affect the pipeline's own gates today (face
and edge counts are explicitly *not* asserted by `semantic-brep-solid-v1`'s topology stage — see
Finding 5), but it is a real, demonstrable inconsistency in a committed oracle artifact, found by
literally the same re-measurement a consumer would run. A systematic sweep of all 24 committed
`expected.metrics.json` files against fresh `measure`/`topology` output found **this is the only
mismatch** (checked solids, faces, edges, volume, area on all 21 non-empty fixtures).

**Finding 2 (MEDIUM-HIGH) — `mechanical-filleted-bracket` under-declares its own recipe.**
`🏭️generator/📜️script.ts` builds this fixture as `fuse(upright, foot)` then **two more `cut`s** against
cylinders that are never exported as fixture files:
```
bracket = cut(bracket, translate(rotate(cylinder(4,60), 25, axis=[0,1,0]), [20,20,-10]))
bracket = cut(bracket, translate(cylinder(3,30), [4,30,20]))
```
Yet the manifest declares only `operand-a-step` (upright) and `operand-b-step` (foot) — the same
two-operand shape as a plain `cut`/`fuse` pair fixture (e.g. `cut-bored-box-through`). Every other
multi-step "mechanical" fixture (`mechanical-fixture-plate`, `-pipe-manifold`, `-ribbed-enclosure`)
avoids this trap by exporting **only** the pre-mutation base as `operand-a-step` and no `operand-b`,
correctly signalling "this is a chain, not decomposable into two files." `mechanical-filleted-bracket`
instead looks exactly like a two-operand boolean fixture while silently folding in two more un-exposed
shapes. Anyone attempting to reproduce or independently re-derive `expected.step` from the declared
operand files alone cannot do so — a real corpus-honesty defect, not merely a naming quibble.

**Finding 3 (MEDIUM) — `toleranceProfiles` registry is empty; the six declared per-fixture profile names are decorative.**
Every one of the 24 manifests declares a `toleranceProfile` (`analytic-strict`, `contact-sensitive`,
`epsilon-degenerate`, `mechanical-standard`, `micro-scale`, `large-coordinate`) implying that
scale/contact-aware thresholds exist. `oracle/🔣️.json`'s `toleranceProfiles` array is `[]` — completely
empty. The one and only pipeline that actually gates anything, `semantic-brep-solid-v1`, hardcodes a
single fixed set of relative thresholds (`relativeVolumeErrorMax` 1e-8, `relativeAreaErrorMax` 1e-7,
`normalizedCentroidDistanceMax` 1e-8, `normalizedBoundingBoxDiagonalErrorMax` 1e-8,
`connectedComponentsEqual`) under its own `toleranceProfile: "mechanical-standard"`, applied uniformly
regardless of what a given fixture declares. The per-fixture `toleranceProfile` field currently has zero
observable effect on any comparison — it reads as documentation of intent that the machine-readable
config does not implement.

**Finding 4 (LOW-MEDIUM) — the `disjoint` vs `applied` split is not "more than one solid," despite the intuitive reading.**
`cut-disconnected-result` is declared `applied` and legitimately measures 2 solids (its own committed
`expected.metrics.json` says so plainly: `"declaredOutcome":"applied","solids":2`). `fuse-edge-touching-boxes`
is declared `disjoint`, also 2 solids. The distinction is *intent* (a cut meant to split vs. a fuse that
failed to truly merge), not solid count — but nothing in the schema or manifest encodes that distinction
machine-readably. A consumer that gates "is this fixture's multi-solid result acceptable?" purely on
`outcome === "disjoint"` will reject `cut-disconnected-result`'s equally legitimate 2-solid answer. Not
a bug in the corpus's own data, but a taxonomy that invites exactly this misreading.

**Finding 5 (MEDIUM) — the outcome type is 5-valued (`applied | no-op | empty | disjoint | rejected`) but only 4 values are used; `rejected` has zero coverage.**
`🏭️generator/📜️script.ts` types `outcome` as including `"rejected"`, but none of the 24 recipes use it.
There is no fixture anywhere in this corpus that exercises "the operation is expected to fail/throw,"
only ones that succeed with an empty *result*. `failure`-family coverage (2 fixtures) tests "correctly
nothing" but never "correctly an error." That is a real, named gap in the schema's own outcome space,
not a family-labelling nitpick.

**Finding 6 (LOW) — generator source comment drift.**
`cut-tangent-cylinder-epsilon-above`'s note says "a sliver of 8.44e-8 is removed"; freshly measured value
is 8.430834e-08 (≈0.1% off). Harmless — the committed `expected.step`/`.metrics.json` are correct, only
the human-readable comment is stale — but it is exactly the kind of drift that erodes trust in adjacent
prose claims (e.g. the bracket write-up), so it's listed for completeness.

## 5. Falsifiability probe (item 6)

**Cross-bundle sanity (errors DO move):** `reimport-compare(cut-bored-box-through/expected.step,
cut-sphere-from-box/expected.step)` → `relativeVolumeError=1.326e-01`, `relativeAreaError=1.105e-01`.
Comparing two genuinely different shapes produces large errors, confirming the metric isn't a rubber
stamp. `reimport-compare(cut-bored-box-through/expected.step, cut-large-coordinate-bore/expected.step)`
— same shape, translated 1,000,000 mm — gives `relativeVolumeError=3.7e-13`,
`normalizedCentroidDistance=2.887e+04` (not normalized against a shared reference; expected, since the
two bodies really are ~1,000,000 mm apart) confirming the pipeline reference-scales per-comparison
rather than globally.

**Smallest perturbation that slips through — the actual finding.** Using
`cut-bored-box-through/operand-a.step` (a plain 20³ box, bbox diagonal 34.64 mm) as the base, two
independent scratch perturbations were built (never touching the corpus) and measured with
`reimport-compare` against the untouched original:

| Perturbation | Magnitude | relVolErr | relAreaErr | normCentroidDist | Would pass 1e-8/1e-7/1e-8 gates? |
|---|---:|---:|---:|---:|---|
| Rigid translation (x only) | 1e-6 mm | 1.1e-16 | 0 | 5.774e-08 | **NO** (centroid over) |
| Rigid translation (x only) | 1e-7 mm | 1.1e-16 | 0 | 5.774e-09 | **YES** — every gate passes |
| Rigid translation (x only) | 1e-9 mm | 0 | 0 | 2.887e-11 | YES |
| Single-vertex "crack" (one of two coincident corner points moved, the other left behind) | 1e-2 mm (10 µm) | **0** | **0** | 1.443e-04 | NO (centroid/bbox over) |
| Same crack | 1e-8 mm | 0 | 0 | 1.443e-10 | **YES** — every gate passes, `isValidSolid` still reports `true` |

Two things fall out of this, independent of each other:

1. **A pure rigid mislocation is invisible to volume, area, and component-count at *any* magnitude** —
   translation preserves volume and surface area exactly, so those three gates read zero error whether
   the body is off by a millimetre or a nanometre. The *only* gate that can ever catch a mislocation is
   `normalizedCentroidDistanceMax`/`normalizedBoundingBoxDiagonalErrorMax`, and both are set at 1e-8.
   For a 20 mm part (34.6 mm bbox diagonal) that threshold is crossed at roughly **2×10⁻⁷ mm — about 0.2
   micrometres.** A correctly-shaped, correctly-sized solid that is silently mislocated by less than a
   quarter of a micrometre (a very ordinary class of bug: a stale transform, a reference-frame origin
   off by float noise, a double-applied unit conversion at the femto-scale) satisfies **every currently
   declared assertion** of `semantic-brep-solid-v1`.
2. **A genuine topological crack — a boundary vertex duplicated into two non-coincident points, so the
   solid does not actually close — is not caught by the dedicated validity stage.** `brep-validity`'s
   `isValidSolid` returned `true` for the cracked box at every magnitude tested, up to and including
   10 µm (1e-2 mm). The pipeline's own description of that stage — "Both shapes are valid solids... a
   mesh cannot see a lost cavity" — oversells what OCCT's `isValidSolid` actually flags for this defect
   class; it does not detect a locally unstitched vertex at all. Such a crack is then caught, if at all,
   only through the same centroid/bbox drift as a translation, with the same ~0.2 µm floor.

**What class of wrong result slips through, stated plainly:** any candidate solid that has the right
volume, the right surface area, the right solid count, and is mislocated or locally un-stitched by less
than roughly 2×10⁻⁷ of its own bounding-box diagonal will pass `semantic-brep-solid-v1` end to end —
including the dedicated validity stage — even though it is a different, and in the crack case an
invalid, shape. The pipeline's declared `optional: true` stages (`step-external-canonicalizer`,
`cgal-mesh-comparison` — Hausdorff distance, self-intersections, boundary-edge count) are precisely the
checks that would catch this class, and both are unqualified/unrun, per the pipeline's own comparisonPipeline
listing and `📓️w4-brepjs-qualification.md`.

(Aside not in the brief's threshold list but present in the actual pipeline: the real
`semantic-brep-solid-v1` definition asserts **five** terms, not four —
`normalizedBoundingBoxDiagonalErrorMax: 1e-8` is also gated, alongside `relativeVolumeErrorMax`,
`relativeAreaErrorMax`, `normalizedCentroidDistanceMax`, and `connectedComponentsEqual`.)

## 6. Family coverage

| Family | Count | Members |
|---|---:|---|
| spatial-relationship | 7 | cut-bored-box-through, cut-disjoint-operands, cut-contained-operand, cut-full-subtraction, fuse-face-touching-boxes, intersect-overlapping-boxes, cut-disconnected-result |
| robustness | 8 | fuse-edge-touching-boxes, cut-tangent-cylinder-{epsilon-below,exact,epsilon-above}, cut-coplanar-face-cutter, cut-thin-wall-shell, cut-micro-scale-bore, cut-large-coordinate-bore |
| shape-complexity | 3 | cut-skewed-bore, cut-sphere-from-box, fuse-cylinder-cross |
| mechanical | 4 | mechanical-fixture-plate, -pipe-manifold, -ribbed-enclosure, -filleted-bracket |
| failure | 2 | intersect-disjoint-operands, cut-identical-operands |

Every family assignment is geometrically defensible given what each fixture actually builds (verified
above). `shape-complexity` (3) and `failure` (2) are the thinnest slices; `failure` in particular tests
only "correctly empty," never "correctly rejected" (Finding 5), so the family name over-promises
slightly relative to what it covers. Not a misclassification, but the thinnest and most narrowly-scoped
of the five claimed families.

## 7. What was attacked and could not be broken

- **All 24 declared outcomes** (applied/no-op/empty/disjoint) — verified against fresh `measure`/
  `topology`/`brep-validity` output, not one mislabel found.
- **All six closed-form volumes** — machine-epsilon accurate (1e-14 to 1e-16 relative), far inside the
  1e-12 bar.
- **The three-rung contact bracket** — genuinely, measurably distinct (0 / 0-with-imprint / 8.43e-8),
  exactly as claimed.
- **All 112 file digests and sizes** across the manifest — zero mismatches, zero missing files.
- **Translation invariance at large coordinates** — `cut-large-coordinate-bore` (base translated
  1,000,000 mm from the origin) reproduces `cut-bored-box-through`'s volume to 3.8e-13 relative, exactly
  matching the fixture's own claim that "only the relative term is meaningful" at that scale.
- **Cross-bundle discrimination** — comparing unrelated fixtures via `reimport-compare` produces large,
  clearly non-zero errors (13%, 11%), so the comparison metric is not degenerate/always-passing.
- **`isValidSolid` on all 66 probed files** — no false negative found; every legitimately valid STEP file
  in the corpus is correctly reported valid.

The corpus's actual geometric content held up under every direct attack. The defects found are about the
**scaffolding around** the geometry — one wrong number in a committed metrics file, one fixture whose
declared operands don't cover its own build recipe, an empty tolerance-profile registry, an unused
outcome value, and — the most consequential one — a real, quantified gap in what the comparison pipeline
can detect at sub-micrometre scale, including inside its own "validity" stage.
