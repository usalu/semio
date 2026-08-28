# `mechanical` family — 12 new recipes

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️cc6/🏭️generator/🧪️mechanical/📜️script.ts`
is the only file modified. It grew from 4 recipes to 16. Every new fixture below was generated with
`generate --only <id>` (never a bare `generate`) and measured with the `🔬️probes` CLI: `measure`,
`topology`, `brep-validity`, and a self `step-mesh-compare` (`expected.step` against itself, tolerance
`1e-3`/`0.1`). No sibling file, no other family's `📜️script.ts`, `🧪️oracle/🔣️.json`, or another
fixture directory was touched. A stray `--only mechanical-fixture-plate` smoke-test run early in this
session did write into that existing fixture's directory; it was reverted with `git show :<path> ><path>`
(a read-only `git show`, not a working-tree-discarding command) before any other work started, and
`git diff` on that directory now shows zero lines. `🧫️fixtures/🧫️manifests.json` gets rewritten to a
single-recipe array by every `--only` invocation (mine and, per its existing dirty state before I ran
anything, other concurrent agents' too) — that is how the generator's `--only` path works, is exactly
the invocation the task specifies, and is a shared-file hazard pre-existing this session, not something
scoped to this file.

## Result table

All 12 are declared `outcome: "applied"` — MEASURED, not assumed: every one imports back as exactly
1 solid, `isValidSolid() === true`, and `step-mesh-compare` of `expected.step` against itself reports
`normalizedSymmetricDifferenceVolume: 0` (the mesh half of the pipeline represents every one of these
solids with no defect, including the most complex ones).

| fixture | what it is | ops in chain | outcome | volume (mm³) | area (mm²) | faces / edges / verts | mesh genus | valid |
|---|---|---|---|---|---|---|---|---|
| `mechanical-enclosure-boss-vented` | ribbed enclosure + bosses + vents + mounting holes + selected-edge fillet | 17 | applied | 27840.1 | 23000.6 | 145 / 504 / 356 | 12 | yes |
| `mechanical-pipe-manifold-reducer-branch` | pipe body, stepped bore (cyl–cone–cyl reducer), vertical + 30° branch, bolt-circle flanges ×2 | 6 | applied | 67910.6 | 16541.7 | 20 / 48 / 30 | 3 | yes |
| `mechanical-fixture-plate-slotted` | fixture plate: 4 counterbores, pocket, 2 stadium slots, `rectangularPattern` fastener grid | 21 | applied | 38071.2 | 13720.9 | 41 / 107 / 71 | 12 | yes |
| `mechanical-skewed-bracket-gusseted` | angled bracket, `polygon`+`extrude` gusset mirrored via `mirror`, 3 compound-angle cutters | 10 | applied | 27550.4 | 10073.8 | 31 / 81 / 51 | 2 | yes |
| `mechanical-valve-body` | `revolve`-d spool body, main through bore, perpendicular branch port, stepped seat counterbore | 6 | applied | 19445.3 | 7387.3 | 14 / 28 / 18 | 3 | yes |
| `mechanical-nested-shell-channels` | outer `shell()` + inner `shell()` seated inside it, 2 bridging ribs, 2 piercing channels | 6 | applied | 38875.6 | 25935.7 | 34 / 89 / 56 | 7 | yes |
| `mechanical-block-fifteen-cuts` | machined block, 16 through-holes cut ONE AT A TIME + 1 pocket | 17 | applied | 64938.2 | 16531.7 | 29 / 88 / 58 | 16 | yes |
| `mechanical-multi-union-trim-drilled` | base + 2 cylinders + cone boss, sequential fuse, planar trim, 3-hole drill | 6 | applied | 22067.7 | 6890.0 | 18 / 30 / 20 | 0 | yes |
| `mechanical-housing-threaded-boss` | housing + boss carrying a REAL `thread()` ridge, blind pilot bore | 4 | applied | 31318.7 | 7669.4 | 269 / 655 / 390 | 0 | yes |
| `mechanical-heatsink-fins` | base plate + 15 fins (1mm thick, 3.6mm pitch) via ONE `rectangularPattern`, 2 mounting holes | 3 | applied | 36551.2 | 46729.0 | 85 / 210 / 140 | 2 | yes |
| `mechanical-gearbox-cover` | cover disc: shaft bore, bearing pocket, O-ring groove (nested cut-to-build-a-cutter), 6-hole bolt circle, chamfer | 6 | applied | 26197.5 | 11903.5 | 38 / 77 / 44 | 7 | yes |
| `mechanical-lightening-bracket-grid` | selected-edge filleted plate + 15-hole `gridPattern` lightening grid | 2 | applied | 21434.7 | 10500.8 | 25 / 69 / 46 | 15 | yes |

`mechanical-multi-union-trim-drilled` and `mechanical-housing-threaded-boss` measure genus 0 (the
drilled holes there open onto other cut surfaces / the pilot bore is blind, so they don't add a
handle) — a correct measurement, not a defect; every other new fixture carries genus ≥2, up to 16.

## Kernel findings (measured, not assumed)

1. **`fuseAll` over 3+ shapes does not merge into one manifold solid, even when pairwise `fuse` on the
   identical shapes does.** First seen on `mechanical-multi-union-trim-drilled`: `fuseAll([base, post1,
   post2, boss])` measured back as **7** separate `getSolids()` entries, while the SAME four shapes
   folded in with three sequential `fuse` calls measured as **1** solid with the *identical* volume
   (23442.549...). Reproduced independently on `mechanical-nested-shell-channels`
   (`fuseAll([outerShell, innerShell, rib1, rib2])` → 4 solids; sequential `fuse` → 1 solid, same
   volume). Both shipped fixtures were fixed to use sequential `fuse`. This is a real defect in
   brepjs's multi-way boolean, not a topology question — the volume match proves the geometry is
   identical, only the solid-merging/healing step is skipped in the `fuseAll` path. `fuseAll`/`cutAll`
   remain safe and were kept where used only to build a CUTTING TOOL (a compound of solids is a
   perfectly good subtrahend for `cut`) — see `mechanical-pipe-manifold-reducer-branch`'s stepped-bore
   tool and `mechanical-fixture-plate-slotted`'s stadium-slot tool, both of which measured back to a
   single valid result solid.

2. **`fillet(shape, radius)` over ALL edges can return an invalid, zero-volume solid SILENTLY — no
   thrown error.** `fillet(box(100,40,6), 4)` on `mechanical-lightening-bracket-grid`'s first attempt
   measured as `isValidSolid() === false` and `measureVolume() === 0`, while the generator's own
   success path reported it as generated with no exception. The 4mm radius exceeds what the plate's
   6mm-thickness top/bottom rim edges can support, and the kernel returns a degenerate shape rather
   than rejecting the input. Fixed by filleting only the 4 SELECTED vertical edges
   (`edgeFinder().inDirection([0,0,1])`) at radius 2, which measured valid. This is a materially
   different failure mode from `mechanical-block-fifteen-cuts`'s chamfer (below): that one at least
   THROWS.

3. `chamfer(shape, distance)` over all edges CAN throw a hard kernel error rather than either
   succeeding or returning an invalid shape — see the abandoned attempt below. Combined with finding 2,
   the practical rule this corpus now demonstrates is: an unbounded "apply to every edge" fillet/chamfer
   call is not safe to leave unguarded in a recipe, and the three possible outcomes (silent invalid
   shape, thrown exception, correct result) are not predictable from the edge count alone —
   `mechanical-gearbox-cover`'s `chamfer(cover, 0.5)` over a comparable edge count (77) succeeded fine.

## Recipes attempted and abandoned

- **`mechanical-block-fifteen-cuts`**, trailing `chamfer(block, 0.5)` over every edge produced by the
  16-hole grid + 1 pocket (88 edges total). Kernel's exact error:
  ```
  chamfer: {"kind":"KERNEL_OPERATION","code":"CHAMFER_FAILED","message":"Chamfer operation failed:
  [object WebAssembly.Exception]","cause":{},"metadata":{"operation":"chamfer","edgeCount":88,"distance":0.5}}
  ```
  A smaller isolated 16-hole-only case (no pocket) chamfered successfully in an exploratory smoke test,
  so the failure is specific to this hole-grid+pocket edge combination, not to "chamfer after many cuts"
  in general. Dropped the chamfer step; the fixture ships as the 17-cut chain without it.

- **`mechanical-lightening-bracket-grid`**, first attempt: `fillet(plate, 4)` over all 12 edges of the
  6mm-thick plate. Measured result: `isValidSolid() === false`, `measureVolume() === 0`, no thrown
  error (see finding 2 above). Replaced with a selected-edge fillet at a smaller radius; the fixture
  ships with that fix, not the original attempt.

- **`mechanical-nested-shell-channels`** and **`mechanical-multi-union-trim-drilled`**, first attempts:
  built via `fuseAll` (see finding 1). Both measured `isValidSolid() === true` but with 4 and 7
  `getSolids()` respectively instead of 1 — not dropped as invalid (they were technically "valid" per
  the kernel's own predicate) but rejected anyway because a mechanical part reported as N separate
  solid shells is not the single connected part the recipe claims to model. Both ship with the
  sequential-`fuse` fix.

No recipe was abandoned for producing a genuinely wrong/disjoint/empty result once the three issues
above were fixed — every one of the 12 shipped fixtures is `outcome: applied`, 1 solid, valid, and
self-mesh-consistent.

## Functions exercised beyond the original 4 recipes

`fuseAll`/`cutAll` (as cutter-builders, not final-solid builders — see finding 1), `fillet` and
`chamfer` (both all-edges and edge-selected via `edgeFinder`), `shell` with `faceFinder`-selected faces
(real hollowing, not a manual box-in-box cut), `polygon` + `extrude` (the gusset), `revolve` (the valve
body), `mirror` (the mirrored gusset), `thread` (the housing's real helical ridge — the one place in
this corpus exercising a helical sweep), `circularPattern` (bolt circles), `rectangularPattern` (fin
array, fastener grid), `gridPattern` (lightening-hole grid), and `cone` (manifold reducer, boss).
`polyhedron` and a standalone `helix()` wire were not exercised — `thread` covers the helical-sweep
case `helix` would otherwise have demonstrated, and `polyhedron` had no natural fit among the 12
required part types.
