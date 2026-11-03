# Typed enums v4 (wave 2)

No cargo/rustc/build was run. Proof level: Python only (converter `--apply`, `schema twins generate` with zero drift afterwards, `🏷️field-meta-choices` `generate.py` + `check.py`, `🧬️schema-sync/🐍️check.py`). Every Rust edit is unverified by a compiler; the first rebuild of each artifact is the real test. en1994 was not touched (freeze; queued in `📌️important/requests.md`). No git writes.

## Result per artifact

| artifact | enums added | schema-sync | choices | rebuild line |
| --- | --- | --- | --- | --- |
| en1998 | `AnalysisDirection`, `T1Method`, `DriftLimitClass`, `LimitState` (spec of v3), `EnGroundType`, `EnSpectrumType`, `ImportanceClass` (variants `I, Ii, Iii, Iv`, tokens `i..iv`), `SystemType`, `StructuralMaterial` (system + member), `DuctilityClass`, `KnowledgeLevel`, `DiaphragmMaterial`, `BearingDevice`, `ImposedCategory` (storey/bridge/tower variable category + silo/tank content category) | 0 | 0 mismatches | yes |
| en1997 | `SituationChoice` (BS-P/T/A, snapshot + load case), `ApproachChoice`, `SoilType`, `PileType`, `EarthPressureMode`, `WallMovement`, `TendonType`, `EmbeddedWallKind`, `LabTestKind` | 0 | 0 | yes |
| en1996 | `LoadCaseSituation` (wall load case, `From` into the compliance `DesignSituation`), `ImposedCategory` (A to H only, Snow/Wind removed because the integrity rule never accepted them) | 0 | 0 | yes |
| en1990 | `DistributionKind`, `TestDistribution`, `PermanentKind` | 0 | 0 | yes |
| en1993 | 20 enums: `SteelMemberType`, `LoadApplication`, `MomentDiagram`, `AnalysisKind`, `StainlessForming`, `TorsionLoad`, `PatchLoadType`, `FabricationToleranceClass`, `StiffnessClass`, `LatticeKind`, `SheetPileEnvironment`, `RestraintSupport` (start + end), `RestraintDistribution`, `ChordShape`, `WeldPattern`, `JointRowType`, `FrameNodeSupport`, `FrameBarRelease` (start + end), `FireRating`, `FatigueMethod` (detail + bridge) | 0 | 0 | yes |

The coordinator item for en1996 `designSituation`: the snapshot already used the compliance `DesignSituation` (with `Seismic`); the 2 schema-sync mismatches were a checker defect (`global_enums()` picked the 3-member en1997 enum of the same name first). Fixed in `🧬️schema-sync/🐍️check.py` (compliance directory is read first); en1996 shows 0. The wall load case `designSituation` is now typed too (`LoadCaseSituation`); `situation_from_str` and the scene `situation()` are deleted, callers use `DesignSituation::from(lc.design_situation)`.

## Converter changes (🧰️scripts/typed-enums/convert.py)

- Diff schemas that spell a patch field as plain `string | null` (en1998 style) are detected via the `Patch of …::Host { set { … } }` macro; `rule_diff_json` rewrites the property to a `$ref` of the snapshot property. Leaves are now found for these artifacts.
- Two new JSON layouts (`dump_spaced`, en1998 diff) and a layout-preserving text patch fallback (`text_patch_schema`) for hand-formatted schema files (en1997/en1993 snapshot).
- New helpers next to it: `fix_diff_macro.py ART` (types the `Patch of … { set { f: String` macro fields of the diff crate, the converter only knew `for Host {`), `as_str_fix.py ART LOG [ENUMS]` (adds `as_str` + `Display` to the model enums and rewrites the residual lines `&x.f` to `x.f.as_str()`), `addspec.py` and `specs_en199x_v4.py` (the specs), `en1998_manual.py`, `en1997_manual.py` (hand edits with asserted counts). Specs are in `specs/en1990|en1993|en1996|en1997|en1998.json`.

## Hand edits worth knowing

