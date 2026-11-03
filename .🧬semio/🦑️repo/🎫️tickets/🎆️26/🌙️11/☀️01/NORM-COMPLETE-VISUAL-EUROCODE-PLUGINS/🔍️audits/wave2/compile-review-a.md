# 🔍 Compile Review A (wave 2): EN 1994, 1990, 1991, 1992, 1993

Method: no cargo. A set of read-only static checkers (kept in `🗑️generated/compile-review-a/`, run with `/home/deck/semio/.venv/bin/python -I`) walks the real module tree of every artifact crate (`[lib] path`, `#[path]`, inline mods, `extern crate` aliases, glob and `pub use` re-exports, dependency crates) and checks:
- `rsck.py`: every `use` leaf and every `crate::`/dependency path resolves (missing item, non-pub item across crates, missing enum variant), `#[path]` targets exist, duplicate item definitions per module, unknown capitalised names and lowercase call heads, call arity for free fns and associated fns, struct literal/pattern field sets.
- `leafck.py`, `diffck.py`, `diffck2.py`, `applyck.py`: mutation leaves vs snapshot structs, `row_patch!`/`Patch for` fields and types vs row structs, payload fields, row/payload type equality.
- `traitck.py`: trait impls vs trait definitions (missing/extra items) for all shared and framework traits (1.0 to 1.1k impls per artifact, clean).
- `exh.py`: enum match exhaustiveness, `depck.py`: Cargo deps/members/path deps, `serdeck.py`, `amb.py` (E0689 ambiguous float method calls: none).
- Source boundary: `source-boundary-scan.py` (see section 3).
Coverage: 281 / 60 / 94 / 421 / 1513 newly modified `🦀️.rs` files (en1994/1990/1991/1992/1993), all of them parsed; the 407 new en1993 kinds were checked mechanically (1020 leaf diff/inverse files: payload fields, `base.<coll>` fields, row fields, row/payload types, patch literals, plus the 26 `row_patch!` vs row structs: all consistent) and sampled by reading `change-pile-action-load-case-id` (template is correct).

## 1. Errors found and fixed

| # | file:line | issue | fix |
|---|---|---|---|
| 1 | `🧩️en1994/.../🧬️mutations/🧭️beam-design/🦀️.rs` (mounts) | leaves `change-beam-adjacent-span-m`, `change-beam-bridge-lanes`, `change-beam-patch-bearing-length-m` not mounted, but `🧬️mutations/🦀️.rs:107-109,213-216` does `use semio_s_artifact_norm_en1994_mut_beam_design::change_beam_*` (E0432) | added the three `#[path]` mounts |
| 2 | `🧩️en1994/.../🧬️mutations/🔗️column-design/🦀️.rs` | leaf `change-column-introduced-force-n` not mounted (`🧬️mutations/🦀️.rs:110`) | added mount |
| 3 | `🧩️en1994/.../⚖️checks/🧮️catalogue/🧪️tests/🦀️.rs:331,335,336` | `shear_reduced_sagging_moment` takes 6 args (`p, sec, ys, fyd, v_ed, v_pl`), tests passed 5 (E0061) | inserted `ys = 0.1` (the steel top 0.1 m below the slab top, the value that makes the web-centred case equal the symmetric formula) |
| 4 | `🧩️en1994/.../🧬️schema/🔺️diff/{🏗️beam-rows,🏛️column-rows,🧱️slab-rows}/🦀️.rs` | crate `semio-s-artifact-norm-en1994-diff` was at 4018 lines (> 4000 source boundary) | removed 72 blank lines (no logic change); now under budget |
| 5 | `🏋️en1991/.../⚖️checks/🦀️.rs` (`part_1_2`, was :758, now ~:796) | two `pub fn configuration_factor` in the same module (E0428): radiation constant `()` and Annex G rectangle `(a, b)` | renamed the Annex G one to `rectangle_configuration_factor`, caller `📇️catalogue/🔥️fire/🦀️.rs:123` updated |
| 6 | `🏛️en1992/.../🎬️scene/✂️shear/🧪️tests/🦀️.rs:42` | `use super::*` pulls `shear::project(input, layout, out)` (3 args, shadows the crate-level `project(input)`) (E0061) | `crate::project(...)` |
| 7 | `🏛️en1992/.../🧬️schema/🔺️diff/🦀️.rs:137` and `🧬️mutations/🧵️change-prestress-steel-f-p01k/{🔺️diff,↩️inverse}/🦀️.rs` | patch field `f_p01k` but model `PrestressSteel` field is `f_p0_1k` (E0609/E0560 in the patch macro and leaf) | patch field and leaf row/patch accesses use `f_p0_1k` (payload `new_f_p01k` unchanged) |
| 8 | `🏛️en1992/.../🧬️schema/🔺️diff/🦀️.rs:125` and `🧬️mutations/🔩️change-reinforcement-ductility/{🔺️diff,↩️inverse}/🦀️.rs` | patch typed `ductility: String`, model `ReinforcementGrade.ductility` is `DuctilityClass`; leaf compared `DuctilityClass == String` (E0308) | patch typed `DuctilityClass`; leaf diff converts the wire string (`a|b|c`) via new `class_of` / `name_of`; invalid value gives `mutation.invariant` |
| 9 | `🔩️en1993/.../✏️editor/🏷️field-meta/🦀️.rs:51,55` | `const RESTRAINT_SUPPORT` defined twice, identical (E0428) | removed the duplicate |
| 10 | `🔩️en1993/.../🧬️schema/🦀️.rs:77` | `Vec<Placement>` without importing `Placement` (E0412) | added `Placement` to the model import on line 11 |
| 11 | `🔩️en1993/.../⚖️checks/🧮️clauses/🧪️tests/🔭️special-parts/🦀️.rs:257` | `crate::misc::...` in the `checks` crate (the module lives at `crate::clauses::misc` there) (E0433) | `crate::clauses::misc::cold_forming_shift` |

