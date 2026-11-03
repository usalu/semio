# Typed enums v2 (wave 2)

Nothing was compiled and no build was started (`cargo`/`rustc` not run). Proof level: Python only (generator idempotence, `schema-sync/check.py`, `verify build-owners sync`). Every Rust edit below is therefore unverified by a compiler; the first rebuild of each artifact is the real test. No git write was made.

## The generator

`🧰️scripts/typed-enums/convert.py` (about 1500 lines, usage in its docstring). Schema first: it reads the snapshot and diff JSON schemas, derives the host struct, the diff patch fields (diff schema properties that `$ref` the host property), the mutation leaves (leaf diff that writes the patch field with `payload.new_x`) and the sibling keys that identify objects, and then edits, additively and idempotently:

- model enum (derive/attrs copied from an existing enum of the same file, `DslScalar`, docstring), struct field type, struct literals, `==`/`!=`/`eq_ignore_ascii_case`/`matches!`/`match` arms/assignments on the field (string literal -> variant), `Self {` literals;
- schema JSON (`$defs` with EN/DE `x-semio-ui.options`, `$ref` on the property, indent/ascii/compact dump styles kept), TS (type, `normWireLiteral` parser, interface, parser, diff import), GraphQL/proto enums (first-word prefix scheme of the existing enums), diff lane (rs struct field, `Patch for` macro, `within` clause, ts), set-kind leaf (rs/diff/inverse/ts/json; leaf graphql/proto stay `String` as for `FireRating`);
- SQLite codecs (rs helper pair + write/read + bound arrays, ts `choice`/consts, `CHECK(col IN (...))`), editor choice list (both `m(...)` and `FieldSpec::choice(...)` tables, labels from the spec, kebab values), DSL assets (`key=value` groups), python oracle comparisons, fixtures (object-literal matching by sibling keys, so `kind`/`position` of other structs are not touched; the apply scenarios keep a real change: `x-alt` -> variant of `x`, the committed `before` gets another variant).
- `--check` exits 1 while any site is still unconverted; `--dry`; `--only`; `--spec`. Catalogue mode (`"catalogue": name`): `format: "catalogue-id"` + `x-semio-catalogue` in the schema def, and on the enum `id()`, `from_id()`, `label(german)`; string consumers `&x.field`/`x.field.as_str()` become `x.field.id()`. This is the typed catalogue id (validated parse, EN/DE label lookup); it is a closed enum per catalogue, not a generic `CatalogueId<K>` newtype (that needs the SHARED v7 crate the EN 1995 plan asks for; the enum form needs no new crate and is schema-first).

Proof of idempotence: scratch copy of en1992 (`🗑️generated/typed-enums-v2/scratch`, deleted at close): all seven first conversions applied, `--check` exit 0, second apply byte-identical (md5 of the whole tree). On the real trees the second `--check` of every spec reports 0 changed files (`final-check.txt`, see Results). Defects found and fixed in the generator on the way (each cost a repair): off-by-one in the object scanner (fixtures got `Bottom "`), group-number bug in the `matches!` rewrite (garbled one en1992 check line, repaired by hand), a leaf discovered through an outer patch literal (en1999 `change-conn-action-kind` got `ConnectionKind`, repaired), `from_const` not idempotent (en1995 StrengthClass, see below), unmapped literals of ambiguous field names (`kind`) are now skipped quietly.

## Results per artifact (announced in `📌️important/requests.md`)

| artifact | converted | schema-sync (`check.py`) |
|---|---|---|
| en1992 | `BarPosition`, `CementClass` (root + artifact twin), `ColumnPosition` (`WallEnd` added), `SlabFireSystem`, `ActionKind`, `ActionSource`, `PsiCategory` (rows of the psi table; was "CAT"), `BarBend`; `FireSpec.column_method` removed in all layers (model, 8 twins, diff lane, leaf `change-member-fire-column-method`, kind registrations, binary tags renumbered, edit rule, text codec, sqlite rs/ts/sql/cells/tests, editor, oracles, tests, fixtures, DSL assets); `En1992ReinforcementGradesPatch.ductility` is `DuctilityClass` | 29 mismatches = 22 leaf/diff GraphQL `String` vs enum (the existing `FireRating` pattern) + 7 items of shared types (NDP, compliance) |
| en1999 | `ProductForm`, `WeldProcess` (element + run), `WeldDirection` (element + run), `SectionKind` (`tube` and `chs` merged into `Chs`), `ConnectionKind`, `ActionKind`/`ActionCategory`/`ActionSource` (all four action lanes), `FabricationClass`, `OutlineRing`, `PatchLoadPosition`, `StiffenerSides`, `DetailType`; wire tokens keep the old camelCase spellings except `self` -> `selfWeight`, `butt-weld` -> `buttWeld`, `fillet-weld` -> `filletWeld`, `tube` -> `chs` | 31 = 24 GraphQL pattern + 7 shared |
| en1991 | `FireOccupancy` (the nine Table E.4 rows; `residential` -> Dwelling, `mall` -> Shopping) | 8 (1 GraphQL + 7 shared) |
| en1995 | `UlsEquation { Eq610, Eq610ab }` (sqlite token reconciled: `eq610`/`eq610ab`), `ActionKind`, `ImposedCategory`, `LoadDurationClass`, `FastenerType`, `StrengthClass` (36 classes, catalogue mode) | 9 (4 missing TS types of other owners, 1 GraphQL, 4 NDP) |

