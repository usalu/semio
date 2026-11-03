# Compile Review I (wave 2): en1992, en1993, en1995 (all crates of the three build closures, test crates included)

No cargo, rustc, build or git write. Window 02:10 to 03:05; the owners kept editing (en1995 typed-enum conversion and a scene-entities refactor, en1993 `StiffenerShape`/`WarpingSupport`, en1992 v9 fire/liquid diagrams), so every statement below is as of the stamped scan. Lane split: H owns en1990/91/94; I did not duplicate it.

## Method (scripts in `🗑️generated/compile-review-i/`, run with `/home/deck/semio/.venv/bin/python -I`)
Reused: `compile-review-a/{rsck,exh,leafck,diffck,diffck2,applyck,serdeck,traitck,amb,depck}`, `compile-review-e/{bal,mdep}`, `compile-review-g/{tomlck,mountck,mutck}`, `🧰️scripts/compile-gate-h/{featck,collide,patchck,enumck}`. rsck was re-run four times (02:12, 02:40, 02:54, 02:57) and diffed against the previous run to separate churn from settled state.
New (kept in the folder):
- `fieldscan.py` field accesses and method calls that exist nowhere in norm + framework (names from 18.8k files): 0 unknown fields, only std methods.
- `littype.py` struct literals vs struct definitions (2997 structs, 12283 field values): int literal for float, float for int, `""` for `String`/enum/number, enum of the wrong family: 0.
- `argtype.py` call arguments vs resolved fn parameter types (35221 args): 0.
- `arrlen.py` `[T; N]` declarations vs element count (const, static, let): 1 error (fixed).
- `methar.py` method-call arity against the definitions: nothing beyond std noise (closure commas).
- `devdep.py` normal-code use of dev-only dependencies, `cfgfeat.py` `feature = "x"` used but not declared, `featck.py` (own) every `dep:`/`crate/feature`/`features = [..]` entry against the target manifest (670 entries): see Fixed 8-10.
- `lockck.py` Cargo.lock vs manifests; `leaves.py` `FacetLeaves`/twin crate (`LEAF`) wiring, `include_str!` targets and dependency declarations of all 9 model/mutations/diff crates, `script.ts` package list vs workspace members (59/124/57, equal).
- `enumsite.py <artifact>` string use of enum-typed fields (`.as_str()`, `== "x"`, `field: "x".into()`, assignment); clean for en1992/en1993, backlog for en1995 below.

## Fixed (certain, after re-reading; minimal)
| # | file | class | fix |
|---|---|---|---|
| 1 | en1992 `🎬️scene/🛢️liquid-pressure/🦀️.rs:27` (new 02:11) | unclosed delimiter | `loc(&format!(..), &format!(..)),` lacked the closing `)` of `loc(`; closed |
| 2 | en1992 `🧬️schema/🔺️diff/🦀️.rs:141` | E0609/E0560: patch field `f_p01k` vs model `f_p0_1k` (review A fix lost in the 02:15 regeneration) | `f_p0_1k`; root cause in the leaf spec feeding `leaf-kinds/en1992_diff.py:64` (routed) |
| 3 | en1992 `⚖️checks/🧪️tests/🎯️oracle-wiring/🦀️.rs:492` | E0308 `[KnownDifference; 10]` with 9 rows | length 9 |
| 4 | en1992 `🚿change-member-tightness/↩️inverse`, `🧽clear-member-tightness/↩️inverse` | E0252 own payload imported twice | duplicate `use` removed |
| 5 | en1995 `mut-structure-rows`: `➕️insert-` / `➖️remove-` `diaphragm`, `truss`, `bracing`, and the new `deck` pair (8 files) + generator `🧰️scripts/en1995-collections/kinds.py:178,180` | E0433 `semio_s_artifact_norm_en1995_mut_entity_rows::remove_x::RemoveX` (crate not a dependency, items live in the own crate) | `crate::remove_x::RemoveX`; generator `partner()` uses `k["crate"]` instead of `CRATE["entity-rows"]` |
| 6 | en1995 scene `🧪️tests/🔬️unit/🦀️.rs:1237` | E0433 `semio_framework_async_macros` not a dependency of `en1995-scene` (and no `await`) | `#[test] fn` |
| 7 | en1995 umbrella `📦️packages/🦀️rust/Cargo.toml` | E0433 `mutations_unit` names `mut_structure_rows`, `mut_diaphragm_{sheathing,forces}`, `mut_truss_{geometry,forces}`, `mut_bracing_{layout,forces}`, `mut_deck_{geometry,prestress}` | 9 `workspace = true, features = ["test-support"]` dev-dependencies |
| 8 | en1995 `⚖️checks/📦️packages/🦀️rust/Cargo.toml` | manifest error: feature `oracle-wiring` = `semio-s-plugin-norm-compliance/compliance-testing`, compliance only a dev-dependency (also needed by the `cfg(feature)` module) | normal dependency (as en1992/en1993) |
| 9 | en1993 `⚖️checks/📦️packages/🦀️rust/Cargo.toml` | E0433 with `--features oracle-wiring` (SUT build): mounted `🧮️mechanics/🎯️oracle-wiring` uses `serde_json::Value`, dev-only | optional dependency + `dep:serde_json` in the feature |
| 10 | en1993 `🧮️clauses/📦️packages/🦀️rust/Cargo.toml` | E0433 `semio_s_artifact_norm_en1993_model::WarpingSupport` in `ends_of` (new) | model dependency added |
| 11 | en1995 `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`, `🧬️schema/⚖️timber/🧱️entities/🦀️.rs` | E0428 identical duplicate items (`uls_equation_text`, `read_uls_equation`, `fastener_name`) from a typed-enum script run twice | second copies removed |

