# Context VM Measurement and Finite Execution Proposal

Status: read-only design during dependency sync 9214. No production edits or native request made by this lane.

## Existing Receipt and Boundary

`OriginalGuestExecutionReceipt` already separates measured VM instructions from guest retained/logical work. Missing measurement is explicit and rejected by its validator. Context admission/close currently return only guest semantic replies; their WIT and semantic schema need no change.

Add `OriginalGuestContextCall<T>` beside the existing interpreter `OriginalGuestJobCall<T>`, containing unchanged `Result<T,TurnFault>` and the existing execution receipt. Change the two host context methods and all implementations; do not introduce a legacy overload or alter guest WIT.

## Complete Host Consumer Inventory

Scoped Rust sweep of framework, Hub and extensions found:
- GuestRuntime trait: plugin/host/guest-runtime/guest-types-and-runtime-trait.
- Owned adapter, Wasmtime implementation, Mock implementation, Recording implementation, and GuestRuntimes forwarding.
- OriginalShardIdentity::admit_actor_original and admit_owned_actor_original.
- Puzzle guest-io interaction context admission and close.
- wp4 reactor macro matches are guest exports, not host-runtime consumers.
- Shard identity admission test also requires its existing assertions retained against the new wrapper.

All paths above use their current emoji taxonomy under framework OS plugin. The sweep excluded build outputs, cache directories and symlinks.

## Backend Measurement

Owned context success currently discards invocation.fuel_used. Resume state counts cumulative pending fuel; the progress callback is not complete for all errors. Capture a checked per-crossing delta from the same pending identity before/after, or the completed invocation counter; retain None if measurement is unavailable. Never clamp or map unavailable measurement to zero. Pre-submit refusal can report measured zero only when no VM was submitted.

Wasmtime already has a matching job pattern: arm actual fuel metering, perform the crossing, then capture checked authorized fuel minus store.get_fuel even when the semantic call traps. Use that existing backend measurement pattern for context, without changing the guest reply.

## Confirmed Phased-Instance Gap

Owned phased startup hands off OwnedGuestInstance with actor=None and original_core=Some. Context actor_context_call still invokes begin_owned_operation. owned_operation_resumes immediately rejects original_core.is_some() (operation-begin-resume-and-cancel line 7). Thus the genuine actor acceptance harness reaches a definite source-level refused context path; no native observation yet.

The existing original_call path must be inspected as the canonical funded phased crossing before any repair. Do not create a second actor or convert/copy original_core to the old unfunded path.

Paid close_original_instance_step is original core storage/source teardown, not a guest export execution. It still counts against the protocol-turn ceiling, but must not fabricate VM instruction spend.

## Production Finite Stock Connection Requires Review

GuestRelayOriginalRoot already owns lifetime execution_fuel_remaining and per-scope execution_remaining, execution_spent and execution_pending. checkout_actor_execution_budget and receive_actor_execution_receipt enforce exact pending budget and measured debit; finish_scope is the authority boundary.

Current non-definition actor-specific checkout/receive callers in the host Rust sweep occur only in root tests. Production ShardLoop::execute_turn_for context admission calls OriginalShardIdentity with turn_budget and emits ShardOutcome::OriginalCall carrying semantic returned/call. No execution authorization/receipt is carried there.

Therefore merely retaining the new host receipt would not settle production stock. Root must authorize the exact crossing before shard dispatch, carry that authorization through the shard/identity call, and receive the matching backend receipt before permitting another crossing or finishing the scope. This includes semantic refusal and trap. Unknown measurement must retain the pending authorization and refuse continuation. Parent review is required before choosing the exact outcome/issued-budget transport edits.

## Acceptance Ledger and Global Turn Counter

The test fixture already authorizes 2000 user-visible fuel quanta. Add a separate execution_remaining initialized once from that authorization. Each actual VM crossing receives min(per-turn allowance, remaining), and actual receipt is debited before inspecting its semantic success/failure. Startup receipts, job calls, kernel poll fuel_used and context receipts all draw this same finite stock. Host-only retirement consumes logical/physical controls without claiming VM spend.

Use one cumulative protocol counter across birth, advance, take, maintenance and close. Check the 10000 bound before every actual caller.turn; keep epochs monotonic. Existing startup/take loops have separate iteration limits and direct close does not update self.turns, so current local checks are insufficient.

## Test Plan

Extend the existing language-neutral Actor execution schema/fixture and mounted receipt laws for exact budget identity, cumulative subtraction, rejection of unknown/overspent/mismatched receipts, no repeat checkout before receipt, boundary allowance, and global pre-dispatch turn exhaustion. Preserve all prior receipt cases.

Backend tests must qualify actual Owned and Wasmtime measurement on success, semantic refusal and trap against real instruction counters. Mock/Recording have no VM: use truthful unsubmitted zero, never present them as the real-actor acceptance oracle.

