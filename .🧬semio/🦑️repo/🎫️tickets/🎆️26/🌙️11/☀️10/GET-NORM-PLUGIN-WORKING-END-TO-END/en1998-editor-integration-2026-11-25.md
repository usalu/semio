# en1998 editor integration failures (2026-11-25)

Source log: `🗑️generated/cp-editor-en1998.log` (private copy). Nothing built here; fixes checked by reading.
Norm root `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/` (called S).

## 1. unit_tests::the_seismic_showcase_asset_decodes_to_the_showcase_fixture_...
Cause: the earlier fixture round added `fd-3`, `l7`, `iso-1`, `pb-1`, `cr-1` to `S/🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` and the rename `Annex B`, but the DSL asset `S/🖼️assets/🏢️seismic-showcase/🏢️seismic-showcase/🗣️.dsl.semio` got only the rename. A token diff of the left (asset) and right (fixture) Debug strings in the log shows exactly four differences: foundation `fd-3`, soil layer `l7`, and the `isolation_systems`, `pier_bases`, `critical_regions` tables empty in the asset.
Fix (asset, fixture is the truth): added the five rows with the fixture values (`fd-3` = the `fd-2` numbers on `bldg-timber-hall`; `l7 fd-3 4m 18000 120 6 5`; `iso-1` with building id `""`; `pb-1`; `cr-1`). Assertion untouched.
Side effect: the asset now evaluates with more rows (more checks and scene entities); no test pins its check count (earlier grep). `seismic-showcase-pass/fail` check-outcome fixtures are separate documents and unchanged.

## 2. unit_tests::every_handle_write_dimension_and_marker_binding_has_a_bilingual_typed_field_spec
Cause: `pierBases[].poissonRatio` (now bound by the pier base grips of the showcase) had an `Inferred` spec (`m!(en, de)`), refused by the law.
Fix: `S/✏️editor/🏷️field-meta/🦀️.rs`: `ratio` kind for `pierBases[].poissonRatio` and the same for `buildings[].members[].wallPoissonRatio` (same dimensionless ratio, also inferred).

## 3. tests_11::every_palette_document_obeys_the_remedy_law
Cause: `check_site_ground` (`S/🧬️schema/💡️inferences/🌍️full-seismic-check/🧩️building-details/🦀️.rs`) offered a `OneOf` remedy whose DE options are the DIN NA combinations of the derived letter. With the showcase's new liquefiable foundation `fd-3` the derived ground type has a letter with no DE combination (the NA only knows A, B, C), so options were empty but the remedy applicable.
Fix: engine: in that case the remedy is `Remedy::advisory` (not applicable, no options, EN and DE text saying the NA covers A, B, C only and a special study or input correction is needed). EN annex and DE letters A-C unchanged.

## Not touched
`history_edits_end_to_end` (framework), the live_acceptance `soilLayers` failures (other agent), `value/decode/owned`, `store/composition/open/operation/record`.

## Unverified / re-sync
Not compiled or run. Re-sync of the private copy needed. Risk: if the derived letter is in fact A-C yet options are empty, finding 3 has another cause (the letter filter); the log gives only the check id. Also the asset parse of `""` for a TEXT cell follows the existing `brg-a br-1 ""` row.

## Coordinator follow-up

### (1) Hand derivation of the ground letter (finding 3)
`check_site_ground`: `effective_v_s30(document)` takes the lowest v_s,30 of all foundations that have layers (Eq. 3.1 over 30 m; a shorter profile continues with its last layer), else the site value. `ground_type_table_3_1`: site has liquefiable_layer false, soft clay 0, alluvium 0 (so no S2, S1, E), then v_s,30 > 800 A, > 360 B, > 180 C, else D (registry rows `ground.vs-{a,b,c}-min`, C minimum 180 seen in `ndp/🗃️table/en1998/🦀️.rs:35`; boundary takes the lower class).
- fd-1: 3/180 + 4/150 + 5/260 + 18/420 = 0.016667 + 0.026667 + 0.019231 + 0.042857 = 0.105421, v_s,30 = 30 / 0.105421 = 284.6 m/s (the site's 285; class C).
- fd-2: 4/170 + 26/400 = 0.023529 + 0.065 = 0.088529, v_s,30 = 338.9 m/s (C).
- fd-3: `l7` 4 m at 120 m/s, continued to 30 m at 120: v_s,30 = 120 m/s (D).
Lowest governs: 120 m/s, 120 <= 180, so ground type D, letter 'D'. Stated class is C-T (DE) and the stated EN type C. So `ok` is false, and the DE option list is `["A-R","B-R","C-R","B-T","C-T","C-S"]` filtered by `starts_with('D')`: empty. The filter is correct (the NA has no D combination); no filter bug. Before `fd-3` was added the lowest was fd-1 (C), which is why the law passed.

### (2) What the user sees
The check itself fails (`CheckStatus::Fail`, since class is Some and differs from the stated one) with the explanation "v_s,30 = 120 m/s (soil layers of fd-3), N_SPT = 0, c_u = 0 kPa give ground type D; stated class C" (and the German text). The advisory remedy is shown with its text (not applicable, so no one-click apply, per `Remedy::advisory`): the DE NA covers A, B, C only; special study or input correction. So a failing check with a clear message and a visible hint, not a swallowed remedy. The showcase therefore fails `en1998.site.groundClass` in DE by design of its new soft fd-3 layer (it is a showcase of failures too: it already has failing rows); if the coordinator prefers a passing ground check, the alternative is to give `l7` a stiffer layer (v_s above 180), which changes the fixture, the DSL, the scene tests that read the liquefiable layer.

### (3) Twins of the showcase asset
`S/🖼️assets/🏢️seismic-showcase/` holds only `🏢️seismic-showcase/🗣️.dsl.semio`: there is no `.pack.semio` and no JSON twin of this asset (only `seismic-rc-frame` and the fail assets have packs; `find S -name '*.pack.semio'` shows `seismic-rc-frame/🎒️.pack.semio`, unaffected). Other embeds: the editor palette (`✏️editor/🦀️.rs:100`, `include_str!`), `📚️examples/🏢️seismic-rc-frame/🧪️tests/🧩️example/🦀️.rs:56` (decode only), the editor unit test, and the fixtures `🧪️tests/🧫️fixtures/🎪️showcase/🔣️.json` (the equal JSON, already holding the rows) and `🎬️scene/🧪️tests/🧫️fixtures/full_showcase.txt` (scene hash golden of `full_showcase()`, which was not changed by this task). Nothing is stale; the pack-twins agent has nothing to regenerate for the showcase.
