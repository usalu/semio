# W6 — Fresh MediaFormat Census

Command run (read-only):

```
grep -rln "MediaFormat" --include="*.rs" ✏️s 🧰️framework | grep -v "🎫️tickets"
```

Result: **32 files**, **307 total occurrences** (`grep -rn ... | wc -l`).

This supersedes the W0 census (§5 of `w0-recon-report.md`: 55 files / 346 lines) — W5a and W5b
migrated most plugins off `MediaFormat` since then. What's left is (a) plugins that were
explicitly told in W5 they could keep `MediaFormat` as their public-facing enum for now (V7
deletion deferred to this wave), and (b) the framework/OS definition + call sites themselves.

## Plugin files — 22 files across 12 plugins

Package names read from each plugin's `📦️packages/🦀️rust/Cargo.toml` `[package] name`.

| Plugin dir | Crate name | Files | Usage |
|---|---|---|---|
| `✏️s/🔌️plugins/📸️remodel` | `semio-s-plugin-remodel` | 1 | Single engine file builds `ArtifactKindSpec.{export,import}_formats: Vec<MediaFormat>` (Glb/Obj/Stl/Ply/Las/Png export, Glb/Obj import). |
| `✏️s/🔌️plugins/🖨️raster` | `semio-s-plugin-raster` | 3 | App + artifact + standards/engine files all populate `export_formats`/`import_formats: Vec<MediaFormat>` (Svg/Png), fully-qualified via `semio_framework_plugin::MediaFormat` / `semio_framework::MediaFormat` in two of the three, plain `MediaFormat` import in the app file. |
| `✏️s/🔌️plugins/🏭️process` | `semio-s-plugin-process` | 2 | App (`🧊️3d`) and standards/engine files both build `Vec<MediaFormat>` for Step/Obj/Stl/Glb export+import, plus the engine file additionally binds a local `let media_format = MediaFormat::Glb;` value (not just list literals). |
| `✏️s/🔌️plugins/📐️cad` | `semio-s-plugin-cad` | 3 | Heaviest user: `MediaFormat` is a real function-signature type here — `export_solid_for_pane(..., format: MediaFormat)`, `export_solid_modelspace(..., format: MediaFormat)`, `export_solids_as(..., format: MediaFormat, ...)` — plus `match format { MediaFormat::Obj => ..., MediaFormat::Stl => ..., MediaFormat::Step => ... }` enum-variant dispatch in the engine file, extension-string-to-variant mapping in the io command file, `.mime_type()` calls, and several unit-test call sites passing `MediaFormat::Obj/Stl/Step` literals. |
| `✏️s/🔌️plugins/🗄️stdio` | `semio-s-plugin-stdio` | 1 | **Comment-only hit**, not real usage — a doc comment on the gltf composer explicitly says `MediaFormat` is "the deprecated stringly one" and notes every current stdio artifact leaves `export/import_formats` empty for consistency. |
| `✏️s/🔌️plugins/🎞️animate` | `semio-s-plugin-animate` | 1 | **Comment-only hit**, not real usage — a doc comment on the DWG-import slide builder calls it "the legacy MediaFormat-era `semio_framework::DwgDrawing`" and states this struct is one "W6 deletes outright." |
| `✏️s/🔌️plugins/🪐️space` | `semio-s-plugin-space` | 2 | `connections` command file imports `MediaFormat` from `semio_framework_os` and lists it in two `ArtifactKindSpec` (Svg export/import for one kind, Glb export/import for another); `media` command file imports `MediaFormat` from `semio_framework` and passes `MediaFormat::Dwg` both as a handler-registration argument (`register_os_media_export_handler("2d.drawing", MediaFormat::Dwg, ...)`) and via `.mime_type()`. |
| `✏️s/🔌️plugins/🌍️gis` | `semio-s-plugin-gis` | 1 | Single engine file builds `export_formats`/`import_formats: Vec<MediaFormat>` (Svg/Png) via fully-qualified `semio_framework_plugin::MediaFormat`. |
| `✏️s/🔌️plugins/🎥️shooting` | `semio-s-plugin-shooting` | 2 | Standards/engine file builds `Vec<MediaFormat>` (Svg/Png) fully-qualified; app file imports plain `MediaFormat` and builds `Vec<MediaFormat>` (Png only) for its own `ArtifactKindSpec`. |
| `✏️s/🔌️plugins/📏️layout` | `semio-s-plugin-layout` | 3 | App file, artifact file, and standards/engine file each independently build `export_formats`/`import_formats: Vec<MediaFormat>` (Svg/Png) — app and artifact files import plain `MediaFormat`, engine file uses fully-qualified `semio_framework_plugin::MediaFormat`. |
| `✏️s/🔌️plugins/🖍️draw` | `semio-s-plugin-draw` | 2 | Both hits are `Vec<MediaFormat>` list literals (Svg/Png) in `ArtifactKindSpec`, one fully-qualified via `semio_framework_plugin::MediaFormat`, the other via `semio_framework::MediaFormat`, in two different structs/functions of the same artifact file. |
| `✏️s/🔌️plugins/💠️lowpoly` | `semio-s-plugin-lowpoly` | 1 | Single engine file builds `export_formats`/`import_formats: Vec<MediaFormat>` (Glb/Obj/Stl export, Glb/Obj import) via fully-qualified `semio_framework_plugin::MediaFormat`. |

