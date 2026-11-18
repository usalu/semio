# Staffage Website Tests (2026-11-17)

Command shape (package directory, absolute config; a relative config with `--configLoader native` loads the wrong root):

```
repo tools exec vitest -- run --configLoader native --config <ABS>/<project>/🧪️tests/🎚️config/🟦️.ts [name filters]
```

Logs: `🗑️generated/{site,react,lib,lib-targeted,lib-heavy}.log`.

## Results

| Project | Result |
| --- | --- |
| `@teaching/staffage-site` | pass: 3 files, 9 tests (includes `🛟️stale-chunk`) |
| `@teaching/staffage-react` | pass: 27 files, 240 tests |
| `@teaching/staffage` (library) | not green. Full run exceeds the 900 s budget (cancelled at about 15 min, 136 failures seen before). After fixes, targeted re-runs: asset-power, phase-change, hydronic-coupling, section-views, schema-conformance, course-coverage pass (209 of 210 tests across the 7 targeted files); fill-rule, catalog-silhouette, plan-projection, three-views, export-performance still fail (below) |

## Fixed

- Site `🧊️model-split` (real regression): the model kernel's validator and `tessellate` landed in the site entry, because `tessellateSolid` (now used by the catalog silhouettes and the axonometric mesh in the 2D path) lived in the same module as `tessellate`/`validateModel`. Split `tessellateSolid` and `restWorldMatrices` into `🔺️tessellation/🧱️solid/🟦️.ts`; `🔺️tessellation/🟦️.ts` re-exports them, so no importer changes. The entry no longer carries the validator.
- `sliding-door` 1:500 representation had no `sections.plan` (the section test requires every authored scale): added it like the other scales.
- `electric-kettle`: the exchange areas changed with the volume model (water 0.012 to 0.0155 m2) but `heatCapacityJM2K` stayed per old area, so the water held 5.4 kJ/K instead of 4.19 kJ/K and never boiled (phase-change and hydronic tests). Capacities rescaled to keep the total J/K per region (water 270039, plate 2308, shell 2028, handle 857; layers equal to the region).
- `🧪️tests/🕳️fill-rule`: stale against the catalog root, which now holds a `🔣️.json` index next to the category folders (ENOTDIR); the test now lists directories only.

## Still failing (library, not site-blocking; asset geometry in flux)

- `🕳️fill-rule`: 9 overlapping filled subpaths in 1:500 elevations of jogger, leaning-person (5), person-with-umbrella (2), committed assets, not edited in the worktree.
- `🧊️catalog-silhouette`: 117 of 162 failing: the derived 3D volume silhouettes disagree with the authored 2D shapes (Hausdorff and IoU bounds, e.g. tower-crane 1:500 elevation 1.86 m against 0.15 m; wheelchair-ramp 10.52 m against 10.5 m). This is the volume-representation work of the coordinator, not the site.
- `🧍️plan-projection`: plan-car heightM 1.497 against expected above 2.08.
- `🧊️three-views`: herring-gull, mallard-duck (2), pigeon draw checks.
- `🚀️export-performance`: pinned reference digest of the export differs (geometry changed).
- Earlier runs also failed asset-power (7) and schema-conformance (1) and course-coverage (1): the coordinator's asset fixes made them pass.

## Files changed

- `🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts`
- `🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🧱️solid/🟦️.ts` (new)
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/sliding-door/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electric-kettle/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🕳️fill-rule/🟦️.ts`
