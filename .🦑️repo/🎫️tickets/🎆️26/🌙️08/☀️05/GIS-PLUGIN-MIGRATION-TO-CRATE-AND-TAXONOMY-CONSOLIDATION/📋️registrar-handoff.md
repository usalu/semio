# 📋️ Registrar handoff — 🌍️gis

Per `📋️TEMPLATE.md` §10. The migrating agent (this ticket) never edits root `Cargo.toml`/`Cargo.lock`,
root `📜️script.ts`, the registry script or `launch.json`.

## Remove these member lines from root `Cargo.toml`

```
    "✏️s/🔌️plugins/🌍️gis/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🔨️modules/⚙️engine/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🔨️modules/🗣️dsl/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🔨️modules/🔧️op/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🔨️modules/🎒️pack/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🔨️modules/📡️protocol/⚡️implementations/🦀️rust",
    "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/🔨️modules/🖱️ui/⚡️implementations/🦀️rust",
```

(15 lines, root `Cargo.toml` lines 247–261 at the time of writing.)

## Add

```
    "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust",
```

## Also remove from `[workspace.dependencies]`

```
semio-s-app-gis-2d = { path = "✏️s/🔌️plugins/🌍️gis/🎛️apps/◻2d/⚡️implementations/🦀️rust" }  # 9 refs
semio-s-app-gis-3d = { path = "✏️s/🔌️plugins/🌍️gis/🎛️apps/🧊️3d/⚡️implementations/🦀️rust" }  # 8 refs
```

(root `Cargo.toml` lines 634–635.) Replace with a single entry if the registrar keeps the survey table:

```
semio-s-plugin-gis = { path = "✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust" }
```

Nothing referenced these two `[workspace.dependencies]` keys via `workspace = true` — every consumer
spelled its own `path =` — so removing them is safe on its own.

## Cross-cutting edits this agent DID make

- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️fixture-sweep/⚡️implementations/🦀️rust/Cargo.toml`
  + `📦️lib.rs` — `[dev-dependencies]` `gis2d`/`gis3d` collapsed into one `gis` dep on the merged crate;
  the two `use` lines repointed to `gis::artifacts::gismap::GisMapDocument` /
  `gis::artifacts::gisterrain::Gis3dTerrainDocument`. (TEMPLATE §8.2.)

## Cross-cutting edits this agent did NOT make (orchestrator's interim repoint)

- `✏️s/🔌️plugins/🎪️demonstrator/🛂️manifest/🗿️artifact/⚡️implementations/🦀️rust/Cargo.toml` lines 129–131
  (`[dependencies]` `gis2d`, `gis2d_engine`, `gis2d_ui`) and its `📦️lib.rs` lines 84–86 + 114.
  New module paths for every symbol demonstrator uses:

  | demonstrator's current path | new path in `semio-s-plugin-gis` |
  |---|---|
  | `gis2d::GIS_MAP_SCHEMA` | `gis::artifacts::gismap::GIS_MAP_SCHEMA` |
  | `gis2d::GisMapDocument` | `gis::artifacts::gismap::GisMapDocument` |
  | `gis2d_engine::gis2d_document_json_to_svg` | `gis::artifacts::gismap::engine::gis2d_document_json_to_svg` |
  | `gis2d_engine::gis2d_document_json_from_dwg` | `gis::artifacts::gismap::engine::gis2d_document_json_from_dwg` |
  | `gis2d_ui::Gis2dPlayApp` | `gis::apps::gis2d::Gis2dPlayApp` |
  | `gis2d_ui::create_gis2d_app` | `gis::apps::gis2d::create_gis2d_app` |

  One dep line replaces all three:
  `gis = { path = "../../../../../../../✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust", package = "semio-s-plugin-gis" }`

## Commands still un-run (need a healthy workspace / the registrar's regeneration)

- `bun 🧰️framework/…/📇️registry/📜️script.ts check`
- `bun nx run @semio-tech/framework-os-dev:plugin -- gis`
- `bun ./📜️script.ts dev gis2d` / `dev gis3d` boot smoke
- `bun ./📜️script.ts verify gate`
- `cargo check --workspace`
