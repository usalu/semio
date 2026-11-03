# 🔍️ API mismatch audit: EN 1995 to EN 1999 artifacts against shared norm crates

Scope: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/{🪵️en1995,🪨️en1996,🌍️en1997,🫨️en1998,🪶️en1999}` (about 1,400 `.rs` files). Shared crates: `⚖️compliance` (`semio-s-plugin-norm-compliance`), `🎬️structural-scene` (+ `🧱️foundation`), `📇️registry/🧬️contract/🖥️app-surface` (+ 6 sub-crates), `🧮️mechanics`, `🪟️results/🎚️config`. Read-only: no source edited, no git writes, no builds run.

## Result

**0 likely compile errors confirmed.** 0 rows in the error table below. Five items were flagged by the checks and then cleared, and a few risks are listed for the build runner to confirm.

## Checks run (all against current source)

| Check | Scope | Result |
|---|---|---|
| Every `use semio_s_plugin_norm_{compliance,structural_scene,app_surface,results_config,mechanics}::…` item exists and is `pub` (or `pub use`) | 362 distinct import lines | 67 distinct paths, all found and public |
| Every qualified path through aliases `norm::`, `scene::`, `app_surface::`, `mechanics::` | 832 references | all resolve to a `pub` definition by name |
| Struct-literal field sets (`Type { … }` and `Enum::Variant { … }`) against definitions | 1,070 literal uses, 106 types | 0 real mismatches (3 hits were string-literal commas, false positives) |
| Enum variant references (`Enum::Variant`) | 2,680 refs | all exist in some shared enum; `QuantityKind` and `CheckStatus` are duplicated, see risks |
| Argument counts: `Type::fn(…)` and qualified free fns `crate::fn(…)` | all shared fns | all match; 20 hits in `dispatch_*` were closure-comma parsing artifacts (`\|document, edit\|`) |
| `NormFamily`, `IncrementalNormFamily`, `UnitSource`, `NationalAnnex`, `NdpEntryAnnex` impls | 15 family impls | all required items present; signatures of `family_id`, `evaluate`, `units`, `summarise`, `references`, `live_report` match |
| `UnitId::*` variants used (97 `Metre`, 56 `Newton`, …) | units table in `⚖️compliance/📐️quantities-clauses-and-text/🔢️units/🦀️.rs` | all 10 distinct names present |
| `Default` available for types used with `..Default::default()` / `Type::default()` | `CheckReport`, `ChecksViewState`, `ChecksWindowSpec`, `InspectorOptions`, `DiagramsWindowSpec`, `LaneOverlays`, `ViewportView`, `PlaybackState`, `NormSelection`, `LaneFrame`, `OutlinerState`, plus component enums | all derive or implement `Default` (`ChecksFilter`, `ViewLane`, `NormTransient`, `Viewport3dWindowConfig` by manual impl) |
| `render_*`, `lookup_norm_field_meta`, `apply_remedy_edit`, `*_value_at_path`, `parse_path` call shapes and `NormFieldMetaFn` (`fn(&str) -> Option<FieldSpec>`) | editor and viewer call sites | consistent with signatures |
| Per-package `Cargo.toml` `[dependencies]` vs shared crates used in each package's sources | 121 packages | 10 candidates, all cleared as owner-mapping artifacts (see below) |

## Cleared candidates (not errors)

| Location | Flagged as | Why it is fine |
|---|---|---|
| `🪵️en1995/…/✏️editor/🎮️commands/➕insert-item/🦀️.rs:20` (and the same in 1996 to 1999) | `dispatch_insert_item` called with 6 args, definition has 5 | Closure `\|document, edit\|` commas were counted as arguments. Real call has 5 args; matches `📇️registry/🧬️contract/🖥️app-surface/⌨️command-args-and-dispatch/🦀️.rs:59`. Same for `dispatch_set_field`, `dispatch_remove_item`, `dispatch_apply_remedy` (20 hits total). |
| `🪵️en1995/…/🧬️schema/🧬️mutations/🌍️change-annex/🦀️.rs:4` uses `semio_s_plugin_norm_compliance` | mutations package `[dependencies]` lacks compliance | The file is `#[path]`-mounted from `🪵️members/🦀️.rs:12`, which belongs to `semio-s-artifact-norm-en1995-mut-members`; that package declares compliance (line 28). Same pattern for `🎭️change-design-situation` and the other 1996 to 1999 mutation packages. |
| `…/📚️examples/…/❌️multi-fail-timber/🧪️tests/🧩️example/🦀️.rs:3` | examples package lacks compliance in `[dependencies]` | Test code; the package declares compliance under `[dev-dependencies]`. |
| `…/🧬️schema/📦️packages` (1995 to 1998) with app-surface references | schema package lacks app-surface | The non-test files live in `📸️snapshot`, `💡️inferences/⚖️checks` and `🧬️mutations/🧭️edit-rules`, which are separate packages that declare app-surface (`snapshot` and `checks` confirmed). |
| `UnitPreset` in `📇️…/🪟️visual-windows/🧾️properties/🦀️.rs` | `InspectorOptions.preset` typed `UnitPreset`, which is defined nowhere | The file changed while the audit was running. The current definition is `pub preset: UnitSystem` (line 22), which is re-exported from compliance. Re-check if the field type was changed back. |

