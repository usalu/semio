# 📓️ Robustness corpus expansion — `s.stdio.step@ap214/✳️cc6`

28 new recipes added to `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/🧪️robustness/📜️script.ts`
(8 → 36 total in the `robustness` family). One further recipe (`cut-bore-scale-1e6`) was attempted and
abandoned — see "The sharpest boundary" below; it is documented as a comment in the script rather than
declared, because a hang is not a clean `rejected`.

Every `outcome` below was set from a real `generate --only <id>` run of this fixture through brepjs's
OCCT kernel, followed by `measure` / `topology` probes against the re-imported `expected.step` — never
from intent. Two declared-from-intent guesses were corrected after measurement (see the two starred rows).

## Method note on scope

Each recipe was generated individually (`generate --only <id>`) as instructed. One early mistake:
`generate --only cut-bored-box-through` was run against an EXISTING (not-mine) recipe while probing the
generator's behaviour, which rewrote that fixture's non-reproducible STEP bytes. It was restored from the
git index (`git show :<path> > <path>`) immediately, before any further work, and confirmed clean via
`git status`. The shared `🧫️manifests.json` aggregate is overwritten by every `--only` run (by design —
it is not scoped per-fixture) and is under constant contention from the rest of the fleet; it was restored
from the index after every batch of runs in this session.

## Table

| Fixture | Brackets/stresses | Declared outcome | Measured volume | Faces/edges (solids) | Tolerance profile |
| --- | --- | --- | --- | --- | --- |
| `fuse-vertex-touching-boxes-epsilon-below` | vertex bracket, rung 1/3 | disjoint | 2000.0000000000002 | 12/24 (2) | epsilon-degenerate |
| `fuse-vertex-touching-boxes-exact` | vertex bracket, rung 2/3 | disjoint | 1999.999999999999 | 12/24 (2) — **identical topology to rung 1** | epsilon-degenerate |
| `fuse-vertex-touching-boxes-epsilon-above` | vertex bracket, rung 3/3 | applied | 1999.9999999999993 | 12/30 (1) — **mesh: Not manifold, see below** | epsilon-degenerate |
| `fuse-coplanar-partial-face-epsilon-below` | coplanar partial-face bracket, rung 1/3 | disjoint | 8000.000000000001 | 12/24 (2) | epsilon-degenerate |
| `fuse-coplanar-partial-face-exact` | coplanar partial-face bracket, rung 2/3 | applied | 7999.999999999999 | 12/26 (1) | epsilon-degenerate |
| `fuse-coplanar-partial-face-epsilon-above` | coplanar partial-face bracket, rung 3/3 | applied | 7999.9998000000005 | 14/32 (1) — **mesh: Not manifold, see below** | epsilon-degenerate |
| `fuse-coaxial-cylinders-epsilon-below` | coaxial-cylinder bracket, rung 1/3 | disjoint | 3141.5926535897934 | 6/6 (2) | epsilon-degenerate |
| `fuse-coaxial-cylinders-exact` | coaxial-cylinder bracket, rung 2/3 | applied | 3141.5926535897934 — **same volume as rung 1** | 4/5 (1) | epsilon-degenerate |
| `fuse-coaxial-cylinders-epsilon-above` | coaxial-cylinder bracket, rung 3/3 | applied | 3141.5925750499773 | 5/7 (1) | epsilon-degenerate |
| `fuse-sphere-tangent-plane-epsilon-below` | sphere/plane tangency bracket, rung 1/3 | disjoint | 4523.598775598298 | 7/15 (2) | epsilon-degenerate |
| `fuse-sphere-tangent-plane-exact` | sphere/plane tangency bracket, rung 2/3 | disjoint | 4523.598775598298 — **numerically identical to rung 1** | 7/15 (2) | epsilon-degenerate |
| `fuse-sphere-tangent-plane-epsilon-above` | sphere/plane tangency bracket, rung 3/3 | applied | 4523.598775598248 | 7/15 (1) | epsilon-degenerate |
| `fuse-edge-on-face-epsilon-below` | edge-on-face bracket, rung 1/3 | disjoint | 5279.999999999904 | 12/24 (2) | epsilon-degenerate |
| `fuse-edge-on-face-exact`★ | edge-on-face bracket, rung 2/3 | **disjoint** (corrected from intended "applied") | 5279.999999999904 — same as rung 1 | 13/27 (2) — imprinted, still 2 solids | epsilon-degenerate |
| `fuse-edge-on-face-epsilon-above` | edge-on-face bracket, rung 3/3 | applied | 5279.99999999992 | 15/33 (1) | epsilon-degenerate |
| `cut-sliver-intersection`★ | full-face 1e-7-thick sliver | **rejected** (corrected from intended "applied") | n/a — `box()` itself throws | fallback: 6/12 (1) | epsilon-degenerate |
| `cut-tiny-edge-below-tolerance`★ | 1e-7³ corner notch | **rejected** (corrected from intended "applied") | n/a — same root cause as above | fallback: 6/12 (1) | epsilon-degenerate |
| `cut-narrow-channel` | 1e-6-wide full through-slot | disjoint | 5999.9996999999985 | 12/24 (2) | epsilon-degenerate |
| `fuse-near-coplanar-faces-1e-9-radians` | 1e-9 rad tilt, growing overlap | applied | 7999.999996 | 10/20 (1) | epsilon-degenerate |
| `cut-high-aspect-ratio-bore` | 1e6:1 aspect-ratio bore | applied | 999999999.9992146 | 7/15 (1) — **mesh: Not manifold, see below** | epsilon-degenerate |
| `cut-bore-scale-1e3` | scale sweep, rung 3/4 | applied | 6429203673205.049 | 7/15 (1) | large-coordinate |
| `cut-tiny-bore-far-from-origin` | 1e6 offset + 1e-3 feature | applied | 7999.999937168143 | 7/15 (1) | large-coordinate |
| `cut-chain-ten-sequential` | 10-deep sequential chain | applied | 34345.1332235384 | 16/42 (1) | mechanical-standard |
| `cut-chain-order-b-then-c` | operand order A−B−C | applied | 13656.168803337727 | 9/21 (1) — **identical to A−C−B** | mechanical-standard |
| `cut-chain-order-c-then-b` | operand order A−C−B | applied | 13656.168803337727 | 9/21 (1) — **identical to A−B−C** | mechanical-standard |
| `cutall-many-cutters` | `cutAll` batch, 25 tools | applied | 18869.02664470766 | 31/87 (1) | mechanical-standard |
| `fuse-nested-void-in-void` | void inside a void | disjoint | 19447.999999999993 | 24/48 (2 solids, 4 shells) | mechanical-standard |
| `cut-unit-boundary-slot` | mm/m coordinate crossover | disjoint | 19999989.999999996 | 12/24 (2) | epsilon-degenerate |

