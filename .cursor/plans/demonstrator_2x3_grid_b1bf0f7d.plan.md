---
name: Demonstrator 2x3 Grid
overview: Extend the Entwerfen mit Bestand demonstrator from a 3-app horizontal strip into a 2x3 grid by adding Aussuchen (sourcing), Bearbeiten (process 3d) and Verfolgen (gis 2d) as fully branded apps, and generalize the cursor-driven scroll and tint effect from one axis to both.
todos:
  - id: variants
    content: Add aussuchen/bearbeiten/verfolgen playground variant rows to the sourcing, process and gis manifests with ports 6030-6032 / 6130-6132 and brand ids, then regenerate the plugin registry catalog
    status: completed
  - id: brands
    content: Author the three new ShellBrands in demonstrator brand.ts (German two-step intros, real window ids, staging distDirs), extend ENTWERFEN_MIT_BESTAND_BRAND_IDS to six, update the general introduction copy, and register them in the shell brand catalog
    status: completed
  - id: grid-geometry
    content: "Generalize the landing from a 1-D vw strip to a 2-D 300vw x 200vh grid: 2-D scroll offset state, cursor X and Y mapping, per-pane screen rect and reveal rect, grid strip transform"
    status: completed
  - id: cards
    content: Grow the pane table to six entries with row/col, icons and hrefs, and lay the glass cards out in a fixed 3x2 screen grid sized for two rows
    status: completed
  - id: wiring
    content: Extend demonstrator script.ts slugs and variants, add the three dev proxy entries, root package.json dev scripts and launch.json dev configurations
    status: completed
  - id: tests-verify
    content: Update the renderer brand test to expect six brands, run registry generation, vitest and the full demonstrator build, and verify diagonal scroll, per-pane untint and click navigation at runtime
    status: in_progress
isProject: false
---

# Demonstrator 2x3 Grid

## Layout target

Strip becomes 300vw x 200vh. Row 1 stays Generator / Koordinator / Aggregator; row 2 is Aussuchen / Bearbeiten / Verfolgen. Cursor X drives horizontal scroll (0-200vw, unchanged), cursor Y drives vertical scroll (0-100vh, new). Six glass cards sit in a fixed 3x2 screen grid; hovering one still pins the scroll to that pane and cuts the veil open over that pane's visible rect.

## 1. Playground variants

Add one `[[package.metadata.semio.playground]]` row per plugin manifest, mirroring the existing `generator` row in [procedural Cargo.toml](✏️s/🔌️plugin/🌀️procedural/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust/Cargo.toml):

- [sourcing](✏️s/🔌️plugin/🪵️sourcing/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust/Cargo.toml): `variant = "aussuchen"`, `app = "sourcing-curate"`, `brand = "entwerfen-mit-bestand-aussuchen"`, `ports = { react = 6030, wgpu = 6130 }`
- [process](✏️s/🔌️plugin/🏭️process/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust/Cargo.toml): `variant = "bearbeiten"`, `app = "process3d-play"`, `brand = "entwerfen-mit-bestand-bearbeiten"`, `ports = { react = 6031, wgpu = 6131 }`
- [gis](✏️s/🔌️plugin/🌍️gis/🛂️manifest/🗿️artifact/⚡️implementation/🦀️rust/Cargo.toml): `variant = "verfolgen"`, `app = "gis2d-play"`, `brand = "entwerfen-mit-bestand-verfolgen"`, `ports = { react = 6032, wgpu = 6132 }`, plus the same `engines = ["./🧰️framework/🔨️module/🗺️surface/🗺️tiled-map/⚡️implementation/🦀️rust"]` as the `gis2d` row

Ports 6030-6032 and 6130-6132 are unused today (6029 is the landing, the next taken react port is 6040).

Then regenerate the catalog with `bun nx run @semio-tech/plugin-registry:generate`, which rewrites [🟦️playgrounds.ts](🧰️framework/🛍️product/💻️os/🔨️module/🔌️plugin/⚡️implementation/🟦️typescript/📇️registry/🤖️generated/🟦️playgrounds.ts) and its json/rs siblings.

## 2. Brands

In [🟦️brand.ts](♻️mit-bestand/🧺️demonstrator/🟦️brand.ts), add three `ShellBrand` consts in their own regions, shaped like `ENTWERFEN_MIT_BESTAND_GENERATOR_BRAND` (locks `locale: "de"` and `themeId: "semio"`, `ephemeral`, `replayIntroductionOnLoad`, shared `assetsDir`, `distDir` under `${DEMONSTRATOR_DIST_STAGING}/<slug>`), each with a two-step German introduction (viewport + panels) anchored to real window ids:

