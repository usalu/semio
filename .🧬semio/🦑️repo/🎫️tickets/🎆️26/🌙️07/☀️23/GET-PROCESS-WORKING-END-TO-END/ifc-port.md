# IFC 4x3 `any` Plugin Port to Current `semio_plugin` APIs

Package `semio-s-artifact-stdio-ifc-4x3` (workspace `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.toml`, run with `--offline --manifest-path`; the crate is not reachable from the root workspace, so `-p` alone does not match). Logs: `🗑️generated/stale-ifc1.txt` (before) to `stale-ifc8.txt` (latest).

## Result

| Step | Before | After |
| --- | --- | --- |
| `cargo check --tests` | 121 errors (editor and templates) | compiles, exit 0 (`stale-ifc6.txt`) |
| `cargo test --lib` | did not compile | compiles; 42 of 80 pass (`stale-ifc8.txt`) |

## Port rules applied (the same shape the compiled plugins use)

- `semio_plugin::app_testing::*` is private. Test items come from `semio_plugin::artifact_app_laws::{OriginalWorkFixture, OriginalFixtureCaller, FixtureUiArena, fixture_mounted_policy}` (the `artifact-app-testing` dev feature is already enabled).
- Grant-shaped retirement (`close_original_retained_step(cx, demand, |grant| ...)`, `RetainedCloneClose::begin_granted(owner, grant)` / `step_granted(grant)`) is gone. Replacements:
  - owner retirement in tests: `caller.retire_owned(value, grant)`;
  - result-shaped cursors (`OriginalBuiltTreeRetirement`, receivers, overlay): `close_original_retained_effect_step(cx, demand, |effect| cursor.step(effect.grant()))` or `closed_in(cx, demand, |effect| receiver.close_step(effect))`;
  - `RetainedCloneClose` in tests: `closed_in` with `owned_step(close.begin_granted(owner, effect), ..)` and `settled(close.step_granted(4096, effect))` (the product helpers re-exported from `crate::details`).
- `StepContext::with_retained_loan` requires `R: Copy`; non-`Copy` step outputs go through a local `loaned` helper that stores the output in an `Option`.
- `ArtifactPreparedOperationCursor::advance/close` take a `RetainedAllocationEffect`; the reference-wire helper runs them inside `cx.with_original_effect`.
- Staged body build: `receive_effect(cx, demand, |native, effect| ...)` replaces `with_native_receiving(|native| cursor.step(.., grant))`.
- `Terminology` has no default (no default language or terminology): tests use `Terminology::Native`.
- `post_clone` no longer exists: `receive_original_retained_work(cx, RetainedWorkOutcome { result: Err(error), work: 0 })`.
- `render()` already takes `&ViewModel` (carries `Locale` and `Terminology`); the render tests build a `ViewModel` with `Locale::De`; no default locale is assumed.

## Files changed

Product (`semio_plugin` side):
- `🧰️framework/📦️packages/🦀️rust/🦀️.rs`: `action_staging` now re-exports `StagedFormBorrowedEdit` and `StagedFormBindingPlan` explicitly (they were public in `semio_framework_manifest_arg_schema` but missing from the plugin-facing surface; the staged-form tests need the plan item type).

Product (IFC):
- `🔎️details/🦀️.rs`: `next_entity_id`, `encode_global_id`, `fresh_global_id` are `pub` (the editor re-exports them for its tests).
- `🔎️details/work.rs`, `🔎️details/📨️wire/wire.rs`: the test probes (`cancel_at_reduction_tick`, `reduction_ticks`, `staged_native_value`, `retained_text_bytes`, `cancel_at_publication_turn`, `publication_turns`, `borrowed_inverse_wire`) are gated by `cfg(any(test, feature = "testkit"))` instead of `cfg(test)`; both crates got a `testkit` feature and the 4x3 crate enables it in its dev-dependencies.
- `🔎️details/📨️wire/source_loan.rs`: `funded_turn` runs in `with_original_source_effect` (a Source admission must follow the live turn between actions, `follow_between_actions` only rebinds when the effect carries a Source turn); `closed_cursor` and `close_cursor` start a not-yet-started clone cursor with `begin_close_into` before `close_into` (the dynamic cursor refuses `dynamic close has not started`).
- `🔎️details/✏️edit/contextual_edit.rs`: same cursor close start.
- `🏅️standards/🔖️4x3/.../🧬️schema/👁️mvd/🧩️concepts/lease.rs`: `funded_turn` uses the Source effect.