**Note on the two comment-only files** (`🗄️stdio`, `🎞️animate`): these two would NOT need any
code change for a `MediaFormat` deletion — they only need their doc comments revisited/reworded
once the type is gone (the animate comment literally already anticipates "W6 deletes outright").
The other 10 plugins have real enum/type usage (list literals at minimum; `📐️cad` and
`🏭️process`/`🪐️space` also have function signatures, match arms, or direct variant bindings) and
are the actual migration surface if this wave proceeds to delete `MediaFormat`.

## Framework/OS files — 10 files (deletion targets + call sites)

| File | Occurrences | Role |
|---|---|---|
| `🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs` | 48 | **Definition site** — the `MediaFormat` enum itself (per W0, at line 816), its `ArtifactCodec<T>` trait, exporter/importer impls, and the DWG/OBJ/GLB/STL codec bodies live here. |
| `🧰️framework/🔨️modules/🚪️io/🦀️component.rs` | 1 | IO module glue referencing `MediaFormat`. |
| `🧰️framework/🔨️modules/🧊️3d/📐️brep/🧰️kernel/🦀️component.rs` | 18 | BREP kernel — STEP/OBJ/STL export code paths keyed on `MediaFormat`. |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` | 18 | Manifest module — artifact-kind manifest entries referencing `MediaFormat`. |
| `🧰️framework/🛍️products/💻️os/🦀️component.rs` | 70 | OS product root — heaviest framework/OS user; includes `registry_export_media`/`registry_import_media` call sites plumbing `MediaFormat` end to end. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🦀️component.rs` | 1 | OS `run` module — single reference. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | 5 | OS `plugin` module — plugin-registration surface for `MediaFormat`-bearing `ArtifactKindSpec`s. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️component.rs` | 4 | OS `workflow` module — workflow-level references. |
| `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs` | 72 | OS host — second-heaviest user; the neutral-model / `STDIO_FORMAT_CATALOG`-adjacent host wiring plus `registry_export_media`/`registry_import_media` call sites live here alongside the OS product root. |
| `🧰️framework/📦️packages/🦀️rust/📦️glue.rs` | 1 | Framework crate-root glue — the public `pub use ... MediaFormat` re-export. |

## Totals

- Plugin files: 22 (across 12 plugin crates)
- Framework/OS files: 10
- **Grand total: 32 files, 307 occurrences**
- Of the 22 plugin files, 20 have real `MediaFormat` usage (enum literals, function
  signatures, match arms, or direct variant bindings); 2 (`🗄️stdio`, `🎞️animate`) have
  comment-only hits.

## Delta vs W0 (55 files / 346 lines)

W5a and W5b's plugin migrations removed real usage from most plugins that had it at W0 time.
What remains at W6 is: the framework/OS definition + call-site core (unchanged in role, still
10 files), plus a smaller set of plugins (12, down from a larger W0 set) that W5 explicitly
deferred — either because they were told they could keep `MediaFormat` as a public-facing enum
for now, or because their only remaining trace is a doc comment acknowledging the type is legacy
and slated for this wave's deletion.
