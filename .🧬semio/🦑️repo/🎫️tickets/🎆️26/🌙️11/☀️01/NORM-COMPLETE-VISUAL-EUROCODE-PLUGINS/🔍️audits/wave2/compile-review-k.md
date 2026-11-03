# Compile Review K (wave 2): en1996, en1997, en1998, en1999 and the shared crates (files saved after 02:45)

No cargo, rustc, build or git write. Window 03:05 to 03:30; owners kept editing (en1998 sqlite/edit-rules, shared incremental-evaluation, forms, handles), so statements are as of the scan stamps. J owns en1990/91/94 and was not duplicated.

## Method (scripts reused from `🗑️generated/compile-review-{a,i}` and `🧰️scripts/compile-gate-h`, output in `🗑️generated/compile-review-k/`, all niced and run sequentially)
- `compile-gate-h` gate (featck, collide, patchck, enumck, methck) on the four artifacts, plus its stock `all` run for the shared crates.
- `rsck` (imports vs definitions, struct literal completeness, arity, extern crates) on en1996/97/98/99 and on registry, compliance, structural-scene, mechanics; `exh` (match exhaustiveness), `littype` (struct literal field types), `argtype` (call argument types), `fieldscan`/`methar` (fields/methods that exist nowhere), `devdep`, `cfgfeat`, `arrlen`, `depck` (imports vs Cargo deps).
- Hand checks of the K list: `ActionFactors`/`ReliabilityClass`/`DesignSituation` API against `⚖️action-factors` (all five callers, methods `gamma_g_sup/inf`, `gamma_q_building`, `psi`, `k_fi`, `gamma_g_sup_610b`, `in_situation`, `serviceability`, `domain`, `gamma_q`); `reliability_class` threading in en1996 (model, snapshot, artifact, diff, evaluate/design_effects/gamma_q callers); `NormEditRules` literals incl. `move_item` and the rule structs (field sets of all 4 `EDIT_RULES` files counted against the definitions); `norm_live_editor!` 3-arg `scene = |d, r, _|` closure vs `live_scene(document, view)` and `live_scene!` 3-arg `scene`/`preview`; `KitDoc`/`ViewOptions::none()`; `QuantityKind::Velocity` in both enums (compliance 20 variants, scene 18) and all three exhaustive matches; `RemedyBound` six variants in `edit-rules`, `remedy-chips`; `ShortcutAction` in `route_of` (all variants), `shortcut_action_id`, `dispatch_shortcut`; `MemberKind::{Beam,Column,Wall,Brace}` codec; mechanics `continuous_beam`/`oracle-wiring` vs `Model`/`Material`/`Section`/`Support`/`Inputs`/`Outputs`/`Adapter`; `highlight`, `Owners`, `handle-kit`, `forms/layout`, `incremental-evaluation` mounts and `$crate` paths.

## Fixed (certain, after re-reading; minimal)
| # | file | class | fix |
|---|---|---|---|
| 1 | en1997 `🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml` | manifest error: feature `test-support` names `mut-wall-seismic-reinforcement/test-support`, crate not a dependency (featck) | `semio-s-artifact-norm-en1997-mut-wall-seismic-reinforcement = { workspace = true }` added after `mut-wall-loads` |
| 2 | en1998 `Cargo.toml:280` | workspace dependency `semio-s-artifact-norm-en1998-mut-site-ground` path `🏢️site-ground` while the member and the directory are `🌍️site-ground` (path manifest missing, kills the whole workspace load) | path emoji `🌍️` |
| 3 | en1998 `✏️editor/🦀️.rs:271` | E0425 `semio_s_artifact_norm_en1998_mutations::resolve` does not exist (the resolver lives in the `edit-rules` crate as `resolve_edit`; en1996/97 export `resolve_edit` from mutations, en1999 exports `resolve`) | `resolve = semio_s_artifact_norm_en1998_edit_rules::resolve_edit` (the editor already depends on that crate) |
| 4 | en1998 `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:4` | E0422 `En1998Joint` used at line 118 (new joints table) but missing from the `use crate::{..}` list | added `En1998Joint` |

## Verified clean (no finding)
- featck / collide / enumck on all four artifacts after the fixes; `patchck` lines for en1996-98 are the known `nest{..}`/list-delta parser noise (nested `Vec<..>` rows with `*Delta` types), same as H/I.
- `rsck`: only known noise (derive names, grouped `use`, closures `put`/`mk`/`push`/`choice`/`ranged`/`note`/`emit`, `Registry` literal, sqlite `store::`/`artifact::` globs, enum variants listed by `#[dsl]` attributes, name collisions `capacity`/`pick`/`chord`/`press`/`derived_yaw`/`Tick`/`EvalProgress` that are closures or other items). Each collision was opened and is a local closure or a different item.
- `exh` (match exhaustiveness): one tuple-match false positive (`UnitMaterial` with wildcard arms); RemedyBound, ShortcutAction, QuantityKind, MemberKind complete everywhere.
- `littype`: 0 (en1996/97/99; en1998 reports 32 `Grip.kind` lines in `🎬️scene/🕹️handles` that are a name collision: the local `struct Grip<'a> { kind: HandleKind }` at line 35 vs the projection-kit `Grip` with `GripKind`; opened, local struct is consistent), `argtype`: 0, `devdep`: 0, `cfgfeat`: 0, `arrlen`: 0, `depck`: clean after Fixed 2/3 (en1998 scene now declares `scene-analysis`; the owner saved it at 03:22).
- ActionFactors API usage: every call resolves; `psi_row_of_key`, `action_factors` module path reachable from the compliance root through `core::annex_and_tables` globs.
- Live-scene: 3-arg closures consistent in all four editors; `live_scene!` 3-arg; `KitDoc { snapshot, placements, view }` complete in 12 sites.
- `NormEditRules { move_item: &[], set_field, insert_item, remove_item }` complete in all four; rule literals have exactly the fields of `SetFieldRule`/`InsertItemRule`/`RemoveItemRule`/`IdRemoval`.
- Mechanics: `solve_continuous_beam`, `continuous_beam_envelope`, `beam_span_envelope_case` against the schema types (all field names and arities).

## Open / notes (no action taken)
- Shared files were still being saved at 03:15 (`direct-manipulation`, `handles`, `highlight`, `compliance/🦀️.rs`); anything after about 03:15 is unread.
- `methck` (stock gate) lists method names that exist (`gamma_g_sup`, `in_situation`, `serviceability`, `ver_en`, `verification`, `with_some`, `stamp_text`, `track_value`, ...): its method index is stale for definitions inside `impl` blocks of freshly moved files; each reported name was grepped and defined.
- Expression-level typing, trait resolution and borrowck beyond the files named above are not covered.

`- ready for rebuild: compile-review K` appended to `build-en1997/latest.md` and `build-en1998/latest.md`.
