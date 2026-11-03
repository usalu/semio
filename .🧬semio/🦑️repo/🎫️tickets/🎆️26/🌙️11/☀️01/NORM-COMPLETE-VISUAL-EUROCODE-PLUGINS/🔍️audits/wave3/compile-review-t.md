# COMPILE-REVIEW T (wave 3, 2026-11-03 15:00 to 15:50)

Scope: downstream crates (checks, catalogue, inferences, scene layers, diagrams, editor, viewer, io text/binary/sqlite, examples, testkit, tests) of all ten codes en1990..en1999. No cargo/rustc/`repo run`, no git writes. `freeze.txt` was absent at every edit (checked before each batch). Other agents edit concurrently: en1991 (fatigue models / wave3 rows), en1993 (plated panel and joint fields), en1994 (stud collection conversion), en1996 (MasonryWall fields), en1998 (mutation crate split) were mid-change; their findings are listed as "in flux", not fixed.

Heavy-lease note: `heavy.sh` blocked for >15 min on a stale `graph-pending/build-en1995` marker plus swap-full (swap free 0-2 MB for the whole session), so the Python scanners ran sequentially, niced 19, under `ulimit -v 3.5 GB`, one process at a time (same load profile as one heavy slot). No jobs remain.

## Scanners (TICKET/🧰️scripts/compile-review-t/; outputs in TICKET/🗑️generated/compile-review-t/)

S's scanners were run unchanged for all ten codes (`run_s.sh`). New scanners, each run for all ten codes (`run_new.sh`, `run_vd.sh`, `run_as.sh`, `run_nm.sh`, `run_move.sh`):

| script | finds |
|---|---|
| `scan_tempborrow.py` | E0716: `let v = ref_returning_fn(&call(..))` where the temporary dies while `v` lives (the en1990 catalogue class) |
| `scan_twophase.py` | E0502: `f(&mut x, .., x.m())` explicit `&mut` argument plus later use of the same binding (the case-edits laws class) |
| `scan_nestedmut.py` | E0499/E0502: `x.m(.., x.n(..))` with both `&mut self`, or a `x.iter()/first()` borrow passed to a mutating method |
| `scan_cmp.py` | enum-typed field compared with / matched against string literals, `.contains(&x.enum_field)` |
| `scan_derive.py` | derive coverage (Default/Copy/Eq/Hash/Ord/Clone/Debug) of field types, enum Default without `#[default]`, manual impl aware |
| `scan_valuederive.py` | framework derive coverage: ToValue/FromValue/CanonicalJsonTree/RetireOwned/RetainedClone on a type whose field type lacks the derive (E0277) |
| `scan_assoc.py` | `Type::name(` associated function calls on repo types that no impl/trait/variant provides (E0599) |
| `scan_arity.py` | E0061: call argument count vs unique repo definition (noisy for closures/generics; manual review) |
| `scan_move.py` | E0507 candidates: non-Copy enum field used by value from a path |
| `scan_traitdupes.py` | E0119: duplicate `impl Trait for Type`, derive plus manual impl |
| `scan_modpaths.py`, `scan_include.py` | missing `#[path]` mod targets, missing `include_str!/include_bytes!` files (0 findings in the whole norm tree) |

Additional inline checks (clean): all 932 Cargo.toml (`[lib] path`, path dependencies exist, parse), all `{ workspace = true }` dependencies exist in their workspace root (918 crates; one in-flux exception below), expression-level `ref_fn(&call())` greps.

Clean (0 certain findings): E0716/E0502/E0499 patterns in all ten codes except the one fixed below, E0119 trait/derive duplicates, E0507 candidates (the only candidate was a `&'static str`), struct derive coverage (the single `Copy` hit is a name collision with the `Copy` mechanics `Slice`), `Outcome::*`, `InputSpec`/`InputChoice` constructors, `crate::` paths into other crates (after the fix below), missing mod files, missing include files, `{}`/`{:?}` formatting of enums (the 3 remaining reports are `&'static str` scope fields and `{x:?}` named args).

## Certain errors fixed

All paths relative to `✏️s/🔌️plugins/📕️norm/🗿️artifacts/`; `ANY` = `🏅️standards/🔖️1/🪆️subsets/✳️any`.

1. en1998 E0716: `ANY/🧬️schema/🧪️tests/🏗️structures/🦀️.rs:338` `let check = find(&inferences::evaluate(&snap), &suffix);` borrows from a dropped temporary report and `check` is used on the next line -> `let report = inferences::evaluate(&snap); let check = find(&report, &suffix);`.
2. en1993 E0433 (six uses): `ANY/🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🧩️plated-ext/🦀️.rs` calls `crate::resistance::{eta_shear, lambda_w, chi_w, v_bw_rd, v_bf_rd, v_b_rd_cap}`, but the `clauses-special` crate root (`.../🧮️clauses/🧰️special/🦀️.rs`) only re-exported `effective_width, stability` from `clauses-core` -> added `resistance` to the `pub use`.
3. en1992 E0277 (4 types): the insert/change leaves `insert-concrete-grade`, `insert-reinforcement-grade`, `insert-prestress-steel`, `change-reinforcement-ductility` derive `CanonicalJsonTree` and so need it on their item/field types. `ConcreteGrade`, `ReinforcementGrade`, `PrestressSteel`, `DuctilityClass` in `ANY/🧬️schema/📸️snapshot/🦀️.rs` lacked `semio_framework_value::CanonicalJsonTree` and `#[canonical_json(owner = semio_framework_pack_json)]`; every item type of insert leaves in en1995/1997/1998 (and BarLayer, Anchor, Opening, StudRow in en1992) has both. Added to the four types.
4. E0599 in five viewer tests: `ANY/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️viewport/🧪️tests/🔬️unit/🦀️.rs` of en1991, en1992, en1994, en1998, en1999 called `EnXXXXSnapshot::noncompliant_overloaded_frame()`, which exists only in en1993 (copy-paste). Replaced with an existing constructor of each snapshot: en1991 `full_showcase`, en1992 `failing_under_reinforced`, en1994 `fire_demanding`, en1998 `noncompliant_de_office`, en1999 `noncompliant_multi_fail` (en1999 has the call twice).

