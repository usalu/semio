# COMPILE-REVIEW S (wave 2, 2026-11-03 12:30 to 13:30)

Scope: en1990, en1991, en1993 (beyond v10), en1996, en1997, en1998, en1999 plus the crates of en1992/en1994/en1995 downstream of where their builds stopped. No cargo/rustc/`repo run`; no git writes; freeze.txt was read (en1995 only at 12:49, cleared afterwards; nothing under en1995 was touched).

## Scanners (TICKET/🧰️scripts/compile-review-s/, run as `/home/deck/semio/.venv/bin/python -I <script> <code>`; outputs in TICKET/🗑️generated/compile-review-s/)

| script | finds |
|---|---|
| `rs.py` | shared reader (comment/string stripping, brace matching, struct/enum index) |
| `scan_resolve.py` | unresolved CamelCase names per scope (missing imports), through `#[path]` mounts, `pub use` globs, extern aliases, inline `mod` scopes |
| `scan_imports.py` | explicit `use` items that the target module does not export or that do not exist (E0432/E0603) |
| `scan_exprpaths.py` | `semio_crate::module::item` and `crate::module::item` expression paths |
| `scan_visibility.py` | sibling-module paths to non-`pub` items (the `Lever<T>` class) |
| `scan_deps.py`, `scan_paths.py` | `semio_*` path roots that are no Cargo dependency, other unresolved path roots |
| `scan_literals.py`, `scan_assign.py`, `scan_callargs.py` | struct literals, assignments and call arguments mixing String and typed enums |
| `scan_strmeth.py`, `scan_receivers.py`, `scan_params.py` | string-only methods on enum fields, methods missing on the (unique) field/param type (`ReliabilityClass::label()` class) |
| `scan_format.py` | `{}`/`to_string()` of enum fields whose enum has no Display |
| `scan_structs.py`, `scan_io_fields.py`, `scan_sqlite_text.py`, `fix_read_text.py` | missing/unknown fields in struct literals (Snapshot initializers incl. `Self {`), sqlite io coverage of snapshot fields, `read_<enum>(text(..)?)` borrow |
| `scan_diff.py`, `scan_rowkeys.py` | diff patch field types vs model rows, `nested_patch!` section order/duplicates against the macro of that crate, `row_key!`/`KeyOf` coverage and `id: String` |
| `scan_match.py` | non-exhaustive matches over artifact enums (E0004) |
| `scan_dupes.py`, `scan_kebab.py`, `scan_fieldnames.py`, `scan_methods.py`, `scan_lock.py` | duplicate items/imports/inherent methods, kebab identifiers left by generators, fields that exist in no struct, methods defined nowhere, Cargo.toml vs Cargo.lock |
| `add_dep.py`, `fix_x_semio_rust.py` | tools used for the fixes (Cargo.toml plus lock entry; schema-first `x-semio-rust` type on a leaf payload property) |

Clean (0 certain findings after the fixes): diff patch types (all 10), `nested_patch!` order and duplicates (en1992, en1994), row keys, `Snapshot { .. }` initializers (all `Self {`/`En19xxSnapshot {` literals), non-exhaustive matches, Cargo dependency roots, kebab identifiers, duplicate items, methods on shared enums (no remaining `ReliabilityClass::label()`-like calls), expression paths into other crates, `crate::` references across `-more` splits (split-refs scanner: only opaque-glob macro names).

## Certain errors fixed (all in artifact trees; paths relative to `✏️s/🔌️plugins/📕️norm/🗿️artifacts/`, `ANY` = `🏅️standards/🔖️1/🪆️subsets/✳️any`)

### en1991
- Six insert leaves used bare model types without import (`pub item: RotatingMachine` and so on, E0412): `⚙️insert-rotating-machine`, `🌾️insert-bulk-solid`, `🏗️insert-crane-hoist`, `📏️insert-load-dispersal`, `🚶️insert-footbridge`, `🛤️insert-rail-dynamic` under `ANY/🧬️schema/🧬️mutations/`. Fixed in `🧬️schema/🔣️.json` (`x-semio-rust` on `item`, the same as `🚛️insert-special-vehicle`; a hand edit of the `.rs` alone was overwritten by the generator within seconds) and in the `🦀️.rs`.

