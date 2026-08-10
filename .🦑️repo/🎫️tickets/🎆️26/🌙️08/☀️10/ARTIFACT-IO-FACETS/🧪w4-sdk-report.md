# 🧪 W4 SDK Report — Artifact Io Facets

Ticket `26/08/10/ARTIFACT-IO-FACETS`. Wave 4 only.

## 1. Plugin SDK traits

Added in `🔌️plugin/🦀️component.rs` (`//#region 🔖️ArtifactIo`):

- `IoFormatSpec { format, import, export }`
- `ArtifactImport` / `ArtifactExport` / `ArtifactIo`
- Re-exported from the plugin crate root
- `IoError` re-exported beside the traits

## 2. Coverage lattice

Replaced `required_os_media_*_formats` with:

```rust
pub enum MediaDirection { Import, Export }
pub fn required_media_formats(media_type: MediaType, direction: MediaDirection) -> Vec<MediaFormat>
```

Twins updated:

- `os/🦀️component.rs`
- `os/🖥️host/🦀️component.rs`

Import mirrors export; TwoD import always keeps `Dwg`; `Json` always included. Kit × Brep adds `step`/`ifc`.

`assert_os_media_export_coverage` / `assert_os_media_import_coverage` (and tests) use each descriptor's `media_type`. Legacy dimension×capability helpers removed.

## 3. `media_type` / `dimension` fills

| Artifact | dimension | media_type | kind id |
| --- | --- | --- | --- |
| architect `🏛️program` | data | Data × Value | `data.🏛️program` |
| energy `🔋️model` | data | Data × Value | `data.🔋️model` |
| space `🏠️home` | data | Data × Value | `space.shome` |
| trinity `♻️rewrite` | text | Text × Document | `text.♻️rewrite` |
| all 15 norm artifacts | data | Data × Value | `computation.norm.<variant>` via `app_surface::artifact_kind_spec` |

Norm kind ids kept (`computation.norm.*`) so `report:out` ports stay wired; dimension/media_class moved from compliance/Computation → data/Data for the IO lattice.

## 4. Follow-ups

- W5/W6 register `🚪️io` leaves; coverage asserts expand with the lattice until handlers land.
- Repo MCP unavailable in this session; work stayed in the existing ticket folder.

## 5. Compile checks run

- `semio-framework-plugin` — ok
- `semio-framework-os-kernel` — ok
- `semio-framework-os` (`os-host-full`) — ok (fixed duplicate MediaClass re-export + `base64::Engine` in host workflow module)
- `semio-s-plugin-{architect,norm,energy,trinity,space}` — ok
