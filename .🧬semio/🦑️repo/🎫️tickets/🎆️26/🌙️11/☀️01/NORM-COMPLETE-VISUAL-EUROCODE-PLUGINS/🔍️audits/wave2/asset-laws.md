# Asset laws, wave 2

Nothing compiled or run (no cargo, no build). Every Rust edit is unverified by the compiler; every asset edit is verified only by the static scripts below.

## 1. Tests (new, one module per artifact)
`✏️editor/🧪️tests/📚️assets/🦀️.rs` in en1990 to en1999, mounted as `mod asset_laws` (cfg(test)) right after `unit_tests` in each `✏️editor/🦀️.rs`. Generated from `🛠️asset-laws/gen_tests.py`. The editor crate is the one place that already depends on io, scene, model, examples, structural-scene and compliance, so no Cargo.toml changed. Five tests per artifact plus one for the demo session:
- `every_asset_is_classified_and_in_the_palette_with_both_titles`: the palette is `En19xxPlayApp::examples()` (what the editor shows). Palette ids unique; EN and DE title non-blank; every file under `🖼️assets` is a `dsl.semio`, `pack.semio` or `.json`; each committed DSL equals the text of the palette example with its id; a DSL without a palette example fails unless it is in `FIXTURE_ONLY` (only en1990 `default-office`, used by `compliance-report` through `include_str!`).
- `every_palette_document_round_trips_byte_identically_through_the_dsl_codec`: `decode_en19xx_dsl` (the artifact's own codec) then `encode_en19xx_dsl` must give the identical bytes, and decode(print) equals the document. Covers asset-backed and code-encoded palette entries (full catalogue, low-seismicity, and so on).
- `every_bundled_asset_evaluates_projects_and_obeys_the_projection_laws`: decode, `NormFamily::evaluate`, live scene and full scene through the editor viewport functions, `diagrams`, handles for every entity, then `assert_projection_laws` on both scenes.
- `every_bundled_asset_emits_traced_checks_and_the_trace_has_no_structural_defect`: per asset at least one emitted check id belongs to a traced spec row; over all assets `spec_trace::problems` may only contain coverage gaps. Full `assert_traced` is not possible from assets alone: the existing trace tests need extra synthetic documents (en1992 `reach_tests`, en1994 `detailed`/`punched`, en1999 `wave5_document`) that are private to their schema crates. The strict `assert_traced` stays in those tests.
- `every_pack_and_json_twin_equals_the_dsl_document_of_its_example`: `decode_pack` equals the DSL document and `encode_pack` returns the identical bytes; JSON twin decodes to the same document; at most one pack per example.
- `the_demo_session_asset_is_this_artifacts_command_script_with_both_titles`: `.cmd.semio` header and EN/DE title. The demo session is a command script, not a snapshot, so it is deliberately not a palette document.

## 2. Static pre-check and hand fixes
Scripts (ticket only, nothing left in the repo): `🛠️asset-laws/dslmodel.py` (parses the Rust model: struct fields in order, `#[dsl(unit)]`/`#[dsl(table)]`, enums, `impl Default` literals), `patch.py` (dry run lists stale fields; `--apply` rewrites), `jsonpatch.py`, `jsoncheck.py`, `precheck.py` (older check against the schema JSON), `inventory.py`, `rust_vs_json.py`, `gen_tests.py`. Run logs: `precheck-before.txt`, `patch-run1.txt`, `precheck-after.txt`. Originals of every touched asset: `🗑️generated/asset-backup/`.

Finding that shaped the method: the generated `🧬️schema/📸️snapshot/🔣️.json` is already stale against the Rust model in en1991 (93 properties but different set and order), so the Rust struct is the ground truth, not the JSON. After the fix a dry run of `patch.py` reports 0 changed assets (idempotent, 33 of 33 canonical against the Rust model).

All 33 DSL assets were stale. Fixed by hand-equivalent edits (header columns and cells in model order, new collections, enum spelling, number canonical form):
- en1990 (8 DSL): members +13 columns (frame/storey/material/dimension/alpha_cr/second order), bridge-sls +8, empty `placements`, `cases`, `factor-overrides`.
- en1991 (2): +12 scalars (`bridge-tandem-x`, `rail-alpha`, rail and crane and silo values taken from `impl Default`, so rail-alpha 1, bridge-tandem-x 10, crane-x 15, silo-diameter 6 and so on), +11 empty collections.
- en1992 (5): members +5 (`relative-humidity` 0.5 as in the code fixtures, bars, section-points, tendon-points, stud-rings), anchors `head-diameter` (0.03 for the liquid-retaining anchor as in `liquid_retaining_fem_anchor`, else 0), `placements`, `cases`.
- en1993 (2): the files were damaged and non-canonical: stray `"" 0.0 0.0 0.0 0.0` tokens after `built-up-members` and a 5-cell partial row in `weld-groups` (dropped), and every number printed `4.0`/`3.923e-05` where the printer (`format!("{value}")`) writes `4`/`0.00003923`. Canonicalised, plus empty `placements`.
- en1994 (3): beams +10 (`patch-bearing-length-m` 0.1m and `cracked-length-fraction` 0.15 as in the code beam), columns +6, `placements`.
- en1995 (4): empty `placements`, `tapers`, `cutouts`, `fasteners`, `panels`, `floors`.
- en1996 (2): empty `placements`, `cases`, `chases`, `movement-joints`.
- en1997 (3): `placements`, `cases`.
- en1998 (4): `placements`.
- en1999 (2): materials `product-form` (extrudedProfile as in the code), members +4, shells `welded` and `fabrication-class` B, empty `placements`, `weld-runs`, `outline-vertices`, `fatigue-categories`, `patch-loads`, `ring-stiffeners`; `support` spelled as the enum now is (`simply-supported`).
- Nested records and lists in rows are re-printed canonically (`{k=v ...}`, `[ {..} ]`); the two nested block forms (en1996 `walls[].load-cases`, en1998 `buildings[].storeys`) are left byte-for-byte.
- en1999 JSON twins: added the missing keys (`weldProcess`, `edgeStiffenerDepth`, `bearingLength`, `placements`, `weldRuns`, and so on) in model order; en1993 `snapshot.json` twins already match the model and were not touched.

## 3. Palette and orphans
- All 33 DSL assets are palette examples with EN and DE titles, except en1990 `default-office`: a compliance-test fixture, declared as `FIXTURE_ONLY`, not offered to users. A palette entry for it needs a new example module in the owner's tree (`📚️examples` mount plus `examples()` list); recommended, not done because it is outside assets and test modules.
- Removed duplicate packs: en1995 four shallow `<ex>/🎒️.pack.semio` (byte-identical to the nested copies), en1996 shallow `loadbearing-wall` pack (differed from the nested copy; the nested sibling of the DSL is kept). Renamed en1991 and en1997 `📦️.pack.semio` to `🎒️.pack.semio` (fixture-sweep convention).
- `📚️examples/<ex>/🖼️assets/*.op.semio`/`*.spr.semio` (8 artifacts) are protocol fixtures found by the stdio fixture sweep by convention, not by code; left alone.
- `.cmd.semio` demo sessions are tested but intentionally not documents.

## 4. Expected first failures after the rebuild (cannot be fixed without cargo)
- Packs are binary encodings and cannot be hand-made. They are likely stale after the model changes; the pack test will name them. Regenerate with `encode_en19xx_pack(decode_en19xx_dsl(dsl))`. Clue: en1997 `demo` and `compliant` packs are byte-identical (md5 64eeb7aa) while their DSL differ; en1996 `loadbearing-wall` had two differing copies.
- Byte-identical round trip: new cells for enum-typed and `Option` columns follow the printer rules read from the framework source (`format!("{value}")`, kebab-case variants, `_` for `None`); a printer rule I mis-read shows up as a diff in `print(decode(asset))`.
- Scene laws can expose defects in the code projections for the 33 documents (handles with empty `writes`, English in a German slot).
- en1990 `seismics[].importance-class` (`ii`) and en1992 enum spellings were normalised to the Rust variants; the old JSON-schema checker (`precheck.py`) still lists them as false positives because the schema JSON spells variants in camelCase.

## 5. Files
Edited: ten `✏️editor/🦀️.rs` (3-line mount), 33 `🗣️.dsl.semio`, two en1999 `🔣️.json`, packs as in section 3; created ten `✏️editor/🧪️tests/📚️assets/🦀️.rs`; appended "- ready for rebuild: asset laws" to `🗑️generated/build-en1990..en1999/latest.md`. No git writes.
