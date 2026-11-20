# Snapshot API Migration (WFC Artifacts)

The framework retained-value and SQLite snapshot traits changed, so every WFC artifact's `🧬️schema/🔺️diff`, `🧬️schema/📸️snapshot`, `✏️editor/🫧️transient` and `🚪️io/*/📸️snapshot` sources no longer compile. The bitmap artifact was ported first. Its reference implementation follows the remodeling crate (`✏️s/🔌️plugins/📸️remodel/…/🔺️diff/🧳️retained/🦀️.rs`).

## Pattern

- **Retained apply trait (`RetainedDiffApply`):**
  - Replace `preflight`, `close_step` and `begin_close` with `preflight_reply` and `close_reply`.
  - Run them under `retained_routed!`, with a `RetainedFlight` field.
  - Preflight stops at phase `PREPARED` and leaves the field preparations unclosed. `close_reply` closes them through `begin_close_paid` and `close_into` or `close_original_into`.
  - `commit` takes `effect` and returns through `settle`.
- **Demand methods:** `preflight_demand` and `preflight_work_demand` gain `maximum_work_units`. `RetainedFieldPreparation::demand` and `advance_work_demand` gain the same argument.
- **Field preparation:** `take(grant)` becomes `take_into(&mut slot, effect, reply)` through `native_child`. A preparation is no longer an iterator and has no `advance` method.
- **`RetainedWorkPoll`:** `work` is now a method, and the `result` field is gone.
- **`SnapshotProjection`:** replace the hand-written impl with `#[derive(semio_framework_value::SnapshotProjection)]`. Add `by_value_projection` where the snapshot is projected by value, as the IDML snapshot does.
- **SQLite snapshot hosts:** `decode_sqlite_snapshot_native` and `encode_sqlite_snapshot_native` take an extra `output` slot and return `SnapshotReceivingOutcome`. The post-decode `admit_sqlite_values` row check goes away; `construct_native_record` still enforces the row limit.
- **Binary snapshot:** `decode_into` is removed, because `admit_owned_retirement` now needs an effect.
- **Tests:** port the retained tests (`🧬️schema/🔺️diff/🧪️tests/retained.rs`, `🧬️schema/🧬️mutations/🧳️retained/🧪️tests/🦀️.rs`) off `preflight`, `close_step` and `begin_close`.
- **Pack import:** `ArtifactPack` comes from `pack::snapshot`, not `semio_os_kernel::pack`.

## Reference

The ported bitmap files are under `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🖼️bitmap/🏅️standards/🔖️1/🪆️subsets/✳️any/`.

## Mutation preparation (`🧬️mutations/🧳️retained/🦀️.rs`, `RetainedMutationPreparation`)

Template: `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🎛️set-field/🦀️.rs`.

- `decision_demand`, `decision_work_demand`, `inverse_demand` and `inverse_work_demand` take `maximum_work_units`. Return the default demand and `0` work when it is `0`. A purely local state machine costs one unit per turn.
- `decide` becomes `decide_reply(m, p, body, max, effect, reply, recover)`. `inverse` becomes `inverse_reply`. A local turn returns `Err` when `recover` is set, since it has no unrecorded callback. Run the old body inside `let result: Result<RetainedCloneStep, RetirementPollExit> = (|| { ... })();` and finish with `conclude(effect, base, result, work, |s| s.progress(), reply)`. The old `Ready(p)` becomes `Complete(p)` and `Progress(p)` stays `Progress(p)`. The `take_decision` and `take_inverse` methods are unchanged.
- Close split: add `close_started`, `begin_close_demand`, `begin_close_work_demand` and `begin_close_paid(effect)` (pay one unit with `effect.performed(1)`, then set `closing`). Replace `close_step` with `close_reply(context, original, body, effect, reply, recover)`, plus `close_work_demand`, `requires_original_birth`, `requires_original_control` and `close_original_birth_step`.
  - Children close through `retained_routed!` with a `Flight` field: `retirement.step_into` or `step_original_into`, and `begin_granted(&mut field, effect)` to start an owner.
  - `FactoryVectorRetirement` closes through `polled(&mut flight, effect, recover, route, body, &mut work, |e| ...)`: `admit(&mut *inverse, &mut inverse_close, e)` to start, `close_step(context.mutations, body, e)` to continue.
  - `MutationMessageLedgerRetirement` stays synchronous: `close_step(effect.grant())` plus `work += last_turn_work()`.
- Tests: use `FixtureSource::from_owner` and `PostFixture::new` (`#[path]` to `🧰️framework/🔨️modules/🌱️value/📽️projection/👁️post/🧪️testing/🦀️.rs`, which resolves physically, so count the `..` from the real directory). Drive turns with `semio_framework_value_clone_source_testing::drive::{drive, drive_reply}`. See the bitmap tests for the `preflight`, `commit` and `close_once` helpers. Add `semio-framework-value-clone-source-testing` and `semio-framework-value-snapshot-projection` as dev-dependencies of the crate that holds the tests.

## Host inferences (`🔨️modules/🏠️host/💡️inferences/🦀️.rs`)

