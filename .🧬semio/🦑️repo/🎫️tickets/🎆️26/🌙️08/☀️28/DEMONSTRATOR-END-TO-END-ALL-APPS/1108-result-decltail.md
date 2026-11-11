# 1108 decltail result (window-kits, decl-media, decl-emission)

## Edits
- window-kits `📇️registry/🌱️original`: owner and mounted alias both close through `artifact_retirement_admit_shared_lease` (`ArcLease<AppDefinition>` elects the last payload), `artifact_retirement_shared_lease_birth_demands`; index `advance(effect)` pays `effect.performed(1)` per unit and reserves pages with `reserve_one_funded(effect)` (Denied keeps state); `AppActionRegistry::advance_original_index(effect)`; the caller in `instance/…/🌱️capture` runs it through `cx.with_original_effect` (+`consume_fuel`/`consume_retained`), its `units` array shrank to 2.
- decl-media: `gated_arc_owner!` macro (inner RetireOwned via gate payload seed, handle RetireOwned as `ArcLeaseRetirement`) for ArtifactOutputChunks, MediaExportCompletion, MediaExportCredit; handle `retirement_seed`.
- decl-emission: task `source` (AsyncTaskRun `work`/`close_step(body,effect)`, `AsyncTaskRunSource` inherent close_step + `native_retirement_reply!`, cold dispose via `OriginalRetirementPump`), `lifted/preparation` (effect-shaped `step`/`close_step`/`retire_one`, `factory_retire_owned`, `plugin_settle_poll`, `retirement_work` quotes, both retirement cursors with `native_retirement_reply!`), `metadata/view` (`funded`, `close_pending`, `begin_close` helpers replace `admit_original_construction`/`select().step(g)`), ViewLease via `ArcLeaseRetirement`, ActionMeta via tuple seed.
- Public API change: `prepare_child_original/_one/_preview_*` take `&mut RetainedAllocationEffect` (was grant) and return `Result` (work is on the effect); `ChildEmitPreparation::step/close_step` take the effect. Production callers moved to `with_original_effect`/`granted_effect`: retained-command-job, plugin job (`👷️job`), typed-operation-unit, sequence import. These four caller crates were not compiled by this lane (their own crates are not in the norm closure reached yet).
- Store: `BatchRetirement::work` and `MemberStoreOwnedBatch::next_work` (close work quotes).

## Verified
- W-decltail-4/6 (wasm32-wasip2, root Cargo.toml): window-kits, decl-media, decl-emission compile, 0 errors (warnings only).
- W-decltail-9 (wasm32-wasip2 via `🌎️hub/Cargo.toml`, rc0): all three.
- W-decltail-10 (native, root Cargo.toml, rc0): all three.
- W-decltail-8 (norm app-surface, wasip2, manifest `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/📦️packages/🦀️rust/Cargo.toml`): the three no longer fail. Norm is still rc1 behind NEW unclaimed reds that this lane did not cause: `semio-plugin-job-receiving` 3 (`OriginalTurnPrimary: From<OriginalReceivingPrimary>`, `self.limits` move, double `effect` borrow), `semio-plugin-host-configuration` 4 (`job/📨️emission/🏛️host`), `semio-plugin-child` 10 (`♻️child-retirement-and-return-pump`: 3-arg `box_close_step`, `into_message`), `semio-plugin-peer` 7, `semio-plugin-decl-surface` 48 (`🌱️original` still `fn retirement`, editor-app `DraftMutation: OpBinaryReceiving`, natural-file), `semio-plugin-retained-work` 2, `semio-plugin-tool-machine` 38.
- Rerun norm: `"$C/1105b-tools/chk.sh" W-decltail-<n> "<repo>/✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface/📦️packages/🦀️rust/Cargo.toml" wasm -p semio-s-plugin-norm-app-surface`.

## Debts
- Grant-shaped tests (`prepare_child_one(grant)` in cad/flow/sequence/stdio tests, `source.close_step(g)`/`advance_original_index(g)` in window-kits registry tests, receipt tests) not ported.
- Cost quotes for `OwnedChildEmit` batch close use `retirement_cursor_birth_work()` for the typed-frame birth (no exact birth-work query exists); Pending maps to zero progress.
- `prepare_event` records its String allocation through `effect.actual()` (Some(0) before, capacity after), unverified by a heap-observing test.

STATUS: FIXED-verified W-decltail-9 (hub wasm) + W-decltail-10 (native): window-kits, decl-media, decl-emission; norm app-surface (W-decltail-8) now blocked only by non-decltail crates listed above
