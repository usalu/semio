# 📓️ `spatial-relationship` and `failure` corpus additions

**Scope:** two files only —
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/🧪️spatial-relationship/📜️script.ts`
(7 → **33** recipes) and
`.../🏭️generator/🧪️failure/📜️script.ts` (2 → **12** recipes).

Every recipe below was generated with `bun 🏭️generator/📜️script.ts generate --only <id>` (never a bare
`generate`) and measured off the re-imported `expected.step` the generator itself writes — the numbers
in every table are `expected.metrics.json`, not the in-memory kernel result, except where a row
explicitly calls out an in-memory-vs-reimport gap. `step-mesh-compare --input expected.step --input
expected.step` was run for a representative sample (`fuse-vertex-touching-boxes`,
`fuse-coincident-faces`, `intersect-splits-into-several-bodies`, `cut-self-intersecting-tool`,
`fuse-nonmanifold-vertex-compound`, `reject-ellipse-structured-error`, `reject-open-shell-fuse`) and
reported `normalizedSymmetricDifferenceVolume: 0` in every case.

Scratch harnesses used to interrogate the kernel before writing any recipe (kept, per ticket rules):
`🔬️spatial-failure-explore.ts`, `🔬️spatial-failure-explore2.ts`, `🔬️spatial-failure-explore3.ts` in
this ticket folder.

## Task A — `spatial-relationship`: the arrangement × operation matrix

"N/A" marks a cell that is not conceptually meaningful for that operation (documented per row, not
silently skipped). Fixture ids in *italics* live in a different family (read-only for this ticket,
included here because they already fill that cell of the matrix); plain ids are new, in this file.

| Arrangement | cut | fuse | intersect |
| --- | --- | --- | --- |
| disjoint | *cut-disjoint-operands* — no-op, vol 1000, 6/12/8 | **fuse-disjoint-boxes** — disjoint, 2 solids, vol 2000, 12/24/16 | **intersect-disjoint-boxes** — empty, 0 solids |
| contained | *cut-contained-operand* — applied (cavity), vol 7784, 12/24/16 | **fuse-contained-operand** — no-op, vol 8000, 6/12/8 (byte-identical to outer) | **intersect-contained-operand** — applied, vol 216, 6/12/8 (= inner exactly) |
| partial overlap | *cut-bored-box-through* — applied, vol 6429.2, 7/15/10 | **fuse-partial-overlap-boxes** — applied, vol 1875, 12/30/20 | *intersect-overlapping-boxes* — applied, vol 125, 6/12/8 |
| face touching (flush, zero overlap) | **cut-face-touching-boxes** — no-op, vol 1000, 6/12/8 | *fuse-face-touching-boxes* — applied, vol 2000, **10/20/12** (NOT simplified to 6/12/8) | **intersect-face-touching-boxes** — empty, 0 solids |
| edge touching | **cut-edge-touching-boxes** — no-op, vol 1000, 6/12/8 | *fuse-edge-touching-boxes* (robustness) — disjoint, 2 solids, vol 2000, 12/24/16 | **intersect-edge-touching-boxes** — empty, 0 solids |
| vertex touching | **cut-vertex-touching-boxes** — no-op, vol 1000, 6/12/8 | **fuse-vertex-touching-boxes** — disjoint, 2 solids, vol 2000, 12/24/16 (re-imported; in-memory the shared corner WELDS to 15 vertices, see below) | **intersect-vertex-touching-boxes** — empty, 0 solids |
| tangential contact | *cut-tangent-cylinder-exact* (robustness, LINE tangency) — applied, vol 8000, **7/15/10** (imprint survives reimport); **cut-tangential-sphere-contact** (POINT tangency) — no-op, vol 8000, 6/12/8 (imprint vanishes on reimport) | **fuse-tangential-sphere-contact** — disjoint, 2 solids, vol 8523.6, 7/15/10 | **intersect-tangential-sphere-contact** — empty, 0 solids |
| coincident faces (same footprint, stacked, partial z-overlap — distinct from flush face-touching, which has zero overlap) | **cut-coincident-faces** — applied, vol 500, 6/12/8 | **fuse-coincident-faces** — applied, vol 1500, **14/28/16** (NOT simplified) | **intersect-coincident-faces** — applied, vol 500, 6/12/8 |
| coplanar cutters | *cut-coplanar-face-cutter* (robustness) — applied, vol 4000, 6/12/8 | **fuse-coplanar-cutter-boxes** — applied, vol 13000, 12/28/20 | **intersect-coplanar-cutter-boxes** — applied, vol 4000, 6/12/8 |
| identical operands | *cut-identical-operands* (failure) — empty, 0 solids | **fuse-identical-operands** — applied, vol 1728, 6/12/8 | **intersect-identical-operands** — applied, vol 1728, 6/12/8 (component count = 1 is the assertion) |
| nearly-identical operands (Δ=0.001mm, corner-aligned) | **cut-nearly-identical-operands** — applied, vol 0.144 (sliver), 6/12/8 | **fuse-nearly-identical-operands** — applied, vol 1728, **10/20/12** (NOT simplified) | **intersect-nearly-identical-operands** — applied, vol 1727.856, 6/12/8 |
| empty intersection | — (same cell as disjoint × intersect above) | — | see disjoint × intersect; also `failure`'s `intersect-disjoint-operands` / `intersect-empty-valid-result` |
| full subtraction | *cut-full-subtraction* — empty, 0 solids | N/A — fuse cannot subtract | N/A — not a fuse/intersect concept |
| a result with a cavity | *cut-contained-operand* (same cell as contained × cut) | N/A — fuse of a contained shape leaves no cavity, see no-op above | N/A |
| splits into several bodies | *cut-disconnected-result* — applied, 2 solids, vol 3400, 12/24/16 | N/A — a two-operand fuse cannot itself splinter a connected result | **intersect-splits-into-several-bodies** — applied, 3 solids, vol 4500, 18/36/24 (comb tool) |

**New in this file: 26 recipes** (`fuse-disjoint-boxes`, `intersect-disjoint-boxes`,
`fuse-contained-operand`, `intersect-contained-operand`, `fuse-partial-overlap-boxes`,
`cut-face-touching-boxes`, `intersect-face-touching-boxes`, `cut-edge-touching-boxes`,
`intersect-edge-touching-boxes`, `cut-vertex-touching-boxes`, `fuse-vertex-touching-boxes`,
`intersect-vertex-touching-boxes`, `cut-tangential-sphere-contact`, `fuse-tangential-sphere-contact`,
`intersect-tangential-sphere-contact`, `cut-coincident-faces`, `fuse-coincident-faces`,
`intersect-coincident-faces`, `fuse-coplanar-cutter-boxes`, `intersect-coplanar-cutter-boxes`,
`fuse-identical-operands`, `intersect-identical-operands`, `cut-nearly-identical-operands`,
`fuse-nearly-identical-operands`, `intersect-nearly-identical-operands`,
`intersect-splits-into-several-bodies`).

## Task B — `failure`: the declared-outcome vocabulary

| Vocabulary item | Fixture | Declared outcome | What was measured |
| --- | --- | --- | --- |
| valid no-op | `cut-valid-no-op` | no-op | byte-identical to base, vol 3375, 6/12/8 |
| disjoint / no-intersection result | `fuse-disjoint-result` | disjoint | 2 solids, vol 2000, 12/24/16 |
| empty result where valid | `intersect-empty-valid-result` | empty | 0 solids |
| rejected invalid input (attempt 1) | `reject-negative-box-width` | no-op *(finding is in the base, not the boolean)* | `box(-5,10,10)` silently mirrored into x∈[-5,0], vol 500, 6/12/8, `isValidSolid` true — **not rejected** |
| rejected invalid input (attempt 2) | `reject-zero-height-cylinder` | applied | `cylinder(5,0)` accepted as a degenerate point-shape (`isValidSolid` false); cutting it still succeeds and the CUT RESULT is *also* `isValidSolid` false, silently — **not rejected at either step** |
| rejected open shell where a solid is required | `reject-open-shell-fuse` | rejected | the ONE case that genuinely throws — see below |
| (same open shell, different op) | `cut-open-shell-accepted-as-empty` | empty | `cut`/`intersect` on the identical open shell do NOT throw; 0 solids, but the in-memory shape's own `isValidSolid` reports true |
| rejected non-manifold input | `fuse-nonmanifold-vertex-compound` | disjoint | a vertex-touching `compound()` is accepted outright; fusing a third box in gives 3 disjoint solids, vol 2125 — **not rejected** |
| rejected self-intersecting input | `cut-self-intersecting-tool` | applied | a bowtie solid with `isValidSolid=false` is accepted by `cut` with no check; result imprints to 7/15/10, vol unchanged at 8000, and the RESULT reports `isValidSolid=true` — **not rejected** |
| a deterministic structured error | `reject-ellipse-structured-error` | rejected | `ellipse(5,10)` — the one input that rejects with a typed, reproducible payload: `{"kind":"VALIDATION","code":"ELLIPSE_RADII","message":"The minor radius must be smaller than the major one"}` |

**New in this file: 10 recipes.** Of the 8 vocabulary items, only two genuinely trigger a kernel-side
rejection (`reject-open-shell-fuse`'s `fuse`, and `reject-ellipse-structured-error`'s `ellipse`); the
other six "rejection" attempts are exactly the finding the ticket asked for: the kernel accepted input
it should have refused, so the declared `outcome` records what it actually did, not what was intended.

## Every case where the kernel's answer surprised

1. **`fuse` never simplifies coincident/coplanar faces — `cut` and `intersect` usually do.** Three
   independent constructions confirm the same pattern: `fuse-face-touching-boxes` (pre-existing fixture,
   flush full-face contact) measures 10 faces / 20 edges instead of a clean box's 6/12; the new
   `fuse-coincident-faces` (stacked, partially-overlapping same-footprint boxes) measures 14/28 for a
   shape that is analytically a plain 10×10×15 box; `fuse-nearly-identical-operands` measures 10/20 for
   a union that analytically equals its larger operand exactly. In every one of these, `intersect` on the
   same or an equivalent pair lands on the clean, minimal topology instead. `brepjs` exports a separate
   `simplify()` (documented as "merging same-domain faces/edges") that these recipes deliberately do not
   call — `fuse`'s raw output keeps every boolean seam as its own face regardless of whether the result is
   geometrically trivial. This is the single most consequential finding for a from-scratch BRep kernel:
   matching volume and even matching mesh is not enough to match `fuse`'s topology.
2. **The vertex-touching weld does not survive STEP.** In-memory, `fuse` on two corner-touching boxes
   welds the shared corner into ONE vertex (15 total) while still returning two solids. Re-imported from
   the committed `expected.step`, the same shape measures 16 vertices — the two solids each keep an
   independent, coincident vertex. This is the same in-memory-vs-reimport gap the generator's own
   `reimport` docstring already documented for `fuse-edge-touching-boxes` (23→24 edges), now confirmed one
   contact-dimension lower, at a single point.
3. **Point tangency imprints in memory, then vanishes on reimport — line tangency does not.**
   `cut-tangential-sphere-contact` (a sphere tangent to a box face at a point) gains a 9th vertex in the
   raw kernel result with no face/edge change, but the committed, re-imported STEP measures a clean 8 —
   indistinguishable from a true no-op. `robustness`'s `cut-tangent-cylinder-exact` (tangent along a LINE)
   imprints a face split (6→7 faces) that DOES survive reimport. The declared outcome for the sphere case
   had to be corrected from an initial "applied" guess to "no-op" once the actual committed file was
   measured — a direct, first-hand instance of the exact mistake the ticket's own examples warn about.
4. **None of `box`, `cylinder`, `compound`, or a self-intersecting solid are validated as boolean
   operands.** A negative box width is silently mirrored (not rejected); a zero-height cylinder is
   accepted as a degenerate, already-invalid shape and its invalidity propagates silently into an
   `isValidSolid=false` boolean RESULT with no exception anywhere; a hand-built self-intersecting
   ("bowtie") solid with `isValidSolid=false` is accepted by `cut` and `fuse` without complaint, each
   producing an `isValidSolid=true` result; a `compound()` of two solids touching at a single
   vertex — non-manifold by construction — is accepted by all three boolean operations. The kernel
   performs essentially no defensive validation at the boolean-operation boundary; every guard lives (if
   anywhere) at construction time, and even there it is inconsistent (see #5).
5. **The one genuine rejection is operation-specific, not input-specific.** The identical open,
   5-of-6-face shell is accepted silently by `cut` and `intersect` (0 solids, no exception, and the
   in-memory shape's own `isValidSolid` still reports `true`) but makes `fuse` throw
   `Cannot determine shape type: shape is null` after first logging `fuse history path produced null
   result; retrying without evolution tracking`. "An open shell is rejected" is not a property of the
   input alone — it depends on which operation you hand it to.
6. **Construction-time errors are not uniformly typed.** `box(0,10,10)` and `cylinder(-5,10)` both throw
   a raw `WebAssembly.Exception` with no message and no enumerable properties — useless for a
   "deterministic structured error" fixture. `ellipse(5,10)` (minor radius > major) is the one primitive
   here that rejects with `brepjs`'s own typed `{kind, code, message}` payload. The same library is
   inconsistent about whether an invalid construction is even reported as an `Error` at all.

**The arrangement whose answer most surprised: face-touching/coincident-faces under `fuse`.** Not
because the volume was wrong — it never was — but because three unrelated constructions all show `fuse`
leaving behind redundant, un-merged coplanar faces for a result that is analytically a single simple
box, while `intersect` on equivalent geometry lands on the minimal topology every time. A from-scratch
kernel that "simplifies as it goes" would disagree with this oracle on every one of these fixtures despite
being arguably more correct.

## Recipes attempted and abandoned, with the kernel's exact error

- `solid([...facesA, ...facesB])` welding two overlapping (interpenetrating) boxes' 12 faces directly,
  as a second route to a self-intersecting solid: threw a non-`Error` value with no message and no
  enumerable properties (`String() → "undefined"`). Abandoned in favor of the bowtie
  wire → `wireLoop` → `face` → `thicken` construction, which succeeds and gives rich, measurable
  invalidity (`isValidSolid=false`) instead of an unusable opaque failure.
- `box(0, 10, 10)` and `cylinder(-5, 10)`, considered for the "deterministic structured error" recipe:
  both throw a raw `WebAssembly.Exception` (`proto=Exception`, zero own keys, `String()` →
  `"[object WebAssembly.Exception]"`) — no message, no code, nothing to assert on or quote. Abandoned in
  favor of `ellipse(5, 10)`, which rejects through `brepjs`'s own `Result` machinery with a typed,
  reproducible `{kind, code, message}` payload.
- `polygon([p1, p2, p3, p4])` with the same self-crossing (bowtie) point order: succeeded silently, no
  validation at all — a second, independent confirmation that this kernel does not check for
  self-intersection at construction time (consistent with finding #4 above). Not used as its own fixture
  since `wireLoop`/`face`/`thicken` already demonstrates the same acceptance with richer, measurable
  invalidity downstream.
- `cylinder(5, -10)` (negative height): accepted without incident, not pursued further as a separate
  fixture once `cylinder(5, 0)` had already produced the more informative degenerate-and-propagating-
  invalidity finding.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/🧪️spatial-relationship/📜️script.ts` — 7 → 33 recipes.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/🧪️failure/📜️script.ts` — 2 → 12 recipes.
- `🧫️fixtures/<id>/` generated (operand STEPs, `expected.step`, `expected.mesh.json`, `expected.metrics.json`) for all 36 new/changed ids, each via its own `generate --only <id>`.
- Scratch (kept): `🔬️spatial-failure-explore.ts`, `🔬️spatial-failure-explore2.ts`, `🔬️spatial-failure-explore3.ts` in this ticket folder.

**Observed, not caused by this ticket alone:** every `generate --only <id>` run overwrites the shared
`🧫️fixtures/🧫️manifests.json` down to a single-entry array for whichever id ran last (the generator
writes exactly the filtered recipe set, not a merge). This was already happening from other agents'
concurrent runs on sibling files before this session started and is not something either of the two
files in scope controls; it will need a single, non-`--only` `generate` pass by whoever owns final
consolidation to rebuild it for the whole corpus.
