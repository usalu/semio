# Wave 1 — vcs integration

Single agent, critical section (you are the only editor of `vcs/rs/lib.rs` and
`vcs/rs/Cargo.toml` for this wave — re-read both fully before editing, other sessions may have
changed them since this file was written). Prerequisite: Wave 0 (the `protocol` crate family) is
merged and `cargo build -p protocol` succeeds.

Read first: `/Users/ueli/Documents/semio/vcs/rs/lib.rs` in full (5500+ lines) — you need the exact
current shape of regions `🔖️Schemas`, `🔖️Errors`, `🔖️Text` (855+), `🔖️Pack` (243+),
`🔖️CodecRegistry` (356+), `🔖️Operation` (606+), `🔖️Materialize` (738+), `🔖️OpsHeaderGrammar`
(872+), `🔖️History` (1218+), `🔖️DocumentVcsStore` (1344+, especially `set_state` ~1488 and
`fold_current`), `🔖️Backbone` (2192+), `🔖️BlobStore` (2746+), `🔖️TestSupport` (3324+). Also read
the finished `/Users/ueli/Documents/semio/protocol/rs/lib.rs` facade and
`/Users/ueli/Documents/semio/protocol/history/rs/lib.rs` for the exact API you're calling. Also
read `/Users/ueli/Documents/semio/.repo/🎫️/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` and
the plan at `/Users/ueli/.claude/plans/we-want-to-create-refactored-harbor.md` for full rationale.

## What to build

1. **`vcs/rs/Cargo.toml`**: add `protocol = { path = "../../protocol/rs" }` next to the existing
   `pack = { path = "../../pack/rs" }` dependency.

2. **New region `//#region 🔖️Protocol`** in `vcs/rs/lib.rs`, placed directly after `🔖️Pack`
   (mirror its structure exactly):
   ```rust
   pub use protocol::{
       EncodeOptions as ProtocolEncodeOptions, DecodeOptions as ProtocolDecodeOptions,
       ProtocolError, VerificationLevel as ProtocolVerificationLevel,
   };
   pub mod protocol_rt {
       // Thin forwards: HistoryLog <-> vcs Edit/Change/Checkpoint/Alternative conversion lives
       // HERE (protocol_history::HistoryLog has no vcs types; this module is the bridge).
       // encode_edit_segment(&Edit<Operation>) -> Result<Vec<u8>, VcsError> for the hot append
       // path (calls protocol_history::encode_edit after converting Edit -> HistoryEdit).
   }
   ```
   Conversions you need both ways: `vcs::Edit<Operation> <-> protocol::HistoryEdit` (forwards ops
   via `Operation::print_op`/`Operation::parse_op` — `Operation: OpText`; `backwards`/
   `operation_meta` map to `HistoryEdit.meta`/the backwards-section, write them when available,
   which is always true for locally-created edits since the hot path always has them in memory),
   `vcs::Change <-> protocol::HistoryChange`, `vcs::Checkpoint <-> protocol::HistoryCheckpoint`
   (+ `vcs::Author <-> protocol::HistoryAuthor`), `vcs::Alternative <-> protocol::HistoryAlternative`.

3. **Delete `DocumentPackFiles { pack: Vec<u8>, ops: String }` entirely.** Replace with:
   ```rust
   #[derive(Clone, Debug, Default, PartialEq, Eq)]
   pub struct DocumentBinaryFiles { pub pack: Vec<u8>, pub protocol: Vec<u8> }
   ```
   `DocumentTextFiles { dsl, ops }` is UNCHANGED — it remains the only home of ops text.

