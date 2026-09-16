# Verfolgen — complete reuse map recovery (2026-09-16)

## Symptom

Demonstrator **Verfolgen** showed only two Liège pins and one reuse line. Older GIS / mit-bestand builds showed the full European reuse network (~150 pins, ~150 relationship lines).

## Root cause

| Layer | What broke |
|---|---|
| Bundled asset | `🗣️.dsl.semio` under `🗺️gismap/…/🖼️assets/🎬️demo/` was a **minimal stub** (2 positions, 2 routes, ~1.6 KiB). |
| History | The full fixture lived as `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/…/📚️examples/🌍️reuse.map.gismap` (~76 KiB, 152 positions) until commit `7eba781d56` (2026-08-05 taxonomy consolidation), which deleted it without transplanting the data into the artifact demo asset. |
| Demonstrator defaults | `♻️mit-bestand/🧺️demonstrator/🪧️brand.ts` still staged `exampleId: "reuse-map"`, but the GIS catalogue id is **`demo`** since the example-facet work (`📓️fix-2026-09-16-gis-selection-dispatch.md`). A stale id faults or never loads the bundled map. |

## Canonical complete reuse map (source of truth)

1. **Descriptor JSON** (152 positions, 149 routes, 0 regions): recovered from git commit `96cf416736` at path `gis/2d/example/reuse.map.gis.json` (ticket copy: `🗑️generated/reuse.map.gis.json`).
2. **Legacy block DSL** (same content, human-readable): commit `fa51b5c82f` at `✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/⚡️implementations/🦀️rust/📚️examples/🌍️reuse.map.gismap` (ticket copy: `🗑️generated/reuse.map.gismap.recovered`).
3. **Conversion notes**: original JSON→`.gismap` one-off is documented in ticket `…/🎆️26/🌙️07/☀️26/DSL-AND-OP-TEXT-WAVE-4-MULTI-APP-TECHNOLOGIES/gis.md` and `convert_gis_fixtures.py`. The mit-bestand graph export / pin plan is `.cursor/plans/gis-map-reuse-pins_a017207e.plan.md` (pre-artifact `gis/map/fixture/reuse.graph.gis.json`).

## Fix applied this session

- Regenerated `🗣️.dsl.semio` as `semio gis.gismap.dsl v1` with hex-encoded `positions` / `routes` / `regions` from the recovered JSON (~118 KiB).
- Example catalogue label: **Reuse Map** / **Karte wiederverwenden** (`📚️examples/🎬️demo/🦀️.rs`).
- Verfolgen brand default: `exampleId: "demo"` (`🪧️brand.ts`).

## Follow-up

- Restage `gis` / `demonstrator` plugin manifests (`🔣️.json` still list stale `reuse-map` until component restage).
- Re-run targeted `semio-s-artifact-gis-gismap` DSL round-trip tests after native build completes.