Retain real Puzzle actor acceptance with unchanged material, operation Source, mounted turn grants, 120s, 2000 quanta and 10000 total calls. Native/actor proof remains unexecuted.

## OriginalCore Mechanism Follow-through

The original_call module at interpreter/runtime/🎟️call is genuinely funded: begin_original_call_step validates the same retained source and embedding clock; its inline argument frontier captures the incoming input; advance_original_call_step opens, drives, captures scalar results, closes machine storage, then exposes Ready. It returns the actual Core execution receipt separately. Cancellation moves to a paid abort/close path; no second Core is created.

However this is currently a scalar execution subset, not an implementation of context/poll/job operations. CoreOriginalOwner.advance_scalar at interpreter/instance/invocation:
- accepts constants, locals/globals, selected integer arithmetic, memory loads/stores, direct defined calls, drop and function completion;
- explicitly refuses unplanned opcodes;
- direct calls accept only defined callees, not imports;
- has no HostCall outcome or original host-reply/resume API;
- scalar result capture supports zero or one Value only.

Kernel execute_actor_turn also uses begin_owned_operation(Poll), so it is refused for original_core just like context. Current start_actor_job and drive_job also use begin_owned_operation/scalar_operation, so the actual actor job route needs the same genuine operation owner; swapping only context admission cannot qualify actor IO.

### Reusable Boundaries

Reuse the existing authentic original_source, original_core, initiating epoch/clock validation, inline call frontier, exact execution budget and receipt, and paid invocation close. Reuse existing original operation identity receiving protocol semantics (begin/progress/allocation/finish), existing command/cold-pair extraction and strict fixed reply parsers; preserve one canonical declaration of each protocol.

Do not call the old OwnedSemioInstance actor or instantiate a replacement, clone a Core, remove original_core refusal, or use raw step behind a fabricated receipt. Old codec-origin paths are a distinct tooling operation, not an interactive fallback.

### Required Paid Operation Phases

A clean Original operation owner needs explicit phases:
1. Admit the bounded operation identity and retain the genuine source input.
2. Encode operation input with actual existing controlled identity machinery, retaining the resulting owned bytes and receipts.
3. Invoke the actual guest allocator through the same paid Core.
4. Copy bounded input windows into validated guest memory, paying original physical work before mutation.
5. Open and advance the requested guest export through the funded interpreter. Import suspension retains the actual pending HostCall. Bind identity callbacks and same-clock supplier; quote and pay host reply/memory writes before applying them.
6. Capture validated scalar pointer/length or fixed resident receipt. Validate operation result tags and bounds before reading.
7. Transfer/copy bounded output pages through real receiving reservations. Preserve original operation receiver across yields; no ordinary-drop on refusal.
8. Invoke actual guest deallocation when required, close the interpreter invocation using existing paid close, then publish the semantic result and actual measured execution receipts.
9. Cancellation closes retained input/output/import/invocation/source owners under the same original cleanup grant before terminal publication.

The existing original scalar instruction engine must grow genuine physical plans for the actual module instructions and imported calls. A raw legacy interpreter step is not a paid implementation. Prefer one canonical opcode semantic owner with Original physical admission around each effect over a second divergent interpreter; any such responsibility change requires a separate reviewed design and unchanged Source census.

### Public API Constraint

Current admit/close/execute_turn signatures accept numeric Budget and identity, but no authentic StepContext. The original Core APIs require the actual context and parent source; constructing one from copied Budget is forbidden. Therefore receipt wrapping alone cannot make these operations funded. The reviewed implementation seam must borrow the genuine context at the public phased operation step (as existing startup does), or transfer an actual original source loan across the shard. Parent owns the production Root-to-Shard transport design; do not silently issue another parent.

### Neutral and Real Backend Proof

Add neutral operation programs covering input allocation/write, real defined/imported calls, output range validation, exact receipt identity, partial capture, cancellation at each phase, refusal preserving owner identity, and paid terminal closure. Real Wasmtime/WAT evaluation is the independent semantic oracle for the same operation input and output, with host imports supplied through the existing owned interface.

Run a real generated Puzzle actor only after managed component publication through the existing owner receipt. Its baseline material output, execute-phase progress, interior cancel, strict canceled fault and subsequent real application dispatch remain the acceptance gate. Source inspection of the operation seam is not that gate.

### Production Startup Inventory Qualification

Low independently found genuine OS run/wasmtime-node-host.rs startup uses checkout_actor_execution_budget and receive_actor_execution_receipt around phased service startup. This is the correct existing production model. The missing bridge finding above is specifically the shard context admission path within plugin/host, not an absence of genuine actor execution settlement repository-wide.
