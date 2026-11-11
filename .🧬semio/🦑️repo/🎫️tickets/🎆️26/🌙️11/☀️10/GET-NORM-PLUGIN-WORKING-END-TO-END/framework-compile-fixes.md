# Framework compile fixes for the en1990 workspace

Check: `repo cargo --store=shared --configuration=prototype --parallelism=4 -- check --all-targets` in the en1990 workspace.

## Root cause
Commit 8fdb3e10b97 and its neighbours migrated the framework retirement/construction API (effect-threaded `step(body,effect)`, `OriginalRetirementPoll`, `RetainedAllocationEffect`, `admit_original_construction(cx,output,admission,demand,work,..)`, `try_return_to_registry_witness(effect)`), but several consumer crates were left on the old API.

## Fixed
- decl-views `📜️declaration/🪆️child/🛬️lifted/👁️read/🦀️.rs`: try_return_to_registry_witness(effect) now returns (witness,progress) and an (error,original) pair; the pre-paid work unit moved behind the return branch because the call pays its own work.
- decl-views: made `LookupRetirement`, `CursorRetirement` (2x), `AuthorityRetirement` pub (E0446 private type in public RetireOwned interface).
- window crate `📬️mutation/🦀️.rs`: address_close takes (body,effect) and returns RetirementPollExit.
- window transient `🦀️.rs`: publication trait close_step/close_ingress, maintenance_step (trait and registry), partition address, factory close, disposer close migrated to effect style.
- window transient `🎟️capture`, `🌱️registration`, `🧾️issuer`: construction moved to `admit_original_construction_settled` (fresh admission per call, same pattern as the staging/reserved precedent), close paths moved to `close_original_effect_step`, capture decoder identity type changed to `NativeAllocationIdentity`.

## Status
window-transient and decl-views now compile. Next failure is crate semio-plugin-decl-emission (74 errors, all framework): the old RetireOwned API (`retirement()` method, `RetirementCursor::close_step`) no longer exists (new: `RetirementCursor` assoc type + `retirement_seed`), plus step(body,effect) and construction signature changes, in
`📜️declaration/📨️emission/🛬️lifted/📦️preparation/{♻️retirement/🦀️.rs,🦀️.rs}`, `📨️emission/🦀️.rs:761,766`, `🧵️task/🛬️source/🦀️.rs`.
This is a full RetireOwned redesign migration; not started.

## Known leftovers
- Test files of window transient (issuer/tests, retained-window-input) still use the old API; not part of this check.
- raster plugin call to admit_original_mutation unchanged (signature kept).
- No norm-side errors reached yet.

## Round 2 (decl-emission and dependents)
Migrated to the new RetireOwned/RetirementCursor API (associated cursor type, `retirement_seed`, inherent `close_step(body,effect)` plus `native_retirement_reply!`) and effect-threaded steps:
- decl-emission: preparation (`ChildEmitPreparation` step/close/retire_one on `RetainedAllocationEffect`; `prepare_child_original`/`prepare_child_preview_original` now take `&mut StepContext` and run inside `with_original_effect`; callers in plugin `👷️job`, `🧵️retained-command-job`, `🧵️instance/.../typed-operation-unit` updated), `ViewLease`/`ActionMeta` retirement, metadata view (`admit_view_root` via `with_native_receiving` + `receive_original_receiving_work`; routing/delivery closes via `close_original_effect_step`, construction via `admit_original_construction_settled`).
- plugin `🧵️task/🛬️source`: cursor migration, cold disposal through `OriginalRetirementPump`.
- decl-media (`🎞️media/🦀️.rs`): Arc lease owners moved to `ArcLeaseRetirement`, inner types made pub.
- window-kits `📇️registry/🌱️original`: `Arc<AppDefinition>` custody replaced by `SharedOriginalStorage` (`from_cold`, `alias`, `physically_unique`); index advance now effect-based (`advance_original_index(effect)`). NOTE: its caller in `🧵️instance/👷️jobs/📥️staging/🧱️components/🏛️declarations/🌱️capture/🦀️.rs:28` still passes a grant and needs `cx.with_original_effect`.

## Status after round 2 (check7.log)
decl-views, window-transient, decl-emission, decl-media, window-kits compile. Next framework failures (176 errors, none norm-side):
- `semio-plugin-job-receiving` (77): 📥️pack, 🧾️copies, 🔀️transaction, 🏗️steps, 🎟️request/{♻️retirement,♻️reserved,🏭️reserved}, 📸️fields, 🛬️lifted/{♻️retirement(+🌱️birth),🧬️context,🚫️refusal}, 🎬️media-export, 🧾️bounded-proof-and-factory-registry: same API migration (RetireOwned seeds, `admit_original_construction` 6-arg, `RetainedCloneOwner::advance/close_step`, `NativeReceivingOriginal::begin_close/close_step`, `SealedShared::admit`).
- `semio-plugin-decl-app` (99): 👷️job/🎮️command/🧬️op-binary, 🎯️completion-and-owned-context, 🎛️artifact-app-trait (`DraftMutation: OpBinaryReceiving`, `snapshot_read`), reserved-outcome-witnesses (`close_step_granted`).
