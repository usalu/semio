# Host Issued Batch Ledger Read-Only Audit

2026-10-10. Current source only; no builds/tests or production edits. Concurrent dispatch migration was preserved.

## Actual Lifecycle

Base `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard`.

`🦀️.rs::ShardFrame::Grant` carries original RetainedTurnInput, resource Budget and Vec<Envelope>. Both ordinary pack_encode and borrowed pack_encode_grant validate and serialize original retained authority. `admit_received_frame` validates frame then inserts `IssuedShardTurn { retained,budget }` in granted_budgets and dispatches each envelope. Standalone Envelope later uses same cached grant.

Actual IssuedShardTurn at161 is Clone+Copy and contains only retained/budget. `granted_budget`:1057 returns `.copied()`; deferred `AdmittedAuthority::budget` stores Option<IssuedShardTurn>; lifecycle::drive_unissued_authority fills it with another map copy. Event execution:1749 takes that copied authority; `execute_turn_for`:1972 passes it through turn_budget_from_grant into guest Budget. Each successful event currently bridges and sends its own ShardOutcome::Turn at2080. `consume_replay_opportunity` and close variant only reduce scheduling fuel; they do not settle retained currencies. Job budget bridge carries scheduling fuel/deadline only.

Bridge `to_actor_turn_result_in_place`:203 serializes effects/ingress, converts patch transport and returns Actor TurnResult. Current construction beginning250 does not visibly supply retained_receipt. This is a genuine integration frontier under current Actor receipt API, not proof of compiler success/failure. Checked dispatch's retained Decision and exact PackError custody are separate concurrent work and should not be replaced by ledger edits.

## Exact Public Retained API

Actor `🎟️retained-turn/🦀️.rs` exposes Copy RetainedTurnInput(operation,generation,epoch,grant) and Copy RetainedTurnReceipt(input,spent,remaining). `validate` rejects any zero identity. `return_original(spent)` checks all actual spent currencies fit and subtracts items/copy/capacity/release while preserving maximum_depth. `RetainedTurnReceipt::validate_for(input)` demands exact input identity/grant and recomputed conserved receipt. No Rust framework `RetainedTurnOutput` declaration was found; current output name is RetainedTurnReceipt.

Copying an input value preserves identity but does not enforce single spending. The owning Host ledger must enforce exclusivity, ordinal issue/settle order and one terminal return; public Copy alone cannot supply these laws.

## Current Genuine Red Law

Actual shard root mounts `🧪️tests/🔬️unit/🦀️.rs`:2640. New `issued_shard_batch_has_one_original_ledger_and_one_terminal_return` starts at7 and calls absent IssuedShardTurn::new/issue_envelope/settle_envelope. It uses neutral batch fixture, loans current remainder with same original identity, rejects forged epoch receipt without heap effect, settles cumulative spent, expects exactly one final original-input receipt, rejects duplicate settle without heap effect and refuses issue after terminal. The Source batch law checks independent Ajv/SQLite arithmetic only, explicitly separate from Native receiving. This is not a current runtime pass.

## Smallest Schema-First Integration

Author ledger states ready(next ordinal,remaining), issued(exact ordinal/input), terminal(original receipt returned). Keep immutable original input, cumulative actual spent, finite envelope count, current ordinal and scheduling Budget in one non-Copy map-owned IssuedShardTurn. Prevalidate identity/count before replacing an active actor grant. Envelopes/deferred/retry entries carry immutable batch identity plus ordinal, not independently spendable copied ledger. Permit only one active loan; subsequent issue uses same identity and updated remaining. Validate exact receipt before any mutation; refused forged/duplicate/out-of-order receipts must leave original ledger and retained event unchanged with zero physical effect.

