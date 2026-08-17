# W6 Batch 1b — Draw / Raster / Forms / Layout / Playbook

## Cargo (2026-08-10)

All five plugin crates **green** (`cargo check`, exit 0):

| Crate | Log |
|-------|-----|
| `semio-s-plugin-draw` | `🧪w6-batch1b-semio-s-plugin-draw.log` |
| `semio-s-plugin-raster` | `🧪w6-batch1b-semio-s-plugin-raster.log` |
| `semio-s-plugin-forms` | `🧪w6-batch1b-semio-s-plugin-forms.log` |
| `semio-s-plugin-layout` | `🧪w6-batch1b-semio-s-plugin-layout.log` |
| `semio-s-plugin-playbook` | `🧪w6-batch1b-semio-s-plugin-playbook.log` |

Combined run: `🧪w6-batch1b-cargo-all.log` (`Finished dev profile`, no `error`).

Proof line (each per-crate log ends with):

```
Finished `dev` profile [unoptimized] target(s) in …s
```

Notes: forms/layout/playbook may emit `block v0.1.6` future-incompat warning only.

## Disk — stdio artifact shape

For each of draw, raster, forms, layout, playbook under `🗿️artifacts/<plugin>/`:

- **Removed (must not exist):** root `🗣️dsl`, `📸️snapshot`, `🔺️diff`, `🔧️op`, `📡️spr`, root `🧬️mutations`
- **Present:** `🏗️builder`, `🪓️decomposer`, `🧬️schema` (snapshot/diff/mutations absorbed), `🚪️io/📥️import/🧩️deserializers`, `🚪️io/📤️export/🧵️serializers`

Verified via ticket disk script (same session as cargo green).

## Compile fixes (layout — last red crate)

Layout IO had invented stdio field names after a manual rewrite:

- **PDF export:** `PageDoc` from `semio_s_plugin_stdio::artifacts::pdf::schema::snapshot` (not crate-root reexport).
- **SVG:** `SvgSnapshot.doc` + `parse_svg_xml` / `write_svg_xml` (not `body`).
- **DWG export:** `DwgSnapshot` includes `version` and `section_names` per stdio schema.

Forms/playbook were fixed earlier (diff grammar paths, glue `schema::diff::text`, mutations imports).

## Status

**Done:** disk migration + `cargo check` green for all five batch 1b plugins.