★ = outcome corrected after measurement; see "Corrections" below.

## Epsilon brackets — are the three rungs distinguishable?

### Vertex-touching (two 10³ cubes, diagonal corner contact)
Below and exact are **identical** (2 solids, 12/24/16) — a single tangent point produces no imprint at
all. Above merges to 1 solid, 12/30/20. **Distinguishable by solid count only** (below/exact vs above);
below and exact are not distinguishable from each other by any BRep measurement, which is itself the
finding: this kernel treats point contact exactly like a small explicit gap.

### Coplanar partial-face (two 20×20×10 slabs, half-overlapping face)
Below: 2 solids, 12/24/16. Exact: 1 solid, 12/26/16 (imprinted seam). Above: 1 solid, 14/32/20, volume
Δ≈2e-4 from exact. **All three rungs distinguishable** — this is the sharpest of the five brackets by
solid-count-plus-topology-plus-volume, and it is also the one where the exact and above rungs fail mesh
verification (below).

### Coaxial cylinders (two r=5 cylinders, stacked cap-to-cap)
Below: 2 solids, 6/6/4, volume 3141.59265... Exact: 1 solid, 4/5/3, **same volume as below** (imprint,
no material removed — the direct cylindrical analogue of the existing `cut-tangent-cylinder-exact`
finding). Above: 1 solid, 5/7/4, volume Δ≈7.85e-5 matching the analytic overlap π·5²·1e-6 to 4 significant
figures. **All three rungs distinguishable.**

### Sphere tangent to a plane (r=5 sphere resting on a slab's top face)
Below and exact are **numerically identical** (2 solids, 7/15/10, volume 4523.598775598298 to the last
measured digit) — confirming the vertex-touching finding on a curved surface: single-point tangency
produces no imprint, unlike a full circular cap contact (coaxial cylinders, above), which does imprint.
Above merges to 1 solid, same face/edge/vertex counts as the other two rungs, distinguished only by solid
count and a Δ≈5e-11 volume change. **Distinguishable by solid count between (below=exact) and above; below
and exact are not distinguishable from each other**, the second instance of this pattern in the bracket set.

### Edge-on-face (diamond-section blade, bottom edge resting on a slab's top face)
Below: 2 solids, 12/24/16. Exact: **still 2 solids**, but 13/27/18 — the contact line IS imprinted onto
the slab's top face, yet the bodies do not merge. Above: 1 solid, 15/33/20. **All three rungs
distinguishable**, and the exact rung is the reason for one of the two outcome corrections below.

## Corrections (outcome set from measurement, not intent)

1. **`fuse-edge-on-face-exact`** was reasoned as "applied" (line contact ought to weld two bodies).
   Measurement showed 2 solids survive (13 faces / 27 edges / 18 vertices, up from 12/24/16 — the contact
   line is imprinted but the bodies stay separate). Per the same reasoning the existing corpus already
   applied to `fuse-edge-touching-boxes`, the correct class is **disjoint**. Recipe and notes corrected;
   fixture regenerated to confirm `declaredOutcome: "disjoint"` in `expected.metrics.json`.
