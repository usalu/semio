# 🔍 API mismatch audit: EN 1990–1994 artifacts vs shared norm crates

Read-only static audit (no rustc, no cargo, no source edits). Paths are relative to `✏️s/🔌️plugins/📕️norm/` unless absolute. Line numbers are as read at audit time; the shared-crate owner may have shifted them.

Scope: every `use`, path expression, `extern crate … as` alias, call, struct literal and trait impl in `🗿️artifacts/{⚖️en1990,🏋️en1991,🏛️en1992,🔩️en1993,🧩️en1994}` that touches `semio_s_plugin_norm_compliance`, `semio_s_plugin_norm_structural_scene`, `semio_s_plugin_norm_app_surface*`, plus `semio_s_artifact_norm_contract`, `semio_s_plugin_norm_results_config` and `semio_s_plugin_norm_mechanics` (all resolved, no findings).

Tooling: `✏️s` checker kept in `TICKET/🗑️generated/wave2-api-audit/api_check.py` (raw output `api_findings.json` next to it). It resolves module trees, `#[path]` mounts, glob re-exports, `#[macro_export]` macros and `macro_rules!` expansions, then checks arity of calls (313 call sites), field names of struct literals (718 literals), enum variants and inherent methods.

## Confirmed compile errors

4 distinct issues, 22 source sites.

| # | file:line (artifact) | used as | actual definition (shared) | suggested fix |
|---|---|---|---|---|
| 1a | `⚖️en1990`, `🏋️en1991`, `🏛️en1992`, `🔩️en1993`, `🧩️en1994`: `✏️editor/🎭️modes/✏️edit/🪟️windows/📈️diagrams/🦀️.rs:7` | `use semio_s_plugin_norm_app_surface::{render_diagrams, DiagramsWindowSpec};` (5 files) | `📇️registry/🧬️contract/🖥️app-surface/🪟️visual-windows/📈️diagrams2d/🪟️window/🦀️.rs:53` (`render_diagrams`), `:9` (`DiagramsWindowSpec`); not re-exported at app-surface root | Name the path that exists: `semio_s_plugin_norm_app_surface::diagrams2d::{render_diagrams, DiagramsWindowSpec}`. Alternative (shared owner): add `pub use diagrams2d::{render_diagrams, DiagramsWindowSpec};` in `📇️registry/🧬️contract/🖥️app-surface/🖼️visual-chrome/🦀️.rs` after line 11 (`pub mod diagrams2d;`). The app-surface root glob-imports visual-chrome, so either fix surfaces it. |
| 1b | same 5 files, `✏️editor/…/📈️diagrams/🦀️.rs:27` | `render_diagrams(BODY_DIAGRAMS, &shown, &DiagramsWindowSpec::default(), windows, locale)` | arity 5 matches `render_diagrams` at `🪟️window/🦀️.rs:53`; only the path is wrong | Same as 1a. Arguments are fine. |
| 1c | same 5 files, `👁️viewer/🎭️modes/👁️view/🪟️windows/📈️diagrams/🦀️.rs:7` (use) and `:25` (call) | same as 1a/1b | same | same as 1a/1b |
| 2 | `🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🏷️field-meta/🦀️.rs:222` | `FieldSpec { label_en: "Protection c_p", label_de: "Brandschutz c_p", unit: Some("J/(kg·K)"), choices: None }` | `📇️registry/🧬️contract/🖥️app-surface/🔖️field-meta/🦀️.rs:128` struct `FieldSpec { label, help, clause, kind, range, soft_range, step, precision, group, order, tier, visible_if, ndp, handle, affects }`; constructors `FieldSpec::new` (~194), `quantity` (~199), `choice` | Struct literal has four fields that do not exist and misses fifteen (E0560/E0063). Use `FieldSpec::new("Protection c_p", "Brandschutz c_p")`. If a free-text unit label is needed, that needs a shared constructor (shared-owner request), not a literal. |
| 3 | `🧩️en1994/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️compliance-report/🦀️.rs:3` | `use semio_s_plugin_norm_app_surface::{apply_remedy_edit, get_value_at_path, insert_value_at_path, remove_value_at_path, set_value_at_path};` | Items exist at app-surface root (checked). The crate is not a dependency of the test's package. Test is mounted with `#[cfg(test)]` at `🧬️schema/💡️inferences/🦀️.rs:141` | Add `semio-s-plugin-norm-app-surface = { workspace = true }` under `[dev-dependencies]` in `🧬️schema/💡️inferences/📦️packages/🦀️rust/Cargo.toml` (currently lines 30–34 hold only framework, compliance, serde). Compile fails under `cargo test` otherwise (E0432). |

