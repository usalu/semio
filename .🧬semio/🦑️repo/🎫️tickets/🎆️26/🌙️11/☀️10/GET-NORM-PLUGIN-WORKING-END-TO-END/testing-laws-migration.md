# testing-laws and testing-acceptance migration

## Result
- `semio-plugin-testing-laws`: `repo cargo check -p semio-plugin-testing-laws --all-targets` from the repo root rc=0 (log `laws-check-2.log`).
- `semio-plugin-testing-acceptance`: lib test target compiles (all 2 errors fixed). The lib target built WITHOUT feature `artifact-app-testing` still fails with 173 errors; the crate gates its imports on that feature but not its modules. That was always so and is not API drift. With the feature on, which is how every dependent builds it, it compiles.
- The din4108 check (`laws-din4108-check.log`) now compiles testing-caller, testing-laws and testing-acceptance with no error. The next blocker is `semio-plugin` (lib, 19 errors): `🎞️media/📤️snapshot/🦀️.rs` (`plugin_typed_owner_original` now takes an effect, grant passed; u128 += u64) and `🪝️mounts/📜️wp2-declaration` (unresolved `NoPresenceRetirementFactory`, `NoTransientRetirementFactory`). Not in my scope.
- NOT RUN because of BUILD PAUSE #3: iso16757 check, `test -p semio-plugin-testing-laws -p semio-plugin-testing-acceptance`. I stopped my driver script before it queued them. iso16757 will probably hit the same `semio-plugin` blocker.

## OpBinaryReceiving decision (schema-first)
`ArtifactStore<P, Mutation>` requires `Mutation: StoreMutationFor<P>`, which includes `OpBinaryReceiving` (incremental receiving of binary commands) by the store schema. The bound is right. The laws helper (`drain_store_displaced_retirements`) and `assert_node_drag_edits_replay_like_a_fresh_fold` repeated an outdated copy of the bound list. They now say `Mutation: store::StoreMutationFor<P>`. No blanket impl added.

## Changes (framework, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws`)
- `🦀️.rs`: new `run_fixture_effect` (finite `AuthoringWallet`, const `FIXTURE_EFFECT_WORK` = the existing 1<<16 literal) used by `take_fixture_result_turn` and `close_fixture_result_page`; `retire_fixture_owned` uses `retired` with `plugin_settle_value`, `box_close_step(slot, body, effect)`, `admit_owned(effect)`.
- registered-app file: `StoreMutationFor` bound, `Future::poll` path, store constructor close through `run_fixture_effect`.
- fixture-dispatch file: new `admit_document_archive_load` (renews turns until admitted, re-exported) used by `load_document`; `plugin_load_document` uses the admission and a command-less `plugin_exchange_idle` helper over a fresh `OriginalCommandExchangeState`; `FixtureOriginalJob` re-based on `OriginalWorkFixture` (the old context had no Source parent, so original receiving could not run); `new(turn)` keeps its signature and uses operation, generation, deadline and fuel of the turn. `turn.grant` is no longer used, the source stock is `OriginalWorkFixture::stock()`.
- acceptance (`🧪️tests/🧪️history-edit-acceptance`, `🧪️node-drag-history`): `close_original_effect_step` + `ControlledRetirement::step(body, effect)` (2 sites), archive load through `artifact_app_laws::admit_document_archive_load`, `StoreMutationFor` bound, unused imports removed.

## Notes and open points
- Other test crates still call the old 3-arg `begin_document_archive_load` (folder-reload-route, plugin-runtime-plugin-builder-contract-part-*, time-travel, history-edit-acceptance siblings); the time-travel tests call `FixtureOriginalJob::new` (signature unchanged) with `receive` semantics now through a Source turn. Unverified at runtime.
- `FIXTURE_EFFECT_WORK` has no framework quote for a page close or presentation; it keeps the previous literal.
- Logs: `laws-check-1..4.log`, `laws-din4108-check.log` (generated; delete when the ticket closes).
