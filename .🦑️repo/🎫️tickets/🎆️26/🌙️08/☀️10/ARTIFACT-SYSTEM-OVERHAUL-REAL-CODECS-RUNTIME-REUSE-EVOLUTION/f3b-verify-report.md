# F3b Verification Report — svg, jpg, tiff

Independent re-verification of the F3b fan-out (svg, jpg, tiff). All checks below were performed
directly against disk and by re-running `cargo test`, not by trusting the agents' self-reports.

## Method

For each artifact:
1. Ran `cargo test -p semio-s-plugin-stdio --lib "artifacts::<module>"` and recorded the real
   pass/fail counts from the test harness output.
2. Grepped `🧬️schema/🔺️diff/🦀️component.rs` for `impl DiffAlgebra` (must be present) and
   `snapshot: Option<` on an actual struct field (must be absent — only doc-comment mentions of
   the old rejected pattern are acceptable).
3. Confirmed a `field_sweep`-named test exists (svg: `field_sweep`; jpg/tiff:
   `field_sweep_covers_every_mutable_field`).
4. svg only: grepped diff + mutations files for the apply-and-capture shape (`base.clone()` →
   `apply_svg_mutation` → snapshot-diffing helper) and read the `Mutation::diff()` match arms in
   full.
5. jpg/tiff/png: grepped all three snapshot/diff files for `RasterImage` and compared struct names
   to confirm none of the three still share a common raster type.
6. Ran the full crate test suite once at the end.

## Per-artifact results

### svg
- Module path: `artifacts::svg` (standard `v1_1`, subset `any` schema files; also `basic`/`tiny`
  subset dirs exist from the separate subset-multiplicities ticket, present and green).
- Test run: **58 passed, 0 failed** (825 filtered out — this is the svg-scoped slice of the whole
  crate suite).
- `impl DiffAlgebra<SvgSnapshot> for SvgDiff` present at
  `🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:260`.
- No `snapshot: Option<SvgSnapshot>` field on `SvgDiff` (line 24) or any nested diff struct
  (`SvgElementDiff`, `SvgAttributesDiff`, `SvgChildrenDiff`). The only occurrence of the string
  `snapshot: Option<` in the file is inside a doc comment explicitly stating the full-replace slot
  is absent (line 19).
- `field_sweep` test present and passing
  (`artifacts::svg::standards::v1_1::subsets::any::schema::mutations::component::tests::field_sweep`).
- **Apply-and-capture check (svg's headline defect): confirmed genuinely gone.** Read the full
  `impl Mutation<SvgSnapshot> for SvgMutation::diff()` match (mutations component.rs lines
  126-211). Every variant except `SetSnapshot` constructs its `SvgDiff` directly from the
  mutation's own fields (`diff_at_path`, `attribute_diff_at_path`, or a literal `SvgDiff { .. }`)
  — none of them clone `base`, call `apply_svg_mutation` on the clone, and diff the result. The one
  call to a diffing helper, `SvgMutation::SetSnapshot { snapshot } => diff_set_snapshot(base,
  snapshot)`, is a direct recursive tree-diff between the pre-existing `base` and the
  mutation-supplied target `snapshot` — no `apply()` call is involved, so it is not the
  banned simulate-then-compare pattern; it mirrors xml/F1's recursive-tree-diff precedent for a
  wholesale "replace with this document" op. `apply_svg_mutation` calls do appear throughout the
  file, but only inside `#[cfg(test)]` round-trip-law tests (`mutation_diff_law`,
  `inverse_law`, `between_roundtrip_law`, etc.), never inside `diff()` itself.
- `apply_and_capture_confirmed_gone: true`

### jpg
- Module path: `artifacts::jpg` (standard `v_jfif_1_01`, subset `any`/`baseline`).
- Test run: **29 passed, 0 failed** (854 filtered out).
- `impl DiffAlgebra<JpgSnapshot> for JpgDiff` present at
  `📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:803`.
- No `snapshot: Option<` field on `JpgDiff` (struct at line 699) or any nested diff struct
  (`JpgComponentDiff`, `JpgComponentsDiff`, `JpgFrameFieldsDiff`, `JpgQuantTableDiff`,
  `JpgQuantTablesDiff`, `JpgHuffmanTableDiff`, `JpgHuffmanTablesDiff`, `JpgSegmentDiff`,
  `JpgOtherSegmentsDiff`). The two textual hits are doc comments (module-level line 3 and the
  `JpgDiff` doc line 694) explicitly stating the full-replace template was rejected.
- `field_sweep_covers_every_mutable_field` test present and passing.
- `RasterImage` check: the identifier `RasterImage` does not appear anywhere in
  `JpgSnapshot`/`JpgDiff` code — only in two doc comments explaining that the former shared
  `RasterImage{width,height,rgba}` stub was replaced by JPG's own real JFIF/SOF/DQT/DHT-typed
  model. `JpgSnapshot`'s own struct fields (`JfifThumbnail`, `JpgFrameComponent`,
  `JpgFrameHeader`, `JpgScanComponent`, `JpgQuantTable`, `JpgHuffmanTable`, `JpgSegment`) are all
  jpg-specific named types, not shared with png or tiff.

### tiff
- Module path: `artifacts::tiff` (standard `v6_0`, subset `any`/`baseline`).
- Test run: **29 passed, 0 failed** (854 filtered out).
- `impl DiffAlgebra<TiffSnapshot> for TiffDiff` present at
  `🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs:393`.
- No `snapshot: Option<` field on `TiffDiff` (struct at line 352) or nested diff structs
  (`TiffTagsDiff`, `TiffIfdsDiff`). Both textual hits of the string are doc comments (module-level
  line 3, and `TiffDiff`'s own doc line 347) stating the full-replace template was rejected.
- `field_sweep_covers_every_mutable_field` test present and passing.
- `RasterImage` check: only appears in a module-level doc comment noting the old shared
  `RasterImage{width,height,rgba}` stub was replaced with TIFF's real generic tag/type/value IFD
  model. `TiffSnapshot`'s actual fields (`TiffTag`, `TiffIfd`) are tiff-specific, not shared with
  jpg or png.

## Cross-check: png shares nothing with jpg/tiff

`png`'s snapshot file (`📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`)
also only mentions `RasterImage` in a doc comment, and its real struct is `PngSnapshot` built from
`PngRgb`, `PngChromaticities`, `PngPhysicalDims`, `PngTimestamp`, `PngTextChunk`, `PngChunk` — none
shared with `JpgSnapshot` or `TiffSnapshot`. All three artifacts now have fully independent,
uniquely-named raster/image snapshot types; the formerly-shared `RasterImage` stub is confirmed
gone from all three.

## Full crate suite

`cargo test -p semio-s-plugin-stdio --lib` (whole crate, no filter):

```
test result: ok. 883 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.52s
```

0 failures crate-wide, consistent with the individually-filtered svg/jpg/tiff runs (58 + 29 + 29 =
116 of the 883, all passing) plus the pre-existing 767 from prior waves.

## Verdict

All three F3b agents' self-reports are corroborated by direct inspection: `impl DiffAlgebra`
present, full-replace `snapshot: Option<...>` slot genuinely absent from every diff struct (only
doc-comment negations remain), `field_sweep`-named tests present and passing, svg's apply-and-capture
defect is confirmed fixed (every non-`SetSnapshot` variant builds its diff directly; the one
diffing-helper call is a legitimate base-vs-target recursive tree-diff, not simulate-then-compare),
and jpg/tiff/png all now have distinct, non-shared raster snapshot types. No discrepancies found.