## Risks to confirm with the build (not confirmed errors)

1. **Same-name types in two shared crates.** Glob imports (`use super::scene::*`, used in 41 files) and explicit imports can resolve one name to two different types:
   - `QuantityKind`: compliance (`⚖️compliance/📐️quantities-clauses-and-text/🦀️.rs:9`, 19 variants, no `Angle`/`Frequency`/`ForcePerLength`) versus scene (`🎬️structural-scene/🧬️schema/📐️geometry/🦀️.rs:131`, has `Angle`/`Frequency`/`ForcePerLength`/`Stiffness`). Every use of `Angle`/`Frequency`/`ForcePerLength` is in a file that globs scene, so it resolves correctly today.
   - `CheckStatus`: compliance (`✅️check-results/🦀️.rs:9`, has `Info`/`UnverifiedBasis`) versus scene (`🎨️appearance/🦀️.rs:163`, four variants). No `Info`/`UnverifiedBasis` in scene-glob files; compare with `status` values carefully in the 5 files that mix both.
   - `ClauseId`, `Quantity`, `LocalizedCopy`, `EntityRef`, `Station`, `Placement`, `EvalProgress`, `UnitId`: each has a second definition (scene or app-surface, or `UnitId` struct versus enum inside compliance). Constructors match by arity today.
2. **`EntityRef::new` arity differs by owner**: compliance `new(kind: &str, id: &str)` versus scene `new(kind: EntityKind, id, path)`. All call sites in scene-glob files pass 3 args, so they resolve to scene.
3. **Macro-generated enums** (`choices!`, `variants!`, `units!`) are not visible to plain text search; the variant and field checks above parse them textually, so a macro-only definition could be missed. Verify with the build if the runner reports `no variant`.
4. **Builder methods not in the norm tree** (`action_with`, `action_interactive_job`, `window_kind_def`, `mode_def`, `default_layout`, `default_mode_id`, `panel_tab_def`, `keybinding`, `with_args`) come from `semio_framework_plugin` (`Editor::builder`). Not audited here; check against the framework when the build runs.

## Method

Scripts and outputs were written under `TICKET/🗑️generated/audit-api/` (`index.py`, `arity.py`, `callsites.py`, `literals.py`, `lit3.py`, `check_use.py`, `fieldmethod.py`, JSON indexes). Delete that folder when the ticket closes, per the ticket rules.

Concurrency note: `InspectorOptions` changed between the first and the last read, and all indexes were rebuilt after that. The findings above reflect the source as of the last rebuild. Other agents are editing the same tree, so re-run the checks after the next wave.
