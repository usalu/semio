# W1 Runtime Outcome Frozen Audit V3

## Scope

Read-only frozen-tree audit of the runtime P1 remediation against `📋️w1-runtime-outcome-frozen-audit-v2.md` and `📋️w1-runtime-outcome-frozen-remediation-report.md`. No production files were edited. No Cargo or Nx command was run.

## Verdict

**NOT CLOSED.** The low-level conflict tags and the single-edit conflict-index checks are present, and the runtime-only policy is implemented in the executable contract. The current tree still has P1 fidelity/persistence defects below. The repository-wide fallible `MutationDiff::apply` P0 remains open and is outside this audit.

## Findings

### P1-1 — Multi-operation Operations→Snapshot ledger remap is incomplete

The remapper only accepts a match when one local edit has the exact full operation sequence of the remote source edit (`🏪️store/🦀️component.rs:5230-5245`). However, the Operations path fans one source edit out into one wire envelope per forward operation (`🏪️store/🦀️component.rs:5518-5545`), and `edit_from_operation_envelope` reconstructs each envelope as a one-forward local edit (`🏪️store/🦀️component.rs:5718-5735`). For a source edit with two or more forwards, no whole-edit local match exists.

The later deduplication still considers the source edit known when all of its operation ids are present (`🏪️store/🦀️component.rs:5344-5357`), then merges the unremapped source ledger entry (`🏪️store/🦀️component.rs:5359-5366`). Authoritative validation rejects that entry because its top-level source edit id is absent locally. Thus a normal multi-operation Operations→Snapshot replay can fail to converge even though every operation payload and identity matches. The remap must map a source edit's operation sequence to its one-or-many established local edits and redistribute/merge the authored ledger deterministically, rejecting only true ambiguity.

### P1-2 — Host persistence does not preserve the complete cursor

`BackboneDocument` exposes only `applied_edit_ids` and has no redo or checkpoint cursor fields (`🖥️host/🦀️component.rs:329-340`). Its envelope projection unconditionally synthesizes `redo_edit_ids: Vec::new()` and `checkpoint_id: None` (`🖥️host/🦀️component.rs:374-394`). Binary decode likewise extracts only applied ids (`🖥️host/🦀️component.rs:440-462`), and `OsWorkflowStore::new` discards the cursor entirely before rebuilding from applied ids (`🖥️host/🦀️component.rs:651-672`). An undo followed by host binary or text export therefore loses the redo lane and current checkpoint. The checked-in host test asserts only applied ids (`🖥️host/🦀️component.rs:1485-1489`), so it does not cover this loss.

### P1-3 — Generated conflicts can be unpersistable for repeated actors

Conflict validation requires nonempty, unique actor identities (`🏪️store/🦀️component.rs:2552-2554`). Quarantined validation additionally requires the stored actor vector to equal one actor per envelope in order (`🏪️store/🦀️component.rs:2560-2568`). Runtime generation preserves every actor occurrence rather than producing a unique participant set (`🏪️store/🦀️component.rs:5029-5040`, `5287-5293`, `5394-5406`). A rejected/degraded batch containing two operations from the same actor consequently creates a conflict that `print_document_spr`/text validation cannot persist. Either `actors` must be explicitly occurrence-preserving and uniqueness removed, or all generation and validation paths must canonicalize it as a deterministic unique actor list.

### P1-4 — Runtime-only policy remains stale in the authoritative planning contract

The executable contract freeze now correctly defines `Quarantined { envelopes }` and says the deciding policy is runtime-local (`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md:43-49,68-76`), matching the runtime policy docs (`📡️spr/🧾️wire/🦀️component.rs:145-166`). The same ticket's `📋️master-plan.md` still specifies `Quarantined { policy: MergePolicy, envelopes }` and serializes that policy in the reject algorithm (`MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️master-plan.md:51-59,74-85`). This contradictory authoritative plan can drive a future registry/schema/codec back to a persisted policy and prevents the contract claim from being closed until the stale source is corrected or explicitly marked historical.

### P1-5 — Full-fidelity text omits `Edit.sequence_number`

The `.ops` edit grammar and printer carry id, actor, timestamps, coalesce key, and description but no sequence number (`🏪️store/🦀️component.rs:2196-2204`, `2301-2306`). The parser silently derives `sequence_number` from edit position (`🏪️store/🦀️component.rs:3044-3060`). Sequence numbers are used to seed the store's next sequence (`🏪️store/🦀️component.rs:4135-4149`) and are assigned to new edits (`🏪️store/🦀️component.rs:4715-4725`, `4776-4785`), so an imported history with non-contiguous or remote sequence values is not text-lossless and can change later edit identity/ordering behavior. The strict parser must persist and validate this field, or the contract must explicitly make it derived and remove it from the authoritative model.

### P2-1 — Strict text/Spr parsers do not run full authoritative-history validation

`replay_ops` and `parse_document_spr` validate conflicts and cursor references but return an envelope without invoking the store's full history validator (`🏪️store/🦀️component.rs:3140-3200`, especially `3192`; `2857-2977`). Duplicate change/checkpoint/alternative ids and dangling change/checkpoint/alternative references can therefore pass the codec boundary and fail only later in `ArtifactStore::new`. The strict codec boundary should reject these structural records itself.

## Closed checks

- Conflict kind/status tags are range-checked on both encode and decode (`📜️history/🦀️component.rs:1251-1294`), with adversarial coverage (`📜️history/🦀️component.rs:2180-2204`).
- Conflict `op_index` is range-checked against the kind-specific flattened operation sequence (`🏪️store/🦀️component.rs:2556-2623`); generated conflict messages are offset from their owning edit sequence (`🏪️store/🦀️component.rs:2521-2542`).
- Text parsing requires explicit inverse, complete metadata, durable messages where present, conflicts, and exactly one cursor; it rejects unknown/duplicate/overlapping cursor ids (`🏪️store/🦀️component.rs:3140-3176`). No metadata/id/actor/timestamp fallback synthesis remains in this path.
- Runtime resolution uses a local `LaissezFaire` candidate and does not serialize `MergePolicy` (`🏪️store/🦀️component.rs:5119-5137`, `5178-5224`).

## P0

No new P0 was found in this narrowed runtime audit. The frozen repository-wide `MutationDiff::apply` infallibility P0 remains open as recorded by the prior reports.