- en1998: formulas take enums (`psi2_de`, `phi_table_4_2`, `psi_e`, `gamma_i`, `q_max_for_system`, `drift_theta`, `fundamental_period_s`, `confidence_factor`, `masonry_wall_ratio_limit`, `rc_*`); `EnGroundType::letter()`, `LimitState::token()`, `AnalysisDirection::token()` added; the three `change-site-*` leaves (full-site diff) converted by hand (rs, schema, fixtures); fixtures lower-cased for the category/importance tokens (5520 values in 647 files); DSL assets: `en-ground-type=""`/`en-spectrum-type=""` became the inert values `a`/`type1` for DE-annex documents, `importance-class=II` became `ii`. The "direction known" check is now constant true (no unrepresentable value); the limit-state-set check always passes (kept as an informational row).
- en1997: `FoundationLoadCase.design_situation` no longer inherits the document value when empty (empty is unrepresentable; fixtures default to Persistent). Tests that fed invalid strings (`da9`, `bsX`, `bogus`, sqlite `independent …`) were removed or switched to valid variants. The python kernel oracle accepts `persistent|transient|accidental`.
- en1996/en1990: new enums carry `camelCase` serde/value attributes (the template enums of those files had none); the choice tokens follow the schema.
- en1993: the empty-string choice of seven editor lists became an explicit `Unset` variant; `assumed_class.is_empty()` became `!= StiffnessClass::Unset`; python oracle compares `momentDiagram`/`loadApplication` case-insensitively.

## Not done (queued)

- en1994: frozen. Queue entry in `requests.md`.
- en1998 `annex` (String, `"de"`/`"en"`): should become the compliance `AnnexChoice` like en1995/en1994; needs the existing-enum mode of the converter (not written). Same for en1998 free text fields (`samples`).
- en1990: `VariableAction.category` (registry keys, 25 snake_case values, flows into the combination generator as String), `VariableAction.group`, `FactorOverride.factor`, `TestSeries.quantity`, `structure_kind`, `supervision_level`, `inspection_level` (the last three are already enums or absent from the current schema). The category set drives registry ids (`ndp.en1990.psi.<category>`); converting needs the combination generator and the traffic-group helpers changed together.
- en1993: `kind` fields (SteelMaterial, SteelSection, LoadCase, SteelJoint, BuiltUpMember: 52 consumers), `category` (LoadCase psi category, SteelJoint connection category), `ParametricSection.shape`, catalogue references (`grade`, `subgrade`, `boltClass`, `weldGrade`, `designation`), `stress_history` (numeric list).
- en1997 `soil_type` is a closed list of five; the soil classification catalogue (CAT) is still a follow-up.

## Risks for the first compile

`as_str()` returns the legacy key (so string-keyed kernels and registries keep working) and `Display` delegates to it; lines the converter lists as `residual` were rewritten by regex on exactly those lines, so a `String`-typed binding that now receives `&'static str` may need a `.to_string()`. The en1998 `matches!` of `known`/`ls_ok` became constants (possible `clippy::overly_complex_bool_expr`, no error). Generated enums have no `Default`.

## v4b (after the v4 stall)

State found: v4 had finished en1990/1993/1996/1997/1998 (`convert.py --check` shows 0 changed files except already-converted residual noise); en1996 `designSituation` was already typed (see above), schema-sync 0 for en1996. One half-converted field: en1994 diff `reliabilityClass` (schema `RC1..3`, Rust `AnnexChoice`), fixed to `ReliabilityClass` in `🔺️diff/{🔣️.json,🦀️.rs}`.

Converted in en1994 (freeze was cleared): `StructureKind` (snapshot + artifact + diff), `ConstructionMethod`, `BucklingCurve`, `ActionStage` (beam/slab/column actions) from `specs/en1994.json` via `--apply` (86 files), `as_str_fix.py` (3 residual lines), hand edits: the four enums got `camelCase` serde/value attributes, diff `structure_kind` typed, live scene test `construction` literal replaced by the variant. Gates: `schema twins check --strict` drift 0, schema-sync 0 mismatches (all codes), field-meta-choices 265 / 0 mismatches, `convert.py --spec en1994 --check` 0 files changed (except the diff field now fixed).

Still open in en1994: `fireRating`, `fatigueDetail`, `new_category`/`new_kind` leaves (field-meta: "no single schema enum"). Step 3 for other codes (about 100 plain-string choice fields: en1990 `VariableAction.category`, en1993 `kind`/`category`, en1998 `annex`) not started in this run.

