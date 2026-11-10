# 1108 instance2: fault-publication callers + instance crates

## Step 1 (EDITED-unverified)
- plugin `👷️job/🦀️.rs`: `close_owned_turn(effect)` now effect-shaped (`OriginalRetirementPoll`), called via `close_original_effect_step`; fault_publication.close_step(effect), close_factory_ticket(slot,body,effect), admit_owned_retirement(&mut Option,effect), A::admit_completion_retirement(&mut Option,effect), ControlledRetirement::step(body,effect).
- media-export `🦀️.rs`: `close_turn(effect)` same shape (box_close_step(slot,body,effect), artifact_retirement_admit_owned(..,effect), snapshot lease close_step(effect.grant()) kept as pure bookkeeping, metadata owner.step(body,effect)).
- INTERFACE: `ArtifactApp::admit_completion_retirement(value:&mut Option<..>, effect:&mut RetainedAllocationEffect)`; updated trait, editor/viewer traits+apps, decl-app `derived_admit_completion_retirement`, draw `📬️completion::admit`.
- NOT touched (still grant-shaped callers): retained-command `🪟️mounted/♻️frontier/🦀️.rs:86`; decl-app `🛬️lifted/♻️retirement` CompletionRetirement::close_step(grant) (stale RetirementCursor impl, calls A::admit_completion_retirement(.., grant)); draw `📬️completion` Cursor::close_step(grant).

## Step 2
- W-instance2-2 (native, plugin-job + typed-ops + instance-app) rc 1: instance crates unreached; upstream reds: semio-framework-actor (wire-turn, 13), semio-os-history (3), semio-os-store-command (1), semio-os-store-retirement (4), tool-run (13), os-inference (12), mesh-engine (3), tool-machine (14), kernel-ui (11).
Rerun: chk.sh W-instance2-N Cargo.toml native -p semio-plugin-job -p semio-plugin-instance-typed-ops -p semio-plugin-instance-app
(media-export is outside the root workspace: needs its own manifest `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/🎬️media-export/📦️packages/🦀️rust/Cargo.toml`.)
STATUS: BLOCKED upstream reds (W-instance2-2)
- W-instance2-4 (after job-native-turn green) rc 1: reds now only semio-framework-actor wire-turn (3: typed-operation/📥️receiving 25,32; diagnostic 62), semio-framework-tool-run (11, stream/update PersistentSplice), tool-machine (2). Instance crates and plugin-job still unreached. Rerun W-instance2-5 when those post FIXED-verified.
STATUS: BLOCKED framework-actor wire-turn, tool-run, tool-machine at W-instance2-4

## Step 1b: three grant-shaped cursors (EDITED-unverified)
- draw `📬️completion` Cursor: inherent `close_step(body,effect)` + `native_retirement_reply!()` + `next_computational_work`.
- decl-app `🛬️lifted/♻️retirement` CompletionRetirement: same shape; `child_step(body,effect)` via `erased_cursor_turn(child.close_step(..))`.
- retained-command `frontier:86` publication branch now `original_lifecycle_effect_step(cx,grant,demand,..)` + `A::admit_completion_retirement(.., effect)`. Rest of `granted_retirement_step` is still grant-shaped for other owners (publication_failure, actor_capture, raw_input, pending_*): not ported.
- media-export own manifest, native W-instance2-5 rc 1: unreached; reds in os-dsl (5), os-store-child (8), os-store-canonical-json (1, SharedOriginalStorage import), pack-json-output-receiving (19), pack-json-source, actor-job-state, tool-run (11), tool-typing-edit. wasm not run.
STATUS: BLOCKED upstream (W-instance2-5); cursors EDITED-unverified
- W-instance2-6 (native, decl-app + job + instance crates) rc 1: reds action-bus (2), tool-run-presentation (2), tool-run-tick-wire (2), tool-typing-edit (1), os-dsl (5), os-store-canonical-json (1), os-store-child (8). actor and tool-run themselves now pass. decl-app/job/instance errors unseen (not reached). Rerun W-instance2-7 after those post FIXED-verified.
STATUS: BLOCKED upstream (W-instance2-6)
