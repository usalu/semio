# Staffage Library Test Fixes (2026-11-17)

Scope: `@teaching/staffage` suites `fill-rule`, `plan-projection`, `three-views`, `export-performance`, the tessellation re-export clean-up and the site `model-split`. `catalog-silhouette` belongs to another agent and was not touched.

## Results

| Suite | Result | Cause and fix |
| --- | --- | --- |
| `fill-rule` | pass (11) | Authored geometry: even-odd overlapping subpaths in the 1:500 elevations. Unioned the rings with shapely (`fix-fill-rule.ts`) for jogger `figure`, leaning-person `figure`, `forearm-l`, `forearm-r`, person-with-umbrella `forearm-r`. person-with-umbrella `forearm-l` was a self-crossing noise path (3 kB) and now equals the clean `forearm-r` shape. |
| `plan-projection` | pass (13) | Test expectation wrong, asset right: the city car is authored 1.5 m high (`heightM` 1.497 is its elevation height, already asserted equal to the evaluated elevation) and 1.78 m wide in plan, so "taller than the plan depth + 0.3" cannot hold for a car. The plan branch now asserts that `heightM` differs from the plan depth by more than 0.15 m in either direction. |
| `three-views` | pass (329) | Authored geometry (see below). |
| `export-performance` | pass (31) | Legitimate data change, digests re-pinned (see below). |
| `model-split` (site) | pass (2) | Still passes after removing the re-export. |
| `validate-all.ts` | failing 0 of 152 | |
| `validate-manifest.ts` | manifest issues 0 | |

## three-views

- mallard-duck 1:100: the front legs ended 0.02 m below the elevation feet (diff exactly at the 0.020 tolerance). Front foot rows lifted by 0.02 m (-0.22668 to -0.20668, -0.22868 to -0.20868).
- pigeon 1:100: the near toes box of the front view only covered the toe thickness; the toe bone points down in flight, so the elevation reaches 0.023 m lower. The front `near-toes` box now spans the true toe length (local x max 0.02124 to 0.0471, as the elevation outline).
- herring-gull 1:50: the plan tail capsule overshot the elevation tail by 0.021 m (gliding-down, soar). `x1` -0.1045 to -0.0945.

## export-performance: why the digests changed

Reproduced the pinned digests exactly from a `git archive` of commit 395993ad470a (the commit that pinned them), then exported the current tree and diffed.

- `radiator-rated`, `radiator-transient` (every frame): `panel-radiator` was edited after the pin (new `end-cap-far` detail shape, filled panels, `areaM2` and `ratedPowerBasis` changes): the frames now carry the extra rear panel (including its thermal value in the transient frames). Legitimate asset data change.
- `walker-line`, `walker-thermal`: the SVG frames are byte-identical; only the cast index `🔣️.json` differs, because the cast index gained `logical` (figure height, bounds, parts) and `lod` fields (cast LOD work) and `walking-commuter` was re-authored (new content revision).
- No regression: 14 digests replaced in `🧫️fixtures/🚀️export-performance/🔣️.json` (all frames of both radiator casts plus the index files of the four casts).

## Tessellation clean-up

`🔺️tessellation/🟦️.ts` no longer re-exports `tessellateSolid` and `restWorldMatrices`. Importers updated to the solid module `🔺️tessellation/🧱️solid/🟦️.ts`: the module barrel `🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts` (public export of both names), `🩻️integrity/🟦️.ts`, `🧪️tests/🧵️loft/🟦️.ts` and a doc link of the axonometric mesh module. All other consumers import through the barrel and are unchanged. Typecheck (`repo tools exec tsc -- -p tsconfig.json --noEmit`): site, react and library packages exit 0.

## Helper scripts in this folder

`fix-fill-rule.ts`, `three-views-probe.ts`, `three-views-shapes.ts`, `three-views-rings.ts`, `perf-digests.ts`.
