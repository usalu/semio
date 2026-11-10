# guestrt lane result (11/09)

## Task 1: wasmtime adapter port (semio-plugin-host-guest-runtime), edits done, verify pending
- WIT is authoritative (reactor.wit, jobs.wit, checkpoint.wit): `poll(budget)`, `checkpoint(budget)`, `restore(budget)`, `start-job(job, budget)`; events / restore state / job source travel as `ActorInputWire` through `admit-actor-input` (guest `take_poll` remembers the shape).
- execute_turn: budget = `context_budget(budget.retained, budget.retained_policy, budget.work_fuel, budget.deadline_us)` (same shape as the existing admit/close_actor_context); wire = `retained_turn_wire::poll_wire(events, shape)` (new, host-codec's `event_to_wire` -> DslValue); admitted_shape filled: an empty-event poll with the same shape skips the wire; set only after a non-trap poll, cleared on restore.
- start_job: source moved into `ActorInputWire::StartJob`, encoded, taken back (no copy); admission budget from the job budget (`admission_budget`); refused admission = typed `refused_start_from_wit` reply with the source returned; start crossing runs under `after_admission(budget, spent)`; reply charges admission + start (`start_after_admission`). `source == None` is refused (WIT start-job needs the job id; the only caller always passes Some).
- checkpoint/restore: `self_issued_context_budget` (the codec doc names it for pure calls); restore admits `ActorInputWire::Restore` first (`admit_wire`, renamed from admit_codec_wire, pub(super)).
- No host-codec dependency needed (hostcheck note outdated): `start_from_wit` is re-exported by host-wit; `source_to_wit` is not needed (schema-first: the source is a wire variant).
- snapshot `close_host_step`: effect-shaped through `OriginalRetirementPump::turn(..).with_effect(..)` (`pumped`, Pending/unadmitted = no progress, failure -> into_value_error).

## Task 2: receipt overflow
- reactor `🌉️call`: `debit_fault`(+retirement) retyped ValueError -> OriginalTurnFailure; receipt:82 chain restored value -> cleanup -> debit -> close. close_source_header_step only runs while no close/debit fault exists (has_close_failure fence), so close is never overwritten.

## Update (11/10): verification state
- Host (native): `chk.sh W-guestrt-3 Cargo.toml native -p semio-plugin-host-guest-runtime` rc 0 (also fixed extA's snapshot literal: `budget.retained.work.clock_binding`). Dependent host crates (host-wake/-wit/-codec, full hostcheck.sh) NOT yet run: task 3 waits for window-config --tests budget (extB2).
- Source budget: W-guestrt-1 was refused (+755 B); fixed by deduping the six job-call fns into `arm_job` + `job_receipt` (dispatch file 41 KB -> 34 KB). Never raised the limit.
- Accounting decision (coordinator ruling): the guest receipts the admission in the `admit-actor-input` reply only (poll receipt does not cover it). Host sums once: poll = `receipt_after_admission(receipt, charge)` with the poll budget reduced by `remaining_after`; start-job = `start_after_admission`. No double count.

## Guest side (edited, NOT compiled)
- New `⚛️reactor/🧬️wit-bridge/🎟️event-lift/🦀️.rs` (`event_from_value`, inverse of host `event_to_wire`; wasm-gated mod `event_lift`).
- `🌉️call/🎭️exports`: new statics ORIGINAL_WIT_INPUT/SHAPE; `admit_input_wit`, `close_output_wit`, `codec_input{,_plugin,_plugin_none}`, `wrong_codec_{code,operation}`; `poll_wit(runtime, ActorContextBudget)` (events from `take_poll`, Wire::Budget rebuilt from budget + TurnShape); `start_job_wit(job, budget)` (source from the admitted `StartJob` wire, job id must match); failure trio `publish/read/acknowledge_actor_failure_wit(reference, budget)` return a typed UnsupportedOwner refusal with zero spend (DEBT: no WIT failure frame; the cause stays in guest custody, unpublished over WIT).
- `macros/⚛️wp4-reactor`: failure trio signatures changed to (reference, budget) -> ActorFailure*Call.
- Receipt: `debit_fault` retyped to OriginalTurnFailure, restored in 📤️receipt:82 chain (not compiled by any run yet).

## Blocked: wasm proof
`hub.sh W-guestrt-hub-1` and `chk.sh W-guestrt-wasm-1 ... wasm -p semio-plugin-reactor-wit-bridge -p semio-plugin-reactor-context` stop at `semio-plugin-decl-views` (237 errors, RetireOwned/RetirementCursor trait migration, 11 files under 📜️declaration/👁️view; plus window-config-original 7, window-transient 4, others). Unclaimed (flowdoc2 released it as too large). No error points into my guest files, but they are downstream and were not reached. Needs its own lane first.

## Next steps
1. Someone ports decl-views; then rerun `chk.sh W-guestrt-wasm-1 Cargo.toml wasm -p semio-plugin-reactor-wit-bridge -p semio-plugin-reactor-context`, fix guest errors (likely: Wire type paths, `component_persistent_local!` static forms, `refusal_code` path, Budget field Copy), then `hub.sh W-guestrt-hub-2`.
2. After window-config --tests fits: `"$C/1105b-tools/hostcheck.sh" W-hostchk-8`.
3. Debts: WIT failure publish/read/ack frame; `start_job` with `source == None` refused; Poll shape skipping relies on guest-remembered shape.

STATUS: BLOCKED host FIXED-verified native W-guestrt-3; guest-side WIT port edited but unverified, wasm proof blocked by unclaimed semio-plugin-decl-views (237 errs); hostcheck.sh waits on window-config --tests budget
