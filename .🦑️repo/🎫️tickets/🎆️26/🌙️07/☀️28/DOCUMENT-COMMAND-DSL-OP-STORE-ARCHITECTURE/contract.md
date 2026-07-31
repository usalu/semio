# Document + Command + Dsl + Op + Store Architecture — Gap Register & Wave Contract

Ticket: `26/07/28/DOCUMENT-COMMAND-DSL-OP-STORE-ARCHITECTURE` · issue #2372 · goal `r2602/runningsketchpad`

This is the binding contract for the agent workforce closing the remaining gaps between the
repo's current state and the target app architecture. It continues (never forks) the frozen
contracts of `PROTOCOL-BINARY-OP-LOG-LAYER`, `INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING`,
and `EXTRACT-STORE-INTO-ITS-OWN-TECHNOLOGY` (all under `.repo/🎫️/26/07/27..28/`). Read those first.

## Target architecture (authoritative, user-stated)

Every app defines:
- **document** — typed entities, a diff, and two-way conversion document ↔ pack (binary).
- **commands** — a **binary protocol** used for BOTH communication and storage, with two-way
  conversions along the chain: op text ↔ typed command/operation ↔ binary protocol.
- **dsl** — maximum token-efficient, consistent textual representation of a document.
- **op** — maximum token-efficient, consistent textual representation of a command; operations
  yield diffs; the inverse of an operation is expressed as calls to other operations.

Apps use **store** for local-first in-memory state management with an **optional, hot-swappable
backbone** (all IO behind a non-blocking queue, actors in `store_sync`/`store_worker`).

## Current state (verified 2026-07-28, five parallel read-only sweeps)

Already DONE and law-covered — do not rebuild, only extend:
- Typed per-app documents: all 39 `DocumentApp` impls have typed `Projection` with
  `#[derive(dsl::DslDocument)]` entities ([framework/plugin/rs/lib.rs:3141](../../../../../framework/plugin/rs/lib.rs)).
- Diffs + inverses: `protocol::OperationDiff<P>`/`Operation<P>::diff/backwards`
  ([protocol/command/rs/lib.rs:15,29](../../../../../protocol/command/rs/lib.rs)).
- document ↔ pack: `store::DocumentPack` + `pack_rt` + `DocumentCodec` registry, law
  `decode_pack(encode_pack(p)) == parse_dsl(print_dsl(p))` ([store/rs/lib.rs:185-257](../../../../../store/rs/lib.rs)).
- dsl: one grammar engine (`dsl_schema` `RecordSpec`/`RecordValue`), Document/Inline join modes
  ("newline law"), five derives, canonicalize fixpoint.
- op text: `protocol::OpText` via `#[derive(dsl::DslOps)]`; `.ops` header grammar
  (`doc/edit/change/checkpoint/alternative/active`) itself derived via `DslOps`
  ([store/rs/lib.rs:609](../../../../../store/rs/lib.rs)).
- ops text ↔ binary op-log: `protocol::compile_ops/decompile_ops` over `.spr`
  ([protocol/rs/lib.rs:33,38](../../../../../protocol/rs/lib.rs)).
- store: `DocumentStore<P,Op>` with O(1) projection + tail-undo caches, hot-swappable
  `attach_backbone/detach_backbone`, `MemoryBackbone`/`ChannelBackbone`/`PortBackbone`,
  actor IO in `store_sync`, browser worker in `store_worker`.

## Gap register

Grouped by target pillar; each gap names its anchor. Severity: 🔴️ blocks the target,
🟠️ architectural debt, 🟡️ hygiene.

### Commands as a binary protocol (the core missing pillar)

- **G1 🔴️ Op payloads are never binary.** `protocol_history::OpPayload { text, binary }` —
  `binary` is a reserved seam, always `None` (protocol/history/rs/lib.rs:47,610). The durable
  `.spr` log stores op *text lines* inside binary framing.
- **G2 🔴️ Causal/wire command payloads are JSON.** `protocol_causal::OperationEnvelope` carries
  `DocumentDiff`/`InverseOperation` as `serde_json::Value` (protocol/causal/rs/lib.rs:22-41);
  `protocol_wire` frames are binary-framed but `serde_json` bodies (protocol/wire/rs/lib.rs:200).
