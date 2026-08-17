# Runtime Outcome Persistence Remediation

## Scope

Remediated the framework OS store, SPR history codec, and store sync bridge for audit findings 1–7 and 9–11 in `📋️w1-runtime-outcome-persistence-audit.md`. Plugin, host, and glTF code were not changed. `MutationDiff` was not changed.

## Implemented

- Quarantined `Accept` now evaluates every envelope in an isolated candidate store and adopts it only when the whole batch is accepted. Aggregate reports retain all replay results; fatal later entries leave source history, state, conflicts, and generation unchanged.
- Empty-store snapshot adoption validates document/schema before mutation, replays applied history in HLC order, restores cursor/redo/current state, seeds causal operation and edit identities, preserves remote durable message/conflict state, and retains the receiver's local merge policy.
- Runtime reconstruction seeds causal DAG state, edit sequence, and HLC from persisted history. Incoming/local timestamps merge deterministically and local dispatch ticks the merged clock.
- Stable operation identity is mandatory for authoritative SPR history. Parsing rejects absent metadata, absent HLC, negative HLC physical time, absent operation IDs, and metadata/forward cardinality mismatches; no identity, inverse, author, or timestamp is synthesized on this path.
- Message and conflict persistence are authoritative: the JSON envelope requires `editMessages` and `conflicts`; mutation messages use frozen codes/severities and valid indices; conflict identities, actors, referenced edits, dependencies, HLC bounds, and content-addressed IDs are validated before printing or accepting history.
- SPR decoding rejects invalid message severity/presence/index width, duplicate conflict IDs/records, conflict trailing bytes, and malformed cursor presence/trailing bytes. Store parsing rejects unknown, duplicate, and overlapping cursor lanes.
- Conflict and remote-ingest changes invalidate generation on actual authoritative state changes. Receipts retain the exact edit IDs of the command batch rather than deriving tail history IDs.
- Sync’s history conversion retains required empty message collections, rejects negative HLC values, and preserves server rejection diagnostic bytes instead of discarding them.
- Sync test fixtures now implement the `MutationOutcome` mutation contract and explicitly require a valid zero-edit store fixture before reading its envelope.
- The obsolete legacy envelope-dialect test was removed. The current envelope shape has no serde default compatibility for outcome/conflict arrays.

## Focused Adversarial Coverage

- Atomic quarantine acceptance with an accepted first envelope and fatal later envelope.
- Empty-store snapshot adoption in reverse arrival order, identity/schema mismatch rejection, and policy rejection quarantine.
- Mandatory authoritative metadata parsing.
- Strict malformed message, conflict, and cursor codec rejection.
- Durable message/conflict round-trip with content-addressed conflict identity.

## Files Changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`

## Verified

See `🧪️w1-runtime-outcome-remediation-log.md`. The isolated kernel library suite completed with 942 passing tests and no failures. The isolated sync-feature library compile completed successfully.
