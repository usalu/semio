# P7c2 Energy Retained Wire, Channel, and Terminal Independent Audit

Date: 2026-08-25
Auditor: Terra (independent read-only source/static audit)
Scope: exact current Energy P7c2 wire, restore, publication, terminal, fixtures, and static gates

## Verdict

**RED.** The versioned fixed-page wire removes the former serde/JSON route, but the retained protocol is not lossless or exact. Three independent production counterexamples block P7c2 source acceptance:

1. checkpoint restore decodes a small replay capsule then discards almost every decoded field and mounts a fresh job rather than reconstructing the retained numerical graph;
2. queue take/retry has neither an in-flight exact-owner record nor head reinsertion, so retry reorders packets and a consumer loss abandons the page owner;
3. commit preflight performs whole live-table scans and dynamically acquires pages after that scan rather than retaining a one-unit exact simultaneous admission cursor.

These violate the P7c2 contract's retained restore, lossless queue, close/recovery, and one-field/page-grant requirements. No production source or shared verifier was edited, and no Cargo, Nx, Wasm, browser, build, or runtime test was run.

## Live Counterexamples

### 1. Restore is a lossy replay shell, not exact retained-state reconstruction

EnergyJobAuthority owns the active state that the contract requires a checkpoint to reproduce: fixed weather, validation, precompute, precomputed model, simulation state, warmup maps/cursors, run iterator, timestep work/builder, RNG, result tables/orders/history, finalization/result builders, publication, and output cursors (sim component.rs:1194-1258).

The checkpoint writer serializes only four fragments: header; three progress scalars; RNG plus weather cursors; then aggregate cursor, three flags, and three counts (1534-1566). The decoder likewise reconstructs only those scalar fields (536-579). It has no section framing or owner cursor for the nested numerical graph.

More decisively, EnergyRestoreJob::finish calls EnergyJob::admit to create a fresh job and applies only restore_target_hour plus restore_input (582-600). It does not install its decoded stage, tier, RNG, weather cursor/target, aggregate cursors, backing stages, or checkpoint-due flag, much less the omitted tables/builders/state. PublishTimestep subsequently replays from the new job until that target hour (2714-2722). This cannot restore a checkpoint captured in any partial P7c1 substage and violates the contract's exact retained-microcursor requirement.

EnergyRestoreJob::step also has no operation/generation comparison before consuming fuel and mutating a decoded field (536-579). A stale StepContext can therefore drive an otherwise valid restore authority. The live stale-generation law checks only EnergyJob::take_checkpoint_packet, not the decoder (4019-4034).

The inspected parity law checkpoints at the normal periodic publication boundary and compares only final output after replay (4037-4067); it does not falsify an active numerical substage, compare restored field ownership, or compare the next checkpoint packet.

### 2. Consumer take/retry is not lossless FIFO and has no Drop recovery

EnergyWireQueue::take removes the head and advances it immediately (sim component.rs:104-114). Every checkpoint/commit/fault retry then calls push, which appends to the tail (1451-1484).

A concrete live counterexample is queue order [A(sequence 0), B(sequence 1)]: take transfers A, leaving [B]; retry(A) computes the tail slot and creates [B, A]. The original FIFO order is irreversibly changed. The queue has no in-flight slot or retry token that could put the exact packet back at its original head.

The retry guard checks only kind and four operation identity fields (1486-1492), not sequence, payload schema, queue provenance, or ownership. EnergyWirePacket exposes public kind/identity/payload fields (60-64), while the framework exposes RetainedJobPayload::empty (framework job component.rs:348-350), so an arbitrary matching-generation empty packet can be injected through any retry method when capacity exists.

There is also no Drop implementation for EnergyWirePacket, EnergyWireQueue, or EnergyPublicationChannels; taking a packet leaves the job with no durable record of that owner. If a consumer drops a populated taken packet, framework RetainedJobPayload::Drop only debug-asserts and intentionally preserves the backing rather than requeueing or closing it (framework job component.rs:443-449). Thus packet loss leaks/abandons its exact page and cannot satisfy the required consumer Drop/panic recovery. The only explicit recovery tests cover EnergyRestoreJob and the outer EnergyJob, not transferred queue packets (3929-3943, 4505-4544).

The sole queue law proves full-queue rejection and direct take order, but never retries a taken packet or loses a consumer owner (4001-4016).

### 3. Commit admission contains unbounded work and is not a simultaneous fixed-page reservation

preflight_commit_wire traverses all meter names and all time-series names through chained iterators, then traverses all time-series values again to total samples (2206-2234). This is a whole live-table scan in the single EncodeOutput grant that calls it (2783-2790), not a retained census cursor that inspects one admitted owner per grant.

The same preflight calculates payload size from logical len values and does not include pages retained by preview/current-retirement, checkpoint/fault/commit queues, a ready wire packet, or the writer/candidate simultaneously. It therefore does not establish the required exact item/page/operation/process aggregate before owner transfer.

After that incomplete scan, the encoder first creates an unreserved writer (2789) and acquires each physical page later from write_output_fragment through begin_staged_page(context) (2192-2203). The writer's framework ledger correctly rejects a page when a limit is already exhausted, but a late refusal occurs after prior fragments/pages have been materialized; it is not the required pre-transfer MAX/MAX+1 admission.

## Green Structural Evidence That Does Not Change the Verdict

- SMENERGY version 1 has a fixed 80-byte binary header carrying kind, identity, stage/tier, and numerical cap fields (21-158); the production checkpoint writer is field/page stepped (1569-1593).
- Preview replacement moves the current preview into a separate retirement owner and closes one payload page before installing the replacement (1612-1635).
- Checkpoint, commit, and fault queues have fixed four-slot storage and preserve a saturated packet identity at push refusal (82-126).
- Terminal Complete dequeues an already prepared commit payload rather than traversing results at that terminal arm (2816-2831). That prepared-packet property does not repair the earlier preflight, queue ownership, or restore defects.
- The production source census found no prior serde_json checkpoint encoder/decoder, from_checkpoint, encode_state, commit_output, HashMap, HashSet, extract_if, or legacy page-copy helper occurrence. The remaining serde derive use is not a production checkpoint/JSON route.

## Laws, Mutations, and Allowed Gates

Five P7c2 Rust laws are present and were inspected:

1. exact numerical restore MAX/MAX+1 owner/retry and restore drop recovery;
2. live header/cap/trailing mutation rejection;
3. saturation plus direct FIFO take;
4. outer job cancel/deadline/stale gate;
5. boundary-checkpoint final-byte parity at fuel 1/4.

The schema fixture declares 18 hostile mutations. They were not executed, and the five source laws do not cover the retry-order, forged-retry, transferred-packet Drop, stale-decoder, active-microcursor restore, or whole-preflight counterexamples above.

| Gate | Result |
| --- | --- |
| Scoped rustfmt --check --edition 2021 on Energy sim | GREEN (no output) |
| Scoped git diff --check on sim and P7c2 fixtures | GREEN (no output) |
| P7c2 forbidden-route census | GREEN (no prohibited legacy production occurrence) |
| Bun parse of wire-law and mutation fixtures | GREEN |
| Compiler/runtime/mutation execution/parity | Deferred by instruction; not run |

## Required Follow-up

P7c2 needs a retained, sectioned complete-state checkpoint/decoder with operation/generation checking on every decoder grant; an in-flight, provenance-bound queue transfer token that restores the original FIFO position and durably recovers lost consumers; and a bounded preflight/admission cursor that reserves the full simultaneous page/aggregate set before writing. These are source blockers, not P7c3 UI integration work.

