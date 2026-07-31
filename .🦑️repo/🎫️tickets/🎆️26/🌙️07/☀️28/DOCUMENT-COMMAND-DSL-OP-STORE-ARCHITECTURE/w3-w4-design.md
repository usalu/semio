# W3 + W4 Design — Command Text/Binary + Persisted Undo

(Full design produced by a Plan agent against contract.md M-B/M-C/M-D, read-only verified
against the working tree. Execution-ready: file paths, region names, exact signatures, ordered
edit sequence. See /Users/ueli/.claude/plans/steady-cuddling-dijkstra.md for the approved
summary; this file is the full detail the summary was distilled from.)

---

# W3 — `DocumentCommand` text + binary (M-B, closes G3)

## Design decisions

**D1 — Grammar hosting: hybrid (derived header enum + hand-written generic block codec).**
`#[derive(DslOps)]` cannot be used on `DocumentCommand<Operation>` itself: the derive
(dsl/derive/rs/lib.rs:718-770) does not split generics and lowers every variant field through
`DslField`, which `Vec<Operation>` / `Box<DocumentCommand<Operation>>` cannot satisfy. Instead,
mirror the proven `OpsHeaderLine` pattern (store/rs/lib.rs:616): a private, non-generic
`CommandHeaderLine` enum derived with `DslOps` carries every structural line, and a hand-written
generic `print_command`/`parse_command` pair handles the indented `Op` payload lines (exactly the
`print_edit_lines` block shape at store/rs/lib.rs:665).

**D2 — `semantic_command` becomes typed.** `UndoWithPolicy.semantic_command: Option<String>`
currently holds a *JSON-encoded* `DocumentCommand` parsed at store/rs/lib.rs:1349-1354. With
`dispatch_json` deleted, this field flips to `Option<Box<DocumentCommand<Operation>>>` (text
form: a nested, 2-space-dedented command block; binary form: recursive length-prefixed
`encode_command`). Only in-crate users exist (store tests at :3752-3770, :4746-4756) — verified
by workspace grep.

**D3 — Author normalization law.** Like the `.ops` grammar, `Author.avatar` is never serialized
in either command form; the round-trip law is stated for avatar-less commands (same
canonicalization the `OpsAuthor` bridge at store/rs/lib.rs:587-605 already imposes).

## Text grammar (op-line syntax)

```
apply description="..."            # + one 2-space-indented Op::print_op line per operation
undo                               # DocumentCommand::Undo
undo policy=exact-base-only        # UndoWithPolicy (policy tokens = kebab of UndoPolicy variants)
undo policy=semantic-undo          # + nested command block indented 2 spaces
redo
commit-checkpoint message="..." by=[u1 "Ueli Saluz"]
create-alternative name="..."
switch-alternative <id>            # id positional
checkout <id>                      # id positional
amend key=...                      # + indented op lines (AmendLast)
```

Keywords come free from `to_kebab` on these variant names:

```rust
#[derive(Clone, Debug, PartialEq, DslOps)]
enum CommandHeaderLine {
    Apply { description: Option<String> },
    Undo { policy: Option<String> },
    Redo,
    CommitCheckpoint { message: Option<String>, by: Vec<OpsAuthor> },
    CreateAlternative { name: String },
    SwitchAlternative { #[dsl(positional)] id: String },
    Checkout { #[dsl(positional)] id: String },
    Amend { key: Option<String> },
}
```

`policy` tokens: `exact-base-only | transform-against-concurrent | semantic-undo |
compensating-action`, mapped to `protocol_core::UndoPolicy` in `parse_command` (unknown token →
`TextError`).

## Binary layout

`format u8 (=1) | command tag u8 | body` (contract M-B). Framing primitives are
`pack::write_varint_u64` + `pack::ByteReader` (already used identically by `dsl::op_rt`,
dsl/rs/lib.rs:325-367). `str` below = varint len + utf8; `op` = varint len + `Op::encode_op`
bytes.

```
const COMMAND_BINARY_FORMAT: u8 = 1;
tag 0 Apply:              presence u8 (bit0 description) | [description str] | op_count varint | op*
tag 1 Undo:               (empty)
tag 2 Redo:               (empty)
tag 3 UndoWithPolicy:     policy u8 (UndoPolicy ordinal) | presence u8 (bit0 semantic) | [varint len + encode_command bytes, recursive]
tag 4 CommitCheckpoint:   presence u8 (bit0 message) | [message str] | author_count varint | (id str, name str)*
tag 5 CreateAlternative:  name str
tag 6 SwitchAlternative:  id str
tag 7 CheckoutCheckpoint: id str
tag 8 AmendLast:          presence u8 (bit0 key) | [key str] | op_count varint | op*
```

