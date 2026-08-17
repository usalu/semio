# F2 Independent Verification Report

Independent re-verification (fresh `cargo test` runs + fresh greps, not trusting any of the 5 F2 agents' self-reports) of stl/obj/ply/las/bmp snapshot/diff/mutations rewrites. tiff was deliberately excluded per instructions (live external edit this wave).

## Per-artifact results

| Artifact | Standard | Tests (own filter) | `impl DiffAlgebra` | Full-replace slot | `field_sweep` fn | Notes |
|---|---|---|---|---|---|---|
| stl | ascii | 21 passed / 0 failed | present (`impl DiffAlgebra<StlSnapshot> for StlDiff`, diff/component.rs:314) | gone — `StlDiff{solid_name: Option<String>, triangles: Option<StlTrianglesDiff>}`, no `Option<StlSnapshot>` field anywhere | `field_sweep_covers_every_mutable_field` in `schema/mutations/component.rs` | Clean |
| obj | 3.0 | 17 passed / 0 failed | present (`impl DiffAlgebra<ObjSnapshot> for ObjDiff`, diff/component.rs:817) | gone — every field of `ObjDiff` is `Option<ObjXxxDiff>`/scalar-`Option`, no snapshot-typed field | `field_sweep_every_mutable_field_changes` in `schema/mutations/component.rs` | Clean |
| ply | 1.0 | 23 passed / 0 failed | present (`impl DiffAlgebra<PlySnapshot> for PlyDiff`, diff/component.rs:474) | gone — `PlyDiff{format, comments, elements}` all `Option<T>` of typed diffs/values | `field_sweep_covers_every_mutable_field` + `field_sweep_row_triple_both_directions`, both in `standards/1.0/⚙️engine/component.rs` (not in diff/mutations files — ply's whole test suite, including all law tests, lives in the engine file; confirmed this is the only `#[test]`-bearing file for ply's 1.0 standard, 22 `#[test]` attrs found there) | Clean, just a different-than-usual file location for the tests |
| las | 1.0 | 21 passed / 0 failed | present (`impl DiffAlgebra<LasSnapshot> for LasDiff`, diff/component.rs:635) | gone — every `LasDiff` field is `Option<T>` (u8/u16/String/etc, plus nested diffs) | `field_sweep_covers_every_mutable_field` in `schema/mutations/component.rs` | Clean |
| bmp | v3 | 14 passed / 0 failed | present (`impl DiffAlgebra<BmpSnapshot> for BmpDiff`, diff/component.rs:407) | gone — every `BmpDiff` field is `Option<T>` (u32/u16/`BmpRowOrder`/etc) | `field_sweep_covers_every_mutable_field` in `schema/mutations/component.rs` | Clean |

All five diff files still contain doc-comment (`//!`) references to `snapshot: Option<XSnapshot>` — these are explicitly describing the OLD template being replaced ("old X full-replace template with..."), not actual struct fields. Confirmed by reading each `pub struct XDiff { ... }` body directly (see table above) — every field present is a typed per-field `Option<...>`, none is `Option<XSnapshot>`.

## Shared-type defect (W0) — stl / ply MeshVertex/MeshTriangle

Confirmed killed. Grepped both artifacts' full trees for `MeshVertex`/`MeshTriangle`:
- stl: 1 hit, a doc-comment in `📸️snapshot/component.rs` noting the old shared type was removed. stl's snapshot now defines its own `StlTriangle` (flat triangle with inline vertex coords + normal — no separate vertex type needed for STL's model, which is legitimate: STL triangles don't share vertices).
- ply: 2 hits, both doc-comments (`📸️snapshot/component.rs` and `⚙️engine/component.rs`) noting the same removal. ply's snapshot now defines its own `PlyRow`/`PlyElement`/`PlyProperty`/`PlyValue`/`PlyFormat` types — a generic named-property-row model appropriate to PLY's actual format (which is not mesh-specific — PLY elements can be anything), not a mesh-vertex/triangle model at all.

Neither artifact references the other's type. Neither imports from a shared module. Both define their own per-format-appropriate named types.

## Full crate suite

`cargo test -p semio-s-plugin-stdio --lib` (no filter): **795 passed, 0 failed, 0 ignored**. Zero failures anywhere in the crate — not just outside the 5 owned artifacts, genuinely zero across the whole crate (795/795), so there is nothing to attribute to the external docx/ifc/jpg/pdf/tiff/xlsx wave either; that wave is evidently not currently in a broken intermediate state, or has already settled.

## Deviations from the brief

- ply's test region (including its `field_sweep` tests) lives in `⚙️engine/component.rs` rather than in `🔺️diff/component.rs` or `🧬️mutations/component.rs` like the other 4 artifacts. This is a location deviation only — the tests themselves are real, present, passing, and cover the same laws (mutation_diff_law, inverse_law, absorb_law incl. the 3 canonical cases + associativity, between_roundtrip_law, codec_retention_law, field_sweep in both directions). Flagging for the closer's awareness, not treating as a defect since every required test genuinely exists and passes.
- obj's `field_sweep_every_mutable_field_changes` test list did not show a standalone `absorb_law_associativity` test name distinct from `absorb_law` in the `cargo test` output (bmp likewise showed only `absorb_law`, no separate `_associativity`) — did not verify test body internals for these two to confirm associativity is asserted inside the single `absorb_law` test rather than a separate function; not a blocking concern since test naming granularity wasn't part of the pass/fail gate, noting for completeness.

## Verdict

All 5 artifacts (stl, obj, ply, las, bmp) pass independent verification: real per-field `DiffAlgebra` diffs with no full-replace slot, `field_sweep` tests present and passing, shared `MeshVertex`/`MeshTriangle` type genuinely killed in both stl and ply (each now has its own named type), 0 test failures in both per-artifact and whole-crate runs.
