# 1030 review: plugins and hub (read-only, uncommitted diff of waves 1028-1030)

Scope: `✏️s/🔌️plugins/*` and `🌎️hub/*` via read-only git diff plus surrounding code. No builds run. Findings are verified by re-reading; each carries a confidence.

## 1. process3d mounted-registry still calls a deleted API (compile break) - HIGH, confidence 95%
- File: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🧪️tests/🔬️mounted-registry/🦀️.rs:265`
- `semio_s_artifact_process_process3d_host::process3d_release_publication_authority(...)` is invoked in the mounted close path of the test registry, but the whole `📬️publication-authority` crate (and its `pub use` in the host's `🧰️owned/🦀️.rs`) is deleted. A repo-wide grep finds no definition left; this is the only remaining caller (the same file's law at :412 was already migrated to `DocumentStorePublicationToken`).
- Failure: the process3d editor wasm/mounted-registry test target (testkit) fails to compile (E0425), so the whole target is red, hiding every other law in it.
- Fix: drop the call (the token has no process-global lease to release); the following slot clearing already retires the operation.
- Owner lane: resid (process3d tests) / runtime.

## 2. Energy engine projection silently drops rows that fail conversion - MEDIUM, confidence 70%
- File: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧳️state/👁️read/🦀️.rs` (`engine_row`: `E::from_value(row.to_value()).ok()`; `engine_rows`: `filter_map(engine_row)`; `Rows::get/find`).
- Scenario: a native row whose canonical value fails `E::from_value` (e.g. a range or enum the engine type refuses) vanishes from `Rows::iter()` with no fault. The engine then simulates a model with a missing zone/surface/construction and returns plausible but wrong numbers. In editor reduce (`🧮️reduce-and-helpers`), `model.zones.find(..)` returns `None` for such a row and the command is reported as `target_missing`, a misleading fault.
- Second effect: `Rows::positions` returns indices into the native list while `iter()` ordinals shift after a dropped row, so index-pairing the two diverges.
- Fix: make `engine_row` return `Result`, and surface the first failure as a typed fault (`energy.model.projection`, which the old code in reduce did raise via `map_err`). The old code failed the whole reduce loudly; the new code regressed that to silent.
- Owner lane: energy.

## 3. Energy native-cold bridge: full-model copies per mutation, and a half-applied error path - MEDIUM, confidence 65%
- Files: `✏️s/🔌️plugins/🔋️energy/…/🧬️schema/🧬️mutations/🌉️native-cold/🦀️.rs` and `…/🧬️schema/🔺️diff/🌉️native-cold/🦀️.rs`.
- `ColdMutation<Native>::diff`, `::inverse` and `ColdMutationDiff<Native>::apply` each call `base.to_reference()` (a full plain-row copy of the model), and `apply` additionally runs `from_reference` over the whole result. One edit on a BESTEST-size model therefore costs 3 full projections plus 1 full rebuild, on the interactive path (drag/slider edits), with no progress or cancellation. This contradicts the persistent-row design the same wave introduced (`Rows`, "nothing beyond the consumed prefix is copied").
- Error path: in `apply`, `retire(std::mem::replace(base, next))` runs after `base` was already replaced. If retirement fails, the function returns `Err` although `base` now holds the mutated snapshot, so the caller treats an applied mutation as refused (store and ledger diverge).
- Fix: swap `base` only after retirement succeeds, or return `Ok` and carry the retirement fault separately; longer term implement per-field cold apply on the native snapshot.
- Owner lane: energy.

## 4. Cold one-item preparation replaces bespoke size guards with a constant footprint - MEDIUM-LOW, confidence 55%
- Sites: playground `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/…/✏️editor/🦀️.rs` (was `PLAYGROUND_STORE_MAXIMUM_BYTES = 8_192` on `new_schema` and `base.schema`), playbook `✏️s/🔌️plugins/📖️playbook/…/✏️editor/🦀️.rs` (was `PLAYBOOK_STORE_MAXIMUM_BYTES = 32_768` on mutation and base). Both now use `store::cold_mutation_apply_preparation_factory` (framework file `🧰️framework/…/🏪️store/📬️paged-one-item/❄️cold-apply/🦀️.rs`).
- The shared factory prices every mutation at `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES` in `preflight`, runs `inverse` + `diff` + `apply_diff_cold` (a full clone of the base) in one step, and reports `RetainedCloneProgress::default()` with `completed_bytes = 1`. The clone is therefore not metered against the grant, and the per-artifact envelope (an oversize schema string for Playground, an oversize recipe config for Playbook) is no longer refused at preflight; it is only caught later, if at all, by `prepare_one_item`.
- Failure: a 1 MiB `ChangeSchema` text on Playground is now accepted into the ledger where it was refused with "exceeds its fixed Store envelope"; no law covers it (the old envelope tests were removed with the code).
- Fix: keep a per-family `max_bytes` hook in the shared factory (footprint = encoded mutation bytes), and charge the base clone to `copied_bytes`.
- Owner lane: runtime (shared factory), playbook (call site).

