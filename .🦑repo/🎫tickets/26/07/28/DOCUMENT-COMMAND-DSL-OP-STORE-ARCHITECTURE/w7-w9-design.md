# W7 / W8 / W9 Execution Plans — Backbone completion, fleet conformance, hygiene

(Full design produced by a Plan agent against contract.md M-F/M-G/M-H, read-only verified
against the working tree 2026-07-28. See /Users/ueli/.claude/plans/steady-cuddling-dijkstra.md
for the approved summary; this file is the full detail.)

Decisions confirmed by the user before implementation: **W7 drops the store cdylib** (nothing
loads `@semio-tech/store-rs`; store_worker is the real store-family wasm artifact). **W9-D
deletes the orphan crates `dsl_token` + `dsl_editor`** (zero consumers; git preserves history for
a future editor app).

---

## W7 — Backbone completion + store cdylib decision (M-F, closes G11, G13; G14 already done)

### Verified current state

- `resolve_backbone` is wasm32-only at `store/rs/lib.rs:2275-2278` — wraps every URI in
  `PortBackbone::new(uri)`.
- `attach_backbone_uri` is wasm32-only at `store/rs/lib.rs:1652-1655`.
- Pattern to copy: `DOCUMENT_CODEC_REGISTRY` at `store/rs/lib.rs:318-335` — `OnceLock<RwLock<
  HashMap<String, DocumentCodec>>>` + `register_document_codec`/`document_codec`.
- Native hand-construction reference: `framework/renderer/wgpu/rs/lib.rs:17733-17754`
  (`parse_persistence_binding`) + `:17894-17914` (DocumentHost::open → register_host_backbone →
  WIT attach), and `framework/plugin/host/rs/lib.rs:72-79` (`backbone_for` explicit-registration
  map, with a stale "resolve_backbone is wasm-only now" comment at :74).
- `store_sync` actor machinery: `DocumentHost::open` at `store/sync/rs/lib.rs:448-466` returns
  `DocumentChannels { cmd_tx, channel_backbone }`; `PersistenceBinding::{Folder, Hub}` at :38-50.
- `DocumentStore` knows its own context: `envelope.id`, `envelope.schema` (`store/rs/lib.rs:53`),
  `local_actor_id` (`:1078`).

### Cdylib evidence (decision confirmed)

- `store/rs/Cargo.toml:15` — `crate-type = ["rlib", "cdylib"]`; wasm-only deps at :31-36
  (`js-sys`, `serde-wasm-bindgen`, `wasm-bindgen`, `wasm-bindgen-futures`, `web-sys`).
- Zero `#[wasm_bindgen]` items in `store/rs/lib.rs`.
- Nothing loads `@semio-tech/store-rs` anywhere in the repo. `store_worker` has real exports
  (`store/worker/rs/lib.rs:41-111`).

### Ordered edit sequence

**Step 1 — `store/rs/lib.rs`, `🔖Backbone` region (after `BackboneChannelPort`, ~:2104): add the
factory registry.**

```rust
pub struct BackboneRequest<'a> {
    pub uri: &'a str,
    pub scheme: &'a str,
    pub document_id: &'a str,
    pub schema: &'a str,
    pub actor: Option<&'a str>,
}

pub type BackboneFactory =
    std::sync::Arc<dyn Fn(&BackboneRequest) -> Result<Box<dyn Backbone>, VcsError> + Send + Sync>;

static BACKBONE_FACTORY_REGISTRY:
    std::sync::OnceLock<std::sync::RwLock<HashMap<String, BackboneFactory>>> = std::sync::OnceLock::new();

pub fn register_backbone_factory(scheme: impl Into<String>, factory: BackboneFactory);
pub fn backbone_factory(scheme: &str) -> Option<BackboneFactory>;
```

Mirror the poisoned-lock recovery style of `document_codec_registry()`.

**Step 2 — make `resolve_backbone` target-independent.**

```rust
pub fn resolve_backbone(request: &BackboneRequest) -> Result<Box<dyn Backbone>, VcsError> {
    if let Some(factory) = backbone_factory(request.scheme) { return factory(request); }
    #[cfg(target_arch = "wasm32")]
    { return Ok(Box::new(PortBackbone::new(request.uri))); }
    #[cfg(not(target_arch = "wasm32"))]
    { Err(VcsError::Backbone(format!("no backbone factory registered for scheme {:?}", request.scheme))) }
}
```

**Step 3 — de-cfg `attach_backbone_uri` (:1652-1655)** to build a `BackboneRequest` from
`self.envelope.id`/`schema`/`local_actor_id` and call `resolve_backbone`.

