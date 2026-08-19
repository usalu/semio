# 📓️ mesh-io STL ASCII export investigation

## Trigger

Z1 zero-warnings cleanup flagged `StlFormat::Ascii` in `mesh-io/🦀️component.rs` as dead code (suppressed with `#[allow(dead_code)]`).

## Production STL export paths

| Path | Format | Mechanism |
|---|---|---|
| Brep kernel `export_stl_sync` | Binary | `export_solid_stl(..., StlFormat::Binary)` → `mesh_to_stl` |
| Flow `ExportStl` operator | Binary | `kernel.export_stl` → same as above |
| Framework `brep-geometry` module | Binary | `guard.export_stl` → base64 wrap |
| CAD `export_solids_as` | Binary | `SemioMeshToStl` → `encode_stl_binary` (base64 in download) |
| Plugin compose `EXPORT_STL_DIALECT` | Text via artifact | `SemioMeshToStl` → `serialize_text` (dialect `s.stdio.stl/ascii`) |
| FEM/plugin STL serializers | ASCII bytes | `SemioMeshToStl` → `encode_stl_ascii` |

No command or UI surface passes a binary-vs-ASCII choice into `export_solid_stl` / `export_stl`.

## Duplicate ASCII implementation

`mesh-io::write_ascii_stl` duplicates the canonical stdio STL artifact codec:

- `🟪️stl/🏅️standards/🔖️ascii/…/🚪️io/🦀️component.rs` — `encode_stl_ascii` / `decode_stl_ascii`
- `SemioMeshToStl` serializer — mesh snapshot → `StlSnapshot` → grammar encoder

Import still needs ASCII detection in mesh-io (`read_ascii_stl`, `is_ascii_stl`) because `import_stl` auto-detects format on read.

## Decision

**Remove** `StlFormat`, `write_ascii_stl`, and the `#[allow(dead_code)]` — not wire them.

Rationale:

1. ASCII STL export is already a first-class artifact dialect (`s.stdio.stl/ascii`), not a brep-kernel concern.
2. Brep kernel / flow / CAD solid export intentionally emit binary STL (CAD base64-wraps binary bytes).
3. Threading format choice into `export_solid_stl` would duplicate the artifact standard/subset mechanism without any current consumer.

## Changes

- `export_stl` / `export_solid_stl` drop the `format` parameter; always binary.
- Remove `stl_ascii_round_trip_preserves_triangle_count` test (exercised only the deleted path).
- Keep binary export/import tests and ASCII **import** parsing.
