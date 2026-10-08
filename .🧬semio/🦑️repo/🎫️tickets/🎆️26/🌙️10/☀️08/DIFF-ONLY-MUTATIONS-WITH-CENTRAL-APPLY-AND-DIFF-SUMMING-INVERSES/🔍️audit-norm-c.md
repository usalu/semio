# 🔍️ Audit Norm C: Diff-Only Mutations In Norm Artifacts (Read-Only)

Read-only audit against `📋️design.md` (laws L1-L5, codes V1-V4). No source file was edited and no build or git-modifying command was run. Paths below are relative to `✏️s/🔌️plugins/📕️norm/🗿️artifacts/` unless stated, and `🏅️standards/🔖️1/🪆️subsets/✳️any/` is omitted.

Scope: `📇️iso16757`, `🏛️en1992`, `🧩️en1994`, `🌍️en1997`, `🏭️vdi3805`, `⚡️din18599`, `🪶️en1999` (158 `impl protocol::MutationKind` leaves; the 2 extra `MutationKind<` mentions are doc comments). `🪟️results` is NOT under `🗿️artifacts/`; it lives at `✏️s/🔌️plugins/📕️norm/🪟️results/` and contains one kind (`☑️change-selected-check-index`), audited separately below.

## Classification Rules Applied

- **V1-SNAPSHOT-DIFF**: diff leaf takes `let mut x = base.f.clone()`, mutates the clone, and writes the whole clone back into the diff. No kind uses `between(`, `from_snapshot` or a whole-artifact diff in its `diff` (0 hits per kind), so the clone-and-mutate pattern is the only snapshot-diff form in scope.
- **V1-GENERIC-DIFF**: diff writes a whole list, map or section field of its `XDiff` struct (not a scalar), per the design Rulings (Minimality: collections diff per id/index; `rename-`/`set-`/`move-` kinds that emit a whole record, list or sub-document are violations). Exception allowed by the Rulings: a `replace-<entity>` kind that sets one entity record. `ReplacePartNumberRule` is therefore NOT flagged. `ReplaceZones` and `ReplaceElements` replace whole lists and are still flagged. Judgement call: `Update*`/`Specify*`/`Change*` kinds that write a whole sub-document (e.g. `UpdateRenewables`) are also counted, because the Rulings do not name those verbs.
- **V2-RESTORE-INVERSE**: inverse writes back a whole list or section cloned from `base` (`base.f.clone()`).
- **V2-EMPTY-INVERSE**: an empty inverse for a state-changing mutation. None confirmed: 14 guarded-empty inverses (Remove/Retire on an absent target) all pair with a forward `mutation.target-missing` error, so they are rejected no-ops, not violations.
- **V2-DIFF-DERIVED-INVERSE**: none. No inverse calls `.diff(` or a diff helper.
- **V3-LEAF-APPLY**: leaf mutates a base clone (same kinds as V1-SNAPSHOT-DIFF, per the taxonomy's "mutates base clone" wording). No leaf calls `MutationDiff::apply` directly; the calls are in shared helpers (table below).
- **V3-HAND-MUTATION**: none. Zero hand-written `impl Mutation<P>` in scope; all enums use `dsl::Mutations`.
- **V4-LAW-UNTESTED**: no inverse round-trip test found at kind or artifact level. Systemic gap for all 158: no test sums inverse diffs with `absorb` (L3 Σ). See L3 section.
- **V5-ABSORB** (new in the Rulings, not in the original table): not triggered. The norm diffs have no keyed rows. Every `absorb` is per-field last-writer-wins over absolute values, which is sound for sequential composition (checked: 7 of 7 bodies are `take!` field replacement or the whole-artifact replace).
- The design file changed on disk during this audit (a `## Rulings` section was added: Minimality, Absorb soundness, Generic seams deleted). This report applies the Rulings.

## Per-Artifact Summary

| Artifact | Kinds | V1-SNAPSHOT-DIFF | V1-GENERIC-DIFF | V2-DIFF-DERIVED-INVERSE | V2-RESTORE-INVERSE | V2-EMPTY-INVERSE | V3-LEAF-APPLY | V3-HAND-MUTATION | V4-LAW-UNTESTED | Clean |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 📇️iso16757 | 29 | 26 | 28 | 0 | 0 | 0 | 26 | 0 | 0 | 1 |
| 🏛️en1992 | 28 | 21 | 23 | 0 | 0 | 0 | 21 | 0 | 0 | 5 |
| 🧩️en1994 | 25 | 19 | 19 | 0 | 0 | 0 | 19 | 0 | 0 | 6 |
| 🌍️en1997 | 20 | 14 | 14 | 0 | 0 | 0 | 14 | 0 | 0 | 6 |
| 🏭️vdi3805 | 19 | 15 | 17 | 0 | 1 | 0 | 15 | 0 | 0 | 2 |
| ⚡️din18599 | 19 | 1 | 10 | 0 | 9 | 0 | 1 | 0 | 18 | 0 |
| 🪶️en1999 | 18 | 8 | 17 | 0 | 8 | 0 | 8 | 0 | 0 | 1 |
| **Total in artifacts** | **158** | **104** | **128** | **0** | **18** | **0** | **104** | **0** | **18** | **21** |
| `🪟️results` (outside artifacts) | 1 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Note: V2-EMPTY-INVERSE and V3-HAND-MUTATION are 0 everywhere. `🪟️results` row: its one kind is V1-SNAPSHOT-DIFF (see violating rows), so the Clean column there is 0 and the V1-SNAPSHOT count is 1.

## Shared Generic Helpers

No shared generic `diff` or `inverse` helper exists in scope: all 158 kinds delegate to their own `super::diff::diff` and `super::inverse::inverse` (contrast the reference `drawing_selection_diff`/`drawing_selection_inverse`). The shared helpers that do exist are below.

| Helper | file:line (under `🗿️artifacts/…`) | Callers | Violation |
|---|---|---:|---|
| `apply_<art>_mutation(base, mutation)` (x7) | `🧬️schema/🧬️mutations/🦀️.rs` iso:257, en1992:148, en1994:302, en1997:221, vdi3805:197, din18599:185, en1999:112 (`diff` then `MutationDiff::apply`) | 0 non-test callers (definition only); used only by 2 test files each | V3-LEAF-APPLY (artifact-local apply bypasses the central `protocol::apply_diff`; L4/L5) |
| `ArtifactBuilder::mutate` / `absorb` in `🚪️io/🦀️.rs` (x7) | `🚪️io/🦀️.rs` iso:141, vdi3805:143, others ~112 (`<XDiff as MutationDiff>::apply`) | framework builder trait | V3-LEAF-APPLY-like (builder applies diffs directly; not a leaf, but outside the central applier) |
| `XMutation::from_snapshot(base, target)` (x7) | `🧬️schema/🧬️mutations/🦀️.rs` iso:145, en1992:102, en1994:97, en1997:85, vdi3805:128, din18599:83, en1999:76 | editor `set-snapshot` command (1 each) + tests | V1-SNAPSHOT-DIFF adjacent: compare-and-emit whole-snapshot projection (`if base.f != target.f`), a `between`-like generic generator |
| `inverse_<art>_mutation(mutation, base)` (x7) | `🧬️schema/🧬️mutations/🦀️.rs` iso:265, en1992:151, en1994:306, en1997:225, vdi3805:205, din18599:189, en1999:116 | not counted | none (thin wrapper over `Mutation::inverse`) |
| `XDiff::diff_set_snapshot(snapshot)` (x7) | `🧬️schema/🔺️diff/🦀️.rs` (one per artifact) | 0 (dead: only the definition) | V1-SNAPSHOT-DIFF capability: whole-artifact diff constructor; not used by any kind |
| `XDiff::apply_to_artifact(&self, artifact)` (x7) | `🧬️schema/🔺️diff/🦀️.rs` (one per artifact) | 0 non-definition callers in norm | V3-LEAF-APPLY-like second apply path bypassing the trait; dead in norm |
| `extract_dn(attributes)` | vdi3805 `🧬️schema/🧬️mutations/🦀️.rs:36` | 1 (`change-product-configuration` diff, inside a base-clone mutation) | pure extractor; participates in V1-SNAPSHOT-DIFF of its caller |
| `catalog_index_entry_for(product)` | vdi3805 `🧬️schema/🧬️mutations/🦀️.rs:26` | 1 (`add-product` diff: pushes into a cloned index) | V1-SNAPSHOT-DIFF: derived index entry written into the diff |
| `din18599_climate_table_child(climate)` | din18599 `🦀️.rs:308` | 1 diff (`update-climate`) + schema and tests | V1-GENERIC-DIFF: derived child re-minted and written into the diff next to the whole `climate` section |
| `text_in(variants, locale)` | vdi3805 `🦀️.rs:41` | 5 (labels, inferences, tests) | none (not used in diffs or inverses) |

## Diff Types And Trait Implementations

Each artifact has exactly one artifact-level diff struct (`XDiff`) that all of its kinds return. Every field is `Option<...>`. `MutationDiff::apply` and `absorb` are hand-implemented in `🧬️schema/🔺️diff/🦀️.rs` for all seven. `DiffAlgebra` (defined in `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:155`, with `inverse`/`between`/`is_empty`) is implemented by none of the seven, and `🪟️results` does not implement it either. The framework has no `ApplyCapability` type (grep count 0), so L5 is not enforced by the type system.

| Artifact | Diff type | Whole-artifact field | Whole list/map/section fields | Scalar fields | `MutationDiff::apply` | `absorb` | `DiffAlgebra` |
|---|---|---|---|---|---|---|---|
| 📇️iso16757 | `Iso16757Diff` | `artifact` | catalogue, dictionary, geometry, selection, part_number_rule, part_number_inputs (map), script_limits, exchange_process (all 8) | none | yes | yes | no |
| 🏛️en1992 | `En1992Diff` | `artifact` | concrete_grades, reinforcement_grades, prestress_steels, members, anchors | annex, title, design_working_life_years, delta_c_dev, cement_type | yes | yes | no |
| 🧩️en1994 | `En1994Diff` | `artifact` | beams, columns, slabs | annex, structure_kind, steel_f_y_pa, fire_rating, insulation_thickness_m, fatigue_detail | yes | yes | no |
| 🌍️en1997 | `En1997Diff` | `artifact` | layers, footings, piles, retaining_walls, slopes, uplift_cases | structure_id, geotechnical_category, design_situation, design_approach, annex, groundwater_level, investigation_depth | yes | yes | no |
| 🏭️vdi3805 | `Vdi3805Diff` | `artifact` | manufacturer_file, catalog, edition_profile (map), index, geometry (map), curves (map), limits | correction_as_of, strict_mode | yes | yes | no |
| ⚡️din18599 | `Din18599Diff` | `artifact` | zones, elements, heating, dhw, ventilation, cooling, lighting, renewables, climate, climate_table | building_category, attachment, use_class, method, net_floor_area_m2, heated_volume_m3, geg_qp_factor, delta_u_wb_w_m2k, automation_class | yes | yes | no |
| 🪶️en1999 | `En1999Diff` | `artifact` | materials, sections, members, connections, fire_scenarios, fatigue_details, cold_formed, shells (all Vec) | annex | yes | yes | no |
| `🪟️results` | `NormResultsWindowConfig` (the snapshot itself is the diff) | n/a | n/a | `selected_check_index` | yes (returns `self.clone()`, ignores base) | yes (replace) | no |

The `artifact: Option<Box<XArtifact>>` field makes a whole-artifact replacement possible in all seven diff types (V1-SNAPSHOT-DIFF capability). No in-scope kind uses it. Under the Rulings' "generic seams are deleted" rule, the field, `diff_set_snapshot` and `apply_to_artifact` should be removed (0 callers).

## L3 Law-Test Coverage

- **Sequential inverse round-trip (apply each inverse step to the state, then compare to base)**: present for 140 of 158 kinds. 139 kinds have a per-kind test file (in the kind directory) that folds `inverse` over `apply` and asserts `restored == base`; `UpdateClimate` has a unit test that calls `inverse` and checks the round trip; `en1997`'s `every_variant_round_trips_via_inverse` covers its 20 kinds through the central `vcs::apply_mutation`. The 18 `din18599` kinds flagged V4 have no such test (some have unit tests that never invert): `UpdateRenewables`, `UpdateCooling`, `UpdateVentilation`, `UpdateLighting`, `SpecifyHeatingSystem`, `SpecifyDhwSystem`, `ReplaceZones`, `ReplaceElements`, `ChangeElementU`, `ChangeGegQpFactor`, `ChangeDeltaUWb`, `ChangeNetFloorAreaM2`, `ChangeHeatedVolumeM3`, `ChangeAutomationClass`, `ChangeBuildingCategory`, `ChangeUseClass`, `ChangeMethod`, `ChangeAttachment`.
- **`assert_mutation_inverse_law` helper**: called for 3 kinds only (`en1997` unit: `change-annex`, `change-footing-width`, `change-design-approach`).
- **`assert_mutation_diff_absorb_law` helper**: called for the same 3 `en1997` kinds, but with two FORWARD diffs `d1 = mutation.diff(base)`, `d2 = other.diff(base)`. It never sums inverse diffs.
- **Σ via `absorb` of inverse diffs (L3 as written)**: no test for any of the 158 kinds. This is the systemic V4 gap.
- `vdi3805`'s unit test comment (line 182) states the law helpers were deliberately not used; they are not called there.

## Violating Kinds

Location: primary violating construct. `diff` is the `🔺️diff/🦀️.rs` line of the base-clone mutation or whole-field write; `kind` is the `impl protocol::MutationKind` line (used for V4-only rows). Paths relative to `🗿️artifacts/` with the standards prefix removed.

| file:line | kind | codes | evidence |
|---|---|---|---|
| `📇️iso16757/🧬️schema/🧬️mutations/✂️retire-subject/🔺️diff/🦀️.rs:11` | RetireSubject (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.dictionary`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🌳️introduce-subject/🔺️diff/🦀️.rs:13` | IntroduceSubject (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.dictionary`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🎛️change-part-number-input/🔺️diff/🦀️.rs:11` | ChangePartNumberInput (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.part_number_inputs`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🎯️change-selection-class/🔺️diff/🦀️.rs:11` | ChangeSelectionClass (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.selection`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🏭️rename-manufacturer/🔺️diff/🦀️.rs:11` | RenameManufacturer (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🏷️introduce-product-class/🔺️diff/🦀️.rs:14` | IntroduceProductClass (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🏷️rename-product/🔺️diff/🦀️.rs:14` | RenameProduct (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/📇️rename-catalogue/🔺️diff/🦀️.rs:11` | RenameCatalogue (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/📐introduce-geometry-object/🔺️diff/🦀️.rs:14` | IntroduceGeometryObject (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/📐️introduce-property-definition/🔺️diff/🦀️.rs:13` | IntroducePropertyDefinition (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/📚introduce-product-series/🔺️diff/🦀️.rs:14` | IntroduceProductSeries (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/📦️introduce-product/🔺️diff/🦀️.rs:13` | IntroduceProduct (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🔄️change-exchange-process/🔺️diff/🦀️.rs:11` | ChangeExchangeProcess (iso16757) | V1-GENERIC-DIFF | diff writes whole `exchange_process` |
| `📇️iso16757/🧬️schema/🧬️mutations/🔌️remove-part-number-input/🔺️diff/🦀️.rs:11` | RemovePartNumberInput (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.part_number_inputs`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🔎introduce-product-index/🔺️diff/🦀️.rs:14` | IntroduceProductIndex (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🔒️add-selection-constraint/🔺️diff/🦀️.rs:11` | AddSelectionConstraint (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.selection`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🔓️remove-selection-constraint/🔺️diff/🦀️.rs:12` | RemoveSelectionConstraint (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.selection`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🗂️rename-product-group/🔺️diff/🦀️.rs:14` | RenameProductGroup (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🗑️retire-geometry-object/🔺️diff/🦀️.rs:14` | RetireGeometryObject (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🗑️retire-product-class/🔺️diff/🦀️.rs:14` | RetireProductClass (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🗑️retire-product-index/🔺️diff/🦀️.rs:14` | RetireProductIndex (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🗑️retire-product-series/🔺️diff/🦀️.rs:14` | RetireProductSeries (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🚦️change-script-limits/🔺️diff/🦀️.rs:12` | ChangeScriptLimits (iso16757) | V1-GENERIC-DIFF | diff writes whole `script_limits` |
| `📇️iso16757/🧬️schema/🧬️mutations/🚫️retire-product/🔺️diff/🦀️.rs:11` | RetireProduct (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🧵️change-selection-series/🔺️diff/🦀️.rs:11` | ChangeSelectionSeries (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.selection`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🧹️retire-product-group/🔺️diff/🦀️.rs:11` | RetireProductGroup (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🧺️introduce-product-group/🔺️diff/🦀️.rs:13` | IntroduceProductGroup (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `📇️iso16757/🧬️schema/🧬️mutations/🧽️retire-property-definition/🔺️diff/🦀️.rs:11` | RetirePropertyDefinition (iso16757) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalogue`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/#️⃣change-bar-layer-count/🔺️diff/🦀️.rs:6` | ChangeBarLayerCount (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/↔️change-member-width/🔺️diff/🦀️.rs:6` | ChangeMemberWidth (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/↕️change-member-height/🔺️diff/🦀️.rs:6` | ChangeMemberHeight (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/↘️change-action-vk/🔺️diff/🦀️.rs:6` | ChangeActionVk (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/⚓️insert-anchor/🔺️diff/🦀️.rs:9` | InsertAnchor (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.anchors`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/➕️insert-member/🔺️diff/🦀️.rs:9` | InsertMember (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/➖️remove-member/🔺️diff/🦀️.rs:10` | RemoveMember (en1992) | V1-GENERIC-DIFF | diff writes whole `members` |
| `🏛️en1992/🧬️schema/🧬️mutations/⤴️change-action-mk/🔺️diff/🦀️.rs:6` | ChangeActionMk (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/⭕change-bar-layer-diameter/🔺️diff/🦀️.rs:6` | ChangeBarLayerDiameter (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🌉️change-member-span/🔺️diff/🦀️.rs:6` | ChangeMemberSpan (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🌦️change-member-exposure/🔺️diff/🦀️.rs:6` | ChangeMemberExposure (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🏋️change-action-nk/🔺️diff/🦀️.rs:6` | ChangeActionNk (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/📍change-anchor-h-ef/🔺️diff/🦀️.rs:6` | ChangeAnchorHEf (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.anchors`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/📐️change-member-effective-depth/🔺️diff/🦀️.rs:6` | ChangeMemberEffectiveDepth (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🔀️reorder-members/🔺️diff/🦀️.rs:6` | ReorderMembers (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🔥change-member-axis-distance/🔺️diff/🦀️.rs:6` | ChangeMemberAxisDistance (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🔥️change-member-fire-rating/🔺️diff/🦀️.rs:6` | ChangeMemberFireRating (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🔩change-reinforcement-f-yk/🔺️diff/🦀️.rs:6` | ChangeReinforcementFYk (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.reinforcement_grades`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🗑️remove-anchor/🔺️diff/🦀️.rs:10` | RemoveAnchor (en1992) | V1-GENERIC-DIFF | diff writes whole `anchors` |
| `🏛️en1992/🧬️schema/🧬️mutations/🛡️change-member-cover/🔺️diff/🦀️.rs:6` | ChangeMemberCover (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🧱change-concrete-f-ck/🔺️diff/🦀️.rs:6` | ChangeConcreteFCk (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.concrete_grades`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🧷change-anchor-as/🔺️diff/🦀️.rs:6` | ChangeAnchorAs (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.anchors`, mutates the clone, writes whole field back |
| `🏛️en1992/🧬️schema/🧬️mutations/🪢change-member-stirrup-spacing/🔺️diff/🦀️.rs:6` | ChangeMemberStirrupSpacing (en1992) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/#️⃣change-beam-stud-count/🔺️diff/🦀️.rs:12` | ChangeBeamStudCount (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/↔️change-beam-transverse-as/🔺️diff/🦀️.rs:15` | ChangeBeamTransverseAs (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/↪️change-column-kind/🔺️diff/🦀️.rs:12` | ChangeColumnKind (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.columns`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/⛔️remove-column/🔺️diff/🦀️.rs:9` | RemoveColumn (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.columns`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/✂️change-beam-stud-spacing-m/🔺️diff/🦀️.rs:15` | ChangeBeamStudSpacingM (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/➕insert-slab/🔺️diff/🦀️.rs:9` | InsertSlab (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.slabs`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/➕️insert-beam/🔺️diff/🦀️.rs:9` | InsertBeam (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/➖remove-slab/🔺️diff/🦀️.rs:9` | RemoveSlab (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.slabs`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/➖️remove-beam/🔺️diff/🦀️.rs:9` | RemoveBeam (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/➗️insert-column/🔺️diff/🦀️.rs:9` | InsertColumn (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.columns`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/⬇️change-column-action-force-n/🔺️diff/🦀️.rs:18` | ChangeColumnActionForceN (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.columns`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/⭕️change-beam-stud-diameter-m/🔺️diff/🦀️.rs:15` | ChangeBeamStudDiameterM (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/🌀️change-beam-action-q-area-pa/🔺️diff/🦀️.rs:18` | ChangeBeamActionQAreaPa (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/💪️change-beam-stud-fu-pa/🔺️diff/🦀️.rs:15` | ChangeBeamStudFUPa (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/📏change-slab-thickness-m/🔺️diff/🦀️.rs:15` | ChangeSlabThicknessM (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.slabs`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/📏️change-beam-span-m/🔺️diff/🦀️.rs:15` | ChangeBeamSpanM (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/📐️change-slab-action-q-area-pa/🔺️diff/🦀️.rs:18` | ChangeSlabActionQAreaPa (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.slabs`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/🛠️change-beam-construction/🔺️diff/🦀️.rs:12` | ChangeBeamConstruction (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🧩️en1994/🧬️schema/🧬️mutations/🧱change-beam-slab-thickness-m/🔺️diff/🦀️.rs:15` | ChangeBeamSlabThicknessM (en1994) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.beams`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/↔️change-footing-width/🔺️diff/🦀️.rs:14` | ChangeFootingWidth (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.footings`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/⛰️change-slope-angle/🔺️diff/🦀️.rs:11` | ChangeSlopeAngle (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.slopes`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/➕insert-footing/🔺️diff/🦀️.rs:8` | InsertFooting (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.footings`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/➕️insert-layer/🔺️diff/🦀️.rs:8` | InsertLayer (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.layers`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/➖remove-footing/🔺️diff/🦀️.rs:8` | RemoveFooting (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.footings`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/➖️remove-layer/🔺️diff/🦀️.rs:8` | RemoveLayer (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.layers`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/⬇️change-footing-embedment/🔺️diff/🦀️.rs:11` | ChangeFootingEmbedment (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.footings`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/🌀️change-layer-oedometric-modulus/🔺️diff/🦀️.rs:11` | ChangeLayerOedometricModulus (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.layers`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/📏️change-pile-length/🔺️diff/🦀️.rs:11` | ChangePileLength (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.piles`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/📐️change-layer-phi-prime/🔺️diff/🦀️.rs:11` | ChangeLayerPhiPrime (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.layers`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/📤remove-pile/🔺️diff/🦀️.rs:8` | RemovePile (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.piles`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/📥insert-pile/🔺️diff/🦀️.rs:8` | InsertPile (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.piles`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/🔢change-pile-count/🔺️diff/🦀️.rs:11` | ChangePileCount (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.piles`, mutates the clone, writes whole field back |
| `🌍️en1997/🧬️schema/🧬️mutations/🧱change-wall-base-width/🔺️diff/🦀️.rs:11` | ChangeWallBaseWidth (en1997) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.retaining_walls`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/✂️remove-geometry-connection/🔺️diff/🦀️.rs:14` | RemoveGeometryConnection (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🎛️change-product-configuration/🔺️diff/🦀️.rs:17` | ChangeProductConfiguration (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalog`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🏭️change-manufacturer-file/🔺️diff/🦀️.rs:12` | ChangeManufacturerFile (vdi3805) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `manufacturer_file`; inverse restores whole `base.catalog.file` (`🏭️vdi3805/🧬️schema/🧬️mutations/🏭️change-manufacturer-file/↩️inverse/🦀️.rs:9`) |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🏷️rename-product/🔺️diff/🦀️.rs:15` | RenameProduct (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalog`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/📈️add-curve/🔺️diff/🦀️.rs:13` | AddCurve (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.curves`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/📉️remove-curve/🔺️diff/🦀️.rs:11` | RemoveCurve (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.curves`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/📍️change-curve-points/🔺️diff/🦀️.rs:15` | ChangeCurvePoints (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.curves`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/📐️resize-geometry/🔺️diff/🦀️.rs:22` | ResizeGeometry (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/📦️add-product/🔺️diff/🦀️.rs:15` | AddProduct (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalog`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🔌️add-geometry-connection/🔺️diff/🦀️.rs:16` | AddGeometryConnection (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🔖️change-edition-profile/🔺️diff/🦀️.rs:11` | ChangeEditionProfile (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.edition_profile`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🗑️remove-product/🔺️diff/🦀️.rs:11` | RemoveProduct (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.catalog`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🚧️change-limits/🔺️diff/🦀️.rs:11` | ChangeLimits (vdi3805) | V1-GENERIC-DIFF | diff writes whole `limits` |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🚮️remove-geometry/🔺️diff/🦀️.rs:11` | RemoveGeometry (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🧊️add-geometry/🔺️diff/🦀️.rs:13` | AddGeometry (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🧮️change-geometry-parameters/🔺️diff/🦀️.rs:15` | ChangeGeometryParameters (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.geometry`, mutates the clone, writes whole field back |
| `🏭️vdi3805/🧬️schema/🧬️mutations/🧹️remove-edition-profile/🔺️diff/🦀️.rs:11` | RemoveEditionProfile (vdi3805) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.edition_profile`, mutates the clone, writes whole field back |
| `⚡️din18599/🧬️schema/🧬️mutations/☀️update-renewables/🔺️diff/🦀️.rs:12` | UpdateRenewables (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `renewables`; inverse restores whole `base.renewables` (`⚡️din18599/🧬️schema/🧬️mutations/☀️update-renewables/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/⚖️change-geg-qp-factor/🦀️.rs:16` | ChangeGegQpFactor (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/❄️update-cooling/🔺️diff/🦀️.rs:12` | UpdateCooling (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `cooling`; inverse restores whole `base.cooling` (`⚡️din18599/🧬️schema/🧬️mutations/❄️update-cooling/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/🌉change-delta-u-wb/🦀️.rs:16` | ChangeDeltaUWb (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/🌡️change-element-u/🔺️diff/🦀️.rs:8` | ChangeElementU (din18599) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY, V4-LAW-UNTESTED | diff clones `base.elements`, mutates the clone, writes whole field back; no inverse round-trip test |
| `⚡️din18599/🧬️schema/🧬️mutations/🌦️update-climate/🔺️diff/🦀️.rs:16` | UpdateClimate (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `climate, climate_table`; inverse restores whole `base.climate` (`⚡️din18599/🧬️schema/🧬️mutations/🌦️update-climate/↩️inverse/🦀️.rs:10`) |
| `⚡️din18599/🧬️schema/🧬️mutations/🌬️update-ventilation/🔺️diff/🦀️.rs:12` | UpdateVentilation (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `ventilation`; inverse restores whole `base.ventilation` (`⚡️din18599/🧬️schema/🧬️mutations/🌬️update-ventilation/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/🎛️change-automation-class/🦀️.rs:16` | ChangeAutomationClass (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/🏠️change-building-category/🦀️.rs:16` | ChangeBuildingCategory (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/🏷️change-use-class/🦀️.rs:16` | ChangeUseClass (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/💡update-lighting/🔺️diff/🦀️.rs:12` | UpdateLighting (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `lighting`; inverse restores whole `base.lighting` (`⚡️din18599/🧬️schema/🧬️mutations/💡update-lighting/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/📐️change-net-floor-area-m2/🦀️.rs:16` | ChangeNetFloorAreaM2 (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/📦change-heated-volume-m3/🦀️.rs:16` | ChangeHeatedVolumeM3 (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/🔥specify-heating-system/🔺️diff/🦀️.rs:12` | SpecifyHeatingSystem (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `heating`; inverse restores whole `base.heating` (`⚡️din18599/🧬️schema/🧬️mutations/🔥specify-heating-system/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/🗺️replace-zones/🔺️diff/🦀️.rs:12` | ReplaceZones (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `zones`; inverse restores whole `base.zones` (`⚡️din18599/🧬️schema/🧬️mutations/🗺️replace-zones/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/🚿specify-dhw-system/🔺️diff/🦀️.rs:12` | SpecifyDhwSystem (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `dhw`; inverse restores whole `base.dhw` (`⚡️din18599/🧬️schema/🧬️mutations/🚿specify-dhw-system/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/🧩replace-elements/🔺️diff/🦀️.rs:12` | ReplaceElements (din18599) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE, V4-LAW-UNTESTED | diff writes whole `elements`; inverse restores whole `base.elements` (`⚡️din18599/🧬️schema/🧬️mutations/🧩replace-elements/↩️inverse/🦀️.rs:9`) |
| `⚡️din18599/🧬️schema/🧬️mutations/🧮change-method/🦀️.rs:16` | ChangeMethod (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `⚡️din18599/🧬️schema/🧬️mutations/🧱change-attachment/🦀️.rs:16` | ChangeAttachment (din18599) | V4-LAW-UNTESTED | no inverse round-trip test (kind or artifact level) |
| `🪶️en1999/🧬️schema/🧬️mutations/⚗️change-material-designation/🔺️diff/🦀️.rs:8` | ChangeMaterialDesignation (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.materials`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/❄️change-cold-formed/🔺️diff/🦀️.rs:11` | ChangeColdFormed (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `cold_formed`; inverse restores whole `base.cold_formed` (`🪶️en1999/🧬️schema/🧬️mutations/❄️change-cold-formed/↩️inverse/🦀️.rs:9`) |
| `🪶️en1999/🧬️schema/🧬️mutations/➕add-member/🔺️diff/🦀️.rs:11` | AddMember (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/➖remove-member/🔺️diff/🦀️.rs:12` | RemoveMember (en1999) | V1-GENERIC-DIFF | diff writes whole `members` |
| `🪶️en1999/🧬️schema/🧬️mutations/⤴️change-member-my-ed/🔺️diff/🦀️.rs:11` | ChangeMemberMYEd (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/🏋️change-member-n-ed/🔺️diff/🦀️.rs:11` | ChangeMemberNEd (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/🏗️change-members/🔺️diff/🦀️.rs:11` | ChangeMembers (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `members`; inverse restores whole `base.members` (`🪶️en1999/🧬️schema/🧬️mutations/🏗️change-members/↩️inverse/🦀️.rs:9`) |
| `🪶️en1999/🧬️schema/🧬️mutations/📏️change-member-buckling-length/🔺️diff/🦀️.rs:11` | ChangeMemberBucklingLength (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.members`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/📐️change-sections/🔺️diff/🦀️.rs:11` | ChangeSections (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `sections`; inverse restores whole `base.sections` (`🪶️en1999/🧬️schema/🧬️mutations/📐️change-sections/↩️inverse/🦀️.rs:9`) |
| `🪶️en1999/🧬️schema/🧬️mutations/🔄️change-fatigue-details/🔺️diff/🦀️.rs:11` | ChangeFatigueDetails (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `fatigue_details`; inverse restores whole `base.fatigue_details` (`🪶️en1999/🧬️schema/🧬️mutations/🔄️change-fatigue-details/↩️inverse/🦀️.rs:9`) |
| `🪶️en1999/🧬️schema/🧬️mutations/🔗change-connections/🔺️diff/🦀️.rs:11` | ChangeConnections (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `connections`; inverse restores whole `base.connections` (`🪶️en1999/🧬️schema/🧬️mutations/🔗change-connections/↩️inverse/🦀️.rs:9`) |
| `🪶️en1999/🧬️schema/🧬️mutations/🔥️change-fire-scenarios/🔺️diff/🦀️.rs:11` | ChangeFireScenarios (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `fire_scenarios`; inverse restores whole `base.fire_scenarios` (`🪶️en1999/🧬️schema/🧬️mutations/🔥️change-fire-scenarios/↩️inverse/🦀️.rs:9`) |
| `🪶️en1999/🧬️schema/🧬️mutations/🔥️change-weld-throat/🔺️diff/🦀️.rs:8` | ChangeWeldThroat (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.connections`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/🔩change-bolt-count/🔺️diff/🦀️.rs:8` | ChangeBoltCount (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.connections`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/🧱change-materials/🔺️diff/🦀️.rs:11` | ChangeMaterials (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `materials`; inverse restores whole `base.materials` (`🪶️en1999/🧬️schema/🧬️mutations/🧱change-materials/↩️inverse/🦀️.rs:9`) |
| `🪶️en1999/🧬️schema/🧬️mutations/🧱change-plate-thickness/🔺️diff/🦀️.rs:8` | ChangePlateThickness (en1999) | V1-GENERIC-DIFF, V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | diff clones `base.sections`, mutates the clone, writes whole field back |
| `🪶️en1999/🧬️schema/🧬️mutations/🫙change-shells/🔺️diff/🦀️.rs:11` | ChangeShells (en1999) | V1-GENERIC-DIFF, V2-RESTORE-INVERSE | diff writes whole `shells`; inverse restores whole `base.shells` (`🪶️en1999/🧬️schema/🧬️mutations/🫙change-shells/↩️inverse/🦀️.rs:9`) |
| `✏️s/🔌️plugins/📕️norm/🪟️results/🎚️config/🧬️schema/🧬️mutations/☑️change-selected-check-index/🦀️.rs:16` | ChangeSelectedCheckIndex (results) | V1-SNAPSHOT-DIFF | the diff type is the whole `NormResultsWindowConfig` snapshot, not a sparse delta; `MutationDiff::apply` returns `self.clone()` and ignores base; no-op path returns `base.clone()` (Rulings: whole-snapshot config/window diffs are removed) |

Borderline: `🏭️vdi3805` ChangeManufacturerFile restores a record-valued payload field (`base.catalog.file`), not a collection. Kept as V2-RESTORE-INVERSE for consistency; treat it as a judgement call. `⚡️din18599` UpdateClimate also re-mints the derived `climate_table` inside the diff.

## Clean Kinds (no V1-V3 code and a sequential round-trip test)

en1992 ChangeAnnex, en1992 ChangeTitle, en1992 ChangeDesignWorkingLife, en1992 ChangeDeltaCDev, en1992 ChangeCementType, en1994 ChangeAnnex, en1994 ChangeSteelFYPa, en1994 ChangeStructureKind, en1994 ChangeFatigueDetail, en1994 ChangeFireRating, en1994 ChangeInsulationThicknessM, en1997 ChangeAnnex, en1997 ChangeGroundwaterLevel, en1997 ChangeDesignSituation, en1997 ChangeInvestigationDepth, en1997 ChangeGeotechnicalCategory, en1997 ChangeDesignApproach, vdi3805 ChangeCorrectionAsOf, vdi3805 ChangeStrictMode, en1999 ChangeAnnex.

Also clean: `iso16757 ReplacePartNumberRule` (a `replace-<entity>` kind setting one record; allowed by the Rulings). All 158 kinds still lack the Σ-absorb L3 test (systemic V4), so none is fully clean under the strict reading.

## Notes

- Scope discrepancy: `🪟️results` is not under `🗿️artifacts/`; it was audited at `✏️s/🔌️plugins/📕️norm/🪟️results/`.
- Rulings seam check: searched the 7 artifacts (non-test) for `SetSnapshot`, `Restore` variants, `apply_in_place`, `apply_to(&mut`, `diff_from_model`, `graph_edit_diff`, `commit_value_tree_edit` and `*_selection_inverse`. Zero mutation-variant or code hits (only doc comments and strings). The only Rulings violation of the `whole-snapshot config/window diffs` kind is `🪟️results`'s `ChangeSelectedCheckIndex`.
- The `🗑️generated/` folder is shared with other auditors in this ticket. This audit's scratch files live only in `🗑️generated/c-norm/`.
- Outside scope but seen in passing: `✏️s/.../📕️norm/🗿️artifacts/🪵️en1995/.../🧬️mutations/🦀️.rs:483` calls `MutationDiff::apply` directly in a helper (not audited).