- `RetainedPayloadBuilder::close_step_granted(grant)` is gone. `RetainedPayloadBuilder::close_step` and `RetainedJobPayload::close_step` take `&mut RetainedAllocationEffect`. Obtain the effect from the job context with `cx.with_original_effect(grant, |_, effect| ...)`, which returns `Option<(outcome, performed)>`. Charge `cx.consume_work(performed)` and then pass the outcome (with `work: 0`) to `receive_original_retained_work`. Bitmap's `close_effect` helper does exactly this; copy it.
- `settled_snapshot::<P, Mutation>` now needs `Mutation: StoreMutationFor<P>`, which includes `OpBinaryReceiving`. Add `dsl::original_pack_value_operation_receiving!(YourMutation);` right after the mutation enum in `🧬️schema/🧬️mutations/🦀️.rs`, as grid2d, grid3d and bitmap do. It is the bounded receiver for plain `ToValue`/`FromValue` pack operations. Do not use `original_ordinal_operation_receiving!` for these enums.
- The `engine::job` types (`WfcRestore`, `WfcJob`) still take `with_retained_loan`; not changed here.

## Tests and leftovers (found porting grid2d)

- **SQLite native tests:** replace `original_owners::with_decoding(|o| T::decode_sqlite_snapshot_native(p, c, o))` with `original_owners::decode_native::<T>(p, c)`. Replace `with_encoding(|o| v.encode_sqlite_snapshot_native(enc, c, o))` with `original_owners::encode_native(&v, enc, c)`. Both return `Result<_, OriginalSnapshotRefusal>`, whose only field is a `String`. Assert cancellation with `format!("{error:?}").contains("Canceled")` instead of `error.kind`.
- **Store fixture tests:** drop the `decode_into` and starved-retirement tests together with the `admission` and `retire_all` helpers, because `decode_into` is removed. Also drop `ErasedSnapshotRetirement` from the binary snapshot imports.
- **Host inferences and fill tool:** the `Pending`-conversion of `close_publication_step` and the `.map(...)` on `with_retained_loan` become `?` followed by `Ok(match step {...})`. Use the same `close_effect` helper as bitmap.

## Editor transient (`✏️editor/🫧️transient`)

- A transient diff declared with `retained_row_patch!` and `retained_patch_diff!` also needs `protocol::retained_empty_option_patch!(Diff { field, … });` listing every option field, placed between them. Without it `Diff: RetainedPatchEmpty` is unsatisfied, and `dsl::Mutations … retained` then leaves `TransientMutation: ColdMutation<Transient>` unsatisfied too. Reference: `generation3d` `👁️viewer/🫧️transient/🦀️.rs`.
- Replace a hand-written `impl SnapshotProjection` on the transient with `#[derive(semio_framework_value::SnapshotProjection)]`.
- Unused imports and `semio_framework_job::` qualifications that the file already imports are warnings under the workspace lints; drop them.

## Editor crate (bitmap, found by the component build)

- Retained helpers are not a `protocol::retained` module from `semio_os_kernel`; import the flat re-exports: `protocol::{retained_conclude as conclude, retained_polled as polled, RetainedDone as Done, RetainedFlight as Flight, RetainedReply as Reply}`.
- Transient: add `by_value_projection` to its `#[value(...)]` so `SnapshotProjection::Output == Self` (the plain derive yields `Box<Self>`), and `dsl::original_pack_value_operation_receiving!(TransientMutation);` right after the transient mutation enum.
- Nested snapshot field types (`BitmapInput`, `BitmapOutputSpec`, `BitmapOverlappingModel`) also need `#[derive(semio_framework_value::SnapshotProjection)]`; a type used in a `Mutation` with `CanonicalJsonTree` needs it too (`BitmapPinnedPixel`).
- Fill tool `InteractiveJob::step` takes `(context, _: &mut OriginalJobReturnClaim<'_>)` and returns `InteractiveJobStep::from_callback(self.step_value(context))`.
- `context.view_state` is `Option<Arc<ViewModel>>`: pass `.as_deref()`.
- Retained command work closing a `ControlledRetirement`: `owner.step(body, effect)` returns an `OriginalRetirementPoll`. Use `close_original_nested_step(cx, |cx, grant| ...)` and `cx.with_original_effect(grant, |_, effect| owner.step(4096, effect))`, then `cx.consume_work(performed)` and `receive_original_retained_work(cx, RetainedWorkOutcome { result: turn.result, work: 0 })`. `last_turn_work()` is gone.

## Headless solve admission (found in grid2d)

- `HeadlessSite { fuel_per_step: 1, step_budget_us: 2000 }` never admits the worker: the headless driver reports `<label>-worker-admission-pending` after 4096 admission turns. Session admission quotes several work units per turn, so use `HEADLESS_FUEL_PER_STEP = 4_096` and `HEADLESS_STEP_BUDGET_US = 4_000` (strictly below the 8 000 µs interactive ceiling), as bitmap does. Confirmed in grid2d: the solves are admitted and produce output. wfc2d still uses `1` and `2000` and likely needs the same change.