**Step 4 — `store/sync/rs/lib.rs`: native factories (new `🔖Factories` region after `🔖Host`,
~:517).** `register_native_backbone_factories()` (Once-guarded): `temp` → `MemoryBackbone::new`;
`folder`/`file` → Folder-binding actor via shared `DocumentHost`; `remote` → Hub-binding actor.
Lift `persistence_bindings_for_uri` out of the wgpu shell (delete `parse_persistence_binding`
:17733, repoint :17896).

**Step 5 — callers**: `framework/plugin/host/rs/lib.rs:74` comment reword;
`framework/product/os/core/rs/lib.rs:1167-1179` collapse wasm/native attach split;
`framework/renderer/wgpu/rs/lib.rs:17896` repoint.

**Step 6 — cdylib removal (confirmed).**
- `store/rs/Cargo.toml:15` → `crate-type = ["rlib"]`; delete `[package.metadata.wasm-pack.
  profile.release]`; prune wasm deps to only `js-sys`/`web-sys` (still used under cfg(wasm32));
  delete `serde-wasm-bindgen`, `wasm-bindgen`, `wasm-bindgen-futures`.
- Root `Cargo.toml`: delete `[profile.release.package.store] codegen-units = 16`.
- `store/rs/project.json`: delete the `wasm` target; `store/rs/script.ts`: delete the wasm
  branch; `store/rs/package.json`: delete; root `package.json`: remove `"store/rs"` from
  workspaces.
- `.vscode/launch.json`: remove/adjust `📦build🗃️store` entry (currently
  `bun nx run @semio-tech/store-rs:wasm`).
- Note in report: `store_worker` remains the only store-family wasm artifact.

**Step 7 — tests**: registry round-trip (`test://` factory), unknown-scheme error on native;
store_sync: `register_native_backbone_factories()` then `attach_backbone_uri("folder:///
<tempdir>")` end-to-end persistence via a second store opening the same folder. Verify:
`cargo test -p store -p store_sync`, `cargo check --workspace --all-targets`,
`bun nx run @semio-tech/store-worker:wasm` still green.

---

## W8 — Fleet conformance fan-out (M-G, closes G8, G10, G19; G9 already closed in W2)

### Verified current state

- Law assert: `store::test_support::assert_op_text_binary_equivalence` at `store/rs/lib.rs:
  2927-2937`. 48 crates currently call `assert_op_line_round_trip` — mechanical swap.
- Snapshot-inverse violations sampled: `note/plugin/rs/lib.rs:328-343` (`PutAsset → SetDocument`
  is the true violation); `draw/rs/lib.rs:1754-1756` (worst case: every op inverts as
  `SetDocument`); `norm/core/rs/lib.rs:526-528` (generic blanket, one fix covers 16 crates);
  `writer/rs/lib.rs:122-128` (field-level inverses fine, only SetDocument self-inverse exempt);
  `sourcing/curate/rs/lib.rs:322-327` (vocabulary is only SetDocument — exempt, semantically
  whole); `infinite/board/.../dag/rs/lib.rs:7130-7138` (collection ops composed, whole-field
  exempt). Already-composed models: `shooting/rs/lib.rs:639-659`, `gis/plugin/rs/lib.rs:163-170`.
- Author threading: `VcsDocumentApp::dispatch_emit` already calls
  `self.store.set_local_actor_id(Some(meta.actor.clone()))` at `framework/plugin/rs/lib.rs:3718`
  — but the store ignores it: `replay_operations` (`store/rs/lib.rs:1590-1615`) builds
  `OperationMeta.author_id = operation.author_id().unwrap_or(ActorId("local"))` (:1605), and
  dispatch then *clobbers* `local_actor_id` from that meta (:1560-1561 via `edit_actor_from_meta`
  :1103-1105).
- Compose: `KitSnapshot(pub Value)` JSON bridge at `compose/client/lib/rs/lib.rs:7822-7862`; live
  `Kit` is an `Arc/RwLock` graph (`:5657-5698`), not a serde struct; `CanonicalKitDiff` covers
  only 9 scalar metadata fields (`wire_json_to_canonical_kit_diff` :7951-7971). Kit schema:
  `compose/client/schema/compose/schema.yaml`, 993 lines, ~42 entity classes.

### W8.0 — serial framework pre-step (1 agent, must land before the fan-out)