4. **Free functions** (mirror `print_document_text`/`parse_document_text`/`print_document_pack`/
   `parse_document_pack`, which you are replacing the pack variants of):
   ```rust
   pub fn print_document_binary<P, Operation>(envelope: &DocumentVcsEnvelope<P, Operation>) -> Result<DocumentBinaryFiles, VcsError>
   where P: DocumentPack, Operation: OpText;
   // pack = initial_projection.encode_pack(); protocol = protocol_history::encode_history of the
   // HistoryLog built from envelope.vcs.{edits,changes,checkpoints,alternatives,active_alternative_id}
   // via protocol_rt conversions, with write_backwards_section: true (vcs always has backwards in
   // memory when printing from a live envelope) — this is what makes the fast materialize path work.
   // Also: call protocol_materialize APIs to embed/append a REC_PROJECTION checkpoint segment at
   // the latest checkpoint (if any) and at log end, per CheckpointPolicy::default(), using
   // envelope.vcs.initial_projection re-derived to the checkpoint frontier (you already have the
   // logic to materialize at any frontier via replay — reuse it, don't duplicate).

   pub fn parse_document_binary<P, Operation>(pack: &[u8], protocol: &[u8]) -> Result<ParsedDocumentText<P, Operation>, TextError>
   where P: Clone + DocumentPack, Operation: OpText + crate::Operation<P>;
   // Fast path: protocol_materialize::resolve_plan + materialize_with(decode_base = P::decode_pack,
   // apply_edit = |p, history_edit| { parse each op line via Operation::parse_op, apply_operation }).
   // Full envelope reconstruction (changes/checkpoints/alternatives/active) still needs a full
   // HistoryReader::log() decode — that's fine, it's cheap (framed binary, not JSON/text parse).

   pub struct DocumentBinaryFiles { ... }  // already declared above
   ```
   LAW to hold (and to add a testkit-style assertion for in step 6):
   `parse_document_binary(&encode_pack(p), &print_document_binary(env)?.protocol)` produces the
   same projection/envelope as `parse_document_text(&print_dsl(p), &print_ops_log(env)?)`.

5. **`DocumentVcsStore` additions** (region `🔖️DocumentVcsStore`, near `set_state` ~1488):
   ```rust
   /// Binary sibling of set_state: adopts a fast-path-materialized state without fold_current's
   /// full O(history) replay — trusts a precomputed projection (e.g. from protocol_materialize).
   pub fn set_state_with_projection(&mut self, envelope: DocumentVcsEnvelope<P, Operation>,
       applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>, projection: P) {
       // identical body to set_state EXCEPT: self.current = projection; (skip self.fold_current())
   }
   pub fn from_binary(pack: &[u8], protocol: &[u8]) -> Result<Self, VcsError>
   where P: Clone + DocumentPack, Operation: OpText + crate::Operation<P> {
       // parse_document_binary(pack, protocol) -> Self::new(...) -> set_state_with_projection(...)
   }
   ```

6. **`CodecRegistry`** (region `🔖️CodecRegistry`, ~356+): update `DocumentCodec` to:
   ```rust
   pub struct DocumentCodec {
       pub schema: String,
       pub extension: &'static str,
       pub print: fn(&str) -> Result<(DocumentBinaryFiles, DocumentTextFiles), VcsError>,
       pub parse: fn(&[u8], &[u8]) -> Result<String, VcsError>,       // (pack, protocol) -> envelope_json
       pub parse_dsl: fn(&str, &str) -> Result<String, VcsError>,      // (dsl, ops) -> envelope_json, unchanged fallback
   }
   ```
   `DocumentCodec::of::<P, Operation>()` keeps its bounds (adjust to require `P: DocumentPack`,
   `Operation: OpText + Operation<P>` as it likely already does); `print`/`parse` implementations
   call `print_document_binary`/`print_document_text` and `parse_document_binary` respectively.
   `register_document_codec_for_app` (in `framework/plugin/rs`) is NOT your file to touch this
   wave — it stays recompile-only and Wave 2 verifies it still compiles.

