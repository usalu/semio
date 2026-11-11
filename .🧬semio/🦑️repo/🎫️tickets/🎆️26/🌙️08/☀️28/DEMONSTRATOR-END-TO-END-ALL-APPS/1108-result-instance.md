# Lane instance result

## State
- Task 1 ported (EDITED-unverified): instance typed-operations latest-wins admission on the new command registry.
  - jobs `🧾️typed-operations/🦀️.rs`: `advance_latest_wins_admission_unit(stores,pending,cx)->Result<bool,OriginalTurnFailure>`; `registry.begin(op,key,lease,cx)` bool sets `lookup_started`; rebind/outcome-take through `acted(fixed)` so denied turns keep state and charge nothing; key copy via `ToolLatestWinsKeyCopy::{admit,advance,take_key}`; `advance_latest_wins_keys` removed.
  - typed-operations `🖱️inline-interaction-and-latest-wins/🦀️.rs`: no `receipt`/`record_original_receipt`; fault stored via `.into()`.
  - jobs `close-ladder`: `close_latest_wins_command_step` uses `pending.close_step(cx)` + lifecycle delta.
  - instance `close-ladder` LatestWinsKeys rung: `close_step(cx)` + delta, demand depth 1; maintenance-ladder stage 17: cx `advance`, demand non-default unless map empty and `can_begin`.
  - plugin-app-impl debug line: `receipt` field dropped.
- Upstream one-liner fixed: reactor `🏃️runtime/📮️publication/🦀️.rs` `owner.step(child.maximum_copy_bytes, effect)`.
- Compile: W-instance-1/2 blocked by semio-plugin-time-travel (timetravel lane) and plugin-program (custody2); W-instance-4 (native, all 15 instance crates) running. gen3d note: generation3d has its own manifest (not `-p` from root).
- Tasks 2 (gen3d native/wasm, hub 8 crates) and 3 (smoke en/de) not yet run: waiting on time-travel.

## Rerun
`chk.sh W-instance-N Cargo.toml native -p semio-plugin-instance-*` ; `chk.sh W-instance-N 🌎️hub/Cargo.toml wasm -p semio-plugin-instance-ladders -p semio-plugin-instance-jobs` ; gen3d per 1108-result-gen3d3.md ; `SLOT_HANG_SECS=100 proc/smoke.sh W-instance-smoke-N en|de`


## Round 2 (after time-travel went green, W-instance-5)
- Unclaimed reds taken: instance-stores (action-ingress close_effect, ingress checkpoint pins + local-interaction query takes cx) and plugin-extension (ExtensionInvocationCause derives RetireOwned; RetainedExtensionFaultReply close_owner via with_original_effect; inference gateway close/admit effect-shaped; document receiving step via plugin_effect_turn, close via close_original_effect_step; extension retirement uses original_lifecycle_effect_step). All EDITED-unverified.
- W-instance-6 blocked upstream: semio-framework-diagnostic (NativeDefaultSlot::advance_work_demand missing, mediaenc claimed). Rerun when it posts FIXED.


## Round 3 (sync-153 landed; manifest loads)
- W-instance-10 native (15 instance crates): instance-stores and plugin-extension now compile (fixes: pack/json cursor calls via with_original_effect, pause/resume tuple errors, RetainedWorkOutcome::<_,ValueError> annotations). Remaining reds: instance-history 1 (annotation, EDITED after W-instance-10) and semio-plugin-reactor-jobs 19 (mediaenc claim 20:34). Instance crates themselves unreached.
- generation3d native W-instance-11 queued on cargo capacity (own manifest); wasm hub and smoke en/de not run.
- Rerun: chk.sh W-instance-12 Cargo.toml native -p semio-plugin-instance-*; then gen3d native/wasm; then SLOT_HANG_SECS=100 proc/smoke.sh W-instance-smoke-1 en|de.


## Round 4 (reactor-jobs green; W-instance-12 native, 15 instance crates)
- Green now: instance-stores, extension, history (annotation), children (stale crate names `semio_framework_plugin_*` -> `semio_plugin_*` in 🪆️children components; 🤝️group uses `close_empty_backing_granted`).
- Red: semio-plugin-instance-jobs, 79 errors, all in instance/👷️jobs/📥️staging: 🏷️reserved 30, 🖼️render 14 (+work.rs 5), 🧾️typed-operations 11, 🧱️components 8 (+declarations 5), 🧷️command-routes 4, 🔒️close-ladder 2.
- ROOT CAUSE (design ruling needed): `Arc<A::Snapshot>`, `Arc<A::Config>`, `Arc<HistoryView>` no longer implement RetireOwned (only ArcLease<T>/SharedOriginalStorage<T> do). `OriginalRenderWork<Arc<S>>`, pending_render_source, reserved snapshot/config/history custody all hold bare Arcs. Fix = move those fields to `ArcLease<T>` (value/retirement/arc-custody) with admission through the factory ticket, as the view-model fix in custody did; this is a cross-file render+reserved redesign, not a local port.
- Smaller: components:405 `owner.receipt.take()` (PendingLatestWinsCommand no longer has receipt; delete that arm); typed-operations 🦀️.rs `close_step`/`progress` on `!` (grant->effect for 3 callers); command-routes 4; declarations capture 3 (grant arg missing -> effect); close-ladder 2.
- runtime-ops (custody2) metadata.rs:50 needs `RetainedWorkOutcome::<_,ValueError>` annotation (unclaimed by me).
- gen3d native W-instance-11 earlier: only instance-children (fixed) and runtime-ops (custody2) were red; rerun after instance-jobs is green. Smoke not run.


## Round 5 (ArcLease split with instjobs2)
- INTERFACE posted: bare Arc custody -> ArcLease<T> (reserved cache_roots = (ArcLease<Snapshot>,ArcLease<Config>,ArcLease<HistoryView>); bind_cache keeps &Arc params). Reserved `decoder` is (NativeAllocationIdentity,u64).
- Mine, done: 🏷️reserved (ArcLease cache_roots, wire cursor via plugin_effect_turn/close_original_retained_effect_step, close via close_original_effect_step), 🔒️close-ladder:73, 🧷️command-routes:135/136 (effect close, canonical handle), declarations capture annotation.
- W-instance-16 native: instance-jobs compiles (instjobs2 FIXED W-instjobs2-3). Red now: semio-plugin-instance-docs 34 errs, claimed by instdocs (21:54) - not touched. Other instance crates after docs not yet reached.
- Not run: instance wasm, generation3d native/wasm, smoke en/de.
- Rerun: chk.sh W-instance-N Cargo.toml native -p semio-plugin-instance-*; gen3d: chk.sh ... ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.toml native -p semio-s-artifact-procedural-generation3d; SLOT_HANG_SECS=100 proc/smoke.sh W-instance-smoke-1 en|de

STATUS: BLOCKED instance-docs (instdocs, 34 errs); instance-jobs compiles; gen3d/smoke not run