Tags follow `DocumentCommand` declaration order (store/rs/lib.rs:64-98) — same "declaration
order is format" rule `op_rt` documents for variant ordinals.

## New surface (all in store/rs/lib.rs, new region `🔖️CommandFormat` inserted between
`//#endregion 🔖️TextFormat` (:927) and `//#region 🔖️History` (:929))

```rust
pub fn print_command<Op: OpText>(command: &DocumentCommand<Op>) -> Result<String, VcsError>;   // multi-line block, trailing '\n'
pub fn parse_command<Op: OpText>(text: &str) -> Result<DocumentCommand<Op>, TextError>;
pub fn encode_command<Op: protocol::OpBinary>(command: &DocumentCommand<Op>) -> Result<Vec<u8>, VcsError>;
pub fn decode_command<Op: protocol::OpBinary>(bytes: &[u8]) -> Result<DocumentCommand<Op>, VcsError>;
```

On `DocumentStore` (inside the existing impl at :1107, method-level extra bounds; replacing
`dispatch_json` at :1622-1626):

```rust
pub fn dispatch_text(&mut self, command_text: &str) -> Result<(), VcsError> where Operation: OpText;
pub fn dispatch_binary(&mut self, command_bytes: &[u8]) -> Result<(), VcsError> where Operation: protocol::OpBinary;
```

`test_support` (region :2821) gains, beside `assert_op_text_binary_equivalence` (:2927):

```rust
pub fn assert_command_text_binary_equivalence<Op>(command: &DocumentCommand<Op>)
where Op: OpText + protocol::OpBinary + Clone + PartialEq + std::fmt::Debug;
// law: parse_command(print_command(c)) == c == decode_command(encode_command(c)), deterministic bytes
```

## Ordered edit sequence (W3)

1. **store/rs/lib.rs `🔖️Schemas`** — flip `UndoWithPolicy.semantic_command` to
   `Option<Box<DocumentCommand<Operation>>>` (keep serde attrs; `Box` serializes transparently).
2. **store/rs/lib.rs `🔖️CommandFormat` (new region)** — `CommandHeaderLine`, tag consts, the four
   fns above. `parse_command`: first non-blank/non-`#` line → `CommandHeaderLine::parse_op(trimmed)`;
   subsequent `"  "`-indented lines → `Operation::parse_op` (Apply/Amend) or dedent-by-2 + recurse
   (`undo policy=semantic-undo|compensating-action`); any body under other headers is a `TextError`.
3. **store/rs/lib.rs `🔖️DocumentStore`** — `dispatch_inner` `SemanticUndo|CompensatingAction` arm
   (:1348-1355): replace JSON parse with `self.dispatch_inner(*command)`; delete `dispatch_json`
   (:1622-1626); add `dispatch_text`/`dispatch_binary` in its place.