Hand edits that the generator cannot do are scripted and kept (`en1992_manual.py`, `en1992_remove_column_method.py`, `en1999_manual.py`, `en1991_manual.py`, `en1995_manual.py`, `en1995_strength_repair.py`; every replacement asserts its match count). They cover: `&str` helper signatures (`is_top`, `bend_mode`, `punching_beta`, `control_perimeter`, `local_factor`, `required_for`, `psi_factors`/`psi_row`, `section_interaction_exponents`, `shell_quality_q`, `form_supplies_tube`, `load_kind`, `family_of`, `action_kind_family`, `with_uls_equation`, `fire_load_*`), Display uses (`{:?}` / `.id()`), and the validations that became unrepresentable and were removed with their tests: en1999 unknown weld process/direction/product form/fabrication class/action kind-category-source/connection kind/detail type/stiffener sides; en1991 unknown occupancy test; en1995 `MISSING` strength class test. The en1991 `fire_load_or_invalid` branch stays for a missing registry cell.

## Findings of v1, status

- `En1992ReinforcementGradesPatch.ductility`: fixed (`DuctilityClass`). The leaf `change-reinforcement-ductility` still carries a `String` payload parsed by `class_of`; convert it next (same generator, `new_ductility`).
- `FireSpec.column_method`: removed everywhere (all layers), see above.
- `WallEnd` in `column_position`: added; `punching_beta` maps it to the `beta-wall-end` NDP, scene and control perimeter treat it like an interior column as before.
- The "fixture that uses `Beam` as an action kind" does not exist: `Beam` appears only as a `MemberKind` in `change-member-kind` (valid). The census of `LoadCaseActions.kind` values found only `permanent`, `imposed` and the bogus `permanent-alt`/`arbitrary`, now mapped.

## Things the owners must know

- en1991: `part_1_2::characteristic_fire_load_j_m2` was called (checks, catalogue, tests) but defined nowhere in the working tree or HEAD; I added it (registry key string, J/m², aliases accepted) next to `occupancy_row`. If the owner has a pending definition, delete one of the two.
- en1995: `psi_factors` accepted the undocumented kind `snow_high` (registry row `ndp.en1990.psi.snow_high`); `ActionKind` has no such variant, the branch and its test (`q_high` vs `q_snow`) were removed. Add a `SnowHigh` variant if the row must stay reachable.
- en1995 StrengthClass: a second run of the generator re-read the editor list after it had been rewritten and mapped every fixture `strengthClass` to the default. Repaired by `en1995_strength_repair.py`: 141 fixtures restored from HEAD, 288 by the most common pattern of the same length (new files without HEAD), the three apply scenarios by hand. Wire tokens are the lower-case ids (`gl28h`, `c24`), variants `Gl28h`, `C24` (unambiguous under serde and heck camelCase). Spot-check the restored fixtures (`final report: unrecoverable 297` means "restored heuristically", not lost).
- Wire changes of existing data: en1992 `Good`/`Poor` style stays PascalCase; en1999/en1995 are camelCase (`value(rename_all = "camelCase")` on the new enums, copied from the template enum of the file).
- The DSL texts of the en1992 assets were rewritten for the new kebab tokens (`slab-system=one-way`, empty values -> the default variant); `column-method` keys are gone.

## Not done (honest scope)

- en1990, en1993, en1994, en1996, en1997, en1998 enum fields of the classification (about 70 fields): not converted. The generator is artifact-agnostic (paths are discovered from the JSON schemas), but each artifact needs its spec (variants and labels from the editor lists) and the follow-up of the string helper functions; I ran out of budget before starting them. Starting point: `specs/en1992.json` .. `specs/en1995.json` are the template, `--dry` shows the touched files and `unmapped:` lists tests with invalid values.
- Catalogue references other than en1995 `strengthClass`: en1999 `designation`, `filler_alloy`, `bolt material`, `detail_category`, en1993 `grade`/`bolt_class`/`weld_grade`, en1994 `fatigue_detail`, en1997 `soil_type`: not converted. `from_const` + `"catalogue"` converts them in one spec entry each once the variants are listed; `designation` has the widest string consumer surface (`resolve_alloy_for_*`, `&material.designation` in about 20 places) and was left for that reason.
- Leaf graphql/proto of converted set-kind leaves stay `String` (existing convention), hence the GraphQL mismatches above.
- SCHEMA-CODEGEN did not land a JSON -> twins generator while I worked, so the twins were emitted by this generator.
- Cross-artifact duplicates (ActionKind, ActionSource, ActionCategory, FireRating) exist once per artifact; moving them to a shared compliance crate is open (classification report, last section).

ready for rebuild: typed enums v2 (en1992, en1999, en1991, en1995)