- ready for rebuild: typed enums v4 (en1994)

## v5 (wave 2)

No cargo/rustc/build was run (python, `schema twins generate/check` through `heavy.sh`, field-meta and schema-sync checkers only). No git writes. The first rebuild of each touched artifact is the real compiler test.

### Task 1: en1994 verification
`convert.py --spec specs/en1994.json --check` is clean (0 files changed for the four v4b enums; residuals are `Display` uses). The fixture checker (`kind-convention-v2/verify_fixtures.py en1994`) reports 126 scenarios, 126 inverted, 0 failures. v4b bugs found and fixed on the way: `ChangeStructureKind` and the `ConstructionMethod`/`BucklingCurve`/`ActionStage` payload literals in `🧰️testkit` were still strings; `CharacteristicAction::{permanent_area,imposed_area,point_force}`, `SlabAction::*` and `ColumnAction::forces` took `stage: &str` and did `stage.into()` into the `ActionStage` field (no `From<&str>`), now typed `ActionStage`/`ActionKind` parameters with all callers and tests updated.

### Task 2: en1994 new enums
`FireRating` (`Unrated`, `R30..R120`; the hand-written `part_1_2::FireRating`/`parse_fire_rating` are gone, `FireRating::rated()` returns `Option` for the checks and scene plates), `FatigueDetail` (`StudWelded`, `ShearConnector`, `Reinforcement`, `FlangeButtWeld`; `steel_detail_category_pa`/`steel_curve` match variants), `SupportCondition` (beam and slab support; `equivalent_span_m`, `span_position` and the whole `part_en1990` kernel take it instead of `&str`), `SheetingProfile` (the old code compared `"re-entrant"` while every fixture said `re_entrant`: fixed by the type), `ReinforcementProduct` (`RebarProduct::from_wire` is replaced by `From<ReinforcementProduct>`, `beam_fire` returns the beam instead of an `Option`), `ColumnKind`, `ActionKind` (`psi_factors`, `psi_fi`, `case_kind`, diagram factor take it). Diff `fireRating`/`fatigueDetail` typed. Assets use DSL kebab tokens (`fatigue-detail=stud-welded`). Python oracles use the wire tokens (`simplySupported`, `continuous2Span`, `reEntrant`, `partiallyEncased`).
Left as strings on purpose: `new_category`/`category` of actions (an open registry key of `ndp.en1990.psi.*`, 7+ categories plus the `imposed` subcategory `B/C/...`), `steel.designation` (catalogue reference), the boolean `ribsParallelToBeam` (already boolean).

### Task 3: other codes
| code | enums added | notes |
| --- | --- | --- |
| en1990 | `StructureKind` (+ `domain()`, `is_bridge()`), `SupervisionLevel` (`dsl1..3`), `InspectionLevel` (`il1..3`) | `Domain::of(&str)`, `is_bridge(&str)` and `STRUCTURE_KINDS` removed from the checks; the "invalid structure kind" input check is unrepresentable and deleted |
| en1993 | `MaterialKind`, `SectionKind`, `ActionKind` (load case), `JointKind`, `JointCategory`, `BuiltUpKind`, `ParametricShape` | wire tokens are the variant names (en1993 has no camelCase enums); new `normalize_wire.py` rewrote the lowercase legacy tokens the converter had left in 8789 fixture values; DSL table cells of the assets rewritten to kebab (`rolled-i`) by a one-off cell rewriter; the converter had mapped the dead `m.kind == "hss"` of materials to `SectionKind::Hss`, removed |
| en1998 | `annex` is now the compliance `AnnexChoice` (`En`/`De`) in snapshot, artifact, diff, `change-annex` leaf, scene, formulas, sqlite SQL/TS/Rust, fixtures (617 values), `example(AnnexChoice)` | the io tests that used `annex` as an arbitrary-text carrier use `buildings[0].name`; fixed a v4 bug in the sqlite size bound (`[&self.annex, &self.site.en_ground_type, ...]` mixed types) |