1. **Store author threading** (`store/rs/lib.rs`): `replay_operations` (:1590) gains
   `default_author: Option<&str>` param; at :1605 build `author_id: operation.author_id()
   .or_else(|| default_author.map(|a| ActorId(a.into()))).or(Some(ActorId("local".into())))`.
   Both call sites (:1543-1544 AmendLast, :1558-1559 Apply) pass
   `self.local_actor_id.as_deref()`. Fix the clobber at :1560-1561: derive `actor` from meta OR
   fall back to `self.local_actor_id.clone()`, and **stop overwriting** `self.local_actor_id`
   from derived meta (delete :1561) — `set_local_actor_id` becomes the single writer.
2. **New test-support assert** (`store/rs/lib.rs` `🔖TestSupport`, after `assert_operation_round_trip`
   :2827-2839):
   ```rust
   pub fn assert_composed_inverse<P, Op>(pre: &P, operation: Op, is_snapshot_class: impl Fn(&Op) -> bool)
   where P: Clone + PartialEq + std::fmt::Debug, Op: crate::Operation<P> + std::fmt::Debug
   {
       let exempt = is_snapshot_class(&operation);
       let backwards = operation.backwards(pre);
       if !exempt {
           for b in &backwards { assert!(!is_snapshot_class(b),
               "composed-inverse law: fine-grained op inverted with snapshot-class op: {b:?}"); }
       }
       assert_operation_round_trip(pre, operation);
   }
   ```
3. **VcsDocumentApp**: add a framework/plugin test asserting a dispatched op's
   `OperationMeta.author_id == meta.actor` and `Edit.actor == meta.actor`.
4. Update `conformance.md` legend: `Auth ✓` = "OperationMeta.author_id stamped from
   ActionMeta.actor via store.local_actor_id; verified by test".
5. Verify `cargo test -p store -p vcs -p semio-framework-plugin` + `cargo check --workspace`.

### Per-app work recipe (identical for every batch agent)

- **(a) OpB laws**: swap every `assert_op_line_round_trip` → `assert_op_text_binary_equivalence`;
  add missing variants so every enum variant appears once.
- **(b) Composed inverses**: audit `fn backwards`; classify each variant composed /
  field-whole-exempt / violation; fix violations by inverting into existing vocabulary (pattern:
  `protocol::invert_collection_operation`) or adding the minimal missing inverse variant (e.g.
  note: add `RemoveAsset { key }`). Add `assert_composed_inverse` calls per variant with the
  app's snapshot-class closure.
- **(c) Auth**: nothing per-app (framework-level, W8.0).
- **(d)**: fill the crate's row in `conformance.md` (re-read the file immediately before editing
  — shared file, concurrent agents).

### Batching (7 parallel agents, disjoint crate ownership)

| Batch | Crates | Weight driver |
|---|---|---|
| B1 | note/plugin, draw/plugin (+draw/rs), writer/plugin (+writer/rs), raster/plugin, sourcing/plugin (+sourcing/curate) | heavy inverse redesign |
| B2 | norm/plugin (+norm/core, all 16 norm family crates) | one generic fix + mechanical swaps |
| B3 | gis/plugin, infinite/board/port/directed/dag/plugin (+rs), layout/plugin (+layout/rs), reasoning/mindmap/plugin (+rs), remodel/plugin (+remodel/rs), architect/plugin (+architect/program) | mostly ✓-verification + law swaps |
| B4 | cad/plugin (+cad/rs), fem/plugin (+fem/2d, fem/3d), flow/plugin (+flow/core), forms/plugin, sequence/plugin (+core), process/plugin (+process/3d) | law swaps + targeted inverse audits |
| B5 | puzzle/plugin (+2d/3d/5d), trinity/plugin (+ram, rewrite/engine), playbook/plugin (+module/procedural, playbook/rs), imperative/plugin (+core), animate/plugin (+animate/present), mathematical/plugin | engine-style crates; mathematical under existing test-hang exclusion |
| B6 | shooting/plugin (+shooting/rs), vcs/plugin, s/plugin (+s/rs), framework/plugin row | already-migrated references |
| B7 | compose/client/lib (KitSnapshot) | see decision below |

### B7 — KitSnapshot decision (G19)

**Phased sub-schema, decoupled from the W8 gate.** Phase A (this wave): introduce
`KitSnapshotDoc` with `#[derive(dsl::DslDocument, Serialize, Deserialize, ...)]` covering kit
metadata + the flat collections (authors, files, folders, tags, qualities, props, attributes,
stats, concepts) — exactly the surface `CanonicalKitDiff`/`apply_diff` actually replays today —
plus a `typologies` subtree carried as a typed but coarse `TypologyNode { id, name, types:
Vec<TypeNode>, designs: Vec<DesignNode> }`. Convert `KitSnapshot(Value) ↔ KitSnapshotDoc` at the
`kit_backbone` boundary; flip `DocumentDsl`/`DocumentPack` impls (:7835, :7854) to the derived
ones; law tests via `assert_dsl_pack_equivalence` on a representative kit fixture.