- **G3 🔴️ `DocumentCommand` has no text or binary form.** JSON-only `dispatch_json`
  (store/rs/lib.rs:64,1614). Structural commands (undo/redo/checkpoint/alternative/checkout)
  have no op-line and no binary twin — the "op = textual representation of a command" pillar
  covers only operations today.
- **G4 🔴️ Backward ops are never persisted.** `HistoryEdit` has no backwards slot;
  `EncodeOptions::write_backwards_section` is a no-op (protocol/history/rs/lib.rs:445-448,843).
  Undo dies across save/load. Related: undo/redo cursor + checkout state are runtime-only
  (store/rs/lib.rs:758-762 replay marks every edit applied).
- **G5 🟠️ `compile_ops`/`decompile_ops` unwired.** `DocumentCodec` persists pack + ops *text*;
  neither `FolderTextStorage`/`FolderSqliteStorage` nor hub sync ever writes/reads the `.spr`
  binary log (store/sync/rs/lib.rs:1585-1698).
- **G6 🟠️ Semantic surface declared but dead.** `UndoPolicy`/`ConflictRule`/`StateClass` carried,
  never dispatched on; `CommandOutcome`, `OperationDescriptor` registry, `OperationUpcaster`,
  `OperationTransform`, `Signer/SignatureVerifier`, `AnchorId` have zero consumers/impls.
- **G7 🟠️ 3/5 `MergeStrategyKind`s are behaviorally identical** (`chronological_compose`)
  (protocol/crdt/rs/lib.rs:84-116).

### Op (inverse & fleet conformance)

- **G8 🔴️ Snapshot inverses.** Several apps invert fine-grained ops with whole-document ops
  (e.g. note `SetDocument`-style backwards, note/plugin/rs/lib.rs:328-343) instead of composed
  calls to other operations — violates the inverse pillar and bloats the op log.
- **G9 🟠️ `DslOps` emission couples `OpText` to store.** Generated `parse_op` error type is
  `::store::TextError` though the trait wants `dsl_core::TextError` (dsl/derive/rs/lib.rs:737).
- **G10 🟠️ `Operation::author_id()` defaults to local everywhere** — cross-actor undo-policy
  classification is a fleet-wide no-op (WS-F wave-1 feedback in ticket 26/07/12).

### Store & backbone

- **G11 🔴️ No native URI→backbone resolution.** `resolve_backbone`/`attach_backbone_uri` are
  wasm32-only (store/rs/lib.rs:1645,2268); docs still advertise `temp://file://folder://remote://`
  on native. Native callers hand-construct backbones.
- **G12 🟠️ Hub sync deferrals**: `pack_schema_hash` sent as zeros unvalidated
  (store/sync/rs/lib.rs:913,1249), snapshot-to-hub unimplemented (:811), `Welcome` pack payloads
  ignored (:969,1336), no congestion control (:1002).
- **G13 🟡️ `store` wasm cdylib exports nothing** (zero `#[wasm_bindgen]` items; pkg exports only
  `memory`); wasm-only deps unused. Either export the JSON dispatch surface or drop the cdylib.
- **G14 🟡️ `store/sync` has no nx/bun wiring** (no project.json/package.json/script.ts).

### End-to-end command flow

- **G15 🔴️ `handle_command` missing on the JS plugin bridge** — arg-carrying Plugin/App/Mode
  commands render but cannot execute in the browser (framework/renderer/wgpu/rs/lib.rs:21928;
  wasmtime side exists at framework/plugin/host/rs/lib.rs:418).
- **G16 🔴️ Two parallel kernels.** `framework/core` twins of `protocol` types
  (`OperationEnvelope`, `OpDag`, `HybridLogicalTimestamp`, `UndoPolicy`, `DocumentDiff` with
  `serde_json::Value` payload); `store_sync` WireBridge exists only to convert between them
  (store/sync/rs/lib.rs:211-322).
- **G17 🔴️ Legacy JSON-patch action kernel in the OS host** — `invoke_action`/`commit_action_result`
  mutate `instance.document_json` with `JSON_PATCH_SCHEMA_ID`, bypassing `DocumentStore`
  (framework/product/os/core/rs/lib.rs:188-202).
- **G18 🟠️ OS fixture registry still JSON** — `register_os_fixture_json` falls back to `json!({})`
  for DSL fixtures (s/plugin/rs/lib.rs:20-29).
- **G19 🟠️ compose kit bypasses typed packs** — `KitSnapshot` rides the schema-less
  `impl DocumentPack for serde_json::Value` bridge (compose/client/lib/rs/lib.rs:7850-7977).