7. **Storage backends**:
   - `FolderTextStorage`: add `read_binary(id, ext) -> Result<Option<DocumentBinaryFiles>, VcsError>`
     (reads `{id}.pack` + `{id}.{ext}.spr`, both required; missing `.spr` = empty log, matching
     today's missing-`.ops` semantics), `write_binary(id, ext, files: &DocumentBinaryFiles, mirror:
     &DocumentTextFiles)` (writes all 4 files: `.pack`, `.spr` authoritative + `.{ext}` dsl,
     `.{ext}.ops` text mirrors — keep both mirrors, human-legible diffs are why `.pack`/`.spr`
     files are policy-banned from commits), `append_binary(id, ext, edit_segment: &[u8], ops_lines:
     &str)` (appends the framed edit record to `.spr` AND the op-text lines to the `.ops` mirror —
     both O(new edit)). Drop the old mixed `.pack`+`.ops` path entirely.
   - `FolderSqliteStorage`: `document` table becomes `(id, schema, pack BLOB, protocol BLOB,
     updated_at)` — drop the `json` column. Dev-disposable DBs; no migration script (repo rule:
     no migration scripts, greenfield).
   - Leave `framework/sync`'s `FolderEndpoint` for Wave 2 (not your file).

8. **`test_support`** (region `🔖️TestSupport`, ~3324+) — add, matching the existing style:
   ```rust
   pub fn assert_document_protocol_round_trip<P, Operation>(store: &DocumentVcsStore<P, Operation>)
   where P: Clone + PartialEq + DocumentPack, Operation: OpText + crate::Operation<P> + Clone;
       // print_document_binary -> parse_document_binary; parsed.projection == store's live
       // projection; AND == the text-path (print_dsl/print_ops -> parse_document_text) projection.
   pub fn assert_ops_protocol_equivalence<P, Operation>(store: &DocumentVcsStore<P, Operation>)
       // full envelope equality: binary-path envelope == text-path envelope (edits/changes/
       // checkpoints/alternatives/active_alternative_id), not just the projection — this is the
       // op-log law itself.
   pub fn assert_materialize_fast_path_equals_replay<P, Operation>(store: &DocumentVcsStore<P, Operation>)
       // parse_document_binary (checkpoint-segment fast path) == materialize_document_projection
       // full replay, and applied_edit_ids agree.
   ```
   Prove the mechanism with vcs's own demo/test projection (whatever `vcs/rs/lib.rs`'s existing
   `#[cfg(test)]` module uses for `assert_document_pack_round_trip` today — call the three new
   asserts right beside it, exactly like the pack rollout's wave 1 did (`demo_dsl_pack_equivalence`
   pattern) — this is your proof the whole binary pipeline works end to end, and it's the one file
   that closes its own completeness-lint gap immediately rather than joining Wave 3's allowlist.

## What NOT to touch this wave

`framework/plugin/rs`, `framework/plugin/host/rs`, `framework/product/os/core/rs`, `framework/sync`,
`framework/wit/world.wit`, root `script.ts` policy lints, `.vscode/launch.json` — all Wave 2.
`framework/product/os/hub` — Wave 2b. Any app crate — Wave 3.

## Verification

`cargo test -p vcs` — all existing tests pass plus your new demo asserts. `cargo build -p vcs`
clean. If `framework/plugin/rs` (which depends on `vcs::DocumentPackFiles`) now fails to compile
because you deleted that type, that is EXPECTED and is Wave 2's job to fix — note it in your
report as "downstream break, Wave 2 will fix" rather than trying to fix it yourself (staying in
your critical-section lane keeps this reviewable). Do not run a full workspace build if it would
require touching files outside your lane to get green — a `cargo check -p vcs` clean pass is your
bar.

## Report back

Every file touched, the exact new/changed public API surface (for Wave 2's agent to consume),
confirmation the three new test_support asserts pass against vcs's own demo projection, and the
list of downstream crates you know will need Wave 2 fixes (from `cargo check --workspace
--exclude <your untouched crates>` or a grep for `DocumentPackFiles`/`print_document_pack`/
`parse_document_pack` usage outside `vcs/rs`).
