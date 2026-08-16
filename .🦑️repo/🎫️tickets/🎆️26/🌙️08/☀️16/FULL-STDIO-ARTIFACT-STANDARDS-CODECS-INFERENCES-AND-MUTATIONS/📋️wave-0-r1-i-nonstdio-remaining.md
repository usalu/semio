# Wave 0 R1-I Nonstdio Remaining

## Completed

The five assigned artifact declarations are explicit, fallible leaf definitions:

- `🌀️procedural` — `s.procedural2d` and `s.procedural3d`
- `🪐️space` — `s.home`
- `🌍️gis` — `s.gismap` and `s.gisterrain`

Each definition owns literal schema, inference, native-composer, standard-composer, document-codec,
and English/German localization capabilities. The native composer capability is now declared beside
its standard-format capability leaves.

Removed from all five roots:

- runtime capability-row loops and string-kind parsing;
- `OnceLock`/`Vec` language construction and `dsl::passthrough_hooks`;
- grammar capability claims that had no callback-free language materialization;
- root-level `io_registry` aliases, lookup dispatch, and imperative registration wrappers.

The typed `schema`, `inferences`, `composers`, and `document_codec` declaration facets remain the
only active attachment paths for these roots.

## Typed Contribution Blockers

| Owner | Imperative residue | Required typed surface |
| --- | --- | --- |
| `🌀️procedural` | `register_dwg_mesh_bridge` | Owned OS media-bridge declaration keyed by artifact kind and stable handler identity. |
| `🌀️procedural` | `ensure_linked_flow_extensions` | Owned linked-flow extension-installer declaration keyed by extension domain and owner. |
| `🪐️space` | `register_s_exports` for `SpaceApp` / `os.space` | Foreign document-codec contribution: app-owned codec keyed by a framework-owned document schema. |
| `🌍️gis` | `gismap::io::register_host_io` | Owned OS 2D media-handler declaration for the map kind. |
| all five declarations | language rows | Callback-free literal grammar/language facet. The current `LanguageSpec` requires `IdiomHooks` function pointers, so preserving those rows would retain passthrough callbacks. |
| `💡️reasoning`, `🎬️sequence`, `✒️writer`, `🏭️process`, `🌊️flow`, `🎞️animate`, `🌿️vcs`, `🏛️architect` | aggregate app `register()` callbacks | Their raw artifact declarations are not present in this lane; each needs typed schema, inference, composer, language, and codec contributions before its aggregate callback can be removed. |

`ArtifactApp::app_schema()` is already a typed contract, but the remaining affected apps still use
the default `None` and their config leaves expose only void `register_app_schema()` functions. Those
registrars remain until each app supplies an `AppSchemaDescriptor` through its `ArtifactApp` facet;
removing them now would drop registrations.

## Verification

- Ran `rustfmt` on exactly the five assigned artifact-root Rust files.
- Source-only scan confirms those files contain no declaration-level capability loops, `OnceLock`,
  `passthrough_hooks`, root `io_registry`, `.languages(...)`, or runtime `.as_bytes()` rows.
- Cargo and Nx checks were intentionally not run: the serial build gate is owned by the coordinating
  lane.