### Pack / protocol hygiene

- **G20 🟠️ `pack_index` built but unreachable** (`field_index_span` hardcoded zero,
  pack/value/rs/lib.rs:1277); `Manifest.schema_name` always empty (:1272);
  `DecodeReport.unknown_segments` never populated (:1307); footer chaining/`REQUIRED_CHUNKED`/
  `KIND_SCHEMA`/`KIND_SNAPSHOT` inert.
- **G21 🟠️ protocol facade re-export gaps** force `protocol_testkit` path-dep escapes and
  hand-duplicated header/commit parsing + non-blake3 diff in `protocol_cli`
  (protocol/cli/rs/lib.rs:16-92,185).
- **G22 🟠️ history/io hot paths O(file)** — `HistoryFile::open_append` full replay,
  `TailFollower::poll` full re-decode (protocol/io/rs/lib.rs:93-115,224-232); compaction drops
  `REC_PROJECTION/INDEX/SEALED/EPHEMERAL` silently; `REC_ACTOR_DICT` never emitted.
- **G23 🟡️ Dead DSL surface** — `dsl_token`+`dsl_editor` orphans, `#[dsl(flatten)]` dead flag,
  stale facade description, context-insensitive completions.
- **G24 🟡️ Doc/marker hygiene** — duplicated `//#region` markers throughout store/rs/lib.rs and
  one mismatched endregion (:4739/:4833); 69 stale `framework/sync` path references;
  `pack_cli` schema registry holds 2 specs (TODO(wave2) at pack/cli/rs/lib.rs:44);
  `protocol_cli` lib+bin double-target warning; `ReconcileReport→StudioConflict` severity loss.

## Mechanisms (design decisions — binding)

### M-A Op binary codec (`OpBinary`) — closes G1, feeds G2/G5
The op enum already lowers to `(variant keyword, RecordSpec, RecordValue)` via `dsl::DslVariants`;
the DSL line and the binary form must be two encodings of that same pair (mirror of the
`DocumentDsl`/`DocumentPack` law).
- `pack_value` exposes a **container-less record-body codec**: `encode_record_body(spec, value,
  opts) -> Vec<u8>` / `decode_record_body(spec, bytes, opts)` (the existing tag codec minus
  header/segments/manifest/footer; canonical rules unchanged).
- `protocol_command` gains `trait OpBinary: Sized { fn encode_op(&self) -> Result<Vec<u8>, ProtocolError>;
  fn decode_op(bytes: &[u8]) -> Result<Self, ProtocolError>; }` beside `OpText`. Layout:
  `format u8 (=1) | variant ordinal varint | record body`.
