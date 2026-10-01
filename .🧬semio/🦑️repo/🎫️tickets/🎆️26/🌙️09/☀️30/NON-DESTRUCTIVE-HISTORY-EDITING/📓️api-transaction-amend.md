# 📓️ API: transaction-scoped amend (design §15)

For S2-STROKES (remodel streamed import) and the later W3-T2-CLOSURE WP (deletes `Emit::amend`, `AmendLast` and static
coalesce keys). Owner: W1-G (store + runtime route). Status and verification: `📓️w1-g-report.md`, section "Session 2".

## 1. What it is

A tool whose yield cannot wait for its commit — a streamed import of up to 200 frames, 40–80 MB, beyond the 1 MiB transient
bound and the 256 KiB dispatch bound — streams it into ONE open edit of the document store. Every streamed operation carries
the transaction's `TransactionRef`. The edit is keyed by the transaction id, never by a coalesce key.

- **Stream.** Every tick appends its operations to the open edit. The first tick opens it.
- **Visible at once.** The store folds the open edit as its applied tail, so the document shows the frames immediately.
- **Not shared yet.** The open edit is not announced on the backbone and not written to any persisted form (`.spr`, `.ops`,
  pack, archive) before the commit.
- **Commit.** The commit closes the edit. It is then exactly the edit ONE `Apply` of the same operations under the same ref
  would have recorded: one history row, one undo step, every mutation editable through time travel. All its operations are
  announced in one batch.
- **Abort.** The abort reverts the open edit with zero trace. That means no edit, no row, no message and no announcement.
  The content revision and the next edit's sequence number are the same as before the first tick.
- **Crash.** A closed tab or a crash also leaves zero trace, because nothing was persisted.

## 2. App API (`OS/🔌️plugin/🦀️.rs`, `Emit`)

```rust
pub enum TransactionPhase { Commit /* default */, Stream, Abort }
pub struct Emit<M, C, D> { …, pub transaction: Option<protocol::TransactionRef>, pub transaction_phase: TransactionPhase, … }

Emit::stream_transaction(transaction, mutations)   // one tick: append to the open edit (opens it)
Emit::commit_transaction(transaction, mutations)   // close it (mutations may be empty); with none open: one plain edit
Emit::abort_transaction(transaction)               // revert the open edit; with none open: empty emission
```

### Shape rule

The shape rule is `tool_transaction_shape_fault`, and its fault code is `toolTransaction.shape`. It refuses these emissions:

- a transaction together with a `coalesce_key`;
- a stream or an abort that names no transaction;
- a stream or an abort with `child_emits` (a streamed transaction grows the app's own document only);
- a stream whose operations have foreign steps;
- an abort that carries artifact mutations.

Config, window-config and ephemeral lanes may ride along with any phase.

### While a transaction is open on the document store

- Any other artifact emission (plain, coalesced, or another transaction) is refused with `toolTransaction.open`
  (`VcsError::TransactionOpen`).
- Undo, redo, checkpoint, alternative, supersede and resolve commands are refused the same way.
- `historyEditBegin` answers `timeTravel.busy`.
- Remote edits keep arriving. The store lifts the open edit off the tail, merges the remote edit, and folds the open edit
  back on top with fresh clocks (keep-and-record: never quarantined).
- A commit or abort naming another transaction is refused with `toolTransaction.unknown`.

### Routes

Both dispatch routes behave the same:

- **Unmigrated `dispatch_emit`.** Uses the store commands below.
- **Migrated typed operations.** A stream is a batched publication with `set_transaction_open(true)`. A closing commit with
  operations is a batched publication that appends and commits. An empty commit or an abort is a direct store command.

The history row is recorded once, at the first tick. An abort retires it (`retire_displaced_document_rows`).

## 3. Store API (`OS/🏪️store/🦀️.rs`)

```rust
ArtifactCommand::AppendTransaction { mutations, transaction }   // ordinal 19
ArtifactCommand::CommitTransaction { transaction_id }           // ordinal 20
ArtifactCommand::AbortTransaction  { transaction_id }           // ordinal 21
ArtifactStore::open_transaction() -> Option<&OpenToolTransaction>  // { transaction, edit_id }
ArtifactEnvelopeOwners::open_transaction / holds_committed_edit(edit_id)
ArtifactStoreBatchPublication::set_transaction_open(bool)
VcsError::{TransactionOpen{transaction_id}, UnknownTransaction(id), HistoryFull{capacity}}  // codes toolTransaction.open|unknown, history.full
```

### Codecs

- Text: `append-transaction` / `commit-transaction` / `abort-transaction` header lines.
- Binary: `format 1 | ordinal | id | tool | ops`.
- Language-agnostic vectors are in `🏪️store/🧫️fixtures/🧫️tool-transaction/🔣️.json` (`codec`).
- On the wire nothing new is added: committed operations carry `MutationEnvelope.transaction` (W1-A).

## 4. Remodel example (S2-STROKES)

The import is a tool machine whose transient state holds only `{ transaction, stream, done, total }`. It never holds the
frames.

```rust
// importVideo / importFrames start: mint the ref once, keep it in the window transient (≤ a few hundred bytes).
let transaction = protocol::TransactionRef::mint(&actor, clock, &format!("{APP_ID}#import"));

// importVideoFramePayload{index,total}: one tick, ops = create-asset + create-stream | add-stream-frame
Ok(Emit::stream_transaction(transaction.clone(), frame_mutations(payload)?))

// importVideoDone: the last op and the commit (or an empty commit)
Ok(Emit::commit_transaction(transaction, vec![replace_stream_source(stream_id, Some(source))]))

// Cancel button / blur policy / HostEvent::Retiring of the importing window / any host abort reason:
Ok(Emit::abort_transaction(transaction))
```

### Rules for the tool

- Progress and cancel are the tool's job:
  - show `{done, total}` from the transient;
  - Cancel emits `abort_transaction`;
  - an import must not leave the document wedged.
- While the transaction is open every other artifact verb of the app is refused, and so is time travel.
- If the tool loses its transient (the window closes), its `Retiring` host event must abort. A reload loses nothing,
  because nothing was persisted.
- Make each tick's operations relative to the stream they append to, so that a remote edit landing underneath re-folds
  correctly. Remote edits never reorder the open edit.
- Replace the static key `remodeling-import:{stream}` with the transaction id. Delete `Emit::amend` from the import verbs.

## 5. For W3-T2-CLOSURE

Every remaining `Emit::amend` / `AmendLast` / static `coalesce_key` gesture converts to one of two shapes:

- a `ToolMachine` with `Emit::commit_transaction`, when the yield fits the transient;
- `stream_transaction` + `commit_transaction` / `abort_transaction`, when it does not.

After that, delete:

- `ArtifactCommand::{AmendLast, AmendLastInLane}`, `Emit::{amend, amend_config}`, `Emit.coalesce_key`;
- `ArtifactStoreBatchPublication::set_coalesce_key` and the coalesce branch of `batch_amend_target`;
- the `coalesced` parameter of `tool_transaction_shape_fault`;
- `Edit.coalesce_key` (persisted in `.spr`/`.ops`: drop the field in both codecs and the fixtures at once).

Config-lane coalescing (camera drags) needs the same transaction route on the config store. `AppendTransaction` is
store-generic, so it carries over.
