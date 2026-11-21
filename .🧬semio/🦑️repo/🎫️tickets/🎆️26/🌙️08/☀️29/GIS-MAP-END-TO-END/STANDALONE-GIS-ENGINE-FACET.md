# Standalone GIS Engine Facet

The standalone GIS2D map and GIS3D terrain rows select different apps within the GIS component. The standalone boot closure contains only the GIS plugin. The main Space playground can open apps across plugins, including the four Puzzle factory routes. The framework selects the Play facet when the chosen contribution declares the Puzzle engine.

GIS2D now selects its authored map contribution with the renderer facet. The map entry calls the shared GIS bootstrap with its empty factory default. That bootstrap supplies the GIS inference presentation, format catalog and service worker. GIS3D already had its renderer-only terrain contribution and shared bootstrap in the active workspace. Its publication landed concurrently. All three playground projections now agree with both authored GIS variants. The Space contribution still declares and imports the Puzzle factories and selects Play.

## Changed Files

- `🌎️hub/🧩️compositions/🌍️gis/🧩️component/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧬️schema/🗺️map.json`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🚀️entry/🗺️map.ts`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🏗️builder/🌐️vite/🗺️map.ts`
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎠️playgrounds.json`
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json`
- `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎮️playgrounds/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🌱️birth/🧫️fixtures/gis-contribution.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🌱️birth/🧪️tests/🟦️.ts`

## Verification

The newly extended language-neutral regression failed before the change on the GIS2D generated contribution. An independent SQLite query with the repository’s actual engine resolver then passed across the authored metadata, two JSON projections and TypeScript projection. The resulting engine IDs were Surface, Editor and NodeGraph for GIS2D and GIS3D, plus Puzzle for Space. GIS2D still publishes the /osm and /vt routes and GIS3D publishes /dem.

Full canonical rendering yielded an empty inventory in the active tooling view. The selected-row update sourced the contribution paths from the authored metadata, checked each current destination before writing, and preserved the complete published rows except for the GIS contribution field.

The first full Vite verification met a concurrent repository graph module mismatch. Its remote-exchange importer requests an `encodeClosureMemoV1` export absent from the closure-shards module. Both the fixture import and an isolated GIS Vite config import reached that same failure. The root task tracks the live startup.