- The runtime lives in `dsl::op_rt` (NOT `store` — the bound is `dsl::DslVariants` itself, and a
  store-hosted twin is a distinct trait instance inside dsl's own test build; hit in practice).
  `dsl` gains regular `pack` + `protocol` deps (no cycle — neither depends on the dsl facade);
  `store` re-exports it as `store::op_rt` for the one-facade rule.
- `dsl_derive::DslOps` additionally emits `impl ::protocol::OpBinary` via `::dsl::op_rt` —
  additive, every `#[derive(DslOps)]` crate gains the impl on rebuild.
- LAW (added to `store::test_support` + `dsl::test_support`):
  `decode_op(encode_op(op)) == op == parse_op(print_op(op))`, asserted per app by the existing
  per-crate law tests.

### M-B Command text + binary (`DocumentCommand` joins the op grammar) — closes G3
- `store::DocumentCommand<Op>` becomes a `DslOps`-shaped grammar: structural variants print as
  op lines (`undo`, `redo`, `commit-checkpoint message="..." by=[...]`, `create-alternative name=…`,
  `switch-alternative id`, `checkout id`, `amend key=…`), `Apply` embeds the ops' own
  `OpText`/`OpBinary` payloads. Binary: `format u8 | command tag u8 | body`.
- `dispatch_text(line)` and `dispatch_binary(bytes)` replace ad-hoc JSON entry points;
  `dispatch_json` is deleted (greenfield rule: no legacy). All wasm-bindgen `dispatch_json`
  surfaces (trinity, playbook, imperative, fem, puzzle engines, OsStore) flip to text or binary.

### M-C Binary end-to-end (communication + storage) — closes G2, G5, G12
- `protocol_causal::OperationEnvelope` payloads become opaque **bytes** (`Vec<u8>`, the M-A
  encoding) + a real `SchemaId`; `DocumentDiff`/`InverseOperation` JSON bodies are deleted.
- `protocol_wire` frame bodies flip from `serde_json` to the same binary record codec
  (`protocol_wire` may depend on `protocol_command` codecs; wire fixtures under
  `store/sync/fixtures/wire/*.bin` and the TS twin in
  `framework/product/os/core/js/index.ts:372-448` are regenerated byte-for-byte together).
- Storage: `FolderTextStorage` keeps `.<ext>` dsl + `.ops` text as the human mirror; the
  authoritative artifacts become `.pack` (projection) + `.spr` (command log via
  `compile_ops`, now with binary payload records); `FolderSqliteStorage` stores pack + spr blobs,
  not JSON. Hub `Hello.pack_schema_hash` = `pack_value::schema_hash(spec)`, validated server-side.
- The WIT backbone seam stays JSON by design (sandbox debuggability) — unchanged.

### M-D Persist undo: backwards section + cursor — closes G4
- Implement `write_backwards_section` (presence bit5 already reserved) in `protocol_history`
  encode/decode; `HistoryEdit` gains `backwards: Vec<OpPayload>` + meta.
- `.ops` grammar + binary log gain a `cursor` header line (`cursor applied=<edit-id>
  redo=[...] checkpoint=<id>`) so undo position, checkout and active alternative survive
  reload; `replay_ops` honors it.

### M-E Kernel unification — closes G16, G17, G18, G15
- Delete `framework/core` twins (`OperationEnvelope`, `OpDag`, `HybridLogicalTimestamp`,
  `UndoPolicy`, `DocumentDiff`); repoint to `protocol` types; delete `store_sync::WireBridge`.
- Delete the OS host JSON-patch kernel (`invoke_action`/`commit_action_result`/
  `JSON_PATCH_SCHEMA_ID`); every OS mutation goes through `DocumentStore` commands.
- OS fixtures load through `DocumentCodec.parse_dsl` (delete `register_os_fixture_json` fallback).
- Wire `handle-command` on the JS plugin bridge (mirror the wasmtime path at
  framework/plugin/host/rs/lib.rs:418 into the generated host-shim/plugin-worker templates).

### M-F Store/backbone completion — closes G11, G13, G14
- `store` gains a `BackboneFactory` registry (`register_backbone_factory(scheme, factory)`);
  `store` stays IO-free — `store_sync` registers `temp/file/folder/remote` factories at init on
  native, the existing host-relay factories on wasm; `resolve_backbone`/`attach_backbone_uri`
  become target-independent.
- `store/sync` gets package.json/project.json/script.ts (nx `test` tiers, same shape as store/rs).
- `store` cdylib: export the M-B `dispatch_text`/`dispatch_binary` + envelope pack/dsl surface via
  wasm-bindgen, or (if os/dev never loads it) drop the cdylib crate-type + wasm deps. Decide by
  checking os/dev loaders; default = export.

### M-G Fleet conformance sweep — closes G8, G9, G10, G19
- Inverse law: `backwards` must be composed of the op vocabulary (no whole-document snapshot
  inverses except semantically-whole ops like import/set-document). New test-support assert:
  apply(forwards); apply(backwards) == base, AND backwards ops ∈ vocabulary ≠ SetDocument-class
  (marker trait or per-app allowlist).
- `DslOps` emission: `parse_op` error type flips to `::dsl_core::TextError`
  (re-exported unchanged by store — pure re-target, one wave with a fleet rebuild).
- Per-app `author_id` threading: `VcsDocumentApp` stamps the studio member's actor id into ops'
  `OperationMeta` (framework-level, not per-op overrides).
- compose `KitSnapshot` gets a real derived spec (`DslDocument`) replacing the JSON bridge.
- Conformance checklist per app (all 39): typed entities ✓️, diff ✓️, composed inverses,
  OpText ✓️, OpBinary (new), DocumentDsl ✓️, DocumentPack ✓️, law tests present, no `serde_json`
  in op/document payload paths. Tracked as a table in this folder (`conformance.md`).

### M-H Protocol/pack hygiene — closes G6, G7, G20, G21, G22, G23, G24
Facade re-export completion; cli dedup (blake3 diff, shared header/commit parsers);
`HistoryAppender` mid-stream resume + seeded `TailFollower`; compaction honors
`drop_ephemeral`/`keep_snapshots`; `REC_ACTOR_DICT` emission; UndoPolicy dispatch in store
undo path; wire `pack_index` into `encode_document`/`decode_document` + `Manifest.schema_name`;
real merge combinators for `OrderedSequence`/`TextSequence`/`TombstonedGraphSet` (or collapse
the enum — decide in-wave with a written rationale); delete dead DSL surface (`flatten`,
orphan crates get consumers or deletion — `dsl_editor`/`dsl_token` fold into the future text
editor app or are removed); marker/path hygiene sweep.

## Wave plan (workforce execution)

Waves are dependency-ordered; parallel agents within a wave own disjoint crates. Every wave:
re-read shared files before editing (live concurrent devs), no git-mutating commands, single-file
lib.rs regions, extend existing tests only, scratch files as `.txt` in THIS ticket folder,
explicit `path` on every ticket_close/reopen. AGENTS.md is never edited by agents — collect
needed AGENTS.md changes in `human-todos.txt` here.

| Wave | Content | Mechanism | Mode |
|---|---|---|---|
| W0 | This contract (done) + conformance.md skeleton | — | 1 agent |
| W1 | `pack_value` record-body codec + `protocol_command::OpBinary` + `store::op_rt` + test-support laws | M-A | 1 agent |
| W2 | `dsl_derive` OpBinary emission + `TextError` re-target + fleet rebuild green | M-A, M-G | 1 agent, serial (atomic derive flip) |
| W3 | `DocumentCommand` text+binary grammar, `dispatch_text/binary`, delete `dispatch_json`, flip engine/OsStore surfaces | M-B | 1-2 agents |
| W4 | History backwards section + cursor header; storage flip (spr+pack authoritative, sqlite blobs); `compile_ops` wiring | M-C, M-D | 2 agents (history / storage) |
| W5 | Causal envelope + wire binary flip + TS twin + fixtures regen + hub schema-hash validation | M-C | 1 agent (wire fixtures are byte-coupled) |
| W6 | Kernel unification: framework/core twin deletion, WireBridge deletion, OS JSON-patch kernel deletion, OS fixture DSL, JS `handle-command` bridge | M-E | 2-3 agents (core / os / bridge) |
| W7 | Backbone factory registry + store/sync nx wiring + store wasm surface decision | M-F | 1 agent |
| W8 | Fleet conformance fan-out over all 39 apps (composed inverses, laws, author threading, compose KitSnapshot spec) | M-G | parallel, disjoint app crates |
| W9 | Protocol/pack hygiene batch | M-H | parallel, disjoint crates |
| W10 | Verification: `cargo check/test --workspace` (exclude `mathematical` pre-existing hangs), clippy gate, wasm builds (store, store-worker, plugin registry zero-diff), os/core vitest incl. wire byte-identity canary, runtime proof via launch.json os/dev with `[DEBUG]` logs (removed after) | — | 1-2 agents |

Cross-wave laws (assert in test-support, cite in every wave report):
1. `parse_dsl(print_dsl(d)) == d == decode_pack(encode_pack(d))`
2. `parse_op(print_op(op)) == op == decode_op(encode_op(op))`
3. `parse_ops_text(print_ops_text(log)) == log` and `decompile_ops(compile_ops(text)) == text`
4. apply(forwards) then apply(backwards) == base; backwards composed from op vocabulary
5. live projection == replay (`assert_live_equals_replay`)
6. wire/text fixtures byte-identical between Rust and TS twins

## Risks

- **Concurrent sessions**: `dsl/derive/rs/lib.rs`, `dsl/rs/*` are mid-edit by another live session
  right now; W2 must re-read immediately before editing and re-verify emission line numbers.
- **Atomic flips**: W2 (derive emission) and W5 (wire bodies + TS twin + fixtures) are
  un-splittable; land each in one sitting.
- **Fixture regeneration**: wire `.bin` fixtures and `.ops`/`.pack` app fixtures must be
  regenerated by code, never hand-edited; the byte-identity canary
  (framework/product/os/core/js/backbone-worker.ts:~731) is the gate.
- **cdylib wasm linker**: keep `[profile.release.package.store] codegen-units = 16` twin if the
  store cdylib stays.
- **No legacy**: delete replaced surfaces outright (dispatch_json, WireBridge, JSON-patch kernel,
  framework/core twins) — no shims beyond a single wave's interior.
