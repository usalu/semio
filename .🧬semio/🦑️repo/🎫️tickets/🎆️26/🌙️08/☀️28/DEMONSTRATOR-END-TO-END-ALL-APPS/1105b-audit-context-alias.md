STATUS: DONE (static, no runs). The premise is partly wrong: for puzzle3d there is NO second alias of the context's SealedShared. The live alias is of a field INSIDE the context, `Arc<ViewModel>` (view_state), retired as a non-lease Arc in both the context and `MountedTypedCommandFullOperation.meta`. The two retirements wait on each other: a close-order cycle. Confidence: high on the mechanism (code-read), medium on which exact Arc (not run).

Roots below are relative to R = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin` and V = `🧰️framework/🔨️modules/🌱️value`.

## 1. Primitive and the stall mechanism
- `SealedShared` (V/♻️retirement/🔗️arc-custody/🔒️sealed/🦀️.rs:28) aliases come ONLY from: `Clone` (:115), `try_duplicate` (:74), `SharedAliasReturn::drain` (:47, a raw-pointer return slot), and `admit` (:66, the original).
- Sealed `Cursor::step_selected` (:133-148): `source.retire()` returns `Alias` while counted aliases > 1. That is one progress step, and the cursor then completes. It never blocks on other aliases.
- When it is the last counted alias it yields `Original`. The context's `issuer.demand` is specialized (derive `shared_original_birth_bytes`), so it goes to `admit_cursor` and a ChildProgress child, which is the context's own `Cursor` (R/📜️declaration/🧩️app/🛬️lifted/📸️fields/🦀️.rs:48-80).
- So a wait on a context alias cannot produce "zero progress forever". Only a zero-progress retirement INSIDE the child can, and the evidence matches that. B1 quoted `{copy0 cap0 rel0 depth4}` with ZERO progress. A non-lease `SharedControlledRetirement` quotes `release_bytes=0` exactly when `Arc::strong_count>1` (V/♻️retirement/🔗️arc-custody/🦀️.rs:21) and returns `Progress(empty)` (:38-40). A lease quotes `arc_bytes` and always progresses (:35-37).
- `Arc<T>: RetireOwned` is `SharedControlledRetirement::new`, which is NON-lease (:88-89).

## 2. Alias table: SealedShared<ArtifactOwnedToolJobContext>
| # | Site (file:line) | Holder | Released |
|---|---|---|---|
| 1 | `ArtifactOwnedContextHandle::admit` -> `SealedShared::admit`: R/🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs:188 (shell lane) and R/🧵️instance/🧷️command-routes/📥️staging/🚦️actions/🦀️.rs:543 (agent preview lane) | `ArtifactOwnedToolJobRequest.context`, then the retained-command job `context` -> `input_retirement` (box) | In the job close ladder (`close_one_original`). The single counted alias. |
| 2 | `ArtifactOwnedContextHandle::try_duplicate` (R/📜️declaration/🧩️app/🛬️lifted/🧬️context/🦀️.rs:28), `duplicate_original` and `provisional_source_admission` (R/📜️declaration/👷️job/🎯️completion-and-owned-context/📥️provisional/🦀️.rs:9,16) | No caller in puzzle3d. `provisional_source_admission` is used only by procedural (✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🌊️flow-source/📥️provisional/🦀️.rs:21,31). That is the generator, not the aggregator. | n/a for aggregator |
| 3 | `SealedShared::clone` on the handle | Impossible. The handle has no `Clone`, and `Source::Admitted` is private. | n/a |
| 4 | `return_slot`, `return_alias`, `return_borrowed_alias` | Used by 🧵️job/⏱️context/🎟️source (V-adjacent tool-run source parent), not by the context handle | n/a |

Conclusion: the context has exactly ONE alias, held by the job's boxed `input_retirement`. The `completion` alias idea (the earlier reorder in s33) was a red herring. That is why s33 stayed stuck.

## 3. Live alias that survives: Arc fields inside the context (non-lease)
`ContextOwnedFields` (derive RetireOwned) retires its fields through the standard impls:
- `view_state: Option<Arc<ViewModel>>` is NON-lease and waits for strong_count==1.
- The residual Arcs (`children`, `peers`, `peer_factory`, `tool_run` entities/payload) are correctly lease-retired via `ContextResidualLeases` (R/📜️declaration/🧩️app/🛬️lifted/📸️fields/🦀️.rs:23-26, 55-61). No issue.
- `draft`/`transient`/`presence` are `SnapshotRead` registry leases (V/📸️snapshot/👁️read/🦀️.rs:653). They return to the issuer slot and do not depend on strong_count. Low risk.
- `gesture`: `GestureCapture::Issued(alias)` is a `SealedShared<GestureSlot>`. `GestureOperation.slot` is the original on the instance (R/🛠️tool-machine/🖐️gesture/🎟️issuer/🦀️.rs:23-25). The first retire to see aliases>1 returns `Alias` (progress). Only the original needs physical uniqueness. Low risk.

### Alias table: Arc<ViewModel> (the cycle)
| # | Site | Holder | Released |
|---|---|---|---|
| A | typed-command/🦀️.rs:83,191 `context_view.take()` into `ArtifactOwnedToolJobContext::new(view_state)`. It originates in R/🎮️command/📥️admission/🦀️.rs:226,228 (`admit_routing_alias` clones ActionMeta, then `context_metadata.view_state.take()` -> `PendingLatestWinsCommand.context_view`). | `ArtifactOwnedToolJobContext.view_state`, then `ContextOwnedFields.view_state`. NON-lease. | During context metadata retirement, inside the worker authority close (stage 1, `job.close_step`) |
| B | `PendingLatestWinsCommand.meta` moved into `MountedTypedCommandFullOperation.meta` (via `start_typed_command_operation(&mut pending.meta ...)`, inline-interaction/🦀️.rs:96, 112) | `mounted.meta.view_state` | `retirement_step` (R/🧵️retained-command/🔄️full-operation/🦀️.rs:585): `artifact_retirement_admit_owned(&mut self.meta.view_state, ...)`. NON-lease (`Arc::retirement`). It runs only AFTER `worker_retirement_pending() -> retire_worker_step` (:575), i.e. after the worker session and its job context are closed. |
| C | `mounted.meta.clone()` at inline-interaction/🦀️.rs:31 | local `meta` in the publication fold | Drops at fn end. Transient. |
| D | The caller's `ActionMeta` into `handle_action` (fixture/host) and the agent preview lane's `meta.view_state.clone()` (actions/🦀️.rs:546) | host or fixture frame | Transient in the production path. In the pane-smoke fixture it may be held longer (see C3 below). |

Cycle: the context's Arc (A) retires non-lease inside the worker close and waits for strong_count==1. `mounted.meta.view_state` (B) is the second strong ref, and it is retired by the ladder only after the worker close completes. A waits on B, B waits on A. Result: the zero progress forever at depth 4 that the B1 gate observed. The close order inside the retained-command job (completion vs boxed context) is irrelevant to it.

## 4. Ranked candidates
1. (most likely) The A<->B `Arc<ViewModel>` non-lease vs non-lease cycle. Evidence: quote rel0/copy0/cap0 is the signature of a non-unique non-lease Arc. B and A are both by-construction clones of the same ViewModel allocation. B is retired after the worker by explicit ladder order. The generator (procedural, no gesture or extra alias) shows the same family.
2. The fixture/host still holds the caller's `ActionMeta` `Arc<ViewModel>` (D) for the whole settle, so even after fixing 1 the last-holder wait may never end. The earlier pane-smoke note says ActionMeta.view_state is held by the fixture or host. It does not explain the depth-4 stall by itself, but it will block a non-lease retirement that is the last-to-go.
3. `WindowConfigSnapshot`/`WindowTransientSnapshot.try_duplicate` (typed-command/🦀️.rs:184,199-200) duplicate registry-lease snapshots. They are also held by `mounted.window_config_authority`, which is retired after the worker (:586-587). If their custody were Arc-non-lease it would be the same pattern. By code read it is lease-slot custody (not strong_count). Verify with a gate, but low.
4. The gesture `SealedShared` alias (not the context). Only blocks if the original `GestureOperation.slot` has already been retired and the alias is not the last counted one. Not observed.
5. The completion alias: ruled out, since the s33 reorder did not change anything and the completion is a separate cell.

## 5. Root fix (ownership/close order, no widened checks)
The invariant is: any Arc that has several issued holders retires as a LEASE in every holder. Only the last holder to drop pays the deep release. This is the convention already used for `children`, `entities`, `payload` (ContextResidualLeases) and `captured_child_content` (`admit_shared_retirement(..., true)`).
1. Context side: move `view_state` out of `ContextOwnedFields` (R/📜️declaration/🧩️app/🛬️lifted/📸️fields/🦀️.rs:16) into `ContextOriginalResidual` and `ContextResidualLeases` as `view_state: Option<SharedControlledRetirement<ViewModel>>` created with `SharedControlledRetirement::lease`. Update `into_retirement_parts` (R/📜️declaration/👷️job/🎯️completion-and-owned-context/🦀️.rs ~:239) and the `Cursor::close_step`/`demands` residual branch accordingly.
2. Mounted side: retire `mounted.meta.view_state` with `artifact_retirement_admit_shared_lease` (V/📸️snapshot/👁️read/🦀️.rs:827) in both `retirement_demands` (full-operation/🦀️.rs:549) and `retirement_step` (:585), using `artifact_retirement_shared_lease_birth_demands` for the quote. Whichever side finishes last frees the ViewModel through `Arc::into_inner`. The order between context and meta then no longer matters.
3. Same for `PendingLatestWinsCommand.context_view`/`meta` (R/🎮️command/📥️admission/🦀️.rs:237-239, `begin_granted(&mut self.context_view, ...)`), which must use the lease path for the same reason (it retires an alias of a still-held Arc).
4. Add the law test: a mounted typed operation whose context and `meta` hold the same `Arc<ViewModel>` closes to terminal in either order. The existing source-order law added for completion-before-context is not sufficient and should be dropped or reduced; it asserts the wrong cause.
5. Validate by a cheap gate before a full smoke: print `Arc::strong_count` of the view_state Arc at the stuck B1 point (expect >= 2: context + mounted.meta). If it is 1, the cause is elsewhere (candidate 3).

## 6. Corrections to the state file
- The root cause recorded in 1105b-result-aggregator.md ("completion holds a context alias") is wrong for puzzle3d. The `completion` is a separate `CompletionCell` and holds no `SealedShared<ArtifactOwnedToolJobContext>` alias. Keep the reorder only if you want it for tidiness. It does not fix the stall.
- The recorded B2 suspect ("Arc<T> with new() => lease_only=false, waits for strong_count==1") was correct. The specific Arc is `ViewModel` (view_state), shared between the context and `mounted.meta`.
