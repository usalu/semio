# Note create: where the 48,000,000 retained items go

Scope: read-only diagnosis of `artifact_create kind=s.note.note` failing after about 54 s with `BUDGET_EXCEEDED guest poll: semantic operation currency exhausted with cleanup reserve retained`, plus one small refusal-message improvement. No limit changed, no compile started, no peer file touched.

## What counts as an item

- The refusal is `physical_policy` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔔️wake/📃️policy/📥️operation/🦀️.rs` (was line 125). It fires when `intersect(drive grant, treasury remaining minus guest holds minus cleanup reserve).maximum_items == 0`. The ledger is `operation.maximumItems = 48,000,000` (`🌉️mcp/🎚️config/🏛️host-driver-policy.json`). The effective semantic request is 47,836,160 after cleanup, transit and infrastructure carve-outs (`guest-input-work-audit-2026-11-21.md`).
- An item is `RetainedCloneProgress.copied_items`. The only producer on the hot path is `StepContext::charge_retained_item` (`🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:316`, `charge_item`): one call posts `copied_items:1` plus one fuel unit.
- The paid Core interpreter (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs`, `advance_scalar`, lines 100-120) calls `charge` for every guest Wasm instruction: scalar `Push`/`Local`/`Numeric`/`Drop`/`Nop` pay `size_of::<Value>()+size_of::<usize>()`, loads and stores pay width plus value plus pc (`admit_memory_instruction`), control ops, calls (`admit_defined_frame`), vector growth and frame returns pay their own charge. So **1 item is approximately 1 guest Wasm instruction**, and the 48,000,000 item cap is a cap of about 48 million guest instructions per operation. The copy axis (3,000,000,000 bytes, about 62 bytes per item) and the item axis therefore bind at the same point; the fuel axis (operationFuel 1,000,000,000) is 20 times looser.
- `GuestPoll` results, stage and context budgets, and the per-turn `drive` grant (4,096 items) only slice this ledger; they do not add to it. Prior notes (`poll-host-copy-audit`, `poll-returned-frame-funding`) found no repeated checkout or guest-hold debit, and I found none either.

## Phase table

| Phase | Guest cost | Items (approx.) | Evidence |
| --- | ---: | ---: | --- |
| Boot, `prepare_actor` | 650,754,221 instructions (Note), 1,795,912,917 (WFC) | paid from the Infrastructure carve-out, not the semantic 47.8 M | `owned-input-core-replay-2026-11-21.md` export table |
| `admit_actor_context` x8 | 2,297,589 | about 2.3 M if charged to the operation | same table |
| **First Input (typed JSON receive of the InstanceOpen wire, 681 B)** | **1,352,007,195 instructions, 3,158 clock imports, 68 s native scalar** | **exhausts the 47.8 M item budget at about 3.5 % completion** | live-grant reconstruction in `owned-input-core-replay-2026-11-21.md`; failing run in `poll-host-copy-audit` (execution fuel 33.2 M at host step 262,144, 50.2 M on the wider witness, still one Input loan, no SRE1 yet) |
| Second Input (return/ack of the wire) | 1,212,401 | about 1.2 M | same |
| Empty 284 B Poll control | 525,525,103 first Input | | same note |
| 480 B InstanceOpen, no capability | 1,100,969,135 first Input | | same note |
| Create command turns after open | not measured | unknown | see "not measured" |

Items are an inference from the 1:1 instruction-to-charge rule; fuel and copy-byte traces in the notes are consistent with it (fuel 37.6 M at the refusal in `paid-note-receipt-hot-path`, 33.9 M in `input-cost-independent-review`, 50.2 M in `poll-host-copy-audit`).

## Live sample (this session, server pid 59511 from `.cache/mcp-fallback-bin/semio-os-mcp-new`, 5 s, 3,044 samples on one pool worker)

The worker spends 2,772 of 3,044 samples (91 %) inside one stack: `plugin_artifact_channel::ensure_instance_scoped` -> `native_poll::poll_turn` -> `GuestPollSession::advance_original_call_step` -> `dispatch_operation_step` -> `execute_current_original` -> `GuestPoll::advance_step` -> `GuestRelayUnitFlow::advance` -> `OwnedRuntime::advance_original_call_step` -> `CoreInstanceOpening::advance_scalar`. There is no repeated whole-state re-receive, no per-poll replay and no per-command snapshot on the hot path: it is one single paid VM invocation that never finishes.

