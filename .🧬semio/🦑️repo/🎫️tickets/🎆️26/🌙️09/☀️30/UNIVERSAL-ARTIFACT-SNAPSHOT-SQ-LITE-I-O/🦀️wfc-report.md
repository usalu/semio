# WFC Snapshot SQLite Execution

The five assigned owners are grid2d, grid3d, bitmap, wfc2d and wfc3d at standard 1, wildcard subset. Their native snapshots are persisted authored problems; solver output, contradictions and entropy remain inference fields and must not be inserted. Every native field must retain its own scalar or typed entity relation.

Bitmap has schema, full unsigned 64-bit seed, input extent, ordered palette with unsigned 32-bit RGBA channels, exact native palette-index base64 string, output extent/periodicity, model pattern-size/symmetry/input-periodicity/optional ground palette index and ordered pinned pixel coordinates/color. The exact owned string is retained as one named scalar; it is not an object serialization or a codec fallback. Relational palette and pin entities are independently queryable.

Grid2d has explicit bitmap/vector/image media; grid3d has explicit flat position/index mesh data and typed child references. Irregular wfc2d/wfc3d add ordered named slots, named slot edges and optional relation-specific tile rules. Grid2d/Grid3d/Wfc2d colors use u32; Wfc3d colors use u8. Floating point fields require exact per-field IEEE companions, seed requires canonical unsigned decimal text. Existing native TypeScript seed number representations need coherent unsigned64 owned models before SQLite mirrors.

The initial bitmap SQL and shared fixture are authored before its provider. New native laws will probe full native state, independent SQLite query/edit, actual owner capability, limits/cancellation and malformed relational ownership. Other owner schemas follow their actual native fields and type widths independently.
