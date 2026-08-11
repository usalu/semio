# F5 Verification Report — xlsx / pptx / bcf / dwg (ac1018 + ac1024)

Independent verification of the 4 F5 fan-out agents. Nothing below is taken from
self-reports — every number comes from a fresh `cargo test` run against the live
tree, and every structural claim comes from a fresh `grep`/`Read` of the actual
files on disk, done in this session.

## Method

For each artifact:
1. `cargo test -p semio-s-plugin-stdio --lib "artifacts::<module-path>"` — real pass/fail count.
2. `grep` its `🔺️diff/🦀️component.rs` for `impl DiffAlgebra` (must be present) and
   `snapshot: Option<` inside the diff struct (must be absent as a full-replace slot).
3. `grep` for a test function named `field_sweep*`.
4. pptx only: confirm `PptxShape` is a real enum with real variants, not a flat/empty model.
5. dwg only: confirm ac1018's field count is unchanged from its frozen pre-wave shape, and that
   ac1024's `architectural.dwg` fixture tests still pass.
6. One full-crate `cargo test -p semio-s-plugin-stdio --lib` run at the end.

## Per-artifact results

### xlsx
- Tests: **41 passed / 0 failed** (`artifacts::xlsx` filter).
- `impl DiffAlgebra<XlsxSnapshot> for XlsxDiff` present (`🔺️diff/🦀️component.rs:701`).
- No `snapshot: Option<XlsxSnapshot>` full-replace slot — module doc at line 2 explicitly states
  this and the struct itself has no such field.
- `field_sweep` test present (`🧬️mutations/🦀️component.rs:637`).
- Full law set present in the test list: `mutation_diff_law`, `inverse_law`, `absorb_law`,
  `between_roundtrip_law`, `codec_retention_law`, plus real per-mutation tests
  (insert/remove/rename sheet, set/remove cell, shared-string mutations).
- Also exercises the sibling ✳️strict/✳️transitional subset composers/builders/analyzers (16
  tests) — all passing, confirming this wave didn't break the now-closed
  ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES sibling ticket's work.

### pptx
- Tests: **48 passed / 0 failed** (`artifacts::pptx` filter).
- `impl DiffAlgebra<PptxSnapshot> for PptxDiff` present (`🔺️diff/🦀️component.rs:1122`).
- No `snapshot: Option<PptxSnapshot>` full-replace slot.
- `field_sweep` test present (`🧬️mutations/🦀️component.rs:631`).
- **Shape tree genuinely reconstructed** — confirmed the W0-flagged defect is fixed.
  `PptxShape` (`📸️snapshot/🦀️component.rs:66`) is a real enum with 4 variants, each carrying
  real typed fields, not a flat/empty model:
  - `TextBox { text_frame: Vec<PptxParagraph>, position: PptxTransform }`
  - `Picture { blip_rel_id: String, position: PptxTransform }`
  - `Placeholder { kind: String, text_frame: Vec<PptxParagraph>, position: PptxTransform }`
  - `Other { xml: String }` (raw retention for graphicFrame/group/connector/unrecognized —
    documented, not silently dropped)
  A corresponding `PptxShapeDiff` enum (`TextBox`/`Picture`/`Placeholder`/`Replace{shape}`)
  exists in the diff module, matching variant-for-variant. Tests directly exercise this:
  `insert_then_remove_shape_apply_and_inverse`, `set_shape_text_and_position_apply_and_inverse`,
  `set_shape_text_on_picture_is_a_no_op` (a real semantic-correctness test — setting text on a
  Picture variant is a no-op, not a silent corruption), `decode_resolves_real_hand_built_package_
  with_shape_boundaries_and_position`, `decode_preserves_unmodeled_shape_kinds_as_other_verbatim`.
- Also exercises ✳️strict/✳️transitional subsets (18 tests) — all passing.

### bcf
- Tests: **16 passed / 0 failed** (`artifacts::bcf` filter).
- `impl DiffAlgebra<BcfSnapshot> for BcfDiff` present (`🔺️diff/🦀️component.rs:347`).
- No `snapshot: Option<BcfSnapshot>` full-replace slot. The one `Option<Option<...>>` hit at line
  238 (`pub snapshot: Option<Option<Vec<u8>>>`) is a per-viewpoint tri-state nullable field
  inside `BcfViewpointDiff` — the actual PNG bytes of one viewpoint's snapshot image, not a
  whole-artifact full-replace slot. Confirmed by reading the surrounding struct: it sits
  alongside `camera: Option<Option<BcfCamera>>` and `components: Option<Option<BcfComponents>>`,
  all documented as "weak (whole-value replaced, never sub-diffed) and tri-state nullable" —
  exactly the recipe's prescribed shape for a weak per-field entity, correctly named `snapshot`
  because that's BCF's own domain term for a viewpoint screenshot, not the schema-overhaul
  anti-pattern.