- Aussuchen: window `sourcing-pool`, example `demo-stock`
- Bearbeiten: window `process-workpiece`, example `timber-beam-joinery`
- Verfolgen: window `gis2d-main`, example derived from `🌍️reuse.map.gismap` (verify the id against the generated catalog before wiring `defaults`)

Also in that file: extend `ENTWERFEN_MIT_BESTAND_BRAND_IDS` to all six ids, and update the `ENTWERFEN_MIT_BESTAND_GENERAL_INTRODUCTION` welcome copy from "drei Werkzeuge" to six.

Register them in [🏷️brand/📦️index.ts](🧰️framework/🛍️product/💻️os/🔨️module/🧑️‍💻️dev/⚡️implementation/🟦️typescript/🏷️brand/📦️index.ts): import, append to `SHELL_BRANDS`, and re-export alongside the existing three.

## 3. Landing geometry: one axis to two

All in [📦️index.tsx](♻️mit-bestand/🧺️demonstrator/📦️index.tsx).

The pane table grows to six entries with explicit `row` and `col` (or derived as `col = index % 3`, `row = (index / 3) | 0`), new hrefs `/aussuchen/`, `/bearbeiten/`, `/verfolgen/`, and icons `library`, `hammer`, `gis2d` (the canonical `app.sourcing` / `app.process` / `app.gis2d` assignments in [🟦️icon_concepts.ts](🧰️framework/🔨️module/🖱️ui/🖼️asset/⚡️implementation/🟦️typescript/🟦️icon_concepts.ts)).

Replace the scalar scroll offset with a 2-D one:

- `scrollOffsetForPaneIndex(i)` becomes `scrollOffsetForPane(pane)` returning `{ x: col * 100, y: row * 100 }`, clamped to 0-200 vw and 0-100 vh.
- `paneScreenBounds` gains a vertical twin so a pane maps to a full screen rect, and `demonstratorPaneRevealRect` intersects both axes instead of always spanning `0..vh`.
- State `scrollOffsetVw: number` becomes `{ x, y }`; `scrollTargetRef` and `scrollCurrentRef` hold the same shape, and the rAF easing lerps both components with the existing `0.12` factor.
- Mousemove sets `x: (clientX / innerWidth) * 200` and `y: (clientY / innerHeight) * 100`.
- The strip wrapper switches from `flex` plus `width: 300vw` to a `grid grid-cols-3` at `width: 300vw; height: 200vh`, transform `translate(-{x}vw, -{y}vh)`, with each iframe `100vw x 100vh`.

`demonstratorTintSegmentsPx` already cuts an arbitrary px rectangle out of a full-viewport veil, so it needs no change. The card overlay becomes `grid grid-cols-3 grid-rows-2`, with the card min-height reduced so two rows fit comfortably.

Hash deep-links, the click-only URL update, and the bfcache reload-on-return stay as they are and simply pick up the three new ids.

## 4. Build, dev and launch wiring

- [📜️script.ts](♻️mit-bestand/🧺️demonstrator/📜️script.ts): extend `APP_SLUGS` and `PLAYGROUND_VARIANTS` to all six, so dev spawns six OS servers plus the landing and build produces six staging dirs merged into `dist/`.
- [⚙️vite.config.ts](♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts): add `/aussuchen`, `/bearbeiten`, `/verfolgen` proxy entries pointing at 6030/6031/6032, same shape as the existing three.
- Root [package.json](package.json): `dev:mit-bestand:aussuchen`, `dev:mit-bestand:bearbeiten`, `dev:mit-bestand:verfolgen` calling `bun ./📜️script.ts dev <variant>`, placed with the existing mit-bestand dev scripts.
- [.vscode/launch.json](.vscode/launch.json): three `🛠️dev🏚️mitbestand…⚛️react` entries following the generator/koordinator pattern (matching `S_OS_PORT`, `serverReadyAction` port regex, `3_dev` group, orders continuing after 215).

## 5. Tests

Update the brand test in [🧪️index.test.ts](🧰️framework/🛍️product/💻️os/🔨️module/📺️renderer/🧑️‍🎨️engine/⚛️react/⚡️implementation/🟦️typescript/🧪️index.test.ts) (the "registers all three Entwerfen mit Bestand demonstrator shell brands" case) to expect all six ids and assert each new brand's `id`, extending the existing suite rather than adding files.

## 6. Verification

- Run `bun nx run @semio-tech/plugin-registry:generate` and confirm the three new rows appear in the generated catalog with the right ports and brands.
- Run the renderer vitest suite covering the brand assertions.
- Run `bun nx run @semio-tech/mit-bestand-demonstrator:build` end to end (six wasm app builds plus landing assembly) and spot-check that `dist/<slug>/index.html` exists for each.
- Runtime check of the landing: the cursor near a corner scrolls diagonally, hovering each of the six cards untints exactly that pane, and clicking sets the URL and lands on the branded app.