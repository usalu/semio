# P7c2 Energy Retained Wire, Channel, and Terminal Remediation Re-audit

Date: 2026-08-25  
Auditor: Terra (independent read-only source/static audit)  
Scope: current Energy P7c2 restoration, retained publication queues, commit admission, preview/wire authority, close, laws, mutations, and allowed static gates

## Verdict

**RED.** The remediation closes the first audit's whole-table commit scan and ordinary lease FIFO defects, and it replaces the old fresh-shell restore with bounded replay. Three production counterexamples remain:

1. three decoded checkpoint capsule fields are only cap-checked, then discarded; they do not participate in restored scalar parity;
2. the public `adopt_checkpoint_packet` API advances the queue without an ACK or retained recovery lease, so a lost adopted packet cannot requeue to its head;
3. the typed preview is neither populated with the facility total nor generated from the live `SMENERGY` packet, while a second legacy preview encoder remains in production source.

No production source or shared script was edited. No Cargo, Nx, Wasm, browser, build, or runtime/mutation execution was run.

## Required Inputs Read

- P7c2 checkpoint/publication repair contract.
- The first Terra P7c2 RED audit and the current Sol remediation report.
- The accepted P7c1 fifth Terra re-audit.
- Current Energy simulation source, P7c2 law/mutation fixtures, and the framework retained-payload/worker-session ownership definitions.

## Remaining Production Blockers

### 1. Restore still ignores three decoded capsule scalars

The 164-byte checkpoint writer records `time_series_count`, `meter_count`, and `history_count` at bytes `132..156` (`sim/component.rs:1954-1964`). The retained decoder reads these values at lines 773-775, but uses them only to reject values greater than census maxima (776-778). It does not retain them in `EnergyRestoreJob` or include them in the expected tuple/parity comparisons at lines 816-852.

Consequently, take a valid periodic checkpoint and alter any of those three encoded counts to a different value still under its census cap. The replay follows the original deterministic path; the original digest remains unchanged; `scalar_match && numerical_digest == digest` becomes true; and `finish` installs the replayed job. The altered raw capsule field has no effect on admission, replay, or installation. This contradicts both the fixture's declared checkpoint fields and the contract requirement that every minted scalar be decoded and validated, not merely bounded.

The inspected hostile law proves only a changed digest at byte 156 remains unready (`sim/component.rs:4641-4650`). It has no mutation for each of the three count fields, despite the fixture grouping them under an `ignored_decoded_field` claim. The replay/recovery architecture itself is otherwise material progress: it checks operation/generation before each restore step (742-749), consumes one field/replay unit per call, retains publication output while replaying (791-893), and checks generation plus digest immediately before installation (897-912).

### 2. `adopt` bypasses ACK-only queue retirement and recovery

For an ordinary consumer lease, `take` creates a tokenized in-flight record without changing `head`, `retry` restores the exact packet to that head, and `ack` requires the payload to be terminal-empty before advancing (`sim/component.rs:181-231`). That portion repairs the first audit's FIFO issue.

`EnergyWireQueue::adopt`, however, clears the recovery slot and in-flight record and advances both `head` and `len` while returning a potentially populated `EnergyWirePacket` (`234-247`). `EnergyJob::adopt_checkpoint_packet` exposes this bypass publicly (`1853-1855`). A caller can take a checkpoint lease, adopt it without closing its payload, then take the next packet. If the adopted packet is dropped or panics, no `EnergyWireLease` remains to enter `ENERGY_WIRE_LEASE_RECOVERY`; `recover_lost` has no in-flight record to restore (250-259). The framework payload's ordinary `Drop` deliberately preserves nonempty pages rather than closing or requeuing them (`framework/job/component.rs:443-449`).

The production restore route uses exactly this bypass to move replay checkpoints into `replay_retiring` (`868-869`), so this is not a test-only private helper. The existing queue laws exercise `take → retry → ack` and dropped *leases* (`4661-4701`), but never an adopted nonempty packet. It therefore does not meet the required ACK-only advance or adopted-owner Drop/panic/lost-handle recovery rule.

### 3. Preview has split authorities and a fabricated total

`EnergyJobPreview` publicly includes `facility_electricity_kwh` (`384-394`), but every live preview initializes it to `0.0` and no other live source assigns it (`1998-2008`; complete caller census of that field found only the initialization and encoders). The live `SMENERGY` preview builder writes only the common header, warmup/hour/timestep fragment, and RNG/weather fragment; its fourth fragment is checkpoint-only (`1940-1967`). Thus it encodes neither the typed facility scalar nor an authoritative typed-preview packet.