Inside `advance_scalar` (inclusive samples):

- `charge_retained_item`: 996 (36 %), of which `post_retained` -> `record_original_source_retained` -> `OriginalReturnBudget` snapshot 179 and `visible_source_remaining` 258.
- `should_yield` -> `fuel_exhausted` -> `fuel_remaining` -> return-budget snapshot: 569 (20 %).
- `control::admit` about 210 (8 %), `admit_memory_instruction` 291 (10 %).
- Top of stack: `_platform_memmove` 690 (23 %), then `Result::branch`, `RetainedCloneProgress::checked_add/fits`, `OriginalReturnBudgetSnapshot::remaining/raw_remaining`.

So per item the host spends about 1.1 us (48 M items in 54 s, unoptimised `prototype` profile), of which roughly 60 % is accounting and budget observation. This matches the peer's finding that memmove leaves are dominated by return-budget observation (`return-budget-observation-cost.md`). Which guest function is executing is not visible from the host stack; the earlier Wasm symbol-map note places it in `NativeDecodeControl::receive_original_effect_dyn` under typed Input JSON receiving.

## Is the limit too low, or is the work wasteful?

Both statements are true, but only the second is actionable.

- The limit is not the defect that blocks a create: one first Input costs 1.35 billion instructions, 28 times the whole item budget. Raising the cap to fit would need about 1.4 billion items; at the measured 0.9 M items/s of the current paid interpreter that is about 25 minutes, and about 4 GB x 28 of copy accounting. A create that legitimately needs this is not legitimate.
- The work is wasteful at the source: decoding a 681-byte JSON input costs about 2 million instructions per byte (even the 284-byte empty Poll costs 1.85 M per byte). `guest-input-work-audit-2026-11-21.md` identifies the amplifiers: UTF-8 validation, then grammar steps of one unit each (`parse_work_quote` batch collapses to 1 after validation), strings walked twice with one output byte per turn, key comparison one byte per turn and quadratic in member count, a generic `DslValue` tree built and then re-received as a typed tree, and 3,158 clock imports (about 430,000 instructions per `should_yield`/clock read). Each unit passes through the full native decode, retained effect and receipt layers.

Right fix and where:

