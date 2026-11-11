# semio-plugin-tool-run migration

Result: `check -p semio-plugin-tool-run --lib` from en1990 is clean (56 -> 0 errors, log `🗑️generated/tool-run-4.log`).

## Changes (all under 🔌️plugin/⏯️tool-run)
- Root `🦀️.rs`: single effect-shaped `retired` helper (`close_original_effect_step` + `RetirementPollExit::settle`); the four grant-shaped copies were removed.
- Entry retirement: `entities`/`payload` moved from `Arc` to `SharedOriginalStorage` (matches decl `ToolRunView`); `replace_entities`/`replace_payload` dispose the old alias cold. Metadata close via `ControlledRetirement::step(body,effect)`, factory retirement via `FactoryOwnedRetirement::admit_original` into `self.retirement`.
- Member owners: `settled` helper, `close_tool_erased_retirement(owner,effect)`, fold/retire visitors use `self.effect`; `ToolRunMemberEmissionOwner::close_step(effect)`.
- Publication ingress: `close_step(effect)->Result<RetainedCloneStep,RetirementPollExit>`.
- Limits/job slot: failure/authority/cancel closes via effect.
- Tick: native state uses settled construction, inherent `close_step(body,effect)` + `native_retirement_reply!`; payload close through `with_original_effect`.
- Cargo: new dependency `semio-framework-action-bus`.

## API changes for other crates
- `ToolRunMemberEmissionOwner::close_step(effect)` (caller: 🧵️instance `🧹️member-retirement-and-close:46`).
- `ToolRunPublicationIngress::close_step(effect)` (caller: 🧵️instance `📤️publication-and-member-refresh:82/86`).
- `ToolRunEntry.entities`/`payload` are `SharedOriginalStorage` (instance `🚀️action-dispatch-and-start` already updated).

## Tests (migrated, unverified)
Test files moved to effect-shaped closes through a new `#[cfg(test)] test_effect` helper module in the crate root (`bank`, `settled`, `exit_step`, `cursor_step` over `EffectBank`) plus the `semio-framework-value-testing-admission` dev-dependency: publication, retirement, port admission/retirement and tick payload tests.
`check -p semio-plugin-tool-run --tests` (crate's own workspace root, log `🗑️generated/tool-run-9.log`) still stops in dependencies before reaching tool-run: media `💥️fault`/`🛫️intrinsic` (14 errors) and job-receiving `🔀️transaction` (2 errors) belong to other agents. Re-run once those compile.
