# semio-plugin-time-travel migration

Crate: `🔌️plugin/⏪️time-travel/📦️packages/🦀️rust` (56 baseline errors).

## Lib (verified)
`check -p semio-plugin-time-travel --lib` from the en1990 workspace: 0 errors (`🗑️generated/time-travel-5.log`).
- `♻️retirement`: actor session/pending-command steps take `&mut RetainedAllocationEffect`; `ControlledRetirement::step(body,effect)`, `close_factory_ticket(slot,body,effect)`, `admit_owned_retirement(&mut slot,effect)`, `FactoryOwnedRetirement::admit_original(original,&mut issuer,&mut output,effect)` (issuer is an Arc clone moved into the frame). Local `lifted` folds the poll (pending = zero progress). Ledger passes its effect through.
- `🎮️decision/🗑️discard`: `discard_effects_retirement_step(effect)`; the erased `close_step(body,effect)` returns `OriginalRetirementPoll` through `RetirementPollExit::settle`.
- `🗄️store-state`: `receive_step` wraps the new `with_native_receiving(&mut scope,..)` (false = unfunded, nothing ran). The source-advance sub-step needs an effect, so StepPreview dispatches it through `with_original_effect` (`preview_source_advancing`); the rest runs in the receiving scope.
- `📬️finalize`: closes via `close_original_effect_step`; replay retirement through `with_original_effect` + `with_native_retirement`.

## Idle / Unfunded
`TimeTravelReplayStep::Unfunded`: the turn could not fund the step, nothing ran, replay owner and session stay `Replaying`; the instance driver returns before touching state (`🧵️instance/🏃️turn-driving-and-replay-steps`, one-line arm) so the next funded turn re-polls. Named `Unfunded`, not `Pending`, because `Pending` carries done/total progress. Test: `replayed_report` in plugin `🧪️tests/🧪️time-travel-part-1/🧨️inert-label-op-and-hostile-input` first runs an underfunded (0 items) `StepReplay` asserting `Unfunded`, then completes on funded turns.

## typing-edit (taken over)
`semio-plugin-typing-edit`: `OriginalTypingSettlement::admit` takes a caller-resident `OriginalConstructionAdmission` (instance call site uses `machines.admissions.settlement`, like press); the owner holds its own admission (retired via `unbuilt` in close, included in `terminal_is_empty`) and `Owners.failure` for `OriginalTypingEdit::admit`; decoder identity is `(NativeAllocationIdentity,u64)`; `SharedUtf8::admit(..,&mut None)`; owners close through `retired_poll` with `owners.step(body,effect)`. Dependency `semio-plugin-job-receiving` added. `check -p semio-plugin-typing-edit --all-targets` (root workspace): 0 errors (`time-travel-9.log`).

## Tests (edited, not yet verified)
`♻️retirement/🧪️tests` run on a local `EffectBank` fixture (`run`/`settled`); dev-dependency `semio-framework-value-testing-admission` added. `--tests` / `test -p` must run from the root workspace.
Status: instance-children now compiles past its earlier errors; latest `test -p` (`time-travel-12.log`) is blocked by `semio-plugin-instance-jobs` (86 errors, mostly `Arc<..>: RetireOwned` unsatisfied and arity drift) concentrated in `🧵️instance/👷️jobs/📥️staging/🏷️reserved/🦀️.rs:139` (17), `:125`, `🧱️components/🦀️.rs:150`, `🧾️typed-operations/🦀️.rs:270`. Not owned here. The zero-item Unfunded test path is unconfirmed until the tests compile; if the fixture rejects a zero-item grant, use a grant one item below the step quote.