## Open (routed to `📌️important/requests.md`, "from: compile-review I")
1. **en1995 typed-enum conversion unfinished (blocks `:build`)**: `🔺️diff/🦀️.rs` patch types still `String` for `ActionKind`, `ImposedCategory`, `LoadDurationClass`, `StrengthClass`, `FastenerType` fields (11 `patchck` lines); string literals for `bar_product`/`lamination_product` in three deck mutation tests. The compliance test sites and the entity/kernel sites were converted while I scanned. Re-run `enumsite.py`, `patchck.py`, rsck when the script finishes.
2. en1994 editor root names `semio_s_artifact_norm_en1994_mutations::resolve`, which does not exist (`resolve_edit` is exported). Belongs to H/en1994.
3. en1992 and en1993 `✏️editor/🦀️.rs` and `🧪️tests/🔬️unit/🦀️.rs` comments are mojibake (comments only; region markers and doc emoji corrupted).
4. Generators that re-introduce fixed errors: `en1992_diff.py` (field `f_p01k`), `en1992_tightness_kinds.py` (duplicate `use`), the en1995 typed-enum script (not idempotent).
5. `Cargo.lock` edges (viewer deps in all three, en1995 umbrella dev-deps, en1993 stale `mut-general-materials` edge). No `--locked` flag found in the cargo runner, so cargo repairs it.

## Verified clean (no finding)
- rsck on all crates of the three artifacts: only known noise (derive names `Diff`/`ReferenceDiff`/`ToValue`/`FromValue`, grouped `use`, `Registry` literal, sqlite `store::` globs, `viewport3d::` heads, closures `put`/`end`/`en`/`de`), plus the findings above.
- match exhaustiveness (`exh`): three tuple-match false positives. `RemedyBound` (six variants incl. `Remove`/`Insert`) handled in the only exhaustive match (`flipped`, en1995). New enum families (`TaperKind`, `CutoutKind`, `CutoutFace`, `TensionSide`, `DiaphragmKind`, `DiaphragmMethod`, `TrussKind`, `BracingKind`, `BracingProduct`, `LayerOrientation`, `MemberRole`, `SupportType`, `BarBend`, `WarpingSupport`): sqlite codecs list every variant, `id()` exists on every enum whose `.id()` is called.
- Struct literals complete: `BarPoint {length, bend}` (7 sites), `BendShape.both_ends`, `Diaphragm`/`Truss`/`Bracing`/`Taper`/`Cutout`/`PanelLayer`/`FastenerPosition` (sqlite, scene tests, fixtures), `DeformationOverlay`, `Marker`, `Spec`, `SceneEntity`, `Appearance` in the new en1992 diagrams (fire-capacity, liquid-pressure, fatigue-range), en1993 scene-members (plate-details, torsion-views, fatigue-views, kit), en1995 scene-entities (kit, typed, rows, grips); `ModalResult`, mechanics `Section`/`Material`/`Model` in `modes`.
- Macros: `norm_live_editor!` (`resolve =` item exists in en1992/en1993/en1995), `dispatch_{insert,remove}_item/set_field/apply_remedy` signatures, `scalar_patch!`/`list_delta!` rows vs model (patchck clean for en1992/en1993), `FacetLeaves` literals (4 twin crates per model/mutations crate, `LEAF` consts, `include_str!` relative to the moved file, dependency declared), `live_scene!`.
- Dependency/feature wiring: every `dep:`/`crate/feature`/`features = [..]` entry resolves; `feature = "x"` always declared; no dev-only dependency used in non-test code (except Fixed 9); all manifests parse; workspace members equal `script.ts` package lists; mountck/mutck: every mutation variant imported and counted (148/545/205 kinds).
- Viewers of the three artifacts are line-identical to the H-reviewed en1994 template apart from names; editor commands match the shared `dispatch_*` API.
- Hand-read in full: en1993 `warping_torsion` (borrows of the `put`/`end` closures, array types, tests), `plate_details`, en1992 `fire-capacity`, `fatigue-range`, `liquid-pressure`, en1995 entity checks `taper_results`/`cutout_results`, new crate `scene-entities` `kit`, `typed`, parts of `rows`, scene `kit::Out` + `impl entities::Sink`, `Spec`/`SceneEntity` mapping, spec-trace test.

## Limits
Expression-level typing, trait resolution and borrowck beyond the files named above are not covered. Files saved after about 03:00 were not seen; en1995 `scene-entities/🧩️cases` and the deck collection (v9) were created while I reviewed and are unread beyond name resolution.
`- ready for rebuild: compile-review I` appended to `build-en1992/1993/1995/latest.md`; for en1995 it holds once Open 1 is done.