Bailout criterion (write into the wave report): if `initial_kit_projection_value`'s JSON shape
proves unstable against round-trip, stop at metadata+flat collections and file the subtree as an
explicit follow-up ticket rather than shipping a lossy pack.

### Verification per batch
`cargo test -p <each owned crate>`; final W8 gate: `cargo check --workspace --all-targets` +
`cargo test -p store -p framework-plugin` and the filled conformance.md.

---

## W9 — Protocol/pack/DSL hygiene (M-H, closes G6, G7, G20, G21, G22, G23, G24 subset)

4 disjoint-crate parallel agents.

### W9-A — Facade re-export completion + protocol_cli dedup (G21)

1. `protocol/rs/lib.rs` `🔖Reexports` (:10-29), add: from `protocol_core`: `REC_END, REC_DOC,
   REC_ACTOR_DICT, REC_STR_DICT, REC_EDIT, REC_CHANGE, REC_CHECKPOINT, REC_ALTERNATIVE,
   REC_ACTIVE, REC_FRONTIER, REC_PROJECTION, REC_INDEX, REC_COMMIT, REC_SIGNATURE,
   REC_REDACTION, REC_UPCAST, REC_EPHEMERAL, REC_SEALED, REC_COMPACTION, REC_PADDING`,
   `DictBuilder`, `DictReader`; from `protocol_format`: `HEADER_SIZE, MAGIC, COMMIT_FRAME_LEN,
   Blake3Hasher, CommitPayload, parse_commit_payload, read_header, recover`; from
   `protocol_history`: `encode_history, decode_history, parse_ops_text, print_ops_text,
   IndexBuilder, IndexReader`.
2. `protocol/cli/rs/lib.rs`: delete hand-mirrors (`HEADER_SIZE`/`MAGIC` :21,:25, REC_* table
   :31-50, `CommitFields`/`parse_commit_fields` :83-104) → use `protocol::{...}`.
3. Blake3 diff (:185-207): replace `DefaultHasher` fingerprint with `protocol::Blake3Hasher`;
   rename `fp=` → `hash=` (:629).
4. Double-target fix: `protocol/cli/rs/Cargo.toml:11-15` points `[lib]` and `[[bin]] name =
   "protocol"` at the same `lib.rs`. Create `protocol/cli/rs/bin.rs` with a thin `fn main()`
   calling `protocol_cli::main_impl`, point `[[bin]] path = "bin.rs"`, delete `#[cfg(not(test))]
   fn main` from `lib.rs:809-813`.
5. `protocol/testkit/rs/Cargo.toml`: delete the `protocol_core`/`protocol_format`/
   `protocol_history` path-dep escapes; repoint `lib.rs` uses to `protocol::…`.
6. Verify: `cargo test -p protocol -p protocol_cli -p protocol_testkit`; `cargo check
   --workspace`.

### W9-B — io hot paths, compaction, REC_ACTOR_DICT (G22)

1. **`SprWriter::resume`** (`protocol/format/rs/lib.rs`): `pub fn resume(sink: S, state:
   ResumeSeed) -> Result<Self, ProtocolError>` where `ResumeSeed { last_commit_seq: u64,
   chain_hash: [u8; 32] }`.
2. **Dictionary seeding** (`protocol/core/rs/lib.rs`, `DictBuilder`): `pub fn from_entries(entries:
   Vec<String>) -> Self`.
3. **`HistoryAppender::resume`** (`protocol/history/rs/lib.rs`): `pub fn resume(sink: S, seed:
   AppenderSeed) -> Result<Self, ProtocolError>` + `pub fn appender_seed_from(bytes: &[u8],
   options: &DecodeOptions) -> Result<AppenderSeed, ProtocolError>` (one decode pass building the
   final dict + ordinal map).
4. **`HistoryFile::open_append`** (`protocol/io/rs/lib.rs:116-147`): replace decode-truncate-replay
   with recover → truncate torn tail → `appender_seed_from` → open sink append-mode →
   `HistoryAppender::resume`. O(decode) once, zero rewrite; stops silently dropping
   projection/index/sealed/ephemeral records.
5. **Seeded `TailFollower`** (`protocol/io/rs/lib.rs:205-247`): add `byte_offset: u64, dict:
   DictReader, edit_ids: Vec<String>`; `poll()` reads only new bytes, applies dict deltas
   incrementally.
6. **Compaction honors options** (`protocol/io/rs/lib.rs:315-366`): walk the trusted prefix
   frame-by-frame; re-emit projection/index/sealed frames per `keep_snapshots`; drop ephemeral
   iff `drop_ephemeral`. Extend tests with a projection/ephemeral-carrying fixture.
