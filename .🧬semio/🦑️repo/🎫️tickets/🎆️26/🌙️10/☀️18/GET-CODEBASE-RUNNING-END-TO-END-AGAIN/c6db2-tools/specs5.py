"""📐 C6DB2 chunk tables for the flat test roots: tag -> rows (first item line of the ORIGINAL snapshot as printed by ol.py, folder emoji, kebab, doc) and optional end_line (first item line that stays in the root)."""
FRAG = {}

FRAG["wtests"] = dict(rows=[
    (7, "🧰", "wal-test-fixtures", "WAL test fixtures: the page, document, frontier and control helpers, the rejected open error, the replay summaries, the segment and committed fixtures and the no-committed-transaction assertion."),
    (161, "🛑", "recovery-abort-laws", "Recovery abort laws: the idempotent abort of incomplete active transactions, the abort fsync across reopens, the abort fault retry, the abort cancellation storage and the abort capacity boundary."),
    (487, "📏", "neutral-committed-span-laws", "Neutral committed span laws: the committed fixture kind, the transaction gate, the immutable source fragmentation, the committed cursor laws and the maximal command batch transaction."),
    (846, "🧮", "capacity-preflight-and-neutral-recovery-laws", "Capacity preflight and neutral recovery laws: the capacity helpers, the capacity preflight boundaries, the recovery seed and the neutral committed prefix and lifecycle recovery."),
    (1102, "🧩", "segment-and-artifact-wal-laws", "Segment and artifact WAL laws: the single segment commit, group commit, fsync durability, torn tail, resumed transaction ids, bounded segment verification, rotation chains, sealed segment tears and the artifact WAL writer reacquisition."),
])

FRAG["atests"] = dict(rows=[
    (7, "🏃", "runner-terminal-authority-and-retirement-laws", "Runner terminal authority and retirement laws: the hash member store producer, the runner terminal authority latch, the terminal resume refusal, the authority drop transfer, the retirement panic and the retirement idle wake."),
    (285, "⏲", "engine-close-fault-and-live-authority-laws", "Engine close fault and live authority laws: the bounded timer backoff, the exhausted budget, the cancelled timer, the live authorities over the pool maintenance hooks and the runner terminal close and closing poll."),
    (528, "🗝", "hash-projection-and-recovery-fixtures", "Hash projection and recovery fixtures: the storage, document and stored json helpers, the hash projection, diff and mutation, the retirement factories and the committed recovery store and journal helpers."),
    (956, "📦", "durable-group-journal-and-source-laws", "Durable group journal and source laws: the durable group journal commits, the committed group decision and recovery, the journal cancellation, the finite pool turns, the group commit close drain and the runner source laws."),
])

FRAG["stests"] = dict(rows=[
    (4, "🔁", "wal-command-decoder-and-replay-laws", "WAL command decoder and replay laws: the admitted wal bytes, the command decoder agreement, the retained hello replay, the missing tail stream, the fifty thousand edit stream and the neutral aborted replay."),
    (259, "🧭", "codec-state-frontier-and-bootstrap-laws", "Codec, state, frontier and bootstrap laws: the sync fixtures, the envelope codec, the replica state, the frontier delta, the missing commands, the bootstrap decisions, the resume token and the hello and frontier advertise handlers."),
    (588, "🪢", "retained-sync-hello-laws", "Retained sync hello laws: the held pool helpers and every retained sync hello handoff, refusal, stream, snapshot cursor, deadline, cancellation, quarantine and census law."),
])

FRAG["aetests"] = dict(rows=[
    (7, "📜", "history-replay-and-staging-laws", "History replay and staging laws: the rejected open error, the neutral aborted open, the create rejection release owner, the real committed batch replay, the neutral inner documents and the staging retirement."),
    (337, "✍", "submit-materialize-dedupe-conflict-and-undo-laws", "Submit, materialize, dedupe, conflict and undo laws: the submit and replay of the wal, the ledger tail, the snapshot suffix materialization, the dependency and dedupe laws, the conflict detection and the undo."),
    (660, "↩", "preview-query-and-actor-laws", "Preview, query and actor laws: the preview transitions, the security gate, the query laws, the index backlog and receipt window, the actor history phases and the source laws."),
])