Each diagrams row is one use line and one call line per artifact, so 10 files × 2 lines = 20 sites from issue 1; 1 site from issue 2; 1 site from issue 3. Total 22.

## Checked and OK (no mismatch found)

- All `use`/path references into `semio_s_plugin_norm_compliance` (`CheckReport`, `CheckResult`, `CheckStatus`, `ClauseId`, `AnnexChoice`, `NormHost`, `NormFamily`, `NormFamilyId`, `IncrementalNormFamily`, `UnitSource`/`incremental_evaluation::{CheckUnit, EntityEdge, UnitSource, Scope, EvalProgress, evaluate_units, audit_reads}`, `NationalAnnex`, `Station`, `LocalizedCopy`, `Quantity`, `QuantityKind`, `Remedy`, `RemedyBound`, `SubjectRef`, `CheckBuilder`, `ImposedCategory`, `NDP_TABLE`, `python_oracle::{command, run_ok, verify_cases_above}`). Each resolves to a `pub` item.
- `NormFamily` impls in `✏️editor/🦀️.rs:285-…` provide `Document`, `Mutation`, `family_id`, `evaluate`, `live_report`, `live_progress`, `cancel_live`; trait requires only `Document`, `Mutation`, `family_id`, `evaluate` (others defaulted). Signatures match.
- `NormHost<F>` methods used (`live`, `document`, `report`, `evaluate`) exist in `🏛️norm-error-and-family/🦀️.rs:124-161`.
- `structural_scene` items (`EntityKind` and its variants, `EntityRef`, `Handle`, `SceneRevision { document_seq, report_seq }`, `StructuralScene`, `Annotation::Dimension`) resolve. Struct literal fields match. Enum variant names checked against definition.
- `app_surface` free functions: `dispatch_set_field` (4), `dispatch_remove_item` (4), `dispatch_insert_item` (5), `dispatch_apply_remedy::<Fam,_>` (4), `get_value_at_path`, `set_value_at_path`, `parse_path` (1 param), `interaction_domain_id`, `render_checks_with` (5), `render_outliner_document`, `render_inspector`, `render_viewport_panel`, and the `norm_*!` macros (all `#[macro_export]`, reachable through glob re-exports): arities and names match.
- `semio_s_plugin_norm_results_config` and `semio_s_plugin_norm_mechanics` uses resolve. `results_config`'s `pub use crate::mutations::…` chain resolves.
- Macros: `semio_s_plugin_norm_compliance::incremental_laws!`, `live_report!`, `semio_s_plugin_norm_app_surface::norm_exact_store_ownership!`, `norm_exact_viewer_store_ownership!`, `norm_command_from_action!`, `norm_owned_tool_job_factory!`, `norm_live_acceptance!` all exist as `#[macro_export]`.
- `#[dyn_enum]`-generated `semio_s_plugin_norm_compliance::__semio_dispatch_NationalAnnex` (generated by attribute macro, not parsed statically): consistent with framework convention, not flagged.
- `UnitId` (quantities/units `units!` expansion) resolves through `pub use units::*`.
- `LaneFrame::default` comes from `#[derive(Default)]` at `📇️registry/…/🫧️live-lanes/🧵️lane-runtime/🦀️.rs:334`, not a missing method.
- Cargo: every artifact file that uses a shared crate has it in the owning package's `[dependencies]`, except issue 3. `semio-s-plugin-norm-compliance` used by `compliance-testing` features (`python_oracle`, `child_identity_oracle`) is enabled in all owning packages except the orphan file below.
- `NormFamily`/`IncrementalNormFamily` require `UnitSource`; `impl UnitSource for En1990Family` etc. exist per artifact.

## Not compile errors, but review

- `🔩️en1993/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/⚖️checks/🧮️clauses/🧪️tests/🔭️special-parts/🦀️.rs`: not mounted by any `#[path]` or `mod` (grep of the whole crate tree). It references `crate::clauses::…` (no such module) and `semio_s_plugin_norm_compliance::python_oracle`, which the `clauses` package does not depend on (its `[dependencies]` is empty). Dead file; delete or mount it with the correct dependency before anyone wires it in.
- Field accesses (`value.field`) on shared types were not type-checked. The checker resolves names and literals but not receivers. Method-call names not defined in the norm tree were all found in the framework (`🧰️framework`), but receiver types were not verified.

## Counts

- Confirmed compile errors: 4 issues, 22 sites (table above).
- Use paths checked: all `semio_s_*` uses in the five artifact trees (1555 `.rs` files scanned, including orphan files).
- Calls checked for arity: 313. Struct literals checked for fields: 718.
- Limits: no rustc run (build runner blocked by cache partition per brief). Field access and receiver types not checked. Items produced by `#[dyn_enum]` attribute macro not parsed.