2. **`cut-sliver-intersection`** and **`cut-tiny-edge-below-tolerance`** were both drafted as "applied" —
   a sub-tolerance sliver/notch was expected to either vanish (no-op) or apply as a negligible cut. Neither
   happened: `box()` itself throws a bare, message-less `WebAssembly.Exception` the instant any dimension
   is exactly 1e-7 (bisected boundary: 1e-7 throws, 2e-7 succeeds — see below). Both recipes were rewritten
   to the established `failure`-family pattern (catch the exception, verify it, fall back to a disjoint
   cut for provenance) and declared **rejected**.

## Abandoned recipe

**`cut-bore-scale-1e6`** (box 2e7³ / 20 km, bore r=5e6, the fourth rung of the scale sweep) was attempted
and dropped, not declared `rejected`, because what happened was not a clean refusal:

- The exact Boolean `cut` completed and `expected.step` was written to disk within the first second of
  the run (19 274 bytes — an unremarkable size for this shape).
- The generator's own next stage — `getBounds`/`measureVolume`/`measureArea`/`mesh()` — then hung.
  `expected.metrics.json` and `expected.mesh.json` never appeared. The process was killed after **12
  minutes 16 seconds**, still in the running state, memory having climbed from ~850 MB to ~2.4 GB and
  still rising.
- No exception, no message, nothing to declare `rejected` against — just non-termination.

The recipe is preserved as a comment in `📜️script.ts` (not a declared `Recipe`, so it can never hang a
future unscoped `generate` run) with this finding, and the partial fixture directory it left behind was
deleted.

## The sharpest boundary — three independent findings converging on one cause

**1. `box()` has a hard floor at exactly 1e-7, with no degraded regime below it.** Bisected directly
against the kernel (outside the generator, to get a readable stack):

| Dimension | `box(d, 20, 20)` |
| --- | --- |
| 1e-7 | throws (bare `WebAssembly.Exception`, no message, no name, `Object.keys(e).length === 0`) |
| 2e-7 | succeeds |
| 3e-7 – 9.9e-7 | succeeds |
| 1e-6 | succeeds |

The boundary sits exactly at the kernel's own documented 1e-7 working tolerance — and it is a wall, not a
slope: there is no "succeeds but degraded" band between throwing and working cleanly. `cut-sliver-intersection`
and `cut-tiny-edge-below-tolerance` hit this same guard from two structurally different constructions (a
full-face sliver, a corner notch), independently confirming it is one root cause, not two.

**2. The exact kernel and its own default-tolerance mesh disagree at exactly the same order of magnitude.**
Three fixtures whose exact BRep is valid (`isValidSolid: true`, single clean solid, sensible volume) all
fail `step-mesh-compare` against themselves — the required self-check — with the independent manifold-3d
engine reporting **`Not manifold`**, at every tessellation tolerance tried from 1e-3 down to 1e-8:

| Fixture | Degenerate feature | Exact-kernel result |
| --- | --- | --- |
| `fuse-vertex-touching-boxes-epsilon-above` | 1e-6³ corner overlap | valid single solid |
| `fuse-coplanar-partial-face-epsilon-above` | 1e-6-thick planar embed | valid single solid |
| `cut-high-aspect-ratio-bore` | 1e-3-diameter bore (1e6:1 aspect) | valid single solid |

This is the exact phenomenon this ticket exists to make visible: two valid kernels — the exact OCCT
BRep and an independent mesh engine tessellating that same BRep — legitimately disagree at a scale a
single absolute tolerance cannot see. All other epsilon-above rungs (coaxial cylinders, sphere-tangent,
edge-on-face — all curved-surface or line-contact cases) mesh cleanly; only razor-thin **planar** overlaps
and a bore **narrower than the tessellation chord tolerance** trigger it.

**3. The generator's own fixed absolute tessellation tolerance (1e-3) is itself not scale-relative,
and that is what actually broke at 1e6× scale — not the Boolean.** `cut-bore-scale-1e3` (1e3×) meshed
and measured in under a second. `cut-bore-scale-1e6` (1e6×) computed its exact cut in under a second too,
then hung indefinitely in measurement/meshing. At 2e7 absolute units, a 1e-3 absolute chord tolerance is a
relative tolerance of 5e-11 — a triangulation request the pipeline cannot service in any bounded time.

Findings 1–3 are the same underlying property of this kernel version, observed three separate ways: an
**absolute** tolerance (1e-7 for construction, 1e-3 for tessellation) is used where the corpus's own thesis
says a **scale-relative** one is required, and every place this corpus pushed hard enough to reach that
tolerance, the kernel either threw with no diagnostic, silently produced a mesh a second engine calls
invalid, or simply never returned. Finding 3 (the generator pipeline itself, not a fixture under test,
failing to terminate) is the sharpest: it is not a disagreement about a boundary case, it is the boundary
consuming the tool that measures boundaries.