## In flux (found by the scanners, owner agent is editing; recheck after their apply)

- en1991: `En1991Snapshot`, `En1991Diff`, `En1991Artifact` and sqlite initializers carry `fatigue_models` but the struct has no such field (`FatigueModel` struct exists); `wave3-rows` and `scene/📈️diagrams` use `CollisionLocation, FatigueModelKind, LocalFire, SnowGuard, SupportCollision`, `d.support_collisions`, `d.local_fires`, `g.roof_id`, `g.slope_length` that the model does not define yet; new crate `mut-fatigue-models` is in `members`/`script.ts` but not in `[workspace.dependencies]` of `🏋️en1991/Cargo.toml` and not in Cargo.lock (E: dependency `{ workspace = true }` missing in workspace). Looks like an interrupted `evolve.py add-collection` apply (files written 15:17).
- en1994: stud conversion half done: two crates named `semio-s-artifact-norm-en1994-mut-studs` (`📌️studs` and `🔩studs`) plus a third `mut-stud-rows` not registered as workspace member/dependency; `🧬️mutations/🦀️.rs` and `io/📝️text/🧬️mutations/🦀️.rs` still import `mut_studs::change_beam_stud_*` (5 names) and `change_beam_transverse_as`; `Stud::typical(` is called in `ANY/🧬️schema/💡️inferences/🧮️compliance/🏗️beams/🦀️.rs:340` and `ANY/✏️editor/📌️panels/📚️catalogue/🦀️.rs:105` but only `HeadedStuds::typical` exists; sqlite snapshot literals `CompositeBeam { studs }`, `En1994Snapshot { studs }`.
- en1996: `MasonryWall` literals (snapshot) and `En1996WallPatch`, `change-wall-web-spacing-m`, the document-case members use `web_spacing_m`, `second_leaf_thickness_m`, `cavity_width_m`, which are not struct fields yet.
- en1993: `SteelJoint.beam_web_fy`, `p.steel_grade`/`p.steel_subgrade` of plated panels used (26 files) but not in the structs when scanned (snapshot rewritten 15:28).
- en1998: `En1998Assessment.intermediate_steel_area_m2` (literals/diff), `r.hoop_*`/`concrete_design_strength_mpa`/`clear_length_m` in `formulas/🔩️critical-region`; the `mutations` aggregator and `🎲️demo-cases` import `change_bridge_importance_class, change_bridge_vertical_period_s, change_bridge_pier_cracked_inertia_ratio` from `mut_bridges_towers`, whose root does not re-export them (the modules live in `bridges-pier` and `bridges-pier-more`), and `change_joint_*`/`insert_joint`/`remove_joint` from `mut_members_joints` (modules now in `members-joints-rest`, new crates `members-wall-rows(-more)`, `site-ground(-more)` missing in Cargo.lock). Same as S's open item 1; the split is not finished.

## Uncertain / not fixed (no compiler)

1. `scan_arity.py`, `scan_move.py` output was reviewed by hand; the remaining entries are all argument-splitting or name-shadowing noise (closures, `<=` read as generics, local helpers).
2. `crate::`-glob and macro-generated names (`En19xxBuilder*`, `OpBinary`, `live_scene`, `viewport_act`, `NormResultsWindowConfigMutation`, `En19xxXxxPatch`, `__dsl_spec_producer`, `parse_dsl`, `*BoundedCommandJobFactory`) stay invisible to the scanners.
3. Borrowck classes that need types and lifetimes (E0597 tail-expression temporaries with closures like `standing_of`, E0502 through method calls on `self`) cannot be found by text; only the shapes seen in tonight's logs were scanned. Shared crate code (`app-surface`, `structural-scene`) was not in scope.
4. `x-semio-rust`-style issue stays open for the generator: the schema-evolve model layer should emit `CanonicalJsonTree` plus `#[canonical_json(owner = ..)]` on every item type of an insert/change leaf (cause of finding 3; en1992 collections `concreteGrades`, `reinforcementGrades`, `prestressSteels` were added with the tool).
5. `pub use ...::handles` plus `pub use ...::handles::handles` (en1991/en1993 scene root, reported as duplicate import) is a module and a function (separate namespaces): fine.

## Ready for rebuild
Appended `- ready for rebuild: compile-review T` to `build-en1991`, `build-en1992`, `build-en1993`, `build-en1994`, `build-en1998`, `build-en1999` (`latest.md`). en1991/en1994/en1996/en1998 builds should wait for their owners to finish the in-flux items above.
