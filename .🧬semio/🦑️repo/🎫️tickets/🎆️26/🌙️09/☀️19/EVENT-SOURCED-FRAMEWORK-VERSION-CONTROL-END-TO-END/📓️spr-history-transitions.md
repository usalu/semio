# 📓️ `.spr` History Log: Transitions Replace Persisted Derived State

The `.spr` history log (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️.rs`) now stores only the semantic event log: edits and history transitions, plus conflicts and the composition owner/dialect overlay. Changes, checkpoints, alternatives, the active alternative, checkpoint pins and the undo/redo cursor are no longer stored. `HistoryLog::fold` derives them.

## 🔌️ API as implemented

```rust
pub struct HistoryLog { doc_id, schema, edits: Vec<HistoryEdit>, transitions: Vec<HistoryTransitionRecord>, composition: Option<HistoryComposition>, conflicts: Vec<HistoryConflict> }
pub struct HistoryTransitionRecord { id: String, actor: String, hlt: (u64, u64, u64), dependencies: Vec<String>, payload: Vec<u8> }
impl HistoryTransitionRecord { fn from_envelope(&MutationEnvelope) -> Self; fn to_envelope(&self, document_id: &str) -> MutationEnvelope }
pub struct HistoryComposition { owner: Option<(String, String, String)>, dialect: Option<(String, String, String)> }
pub const REC_TRANSITION: u8 = 0x43; // critical frame, one per transition
impl HistoryLog { fn fold(&self) -> Result<HistoryFold, ProtocolError> }
pub async fn encode_transition(&HistoryTransitionRecord, &mut DictBuilder) -> Result<Vec<u8>, ProtocolError>;
pub async fn decode_transition(&[u8], &DictReader) -> Result<HistoryTransitionRecord, ProtocolError>;
HistoryAppender::append_transition(&HistoryTransitionRecord) -> Result<u64, ProtocolError>;
```

- **REC_TRANSITION payload:** `format u8 (=1) | id | actor | hlt (3 varints) | dependency_count + dependency* | payload_len + payload`. The id, actor and dependencies are interned in the dictionary. Decoding rejects a newer format, truncation and trailing bytes.
- **`to_envelope`:** the diff and the empty inverse both carry `HISTORY_TRANSITION_SCHEMA`.
- **`fold`:**
  - An edit's clock is the `hlt` of its first op meta. It is zero when there is no meta. A negative physical time is an error.
  - An edit's mutation ids are each op's `op_id`, falling back to `{edit.id}#{i}`.
  - An edit is excluded when it owns an op of a Quarantined conflict (kind 0) whose status is not Accepted. The owning op is found by decoding the conflict's opaque envelope bytes with `crate::os_spr::decode_envelope`, which is how the store encodes them. Trailing bytes are rejected.
- **`.ops` text grammar:** one line per transition:

  ```
  transition <id> actor=<a> hlc=<actor>,<physical_ms>,<logical> dependencies=[...] payload="<base64>"
  ```

  `payload` uses the DSL's `Shape::Bytes64` shape. The `change`, `checkpoint`, `alternative`, `active` and `cursor` lines are now rejected as unknown keywords.
- **`RetainedHistoryDecode`:** decodes `REC_TRANSITION` records and validates each transition within the step budget: the id and actor are non-empty, the id is unique, and the payload decodes as a `HistoryTransition`. The cursor, active-alternative and pin checks are gone.
- **Removed:** `HistoryChange`, `HistoryCheckpoint`, `HistoryAuthor`, `HistoryAlternative`, `HistoryCursor`, and the encode/decode functions for change, checkpoint, alternative, active and cursor. Also removed: `REC_CURSOR`, `append_change`, `append_checkpoint`, `append_alternative`, `set_active` and `append_cursor`.
- **Replication crate:** `REC_CHANGE`, `REC_CHECKPOINT`, `REC_ALTERNATIVE` and `REC_ACTIVE` are removed from `🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs`, together with its `is_critical_kind` entries and tests. They live in the `🧾️wire` crate, not in `📐️format`. There are no TypeScript copies of these constants.

## 🗂️ Files changed

### `.spr` module
- `📡️spr/📜️history/🦀️.rs`: model, fold, text grammar, codec, retained decoder, appender.
- `📡️spr/📜️history/🧪️tests/🔬️unit/🦀️.rs`
- `📡️spr/📜️history/🧫️fixtures/🔀️transition-record/🔣️.json`: new language-agnostic fixture with the record hex and the dictionary.
- `📡️spr/🦀️.rs`:
  - updated the re-exports
  - `content_frontier` now uses the fold's alternatives, checkpoints and changes.
- `📡️spr/⌨️cli/🦀️.rs` and its tests:
  - `verify` prints the transition count
  - `log` folds the history
  - `inspect` labels `transition` records.
- `📡️spr/🔌️io/🦀️.rs` and its tests: resume and compaction now replay transitions.
- `📡️spr/💎️materialize/🦀️.rs` and its tests: checkpoint resolution goes through the fold.
- Protocol-law tests: `📡️spr/🧪️tests/⚖️protocol-laws/🦀️.rs` and `⚖️protocol-laws-unit/🦀️.rs`.
  - The generator now emits Commit and Branch transitions.
  - The structural-record law now covers `REC_TRANSITION`.
- `📡️spr/🧪️tests/🔬️unit/🦀️.rs`

### Store and member open
- `🏪️store/🧾️document/📜️history/💧️hydration/🦀️.rs`:
  - calls `history.fold()` during Begin and rejects with Replay if the fold fails
  - sets `envelope.cursor` and `active_alternative_id` from the fold
  - adds a new `HydrateTransitions` phase that fills `envelope.transitions` with `to_envelope`
  - fills the change, checkpoint and alternative ledgers and the pins from the fold, one item per step
  - hands the fold to bounded retirement.
- `🏪️store/🎚️config/📥️retained/🦀️.rs`: config hydration now folds. It rejects any change, checkpoint or alternative fact as an Identity error, and takes the cursor and transitions from the fold.
- `🏪️store/🧩️composition/🚪️open/🦀️.rs`: updated the retire impls. Added impls for `HistoryTransitionRecord`, `HistoryFold`, `FoldChange`, `FoldCheckpoint`, `FoldAlternative`, `TransitionAuthor` and `TransitionPin`. Removed dead helpers.
- `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs`: removed the unreachable duplicate hydration phases and their fields. These were BeginEdit through RetireHistory, plus Initialize. `Hydrate` already delegates to `RetainedPersistedDocumentHydration`.
- The retained member-dictionary parser (`🗂️dictionary/🦀️.rs` and `🛂️identity/🦀️.rs`):
  - no longer parses pin groups
  - no longer depends on the active or cursor records
  - requires `REC_TRANSITION` to be critical
  - treats any transition as making the history non-initial.
- Fixtures, schemas and tests updated:
  - `🗂️dictionary`: the composition bytes shrink by 8, the pin rows are gone, and a `noncritical-transition` case is new.
  - `🏭️factory`: semantic histories shrink by 14 bytes because the active frame and the pin-group byte are gone.
  - The identity fixture and schema in `📡️spr/📜️history/🛂️identity/`.
- TypeScript oracles in `🖥️host/📦️packages/🦀️rust/📜️script.ts`: the identity and dictionary oracles match the Rust changes.

### Other consumers
- `🔌️plugin/🧪️tests/🧩️composition/🦀️.rs`: the malformed-history cases now use a malformed transition.
- The stdio `flow_retained_decode` test uses a non-initial history made from a Revert that names an unknown operation.
- The trinity rewriting and reasoning wires `♻️reset-document.json` fixtures and tests now count `transitions`.
- `🏪️store/🧪️tests/🔬️unit/🦀️.rs`: removed one `checkpoint_pins` line. This file is otherwise the other agent's.

## 🧪️ Tests run (foreground)

| Command | Result |
|---|---|
| `cargo check -p semio-framework-os-kernel --features sync --lib` | ✅ no errors (the store errors seen earlier are fixed now) |
| `cargo check … --lib --tests` | ✅ |
| `cargo test -p semio-framework-os-kernel --lib os_spr:: -- --skip os_spr::channel --skip op_dag_convergence --skip merge_convergence` | ✅ 230 passed (history 52, cli 24, io 12, materialize 11, protocol_laws 64, spr unit 8) |
| `cargo test … --lib member_open` | ✅ 13 passed, including the dictionary and factory fixture laws |
| `cargo test -p semio-framework-replication --lib kind` | ✅ 6 passed |
| trinity rewriting (`--features component-app-assembly`) and reasoning wires reset tests | ✅ |
| `cargo check -p semio-framework-os-kernel --bin spr`, and the stdio, trinity and reasoning test targets | ✅ |

## ⚠️ Open or unrelated

- **Failing tests that do not touch history:**
  - `os_spr::channel::tests::paged_*` (2 tests): app-command decoding.
  - The protocol-law tests `op_dag_convergence…` and `merge_convergence…`: MutationDag convergence plus Drop aborts in `📡️replication/🔗️causal/🦀️.rs`, a file that is modified concurrently.
  - Many `os_store::` tests fail with `artifact store final envelope detached…` or `mutation dag reached Drop…` aborts. These belong to the store and DAG refactor.
- **stdio `flow_retained_decode`:** the history part passes (Replay rejection). The test later fails at line 406, in the snapshot phase for an unsupported subset, which has nothing to do with history.
- **`semio-framework-plugin` lib tests** do not compile: `🔌️plugin/🧪️tests/🔬️tool-run` uses the removed `BackboneMessage::Snapshot`, which is the other agent's change. My edits to the plugin composition test compile, but those tests could not be run. The same applies to the window-config retained hydration.
- **Stale comment for the other agent:** `🏪️store/🧪️tests/🔬️unit/🦀️.rs:6351` still mentions `HistoryCursor` in a comment.
- **Not rewritten, by choice:** `🏪️store/🧩️composition/🚪️open/📜️history/🧫️fixtures/🔣️.json` `historyHex` still contains a kind-64 frame. It is an opaque framing-only fixture. Rewriting it means recomputing the CRCs and the blake3 chain, and its tests pass as it is.
- **Pre-existing:** `compact` and `open_append` in `🔌️io` drop `composition` and `conflicts`. This was already the case before this change and is not fixed here.
