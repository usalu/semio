# Original Job Loan Readonly Audit — 2026-10-13

Read-only source inspection; no native execution or passing-test claim. Root owns the existing ticket lifecycle. Paths below are relative to `/Users/ueli/Documents/semio`. Let P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`.

## Smallest Existing Paid Context Path

`P/⚛️reactor/🎟️context/🦀️.rs:53` exposes generic `OriginalActorContextSlot::dispatch_step<T,E>(input: RetainedTurnInput, budget: StepBudget, recipient: &mut RetainedCloneProgress, clock: fn()->Option<u64>, produce: impl FnOnce(&mut StepContext<'_>)->Result<T,E>) -> OriginalActorCall<T,OriginalActorCallError<E>>`. It validates original identity and increasing epoch, requires budget.retained == input.grant, and borrows the already admitted cancellation root and payload ledger through `OriginalActorContext::borrow_context` at line 23 (`StepContext::with_payload_authority`). It returns original input plus the external four-axis spent recipient (line 41). Admission births are separate paid yields at lines 21 and 51; no semantic input is consumed first.

Actual WIT and owned guest exports share `ORIGINAL_ACTOR_ENTRY` in `P/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs:8`. `with_entry` at line 14 is private. `OriginalActorGuestEntry::dispatch_step` (`.../🌉️call/🦀️.rs:18`) currently requires semantic `Fault`; it captures that exact Fault in its retained field. It cannot directly preserve `ValueError` returned by registered jobs. The smallest extension is a typed paid dispatch/loan method on this same entry using its existing context slot, then a jobs export method in this exports module borrowing this same TLS entry. Do not create a separate context for each job or bypass a retained actor fault fence. A ValueError must stay in explicit paid custody or pass as a typed owned result; converting through strings loses original custody.

The fixed owned receipt is resident instance storage: `.../🌉️call/📤️receipt/🦀️.rs:8-21`, 104 bytes; exports lines 33-35 lend that exact buffer, and deallocation only releases its loan. Existing variants cover admission, close, failure, not job success. WIT native conversion and progress mapping are exports lines 16-19; both return original full five-axis input and four-axis progress. WIT/owned output slots (lines 10-11) remain retained if return reservation fails; context closure is blocked while outputs are held (line 24/32).

## Registered Job Custody

`P/⚛️reactor/💼️jobs/🦀️.rs:191-217`: `OriginalJobAdmission` owns job/kind/input/checkpoint; JobSlot owns source, source retirement, body, output, output retirement, rejected output and retirement. `start_job(&mut Option<OriginalJobAdmission>, &mut StepContext)->Result<bool,ValueError>` at line 260 moves the exact admission handle only after funded item charge. Denial, cancellation, saturation, duplicate errors keep the caller's Option intact. Therefore guest wrappers must store refused/pending original admission across calls; the current ABI local Option dies on refusal.

`step_job(job, JobBudget, &mut IoRunControl, &mut SqliteSnapshotControl, &mut StepContext)->Result<JobStep,ValueError>` at line 306 rejects differing operation/generation at line 310. Execution ValueError exits by ownership to caller, while factory admission errors become retained `JobBody::Refused` (line 325) and paid `AdmissionFailedJob`/`RetainedExtensionFaultReply` (lines 329-334). Output remains in slot until body and original source have paid closure, then transfers once (lines 317-320). `cancel_job` at line 279 only sets original slot flag and invokes owner's cancel; following `step_job` calls drive funded body, outcome and source retirement (lines 311-315). There is no separately exported job-close endpoint. Context close cannot substitute for this slot retirement.

## Actual Crossing Gaps

`jobs/🦀️.rs:344-392`: current ABI wrapper mints `Generation(0)`, OperationId(job), StepContext::new, root_cancel_token, independent sequence/receipt; step widens policy and demand via max. It is not an original actor loan. It also converts ValueError to dynamically formatted Fault and installs always-true decode/encode/snapshot callbacks. Replace both twins with calls borrowing actual entry and exact supplied input/budget/receipt.

WIT `P/🧬️schema/📜️.wit:1435-1446`: job-budget only fuel/deadline; start returns plugin-error, step returns plugin-error, cancel void. No retained input/spent exists for jobs. Reactor original input/receipt types already exist at lines 1139-1183 and admission/context-close exports at 1319-1320.

Actual WIT guest twins: `P/⚛️reactor/🏃️runtime/🦀️.rs:4975-4997`; actual owned twins: lines 5221-5256. They currently call minted jobs ABI and serialize failures to Vec bytes. Owned input structs are `P/🦀️.rs:156-172` and lack retained input.

Host `P/🖥️host/🦀️.rs:764-766`: GuestRuntime start(job,kind,Vec)->Result<(),TurnFault>, step(job,budget)->Result<JobStep,TurnFault>, cancel(job)->Result<(),TurnFault>. None accepts original per-turn loan or receipt recipient. Wasmtime implementation at 2105/2121 arms watchdog then makes WIT call; watchdog is a host ceiling, not authority. Budget mapping at 2128 contains fuel/deadline only. Dispatcher twins at 2260/2271 forward current signatures unchanged.

Owned interpreter `P/🧠️interpreter/🏃️runtime/🦀️.rs:911-934`: start_actor_job eagerly builds JSON Vec from OwnedStartJobInput and resumes max fuel; step_actor_job eagerly JSON encodes budget; cancel abandons in-flight owned operation then starts a JSON cancel. Original actor admission/close methods at 905/907 already call actor_context_call and parse exact fixed receipts. Extend this actual crossing, not only wrappers or mock.

## Reusable Paid Producers

`🧰️framework/🔨️modules/🧵️job/📬️outcome/📦️builder/🦀️.rs:31-41`: RetainedJobPayloadBuilder initialization, append_original(cx, source:&[u8], cursor:&mut usize), seal(cx) consume the same StepContext and retain partial copied page output. The payload authority comes from original StepContext. Suitable for original byte staging with paid initialization/copy/seal yields, but not directly a contiguous String/Vec ABI input: eventual materialization requires its own explicit paid owner.

`P/🧩️composition/📨️emission/🌱️genesis/📄️input/🦀️.rs:15-45`: PrivateChildGenesisInput has capacity demand, advance(source,grant), take_ready, begin_close and close_granted. Separates encoding source work, paged backing capacity, copy and release; domain-private genesis source means this is a pattern, not a generic job source helper.

`🧰️framework/🔨️modules/🌱️value/🔗️read/🦀️.rs:159` and `🔗️read/🪪️lease/🦀️.rs:38`: original read sources quote source-copy bytes and preserve same source pointer with funded admit_source. Useful for source custody rather than eager clone. No confirmed generic contiguous job kind/input staging producer found in this bounded audit; do not claim the existing JSON Vec builders are paid staging.