- `field_sweep` test present, but organized differently from the other 3 artifacts: bcf keeps
  its law tests inside `🏅️standards/🔖️2.1/⚙️engine/🦀️component.rs` (line 908) rather than under a
  separate `🧬️schema/🧬️mutations` test module — a structural choice, not a missing-coverage gap;
  the full law set is present in that same file: `mutation_diff_law`, `inverse_law`,
  `absorb_law`, `between_roundtrip_law`, `codec_retention_law`, plus `codec_round_trip`,
  `orthogonal_camera_round_trips`, `decode_of_encode_recovers_full_typed_model`.

### dwg (ac1018 + ac1024)
- Tests: **31 passed / 0 failed** (`artifacts::dwg` filter, both standards combined).
- `impl DiffAlgebra<DwgSnapshot> for DwgDiff` present in **both** standards
  (ac1018 `🔺️diff/🦀️component.rs:81`, ac1024 `🔺️diff/🦀️component.rs:244`).
- No `snapshot: Option<DwgSnapshot>` full-replace slot in either — the only textual hits are in
  each file's own module-doc comment, explicitly describing the anti-pattern they deliberately
  avoided ("...a real per-field patch —" cut off, contrasting with what they actually built).
- `field_sweep_covers_every_mutable_field` present in both
  (ac1018 `🧬️mutations/🦀️component.rs:349`, ac1024 `🧬️mutations/🦀️component.rs:436`).
- **ac1018 confirmed NOT expanded beyond its frozen scope.** Read the full `DwgSnapshot` struct
  (`ac1018/…/📸️snapshot/🦀️component.rs:11-39`): 6 fields total (`schema`, `version`,
  `maintenance_version`, `codepage`, `bytes`, `section_names`) — `section_names: Vec<String>`
  is a name-only list with an explicit doc comment: *"Opaque by design: ac1018 is a deliberately
  frozen legacy shim (Decision #5)... there is no honest `data` payload to carry per name — do
  not expand this."* This matches the STATUS.md V6 entry's description of ac1018 exactly (a
  thin wrapper that got a defaulted `sections`/`decode_status`-shaped fix only for compile
  parity, nothing more) — real diff/mutations layer was added on top of the unchanged field
  set, not decode parity brought up.
- **ac1024's `architectural.dwg` fixture still passes.** All 3 fixture tests pass:
  `real_decode_reaches_d2_with_every_named_section`, `real_decode_stays_lossless_on_reencode`,
  `fixture_is_real_ac1024_not_a_stub` (in `examples::architectural::architectural_tests`), plus
  the standard-level `real_fixture_d1_locates_every_named_section`,
  `real_fixture_d2_decompresses_every_section`, and
  `real_fixture_page_directory_matches_header_cross_check` (in
  `standards::v_ac1024::engine::tests`). D3-D5 remain honestly out of scope — no stubs found
  toward them.

## Full crate run

`cargo test -p semio-s-plugin-stdio --lib` (no filter):

```
test result: ok. 1013 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 7.46s
```

Zero failures crate-wide. This includes xlsx/pptx/bcf/dwg plus every other artifact in the
plugin (zip, gif, svg, gltf, png, jpg, etc.) — nothing in this wave's work broke a sibling
artifact.

## Summary table

| Artifact | Tests passed | Tests failed | DiffAlgebra present | Full-replace slot gone | field_sweep present | Notes |
|---|---|---|---|---|---|---|
| xlsx | 41 | 0 | yes | yes | yes | strict/transitional subsets unaffected |
| pptx | 48 | 0 | yes | yes | yes | PptxShape genuinely reconstructed (4 real variants), W0 defect fixed |
| bcf | 16 | 0 | yes | yes | yes | laws live in engine/component.rs, not a separate mutations test module — organizational only |
| dwg ac1018 | (of 31 combined) | 0 | yes | yes | yes | frozen scope confirmed unchanged (6-field snapshot, explicit "do not expand" doc comment) |
| dwg ac1024 | (of 31 combined) | 0 | yes | yes | yes | architectural.dwg fixture (D1/D2) still green |
| **Full crate** | **1013** | **0** | — | — | — | — |

All 4 F5 agents' self-reports are confirmed accurate under independent re-verification. No
discrepancies found between claimed and actual state.
