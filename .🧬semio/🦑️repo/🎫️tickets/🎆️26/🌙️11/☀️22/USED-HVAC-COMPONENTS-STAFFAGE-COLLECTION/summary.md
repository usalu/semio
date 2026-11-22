# Summary — Used HVAC Components Staffage Collection

## Scope

The dev's order was a massive wave of Opus 5.5 agents. After the first wave started, the dev narrowed it to the authors that had already started, so the scope is **18 used HVAC components** (`assign.md` rows 01–18). Rows 19–40 were dropped before any work on them began.

## Result

All 18 assets are published to `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🧱️components/<slug>/🔣️.json`. Each has a manifest row and is a member of the collection `used-hvac-components` ("Used HVAC Components" / "Gebrauchte TGA-Komponenten"). An independent verifier agent checked every asset: catalog-only `r3-validate` VALID, the product check CLI ✓, the tags `component`/`reclaimed`/`used`/`hvac` and the power basis per README "Power".

| slug | slug |
|---|---|
| used-cast-iron-sectional-boiler | used-panel-radiator-pallet |
| used-wall-hung-gas-boiler | used-towel-radiator |
| used-forced-draught-oil-burner | used-trench-convector |
| used-steel-oil-storage-tanks | used-underfloor-heating-manifold |
| used-air-water-heat-pump-monobloc | used-buffer-storage-tank |
| used-brine-water-heat-pump | used-expansion-vessels |
| used-pellet-boiler | used-dry-rotor-inline-pump |
| used-district-heating-substation | used-steel-pipe-rack |
| used-radiant-ceiling-strip | used-insulated-copper-coils |

The per-asset reports with reference tables, scale decisions, proofs and a `## Verification` section are in `reports/<slug>.md`. Open author requests are in `requests.md`.

## Manifest

A concurrent writer twice dropped the `used-wall-hung-gas-boiler` row (see `requests.md`); it was re-published and is present. The catalog-wide `validateManifest` also reported 33 drifted rows of other assets (heights, widths, poses, clips, power), the asset files being the single source of truth. `manifest-sync.ts` re-derived those fields in place under the publish lock shared with the reclaimed-components wave; `catalog-check.ts` now reports 0 issues.

## Tests

- Pass: `⚡️asset-power` (24), `✅️asset-validation` (83), `🧊️volume-schema` (115).
- Failing, none of them on an asset of this ticket (each failure names its asset):
  - `🌡️catalog-physics`: sunflower, runner-with-blade-prosthesis, used-playground-roundabout.
  - `🫁️breathing-catalog`: the humans person-checking-watch, sunbather-on-towel and schoolchild-with-backpack.
  - `🔭️multiview-coverage`: reference case standing-adult.
  - `🧊️catalog-silhouette`: crashes before its first test because `vi.setConfig` does not exist under the plain bun runner that `repo test-file` routes it to.
  - `🧊️model-catalog`: stopped at its 3072 MB memory ceiling.