4. **store/rs/lib.rs `🔖️TestSupport`** — add `assert_command_text_binary_equivalence`.
5. **store/rs/lib.rs `🧪️Tests`** — in `🔖️CommandErrorPaths` (:4657): rewrite
   `dispatch_json_applies_a_serialized_command_and_projection_json_reflects_it` (:4768) into
   `dispatch_text_applies_a_command_block...` (dispatch `apply` block + `not a command` error path
   + `dispatch_binary` wrong-format rejection); fix `compensating_undo_dispatches_semantic_command`
   (:3752) and `compensating_undo_without_a_semantic_command_is_rejected` (:4746) to the boxed
   field; add one test calling `assert_command_text_binary_equivalence` over every variant (reuse
   the crate's demo op fixtures used by the W1 op-law tests).
6. **Flip every `dispatch_json` caller** (complete grep-verified list, main tree only; each is a
   mechanical `dispatch_json(&str)` → `dispatch_text(&str)` (+ add `dispatch_binary(&[u8])` on the
   wasm bridges, `js_name = dispatchText` / `dispatchBinary`)):
   - Native pass-throughs: `s/rs/lib.rs:341` (`StudioStore::dispatch_json`) + wasm bridge :396 +
     test :437 (`{"kind":"undo"}` → `"undo"`); `framework/product/os/core/rs/lib.rs:1113`
     (`OsStore::dispatch_json`) + test :1809.
   - Wasm-bindgen engine bridges: `trinity/ram/rs/lib.rs:1253`,
     `trinity/rewrite/engine/rs/lib.rs:1063`, `animate/present/rs/lib.rs:825`,
     `playbook/rs/lib.rs:1193`, `fem/2d/rs/lib.rs:982`, `fem/3d/rs/lib.rs:1031`,
     `puzzle/3d/rs/lib.rs:3901`, `puzzle/5d/rs/lib.rs:964`, `cad/rs/lib.rs:1463`,
     `writer/rs/lib.rs:162`, `draw/rs/lib.rs:1787`, `flow/core/rs/lib.rs:4850`,
     `shooting/rs/lib.rs:967`, `process/3d/rs/lib.rs:501`, `procedural/2d/rs/lib.rs:691`,
     `procedural/3d/rs/lib.rs:691`, `raster/plugin/rs/lib.rs:571`, `gis/plugin/rs/lib.rs:247`,
     `infinite/board/port/directed/dag/rs/lib.rs:7468`.
   - `imperative/core/rs/lib.rs` uses typed `dispatch` only — no change.
   - TS: **no hand-written callers exist** (workspace grep over ts/tsx/js/svelte/vue finds only
     generated wasm-bindgen glue under `framework/product/os/dev/renderer-modules/wgpu/*.js`, e.g.
     `dagdocumentvcs_dispatchJson` — regenerated by the wasm builds in W10, nothing to hand-edit).
7. Verify: `cargo test -p store`, `cargo check --workspace --all-targets`,
   `bun nx run @semio-tech/store-rs:wasm` (note: W7 drops this target — if W7 lands first, skip).

---

# W4 — Persist undo: backwards section + cursor + storage flip (M-C/M-D, closes G4, G5)

## Design decisions

**D4 — Cursor record kind: extension range, owned by `protocol_history`.**
`pub const REC_CURSOR: u8 = 0x40;` in protocol/history/rs/lib.rs (the extension range 0x40..=0x7E
is caller-defined per protocol/core/rs/lib.rs:110; `protocol_core`'s frozen kind table stays
untouched). Written with the critical bit **unset** → foreign/older readers skip it (skip-unknown
rule, decode loop note at protocol/history/rs/lib.rs:992). Last-wins semantics, same as
`REC_ACTIVE`. Not riding `REC_ACTIVE`: cursor churns on every undo/redo while active-alternative
rarely changes, and overloading a critical record's payload couples two unrelated lifecycles.

**D5 — Cursor grammar carries the full applied list.** Contract M-D sketches
`cursor applied=<edit-id> redo=[...] checkpoint=<id>`, but a single marker id cannot represent
post-undo-then-apply states (`[e1,e2,~e3~,~e4~,e5]`: undone edits precede later applies in file
order and the redo stack is cleared), and checkout reassigns `applied_edit_ids` wholesale. Law 5
(`live == replay`) forces full fidelity, so the line is:

```
cursor applied=[e1 e2 e5] redo=[e4 e3] checkpoint=<id>
```

(`applied` in application order; `redo` in stack order bottom→top; `checkpoint` =
`current_checkpoint_id`, omitted when absent; active alternative stays on the existing `active`
line). Record this deviation-with-rationale in the wave report.

**D6 — Backwards + binary payloads are `.spr`-only; the `.ops` text mirror stays forwards-only
+ cursor line.** Text import (`parse_document_text`) keeps recomputing backwards/meta via replay
(store/rs/lib.rs:802-839 unchanged); the authoritative `.spr` carries them explicitly. Law 3
(`decompile_ops(compile_ops(t)) == t`) is preserved.

**D7 — The schema-opaque seam.** `compile_ops` (protocol/rs/lib.rs:33) can never produce binary
op payloads — op lines are opaque strings to `protocol_history`. Binary payloads therefore enter
`.spr` one layer up, where `Operation: OpBinary` is monomorphized: new
`store::print_document_spr`/`parse_document_spr` build/consume a typed `HistoryLog` directly, and
`DocumentCodec::of<P, Operation>` (store/rs/lib.rs:273) — the existing schema-string-keyed seam
`FolderEndpoint` already resolves (store/sync/rs/lib.rs:563) — carries them through to storage.
`compile_ops`/`decompile_ops` remain the text-tooling path (CLI, hand-imported logs; payloads
stay text-only there).

**D8 — Cursor rides `DocumentEnvelope`.** New serde-optional `cursor` field so it survives every
seam (backbone envelope JSON, codec, sqlite, actor). `DocumentStore::bump()` becomes the single
choke point that mirrors `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id` into
`envelope.cursor` (O(applied) clone per dispatch — acceptable; every cold path already pays
O(n)).

## Wave-internal split (2 agents per contract): **W4a = protocol_history**,
**W4b = store + store_sync**. W4b depends on W4a's model types; land W4a first.

## W4a — protocol/history/rs/lib.rs (+ facade)

1. **`🔖️Model`**: `HistoryEdit` gains `pub backwards: Vec<OpPayload>` (after `ops`; doc: empty =
   "not persisted, recompute on replay"). New:
   ```rust
   #[derive(Clone, Debug, PartialEq)]
   pub struct HistoryCursor {
       pub applied_edit_ids: Vec<String>,
       pub redo_edit_ids: Vec<String>,
       pub checkpoint_id: Option<String>,
   }
   ```
   `HistoryLog` gains `pub cursor: Option<HistoryCursor>` (keeps `Default`). Update the
   `OpPayload` doc (:47-48) — `binary` is no longer reserved.
2. **`🔖️TextGrammar`**: consts `F_CURSOR_APPLIED: u16 = 0; F_CURSOR_REDO: u16 = 1;
   F_CURSOR_CHECKPOINT: u16 = 2;` + `fn cursor_spec() -> RecordSpec` (keyword `"cursor"`,
   `applied`/`redo` = `Shape::List(Text)` keyed, `checkpoint` keyed Text); `print_ops_text` emits
   the cursor line last (after `active`, :422-425); `parse_ops_text` accepts it (last-wins).
3. **`🔖️Payloads`/`🔖️Edit`**: rewrite the module design note (:445-448). New helpers
   `write_op_payload(out, &OpPayload)` / `read_op_payload(input) -> OpPayload`: `op_tag = 0b01 |
   (0b10 iff binary.is_some())`, then text str-field, then `[varint len + binary bytes]`.
   `encode_edit` (:569): presence bit5 = `options.write_backwards_section &&
   !edit.backwards.is_empty()`; layout becomes `... op_count, op-payloads, [bit5: back_count
   varint + back payloads], [bit4: meta]` (backwards before meta; update the layout comment at
   :502-506). `decode_edit` (:624): delete the two rejection branches (:653-658), read binary
   when bit1 set, read backwards when bit5 set.
4. **New `🔖️Cursor` sub-region** in `🔖️Payloads`: `pub const REC_CURSOR: u8 = 0x40;` +
   ```rust
   pub fn encode_cursor(cursor: &HistoryCursor, dict: &mut DictBuilder, edit_ordinal_of: impl Fn(&str) -> Option<u64>) -> Result<Vec<u8>, ProtocolError>;
   pub fn decode_cursor<'d>(payload: &[u8], dict: &'d DictReader, ordinal_to_id: impl Fn(u64) -> Result<&'d str, ProtocolError>) -> Result<HistoryCursor, ProtocolError>;
   ```
   Layout: `format u8 (=1) | presence u8 (bit0 checkpoint) | applied_count varint + id* |
   redo_count varint + id* | [checkpoint id]` (ids via `write_id_field`, dict + edit-ordinal
   refs).
5. **`🔖️Codec`**: `encode_history` writes `REC_CURSOR` (critical=false) after `REC_ACTIVE` when
   `log.cursor.is_some()`; `decode_history` matches `REC_CURSOR` (last-wins) — every edit
   constructed from text/old streams gets `backwards: Vec::new()`.
6. **`🔖️Append`**: `HistoryAppender` gains
   `pub fn append_cursor(&mut self, cursor: &HistoryCursor) -> Result<u64, ProtocolError>`.
7. **`🧪️Tests`** (extend existing regions in-file): TextGrammar cursor-line round trip; Payloads:
   op payload with binary bit round trip, edit with backwards section round trip (bit5 + bit4
   together), cursor payload round trip incl. dict/ordinal refs; Codec: whole-log round trip with
   cursor + backwards + binary payloads under `EncodeOptions { write_backwards_section: true,
   .. }`; Append: `append_cursor` then decode.
8. **protocol/rs/lib.rs `🔖️Reexports`** (:13): add `HistoryCursor, REC_CURSOR, encode_history,
   decode_history, parse_ops_text, print_ops_text` to the `protocol_history` re-export line
   (store needs them; also chips at G21). Every `HistoryEdit` literal in protocol/rs,
   protocol/cli, protocol/testkit tests gains `backwards: Vec::new()` (compiler-driven).

## W4b — store + store_sync

1. **store/rs/lib.rs `🔖️Schemas`**: new
   ```rust
   #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
   #[serde(rename_all = "camelCase")]
   pub struct DocumentCursor {
       #[serde(default, skip_serializing_if = "Vec::is_empty")] pub applied_edit_ids: Vec<String>,
       #[serde(default, skip_serializing_if = "Vec::is_empty")] pub redo_edit_ids: Vec<String>,
       #[serde(default, skip_serializing_if = "Option::is_none")] pub checkpoint_id: Option<String>,
   }
   ```
   `DocumentEnvelope` gains `#[serde(default, skip_serializing_if = "Option::is_none")] pub
   cursor: Option<DocumentCursor>`. Add `cursor: None` at every envelope struct-literal site
   fleet-wide (compiler-driven; includes `create_document_envelope`, os/core `OsEnvelope`, s/rs
   `SStudioEnvelope`, app crates).
2. **store/rs/lib.rs `🔖️TextFormat`**: `OpsHeaderLine` gains `Cursor { applied: Vec<String>,
   redo: Vec<String>, checkpoint: Option<String> }`; `print_ops_log` (:693) emits it last from
   `envelope.cursor`; `replay_ops` (:773) — replace the "every edit applied" doc/behavior
   (:767-772): keep the sequential file-order fold (backwards/meta correctness), capture the
   cursor line, and when present set `envelope.cursor` and recompute the returned `projection`
   as a fold of `initial_projection` over `cursor.applied_edit_ids` only.
3. **store/rs/lib.rs `🔖️TextFormat`** (spr twins, beside `print_document_pack` :757):
   ```rust
   pub fn print_document_spr<P, Operation>(envelope: &DocumentEnvelope<P, Operation>) -> Result<Vec<u8>, VcsError>
   where Operation: OpText + protocol::OpBinary;                     // HistoryLog{ops/backwards: OpPayload{text: print_op, binary: Some(encode_op)}, meta from operation_meta, cursor} → encode_history(write_backwards_section: true)
   pub fn parse_document_spr<P, Operation>(pack: &[u8], spr: &[u8]) -> Result<ParsedDocumentText<P, Operation>, TextError>
   where P: Clone + DocumentPack, Operation: OpText + protocol::OpBinary + crate::Operation<P>;   // decode_op when binary present, else parse_op; persisted backwards/meta when present, else replay-recompute; honors cursor
   ```
   Plus the two private `HistoryOpMeta ↔ OperationMeta` mappers (undo_policy u8 ↔ `UndoPolicy` by
   ordinal 0-3).
4. **store/rs/lib.rs `🔖️Pack`**: `DocumentPackFiles` gains `pub spr: Vec<u8>` (doc: `.pack` +
   `.spr` authoritative, `ops` is the human mirror). `print_document_pack` gains the
   `protocol::OpBinary` bound and fills `spr` via `print_document_spr`; `parse_document_pack` is
   replaced by the spr-first read (delegates to `parse_document_spr`; the dsl+ops text path stays
   `parse_document_text`).
5. **store/rs/lib.rs `🔖️CodecRegistry`**: `DocumentCodec.parse` becomes `fn(&[u8], &[u8]) ->
   Result<String, VcsError>` (pack bytes, spr bytes); `parse_dsl` unchanged; `of<P, Operation>`
   gains `protocol::OpBinary` bound on `Operation` (fleet-satisfied since W2).
6. **store/rs/lib.rs `🔖️DocumentStore`**: private `fn sync_cursor(&mut self)` writing
   `envelope.cursor`; call it inside `bump()`; `new()` (:1115) and `set_state` (:1201) honor
   `envelope.cursor` when present (seed `applied_edit_ids`/`redo_edit_ids`/`current_checkpoint_id`,
   `fold_current` over applied); `set_state`'s explicit params stay authoritative over a stale
   envelope cursor (then re-synced via `bump`).
7. **store/sync/rs/lib.rs `🔖️FolderStorage`**:
   - `FolderTextStorage`: add `pub fn spr_path(&self, document_id: &str, extension: &str) ->
     PathBuf` (`<id>.<ext>.spr`); `read_pack` (:1679) returns `DocumentPackFiles { pack, spr,
     ops }` — a present `.pack` with a missing `.spr` is a hard error (they are written together;
     no legacy); `write_pack` (:1696) writes four files: `.pack`, `.spr`, `.ops` mirror, `.<ext>`
     dsl mirror.
   - `FolderSqliteStorage`: `ensure_schema` (:1543) flips to `document(id TEXT PRIMARY KEY,
     schema TEXT, pack BLOB NOT NULL, spr BLOB NOT NULL, updated_at INTEGER NOT NULL)` (json
     column deleted; dev DBs at `<folder>/.semio/documents.db` are simply deleted/recreated —
     migration-free); `read`/`write`/`read_pack`/`write_pack` (:1563-1605) collapse to
     ```rust
     pub fn read(&self, document_id: &str) -> Result<Option<(Vec<u8>, Vec<u8>)>, vcs::VcsError>;      // (pack, spr)
     pub fn write(&self, document_id: &str, schema: &str, pack: &[u8], spr: &[u8]) -> Result<(), vcs::VcsError>;
     ```
8. **store/sync/rs/lib.rs `FolderEndpoint`** (:549-591): `Sqlite` read/write now go through
   `store::document_codec(schema)` exactly like `Pack` (missing codec = hard error): write =
   `codec.print(json)` → `storage.write(id, schema, &files.pack, &files.spr)`; read =
   `storage.read` → `(codec.parse)(&pack, &spr)`. `Pack` arm passes `pack_files.spr` through.
9. **Fixture regeneration list** (verified by `find`): **no `.ops`/`.pack`/`.spr` fixtures exist
   anywhere on disk** — app `example/` dirs hold DSL sources only (`note/example/semio.note`,
   `s/example/demo.s`, `draw/example/*`, …29 dirs) and are untouched;
   `store/sync/fixtures/{basic-remote-operations,remote-operations-backlog,snapshot-replaced}.json`
   keep parsing (envelope `cursor` is serde-defaulted) — regenerate only if an expected-output
   assert trips; `store/sync/fixtures/wire/*.bin` are W5's byte-coupled set, untouched here.
10. **Tests** (extend existing in-file suites only):
    - store: extend `🔖️TextFormatHelpers` (:4562) — cursor line round trips through
      `print_document_text`/`parse_document_text` after apply-apply-undo; add
      `assert_document_pack_round_trip` update (spr-aware) + new
      `test_support::assert_document_spr_round_trip` asserting decoded backwards/meta equal the
      envelope's; the **save→load→undo proof**: build store → apply e1, e2 → undo →
      `print_document_pack` → `parse_document_spr` → `DocumentStore::new(parsed.envelope)` →
      assert projection == post-e1 state, `dispatch(Redo)` restores e2, `dispatch(Undo)` twice
      reaches initial, `assert_live_equals_replay` after each.
    - store_sync: extend `folder_text_storage_round_trips_pack` (:2539) to assert `.spr` on disk
      + spr-first read; flip the sqlite tests (:2042, :2407, :2469, :2580, :2607) to the blob
      schema; add endpoint-level save→load→undo (write via `FolderEndpoint`, read back,
      reconstruct, undo works).
    - protocol facade test `compile_ops_decompile_ops_round_trip` (:220): extend the fixture log
      with `backwards` + `cursor` to prove text-mirror stability.
11. Verify: `cargo test -p protocol_history -p protocol -p store -p store_sync`,
    `cargo check --workspace --all-targets`, `bun nx run @semio-tech/store-rs:wasm` (skip if W7
    already dropped it), `bun nx run @semio-tech/store-sync-rs:test`.

**Cross-wave laws exercised**: 1 (unchanged), 2 (per-op, unchanged), 3 (extended fixture), new
command law (W3), 4 via persisted backwards equality, 5 via `assert_live_equals_replay` post-reload;
law 6 untouched until W5.

**Sequencing note**: W3 and W4b both edit store/rs/lib.rs — keep them serial (W3 → W4a ∥ → W4b),
and re-read shared files immediately before editing per the contract's concurrent-devs rule.