1. Guest receiving (the peer's area, `🌱️value/🛬️decode/🧳️owned` and the JSON `🫴️unique` receiver in `🎒️pack/🔤️json/📥️decode/🧳️receiving`): receive the typed record straight from the borrowed wire in batches (validate and parse many bytes per paid unit, no per-byte turn for key comparison and string materialisation, no intermediate `DslValue` tree), keeping receipts, cancellation and custody. Target order of magnitude: 1 to 5 million instructions for this wire, so that one create fits well inside 48 M with the cap unchanged.
2. Metering model (host, `🧠️interpreter/🏗️instance/▶️invocation/🦀️.rs`): items are charged per instruction although a scalar op mutates preallocated stack and locals; the earlier review already calls the `Nop` charge a concrete overcharge. Charging items only for retained allocation events (vector growth, frames, memory growth) and leaving instruction cost on the fuel axis would turn the item axis into a real memory bound. This needs an opcode-specific proof and one-byte-short tests; it does not reduce wall time by itself.
3. Host accounting cost (peer is on it): per-item return-budget snapshots are 36 % + 20 % of the hot path. Caching the visible source remaining across items within a step would cut the 1.1 us per item without changing the accounting.
4. Guest build profile: the notes show named packages at opt-level 1 and the receiving profile at 2 only behind a flag; an unoptimised guest executes more instructions per unit of work. Unverified for the live binary.

No fix makes a first-Input of 1.35 billion fit, so raising 48 M, or running the current guest longer, should not be the plan.

## Not measured

- Item counts per phase are inferred from instruction counts; no per-phase item counter exists in the live server and the peer's `dbg::` output was not read.
- Commands after `instanceOpen` (the real `artifact_create` content turn) were never reached; whether each later Input costs a similar fraction of 1.35 B is unknown (the Poll-with-event versus empty-Poll difference suggests cost grows with field count: 601 B 1.347 B, 284 B 0.526 B).
- The second sample of the same process was not taken; the single sample shows one VM invocation, so the conclusion "no repeated phases" rests on it plus the earlier traces.

## Change made (item 2 of the task)

`📥️operation/🦀️.rs`: the refusal now names the dimension and amounts. It keeps the original prefix so existing matching still works and appends `dimension=retained items, issued=<policy.operation items>, spent=<issued - remaining>, remaining=<treasury items>, held by guest=<remaining - available>, cleanup reserve=<reserve items>, turn grant=<drive/wake grant items before intersection>, spendable=<remaining - held - reserve>`. New `ItemsLedger` and `items_exhausted`. Unit test `exhausted_items_refusal_names_the_dimension_and_every_amount` in `📥️operation/🧪️tests/🦀️.rs`. Not compiled and not run (the coordinator holds compile capacity); the `kind` equality needs `ValueRefusalKind: PartialEq + Debug`, which I assumed and did not check. No limit changed, `spent()` literal helper untouched for the other refusals.

## Addendum: coordinator measurement at 960 M items and 4,294,967,295 copy bytes

Result: 81 s, then `guest poll: ordinary Poll copy budget cannot fund its acknowledgment frame`. Source: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔁️poll/🎛️session/🦀️.rs:125-127`. It fires when the poll is `awaiting_control`, a guest loan is held, and `original_operation_drive_grant().maximum_copy_bytes < control_bytes()`. That grant is `min(drive cap 1 MiB, treasury remaining - guest hold - cleanup reserve)` (`physical_policy`, `📥️operation/🦀️.rs`).

Who copies gigabytes: nobody copies document state. The "copy bytes" are the interpreter's bookkeeping charge, not memcpy of the note.

- `advance_scalar` books `size_of::<Value>()+size_of::<usize>()` copy bytes for each scalar instruction, `width + Values + usize` for each load/store, and whole frame, control and local vectors for calls (`▶️invocation/🦀️.rs`, `charge`, `admit_memory_instruction`, `admit_defined_frame`). The operation ceiling of 3,000,000,000 bytes at 48 M items is 62.5 bytes per item; the 4,294,967,295 ceiling divided by about 74 M items (81 s at the measured 0.9 M items/s) is 58 bytes per item. Same ratio on both runs, so the copy axis is proportional to executed guest instructions, and items and copy exhaust together. This is the same single VM invocation seen in the sample: the first Input of the InstanceOpen wire, which costs 1.35 billion instructions in total. 74 M instructions is about 5.5 % of it.
- No per-poll re-encode, whole-snapshot receive, history replay or document archive is on this path. The create never leaves the first Input of the first Poll (stage 0, no SRE1 yet, `poll-host-copy-audit`), so the poll count is 1 and the bytes per poll are the 681-byte wire. Document archive and snapshot work (`mcp-note-export-and-replay`) run only after create returns and are not reached. Not measured here: any later polls, because none is reached.
- Why the error changes at the u32 ceiling: the guest subloan is sized from the logical remaining stock capped by `CORE_GUEST_RETAINED_LIMIT` (`u32::MAX` per axis, `📥️operation/🦀️.rs`) and held against the treasury (`admit_guest_subloan_inner`, `🪙️guest/🦀️.rs:106-109`). With the operation copy cap equal to that ceiling the guest hold takes the whole spendable stock, the host slice left for the acknowledgment frame drops below `control_bytes()`, and the refusal changes dimension from items to copy. It is an exhaustion of the same bookkeeping, observed on the next axis; it does not show a separate acknowledgment-frame defect.

Conclusion stands: raising limits cannot make the create pass because cost scales with guest instructions executed (1.35 billion for the first Input). The fix is to cut instructions executed in typed JSON receiving of the Input (guest side, peer's files) and, separately, to stop charging 24 to 60 copy bytes of pure bookkeeping per instruction in the paid interpreter, which makes the copy axis a second instruction counter instead of a memory bound. Neither is changed here.

Open check for the coordinator, cheap and decisive: log `copied_items` and `copied_bytes` deltas per `advance_scalar` call by category (the existing `trace_work!` categories Scalar, Memory, Control, FrameBirth, HostCall are compiled only under `cfg(test)`); the ratio of Control and FrameBirth bytes to Scalar bytes would show whether frame charges (large per item) or scalar charges dominate the 58 bytes.