Publication independently installs the raw packet and moves the separately constructed `pending_preview` into `last_preview` (`2024-2034`). In addition, the live source retains a second handwritten `ENERGYP1` 42-byte `encode_preview` serializer (`3764-3776`). It is currently uncalled, but it demonstrates that the typed view and `SMENERGY` wire do not share one schema authority. This violates the P7c2 requirement for a real bounded facility-total preview generated from the one owned packet; the latest-wins retirement mechanics do not cure schema/content divergence.

## Re-audited Remediation That Holds Source-Statically

- **Full replay structure and stale handling:** the decoder owns the original packet plus Model/Config; replays a normally admitted `EnergyJob` one grant at a time; retires replay preview/checkpoint/fault/terminal outputs one payload page at a time; and installs the actual replayed authority, not a fresh shell (`802-912`). Drop stores the incomplete decoder, packet, graph owners, replay authority, and retirement cursor in a fixed 64-slot same-generation registry (`916-961`). The remaining ignored count fields prevent acceptance, but the former wholesale restore defect is otherwise addressed.
- **Queue provenance, saturation, and basic recovery:** fixed four-slot queues preserve an overflow packet, issue a private-token lease, retain the same head on retry, and restore a dropped/panicked lease through a fixed 64-slot registry (`152-259`). The packet/lease fields are private, so matching-packet injection is not an exposed API. The `adopt` escape above is the exception.
- **Commit census/reservation:** `CommitCensusWork` visits a base fragment, one UTF-8 character, record, sample, summary, or resident channel position per `InteractiveJob::step`; it does not call the old whole-table preflight. It sums resident queue/in-flight pages and bytes before reserving the commit slot and then allocates exactly one retained page source per grant before a writer exists (`2680-2843`). `write_output_fragment` takes only the matching prepared source, restores it after a failed grant, and final publication checks exact reserved pages/bytes/items before `push_reserved` (`2653-2677`, `3419-3447`). This closes the first audit's third commit-admission RED on source inspection.
- **Close:** Energy close drains writers, ready/restore/preview packets, every queue, census reservation source, and final result owners through bounded steps (`3463-3555` and following). A retained queue lease blocks close until it is ACKed/recovered, rather than recursively dropping it. The adopted packet escape is outside that queue close cursor, which is why blocker 2 remains.
- **Prepared terminal:** `EncodeOutput` publishes a prebuilt commit packet before the `Complete` stage, and `Complete` transfers that prepared payload without result traversal (`3435-3454`). Runtime must still prove outer worker-session terminal handoff/close behavior.

## Laws, Mutations, and Allowed Static Gates

Seven live Rust laws were found at `sim/component.rs:4559-4793`; the JSON fixture declares and contains twenty-eight mutation entries. They were inspected, not run. Their coverage includes fixed checkpoint size, MAX/MAX+1 restore ownership, header/cap/trailing mutations, basic queue retry/lease-drop recovery, stale restore/install, interruption gates, and 1/4-fuel restored commit parity. It does not cover the three ignored count fields, public adopted-packet loss, actual preview total, or typed/wire schema equivalence.

| Gate | Result |
| --- | --- |
| `rustfmt --edition 2021 --check` on sim plus P7c1/P7c2 touched Energy sources | GREEN (no output) |
| Scoped `git diff --check` on those sources and the P7c2 fixtures | GREEN (no output) |
| `jq empty` on both P7c2 JSON fixtures | GREEN (no output) |
| P7c2 law / mutation census | GREEN structurally: 7 / 28; not executed |
| Legacy/forbidden-route census | RED for live `encode_preview` / `ENERGYP1`; previous serde/whole-output patterns absent |
| Cargo, Nx, Wasm, browser, runtime, mutation execution, worker-profile replay | Deferred by instruction; not run |

## Required Repair

Carry all three decoded count scalars through `EnergyRestoreJob` and require equality with the replayed authority before `ready`/`finish`; add individual under-cap count-mutation laws. Remove the public non-ACK adoption escape or make it a retained, generation/provenance-bound owner whose Drop/panic returns to the unchanged head and whose only retirement is page-close plus ACK. Replace both preview paths with one owned `SMENERGY` preview packet that encodes real fixed facility totals and derives the typed view from that packet; remove the stale `ENERGYP1` serializer. Re-run this static audit and later the deferred executable matrix after source remediation.