Tests: `✏️editor/{tests,receive-tests,render-work-tests,guided-type-*-tests}.rs`, `🧬️schema/🏷️templates/✍️apply/{tests,work,chooser-tests}.rs`, `✍️apply/staging-native/tests.rs`, `🧬️schema/⚖️validation/receiving-goldens.rs`, `🧬️schema/👁️mvd/header-editor-tests.rs`.

Note: `🧬️schema/⚖️validation/{receive,machine*,overlay}-tests.rs` and most `👁️mvd/🧩️concepts/*-tests.rs` still mention `app_testing` but are not `#[path]`-included by any module, so they are dead files, not compiled.

## Remaining test failures (38 of 80, all runtime, none compile errors)

These are product-protocol gaps in the IFC details/templates code (not test port issues); each needs a product change, not a test edit.

1. Source admission projection (14): `original projection must be driven or returned through its resident frame` (`✏️editor/tests.rs` retained edit cases, `staging-native` source/draft cases). After `funded_turn` moved to `with_original_source_effect`, the staged admission (`RetainedCloneSourceAdmission` / `StagedSourceAdmission`) additionally requires its projection frame to be driven (`OriginalProjectionFrame`, `finish_projection`) or returned through the resident frame. The IFC admission steps call `admission.advance(source, effect)` only. Product fix: drive the projection frame in `SourceLoan`, `IfcTemplateSourceWork` and the details work admission.
2. Clone cursor close (3): `admission birth changed its original Work or Source` on the close path of `prepared_native_render_owner_retires_the_same_donor_at_partial_frontiers` and `prepared_native_source_cancel_and_wrong_revision_keep_output_unpublished`; the close side of the admission (`admission.close_into`) still runs in a plain `with_original_effect`.
3. Guided type controllers (14, `✏️editor/guided-type-*-tests.rs`, `render-work-tests.rs`): `neutral guidance refuses`, `draft stays editable`, `must complete a body`. They run on the same admission path as group 1 (the donated request never completes its source), so they are expected to follow it; not verified separately.
4. UI tree serialization (3): `BuiltChildren requires retained page transport`. `serde_json::to_string(&BuiltNode)` no longer works for built children; `tests.rs` `revisioned_host_render_...`, `official_4x3_tessellation_...`, `mvd/header-editor-tests.rs`. The assertions must be rewritten against the built tree structure (or its retained page transport) instead of a JSON string.
5. Individual asserts (4): `tests.rs:356` (`binary_select_values...`, a diff that should be refused is applied), `tests.rs:436` and `tests.rs:108` (`Option::unwrap` on `None` in the complex-component and type-switch cases), `✍️apply/tests.rs:177` (`template.semantic-admission-unavailable` expected, a different refusal is returned), `staging-native/tests.rs:48` and `:52` (text birth denial axis and partial clone error receipt: the receipts now count the effect, not the grant).

I did not weaken or delete any assertion. The framework dev edits that were concurrent with this run (`semio_framework_job::original_debug_tag` missing in `operation-wire`, `stamp_published_failure` in sqlite snapshot) were transient and unrelated.

## Other stdio plugins

- pdf (`semio-s-artifact-stdio-pdf`, `cargo check --tests --offline`): blocked before compiling, `no matching package named lopdf` (the `pdf-test-oracle` dev dependency is not in the offline registry). Log `stale-pdf1.txt`.
- csv (`semio-s-artifact-stdio-csv`, `cargo check --tests --offline`): blocked by a framework crate that was mid-edit by another agent, `semio-os-store-operation-wire` fails with 5 errors (`semio_framework_job::original_debug_tag` not found). No csv error was reached. Log `stale-csv1.txt`. The csv `Cargo.lock` gained `semio-framework-test-original-snapshot-owners` automatically (cargo locking message), not edited by hand.
- No evidence yet that pdf or csv share the IFC breakage; both need a re-run once the registry/framework state is stable.

## Build notes

Every run waited 20 to 60 minutes on the shared artifact lock. One run died with `No space left on device` (disk at 99 %); a retry after another agent freed space worked. No new build dirs.