## 5. Deleted publication leases lose checks and the fail-closed tests - MEDIUM-LOW, confidence 50%
- Sites: generation2d host (`🪪️publication-leases`, `🧮️publication-credits`, `🎭️publication-hostile`), generation3d host `🦀️.rs` (about 290 lines), process3d `📬️publication-authority`; replaced by `DocumentStorePublicationToken::validate_live` (generation equality only).
- Lost behaviour: operation-duplicate refusal, base/parent/live revision checks (wrong-base, wrong-parent), the `authority-missing` refusal, and domain credit checks (`maximum_output_pages`, `maximum_controls`). Item credit is now the constant `*_MAXIMUM_DOMAIN_ITEMS` in the envelope decode authorities. The five hostile-lease laws per artifact (Missing, WrongOperation, WrongGeneration, WrongBase, WrongParent) and `vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` were deleted; only one stale-generation law remains per artifact.
- Scenario: a publication minted on generation N whose live store advanced its revision without bumping the generation (base revision moved, generation equal) passes the token check; the old lease refused it as wrong-base. Whether the framework can produce that is a framework question, but no law guards it any more.
- Fix: add one token law per artifact for operation mismatch (token minted for another operation id) and one for the framework-owned base revision; otherwise document that generation equality is the full contract.
- Owner lane: stepctx / runtime.

## 6. Hub gis inference: synchronous one-turn driver and generic member owners - LOW-MEDIUM, confidence 40% (unverified at runtime)
- File: `🌎️hub/🧩️compositions/🌍️gis/💡️inference/🦀️.rs` (`run_controlled`, `sync_grant`, `member_store`).
- (a) `run_controlled` builds a fresh `StepContext` with fixed `OperationId(1)/Generation(1)`, a `maximum_items: 1` retained grant and a clock returning `Some(0)`, and demands the first turn to complete (`step.execution.and_then(canonical_payload).ok_or(Execution)`). A job that needs a second turn (pack above the grant) reports `MapBindingRefusalV1::Execution`, indistinguishable from a real failure. Operation id 1 collides across concurrent calls if the framework keys retained custody by operation id.
- (b) `parent_store`, `drawing_store` and `value_store` moved from the bespoke owner catalogue (`gis_map_document_store_owners()`, `MemberStoreOwner::member_store_owners()`) to `funded_bounded_artifact_store_owners::<P, M>()`. If the generic catalogue lacks the retained owners for GIS mutations, the first mutation on a composed member store faults at apply time, not at construction. Worth one law that applies a region-group mutation on each member store.
- (c) `GisMapInference::infer(..).map_err(|_| Composition)` flattens the typed Result into one code, hiding the cause.
- Owner lane: gen / stepctx.

## 7. Minor
- `generation3d/…/🚪️io/📝️text/🔣️value/🧊️snapshot-and-catalogue-values` and `…/🧬️mutation-delta-and-canonical-values` are now almost empty shells with blank lines and a docstring describing deleted content (`is_false` and many `use` lines are probably dead): stale docs, expect unused-import warnings. Owner: gen.
- The debug-strip left whitespace-only lines in fem 2d tests (`remove-selection/…/unit`, `canvas-gesture/…/unit`); cosmetic. No leftover `[DEBUG]` was found in added plugin lines.
- Demonstrator bundle `🪪️manifest/🎪️demonstrator/🦀️.rs` requests three capabilities with `optional: false` (artifacts.write, ui.dialog, shell.navigate) for the whole bundle; if a host denies `shell.navigate` (only gis2d's external URL needs it) the entire bundle fails to activate instead of degrading. Consider `optional: true` for that one. Owner: bundle.

## Checked and found sound
- Flow presence dropping `derive(RetireOwned)`: `artifact_retire_struct!(FlowPresence{..})` in `♻️retirement` supplies it (no double impl, no missed retire).
- generation2d/3d DSL, pack and sqlite codecs moved into the model crates verbatim (pure relocation; orphan-impl owner is now the type's own crate).
- phased-job fuel to `StepContext` migration: `spend`/`should_yield` loop terminates; `ReadyJob::new` (no source) now steps, previously it always refused, which is intended.
- Energy inspector select lists keep the `SELECT_ITEMS_MAX + 1` overflow probe, so the existing overflow branch still triggers.
