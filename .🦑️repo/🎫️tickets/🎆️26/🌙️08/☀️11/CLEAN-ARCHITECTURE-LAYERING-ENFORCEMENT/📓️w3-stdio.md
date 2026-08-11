# Wave 3 — 🗄️stdio

Scope: `✏️s/🔌️plugins/🗄️stdio/` subtree only.

## Extra task — wire + complete `FormatDescriptor` roster

Files touched:
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` — added a `//#region Manifest` /
  `//#endregion Manifest` block right after the existing `Plugin` region, mounting
  `🛂️manifest/🦀️component.rs` as `pub mod manifest;` (crate root, same style as the
  `pub mod plugin;` mount next to it). Previously `🛂️manifest` was not mounted anywhere in
  glue.rs, so `crate::manifest::*` did not exist.
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` — added
  `crate::manifest::register_stdio_format_descriptors();` as the first statement inside
  `plugin()`, alongside (before) the existing `crate::artifacts::*::engine::register()` calls.
- `✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️component.rs` — replaced the 3-entry illustrative
  subset (json/png/obj) with the full 28-entry roster. Every row (`kind_id`, `short_id`, `mime`,
  `extension`, `name`, `full_name`, `dir_name`, `is_binary`) is transplanted verbatim from
  `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`'s `STDIO_FORMAT_CATALOG` const (line ~1054).
  `neutral` has no equivalent on the generic `FormatDescriptor` side (it's `bool`, not mesh's
  semantic-model `&'static str`) — set `true` for every row, same convention the pre-existing
  3-entry scaffold already used. Cross-checked every `dir_name` against the real directories
  under `🗿️artifacts/` (`ls` of that dir) — all 28 match exactly, no drift. Removed the
  `TODO(wave-3)` marker and updated the `register_stdio_format_descriptors` doc comment (no
  longer "NOT YET WIRED" — it's called from `plugin()` now).

## Shared recipe

### Step A — Schema self-registration
`🎛️apps/🦀️component.rs` is a one-line stub comment ("Apps facet for `🗄️stdio` — library plugin
stub.") — no apps. `stdio` is a pure library plugin (`.library()` in `plugin()`, zero
`AppBuilder`/app usage anywhere in the crate). **Skipped — no apps needing schema
registration.**

### Step B — Open contribution producer conversion
`grep -rn "Contribution::" ✏️s/🔌️plugins/🗄️stdio/` → no matches. `📦️packages/🦀️rust/Cargo.toml`
has no `[package.metadata.semio]` `contributes`/`consumes` table either. **Skipped — no
`Contribution::` producer sites in this plugin.**

## Verification

`cargo check -p semio-s-plugin-stdio` — clean. `grep -c "^error"` on the run's
`--message-format=short` output → `0`. Only pre-existing warnings (unused imports,
`hidden lifetime parameters`, `artifact_state`/`snapshot_state` never read, etc. — all in
files this task didn't touch, e.g. pdf/jpg/tiff/docx/pptx/xlsx/bcf standards engines). No
"document"-refactor-shaped errors encountered.

## Files touched (summary)
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️component.rs`