### en1992
- `🔩️insert-stud-row` (`studRow`), `🕳️insert-opening` (`opening`): same bare type bug, same fix (JSON `x-semio-rust` plus `.rs`).
- `🧪change-cement-type/🦀️.rs`: label formatted `CementClass` with `{}` (no Display) -> `{:?}`.

### en1993
- `ANY/🧬️schema/💡️inferences/⚖️checks/🧬️core/🦀️.rs`: `TowerLeg` missing in the model import (used by `tower_design_point`, E0412).
- `.../🧰️structure-helpers/🦀️.rs`: `misc::cold_forming_shift` / `misc::reference_temperature` without `use crate::clauses::misc` (E0433).
- `.../🔗️special-checks/🔩️joint-checks/🦀️.rs:204`: `joint.category.to_ascii_uppercase()` on `JointCategory` -> `.as_str()` first.
- `.../🔗️special-checks/🦀️.rs`: leftover `use special_parts_checks::check_special_parts` and its call; the function is defined nowhere (its work is split over `check_annex_blocks` and the pub(super) drivers) (E0432).

### en1994
- `ANY/👁️viewer/📦️packages/🦀️rust/Cargo.toml` plus `🧩️en1994/Cargo.lock`: `semio-s-plugin-norm-compliance` dependency added (viewer calls `semio_s_plugin_norm_compliance::AnnexChoice::De`, E0433).
- `ANY/🧬️schema/🦀️.rs`: `StructureKind`, `FireRating`, `FatigueDetail` imported from the model (E0412).
- `.../⚖️checks/🦀️.rs` `part_2`: `FatigueDetail` imported.
- `.../🧮️compliance/🏗️beams/🦀️.rs:623`: `b.support.replace('_', " ")` on `SupportCondition` (EN and DE message) -> `.as_str().replace(..)`.
- `ANY/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs:57`: the large-text test assigned a String to `snapshot.structure_kind` (now an enum) -> `snapshot.beams[0].id`.

### en1996
- `ANY/🧬️schema/⚖️masonry/🛡️conformance-and-integrity/🦀️.rs`: `push_unknown_imposed_category` / `push_unknown_design_situation` called `.trim()` on the typed `ImposedCategory`/`LoadCaseSituation` (and used an undefined `load_case_path`); an unknown value cannot exist any more, both checks and their two calls removed.

### en1997
- `ANY/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: 13 `read_<enum>(text(..)?)` -> `read_<enum>(&text(..)?)` (local `text` returns String).
- `.../⚖️checks/📐️geotechnical-parts/🦀️.rs` `is_cohesive_layer`, `.../🧱️pile-wall-slope-and-uplift-checks/🦀️.rs:317-318`: `.to_ascii_lowercase()` on `SoilType`/`EarthPressureMode`/`WallMovement` -> `.as_str()` first.
- `.../🪨️layer-and-duplicate-id-helpers/🦀️.rs` `situation_for_load_case`: `.trim()` on the enum plus the undefined value `doc` (parameter is `_doc`) -> returns `lc.design_situation.as_str()`.
- `.../🏗️project-checks/🦀️.rs:656-657`: `.trim()` on `ApproachChoice`/`SituationChoice` -> `.as_str()`.

### en1998
- `ANY/🧬️schema/📸️snapshot/🦀️.rs`: `AnnexChoice` not imported (E0412).
- `ANY/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: `En1998TieBeam`, `En1998Slope` missing in the `use crate::{..}` list; the snapshot initializer lacked `tie_beams`, `slopes` (E0063).
- `ANY/👁️viewer/📦️packages/🦀️rust/Cargo.toml` plus `🫨️en1998/Cargo.lock`: compliance dependency (as en1994).
- `ANY/🧬️schema/🧬️mutations/🎲️demo-cases/🦀️.rs:1059`: `new_stiffness_basis: String::from("x")` -> `PierStiffnessBasis::Computed`.
- `ANY/🧬️schema/🧬️mutations/📎️change-annex`: `AnnexChoice` unimported (JSON `x-semio-rust` plus `.rs`).
- 18 generated leaf files (`📍️change-seismic-slope-*` diff/inverse, `📍️change-tie-beam-*` diff/inverse) contained kebab-case identifiers (`r0.circle-radius`, `{ circle-radius: Some(..)`) -> snake_case. Generator bug: `l_mutations.py`/`l_setfield.py` of schema-evolve must emit snake_case field names for new `slopes[]`/`tieBeams[]` kinds.