Settle each actual guest receipt exactly once into cumulative progress; withhold scheduler's terminal receipt until all batch members completed or terminally refused under specified policy. A partial/deadline retry retains its issued loan and must not reissue a fresh grant. Bridge intermediate outputs without fabricating zero-spent receipts; completion needs a defined retained output/effect accumulator before a single final ShardOutcome::Turn. Do not discard earlier envelope UI/effects/ingress when suppressing intermediate outcomes. Give empty batches and standalone envelopes explicit neutral laws; otherwise count0 grants can never terminate or later standalone events can borrow a terminal batch.

Initial bounded step is implement and qualify the map-owned finite ledger plus unit law before production guest integration. Then register the same actual ledger in Grant admission, issue through deferred event selection, settle genuine guest receipt and publish one final aggregate. The map state and original event ownership must survive send refusal until the final transport acknowledgment; deleting ledger before successful send would lose the only single-return guard. Existing original RetainedTurnInput epoch must not advance per envelope; scheduler owns subsequent turn epoch advancement after final validated return.
# Current External Ledger Source Follow-up

Read-only reinspection of current in-progress source; no builds/tests. Historical absence claims below are superseded for authored ledger methods, not for whole production integration or runtime qualification.

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs:161` now has non-Copy IssuedShardTurn with envelopes/next/issued/spent/returned, new/remaining_input/issue_envelope/settle_envelope, and a mutable borrowed IssuedShardRetainedRecipient. Exact receipt validation and checked addition precede ordinal/state mutation; terminal whole receipt derives from the original input, and forged/duplicate settlements preserve ordinal authority. A scoped recipient records progress before limit rejection and finish settles once.

Actual integration remains incomplete at inspected source: granted_budget:1116 returns Option<IssuedShardTurn> by `.copied()` despite removed Copy; frame admission:2185 still constructs `IssuedShardTurn {retained,budget}` without new fields. Other deferred/job paths copy grants from the cached HashMap. Production does not call issue_envelope/settle_envelope/envelope_recipient; only new unit laws issue/settle. Therefore map ownership, envelope ordinal dispatch and final aggregate output have not yet adopted the ledger. These are source/API mismatches, not observed compiler results.

execute_turn_for:2031 still consumes a per-event supplied turn, executes its Guest turn, builds a ShardOutcome::Turn and sends it immediately:2139. Retryable lifecycle fault returns true before publication. No retained aggregate effects/UI/jobs/metrics/ingress output or final-only whole receipt branch was found. send_outcome:2398 allocates/encodes a local output Vec; encoding refusal returns error and drops those bytes, while transport.send has no returned acknowledgment/failure. Decision OriginalShardDispatch retains original decision/cursor through encode refusal but ticks advance after the void send_frame call; it cannot prove acknowledged publication.

Smallest real integration retains one mutable ledger per admitted actor/batch and stores deferred ordinal references rather than copied grants. Issue exactly current next ordinal; preserve issued checkout/progress across yielded/retryable work and refusal; settle validated actual receipt; accumulate actual outputs under original custody; only final settle produces one whole completion. Keep completed terminal output/receipt until genuine send acknowledgment or explicit retained send-refusal state. A new batch must not replace an unfinished map ledger; unregister/cancel must retire held batch/output under original authority.

Current duplicate wire/resource risk remains: ShardFrame::Grant has separate retained and Budget; pack_encode_grant validates and writes grant.original_input then writes Budget::pack_encode, which currently encodes retained again. Host and renderer override callback accepts full Budget and can replace retained without a semantic guard. Separate scheduler authority and arbitrary override authority can diverge. Repair the actual original wire/override contract coherently with one canonical accessor and original remaining authority; do not add dual-layout fallbacks. The execution High agent owns this repair.

Authored unit issued_shard_batch_has_one_original_ledger_and_one_terminal_return exercises genuine new ledger constructors and receipt methods, but this alone does not prove production cachedmap/queue/send integration, original output allocation custody or Native execution. Add real multi-envelope dispatch with retry and send/encode refusal once the owning production path is mounted; inspect observed counters and final single returned receipt before qualification.