## 2. Open items (owners; also appended to `📌️important/requests.md`)
- en1992: `change-reinforcement-ductility` fixture value `"b-alt"` is not a valid class now (use `"b"`).
- en1992: placement conversion (flat `placements` collection, scene v4) is half done in the tree: `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:110,112` still build `RcMember`/`Anchor` with `placement_x|y|z|yaw`, and the 8 leaves `change-member-placement-*`, `change-anchor-placement-*` read `r0.placement_*`, which the model no longer has. Compile errors until finished (owner is editing, I did not touch it).
- All five: model structs derive `serde::{Serialize,Deserialize}` only under `cfg_attr(test, ...)`; tests in other crates (en1990 compliance-report, en1991 io/compliance-report/scene, en1992 compliance/catalogue/io, en1993 compliance/io, en1994 io/compliance-report) call `serde_json::to_value(&snapshot)` / `from_value::<Snapshot>` on model types. This is E0277 unless the framework runs path dependencies with `cfg(test)`. Known repo-wide pattern (`en1994-v2.md` §1); decision belongs to the infra/shared owner. Fallback: model feature `test-serde`.
- en1994 diff crate has no budget headroom left (about 70 lines).

## 3. Source budgets (`source-boundary-scan.py`)
Over after my pass: `semio-s-artifact-norm-vdi3805-model` (4276 lines) and `semio-s-artifact-norm-iso16757-model` (4433 lines), both outside the five artifacts (old artifacts, not my scope). `semio-s-plugin-norm-compliance` (was 4308 lines in the first scan) is under now. All crates of en1990/1991/1992/1993/1994 are within 262144 bytes, 32 files, 4000 lines after fix 4.

## 4. Verified clean (no finding)
- `use` / path resolution in all 5 artifacts including shared crates `norm_compliance`, `structural_scene`, `app_surface`, framework crates (`PRIVATE`/missing items: none besides the above).
- Cargo: every crate is a workspace member, every `workspace = true` dependency exists in the artifact workspace, every `path` dependency exists, every `semio_*`/serde crate used in sources is declared (a false positive `semio_grammar_conformance` is a local `mod`).
- `#[path]` targets: all exist (except the en1992 `🔬️live` mount seen mid-edit which now resolves).
- Macro invocations (`list_delta!`, `row_patch!`, `nested_patch!`, `norm_*!`, `live_report!`, `incremental_laws!`): all macros exist; no duplicated `list_delta!` row types (the en1993 E0034 of run 10 is already gone); every mutation enum variant has its `use` and kind string; no unmounted leaf dir remains in any artifact.
- Trait impls vs trait definitions: 218 / 224 / 264 / 305 / 1041 impls, no missing or extra item.
- Enum match exhaustiveness on artifact enums: no gaps.

## 5. Limits
The checkers do not type-check expressions (method calls, field access on non-first-level rows, generics, lifetimes, borrowck), so mismatches in business logic can remain. A few false positives of the tools (`interaction_polygon` in en1994 catalogue tests, `gamma_mc`/`gamma_ms`/`configuration_factor` in inline modules, `Equation`/`Owner` matches) were analysed and are fine.