### en1999
- `ANY/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`: 15 `read_<enum>(text(..)?)` borrows.
- `ANY/🚪️io/📝️text/🧬️mutations/🦀️.rs`: `ReliabilityClass` not imported for the DSL enum.
- `.../⚖️checks/🧱️member-checks/🦀️.rs:47` (actions digest) and `.../🥫️thin-wall-checks/🦀️.rs:113`: `{}` of `ActionKind`/`ActionCategory`/`ActionSource`/`FabricationClass` -> `{:?}`.

### en1990, en1995
No certain findings. Nothing touched.

## Uncertain / not fixed
1. en1998 mutation crate split in flux (another agent works there): the scanner reports `mut_members_joints::{change_joint_*, insert_joint, remove_joint, insert_tie_beam, remove_tie_beam, insert_seismic_slope, remove_seismic_slope}`, `mut_bridges_towers::{change_bridge_importance_class, change_bridge_vertical_period_s, change_bridge_pier_cracked_inertia_ratio}` and `mut_members::change_joint_col_moment_rd_nm` as not exported by the crate root they are imported from (`🔩️members-joints/🦀️.rs` mounts one leaf only; `-more`/`-rest` exist). Re-run `scan_imports.py 1998` after the split is complete. Also `🧱️members-wall-rows`, `🌍️site-ground` (+`-more`) are not in `🫨️en1998/Cargo.lock` yet (cargo rewrites the lock on the next build; `scan_lock.py` lists the stale entries for en1998/en1999 and the `io-text` crates missing in `[workspace.dependencies]` for 1990/1995/1996/1997/1998/1999).
2. `OpBinary` in en1997/en1999 `io/💾️binary/🧬️mutations` and `viewport_act::`, `live_scene`, `__semio_dispatch_*`, `norm_results_window_config_owner`, `NormResultsWindowConfigMutation`, `impl_norm_artifact_record`, `live_report!`: reported by the scanners but generated by macros or by derive (not resolvable without a compiler); no action.
3. Type errors that need inference stay invisible to the scanners: a String local compared with or passed as an enum through a function return, trait-method resolution, borrow/lifetime errors (E0502/E0597/E0716 classes), const-eval asserts like 'Mutations semantic kind must match its variant' (KIND-KEBAB covered those), value-level E0425 (a scanner exists as `scan_values.py` but is too noisy on the dense sqlite code to be useful).
4. The `as_str` doc comment of the converted en1994/en1993 enums says "Legacy string key"; AGENTS.md forbids legacy language; wording only, left.
5. Generated leaf `🦀️.rs` files are rewritten from their `🧬️schema/🔣️.json`; fixes of this class belong into the JSON (`x-semio-rust`), which is where the schema-first generator should emit them (open: `leaf-kinds`/`schema-evolve` item property of `insert-*` kinds lacks `x-semio-rust` for model rows of en1991/en1992).

## Ready for rebuild
Appended `- ready for rebuild: compile-review S` to `build-en1991`, `build-en1992`, `build-en1993`, `build-en1994`, `build-en1996`, `build-en1997`, `build-en1998`, `build-en1999` (`latest.md`).