### Not converted (and why)
- Numeric choices (`u8`, not strings): en1995 `serviceClass`, en1996 `supportSides`, en1990 `consequenceClass`, `designWorkingLifeCategory`.
- Catalogue/registry keys: en1990 `VariableAction.category`, `group`, `FactorOverride.factor`, `TestSeries.quantity`; en1993 `LoadCase.category`, `boltClass`, jointPlates `role`/`boltClass`, `anchorClass`, `grade`, `subgrade`, `designation`; en1999 `designation`, `fillerAlloy`; en1994 action `category`.
- din4108, din18599, vdi3805 (non-Eurocode families, 40+ plain-string choices): not started.
- en1991, en1992, en1995, en1996, en1997 remaining plain-string choices that field-meta does not list as choice are untouched; the v4 en1993 fixtures keep lowercase tokens for enums converted in v4 (`analysis: elastic`, `momentDiagram: linear` ~1180 values each) while the Rust enums have PascalCase wire tokens: `normalize_wire.py` only handles the v5 enums, extend its `OWNERS` table to fix those.

### Risks for the first compile
- en1993 model enum `SectionKind` has the same name as the checks-local `clauses::stability::SectionKind` (explicit paths are used everywhere, no glob import mixes them).
- en1993 DSL assets: v4 enum cells (`beamColumn`, `shearCenter`) are camel while `DslScalar` tags are kebab (`beam-column`); one asset mixes both. Needs the first load test.
- `as_str()` still returns the legacy key; used for messages and registry keys only.

## v6 (wave 2 finisher)

No cargo/rustc was run; python, `schema twins generate/check --strict` (through `heavy.sh`), schema-sync, field-meta and fixture checkers only. No git writes.

- en1993 fixtures: `normalize_wire.py` now also rewrites the v4 enums (key-only rules for unambiguous fields, owner-keyed for `kind`/`shape`): 11687 values in 1329 files plus 40 values in the hand-laid showcase fixture; a second run reports 0. Pascal wire tokens now everywhere (`analysis: Elastic`, `momentDiagram: Linear`, `rating: R30`, ...).
- en1993 DSL assets: new `dsl_cells_en1993.py` rewrote 23 cells to the kebab tag of the schema variant (`beam-column`, `shear-center`, `damage-tolerant`, empty `""` of Unset enums became `unset`). New `dsl_cells_check_en1993.py` is the dry run: 48 enum cells equal the kebab of the sibling `snapshot.json` wire token, 0 mismatches. No real DSL loader exists outside the platform build; the first load test stays open.
- Name clash fixed, and it was a real compile bug: the converter had skipped inserting the model `pub enum SectionKind` because it found the checks-local one (`model_enum_defined`). Checks-local type is now `BucklingSectionKind` (EN 1993-1-1 Table 6.2 buckling curve section type; stability, parts, restraint-checks, unit tests). Re-running the converter for `SectionKind` inserted the model enum into the snapshot twin. All spec enums of en1990/93/94/98 verified present as `pub enum`.
- Diff twins: `En1994Diff.fire_rating/fatigue_detail` and `En1990Diff.structure_kind/supervision_level/inspection_level` were `Option<String>` (stale `x-semio-rust: String` in the diff schema JSON) while the leaf diffs build typed values: fixed in the diff schema JSON (and the en1990 root schema JSON), regenerated. The two en1994 schema-sync notes are resolved.
- en1990 fixtures: `change-inspection-level`/`change-supervision-level` payloads were `IL3`/`DSL3`, the wire token is camelCase (`il3`/`dsl3`): fixed.
- Shared taxonomy bug (not enum related): `📇️mutation-leaf-taxonomy-v1` had `insert/remove-joint-plate` and `-foundation-block` copied from crane runway (`path craneRunways`, `item craneRunway`): fixed, the 4 en1993 fixture failures are gone.
- Results: schema-sync mismatches 0 for en1990, en1993, en1994, en1998; `verify_fixtures.py` 0 failures for all four (en1990 148, en1993 649, en1994 126, en1998 297 scenarios); field-meta-choices 268 mismatches 0; `schema twins check --strict` drift 0 for en1990, en1993, en1994 (en1998 untouched, 0 earlier). Appended "ready for rebuild: typed enums v5/v6" to build-en1990/1993/1994/1998 `latest.md`.
- Logs: `🗑️generated/typed-enums-v6/`.