7. **`REC_ACTOR_DICT` emission**: additive roster record (not a second-dict rewire — would break
   the frozen `encode_edit` signature and every byte fixture for zero gain).
   `HistoryAppender::append_edit` tracks distinct actors; before each `commit()`, emit one
   `REC_ACTOR_DICT` delta record if new actors appeared; `HistoryLog.actors: Vec<String>`
   populated on decode.

### W9-C — store UndoPolicy dispatch + markers + stale paths; pack_value index/manifest

1. **UndoPolicy dispatch** (`store/rs/lib.rs:1305-1308`): derive the policy from the edit about
   to be undone (max-severity across its operation_meta); Semantic/Compensating → instructive
   error requiring `UndoWithPolicy`; ExactBaseOnly/TransformAgainstConcurrent dispatch directly.
   Behavior unchanged for the current all-ExactBaseOnly fleet.
2. **Duplicated `//#region` markers**: delete the duplicate opening line of each doubled pair
   (Text, Pack, CodecRegistry, Materialize, TextFormat, History, DocumentStore, Backbone,
   BlobStore) + one doubled `//#endregion 🧪Tests`.
3. **Mismatched endregion**: fix the `🔖ReconcileAlternative` / `🔖ContentAddressedCheckpoint
   AndMergeBase` boundary mislabel.
4. **Stale-path sweep**: `framework/sync` → `store/sync`/`store_sync` across ~30 files
   (mechanical replace per sentence; AGENTS.md hits → human-todos.txt).
5. **pack_index wiring + Manifest.schema_name** (`pack/value/rs/lib.rs:1257-1328`): emit
   `KIND_FIELD_INDEX` via `FieldIndexBuilder` during `encode_document`, set
   `manifest.field_index_span`; set `manifest.schema_name` from `spec.keyword`; populate
   `DecodeReport.unknown_segments`; add `read_field(bytes, spec, path)` proving the index is
   reachable. Footer chaining / `REQUIRED_CHUNKED` / `KIND_SCHEMA` / `KIND_SNAPSHOT` explicitly
   deferred (note in report).

### W9-D — CRDT combinator decision + dead DSL surface (G7, G23)

1. **Keep 5-value `MergeStrategyKind`, route the three identical arms directly to
   `chronological_compose`** (delete the three now-redundant wrapper fns
   `ordered_sequence_merge`/`text_sequence_merge`/`tombstoned_graph_set_merge`, making the
   "behaviorally identical" fact structurally explicit). Delete the consumer-less `AnchorId`.
   Written rationale in the wave report: a real positional-aware CRDT needs a `Diff`-trait
   redesign gated on a concrete future consumer.
2. **Delete orphan crates `dsl_token` + `dsl_editor`** (confirmed): remove workspace members
   `dsl/token/rs`, `dsl/editor/rs` from root `Cargo.toml`; delete both directories. Fix the stale
   facade description in `dsl/rs/Cargo.toml:6` (claims it re-exports dsl_token — never did).
3. **`#[dsl(flatten)]` dead flag** (`dsl/derive/rs/lib.rs:27,76-77`): delete the parsed field and
   the `meta.path.is_ident("flatten")` arm so the attribute becomes a compile error instead of a
   silent no-op. Grep-verify zero uses fleet-wide first; re-read `dsl/derive/rs/lib.rs`
   immediately before editing (concurrently edited by another session per the contract's risk
   note).
4. Verify: `cargo test -p protocol_crdt -p dsl -p dsl_derive`, `cargo check --workspace` (member
   removals force a full graph re-check).

### W9 ordering

All four agents run in parallel on disjoint crates. Only cross-cutting file is `store/rs/lib.rs`
(W9-C owns it exclusively — but W3/W4b/W7/W8.0 also touch it; by wave-plan ordering W9 runs after
all of those, so no conflict). Root `Cargo.toml` is touched by W7 (profile removal) and W9-D
(member removals) — if adjacent, serialize via re-read-before-edit.

---

## Cross-wave risks

- A stale agent worktree exists at `.claude/worktrees/agent-afa905cc164f52ed8` — ignore it.
- W8.0 changes `replay_operations`'s signature inside `store/rs` — internal fn, no fleet
  fallout, but must land before any W8 batch asserts `Auth`.
- `conformance.md` is the single shared W8 file; every agent re-reads before its row edit.
- Fixtures: W9-B's compaction and actor-dict changes alter `.spr` bytes only when new record
  kinds appear; regenerate any checked-in `.spr` fixtures by code, never by hand.
