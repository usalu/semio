import fs from "fs";

function findDir(root, name) {
  for (const e of fs.readdirSync(root, { withFileTypes: true })) {
    const p = `${root}/${e.name}`;
    if (!e.isDirectory()) continue;
    if (e.name === name) return p;
    const found = findDir(p, name);
    if (found) return found;
  }
  return null;
}

const ticketDir = findDir(".🦑️repo/🎫️tickets", "OS-EXCLUSIVE-STATE-AUTHORITY");
const [STORE, VCS] = fs.readFileSync(`${ticketDir}/🧪paths.env`, "utf8").trim().split("\n");
console.log({ STORE, VCS, ticketDir });

let vcs = fs.readFileSync(VCS, "utf8");

const oldIds = `/// @emoji 🔑 Content-addressed entity id: \`{prefix}-{hex16(blake3(prefix || 0 || payload))}\`.
pub fn content_addressed_entity_id(prefix: &str, payload: &[u8]) -> String {
    let mut input = prefix.as_bytes().to_vec();
    input.push(0);
    input.extend_from_slice(payload);
    let digest = *blake3::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{prefix}-{hex16}")
}

/// @emoji 🆔️ Deterministic child id scoped to an edit: blake3(edit_id || ":" || ordinal).
pub fn edit_scoped_id(edit_id: &str, ordinal: u32) -> String {
    content_addressed_entity_id("scoped", format!("{edit_id}:{ordinal}").as_bytes())
}

/// @emoji 🆔️ Allocates a content-addressed id for document VCS entities.
/// Prefer [\`content_addressed_entity_id\`] with a distinguishing payload for edits/ops.
pub fn create_document_vcs_id(prefix: &str) -> String {
    content_addressed_entity_id(prefix, prefix.as_bytes())
}`;

const newIds = `//#region 🆔️Ids
/// @emoji 🔑 Content-addressed entity id: \`{prefix}-{hex16(blake3(prefix || 0 || payload))}\`.
pub fn content_addressed_entity_id(prefix: &str, payload: &[u8]) -> String {
    let mut input = prefix.as_bytes().to_vec();
    input.push(0);
    input.extend_from_slice(payload);
    let digest = *blake3::hash(&input).as_bytes();
    let hex16: String = digest[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("{prefix}-{hex16}")
}

/// @emoji 🆔️ Deterministic child id scoped to an edit: blake3(\`{edit_id}:{ordinal}\`).
pub fn edit_scoped_id(edit_id: &str, ordinal: u32) -> String {
    let digest = blake3::hash(format!("{edit_id}:{ordinal}").as_bytes());
    let hex16: String = digest.as_bytes()[..8].iter().map(|byte| format!("{byte:02x}")).collect();
    format!("scoped-{hex16}")
}

/// @emoji ✏️ Content-addressed edit id from actor + sequence + forwards fingerprint (no global counter).
pub fn mint_edit_id(actor: Option<&str>, sequence: i32, forwards_fingerprint: &[u8]) -> String {
    let mut payload = Vec::new();
    payload.extend_from_slice(actor.unwrap_or("").as_bytes());
    payload.push(0);
    payload.extend_from_slice(&sequence.to_le_bytes());
    payload.push(0);
    payload.extend_from_slice(forwards_fingerprint);
    content_addressed_entity_id("edit", &payload)
}

/// @emoji 📦️ Content-addressed change id from ordered edit ids (+ optional description distinguisher).
pub fn mint_change_id(edit_ids: &[String], description: Option<&str>) -> String {
    let mut payload = edit_ids.join("\\0").into_bytes();
    payload.push(0);
    payload.extend_from_slice(description.unwrap_or("").as_bytes());
    content_addressed_entity_id("change", &payload)
}

/// @emoji 🌿️ Content-addressed alternative id from name + ordered checkpoint ids.
pub fn mint_alternative_id(name: &str, checkpoint_ids: &[String]) -> String {
    let mut payload = name.as_bytes().to_vec();
    payload.push(0);
    payload.extend_from_slice(checkpoint_ids.join("\\0").as_bytes());
    content_addressed_entity_id("alternative", &payload)
}

/// @emoji ⚙️ Content-addressed operation id from the operation's binary (or other) fingerprint bytes.
pub fn mint_operation_id(operation_bytes: &[u8]) -> String {
    content_addressed_entity_id("operation", operation_bytes)
}

/// @emoji 🆔️ Legacy-compatible prefix-only mint — identical inputs collide.
/// Prefer [\`mint_edit_id\`] / [\`mint_change_id\`] / [\`mint_alternative_id\`] / [\`mint_operation_id\`] /
/// [\`content_addressed_entity_id\`] with a distinguishing payload.
pub fn create_document_vcs_id(prefix: &str) -> String {
    content_addressed_entity_id(prefix, prefix.as_bytes())
}
//#endregion 🆔️Ids`;

if (!vcs.includes(oldIds)) {
  console.error("VCS id block mismatch");
  const idx = vcs.indexOf("pub fn content_addressed_entity_id");
  console.log(JSON.stringify(vcs.slice(Math.max(0, idx - 80), idx + 700)));
  process.exit(1);
}
vcs = vcs.replace(oldIds, newIds);

const oldColl = `/// @emoji 🧺️ Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionOperation<TId, TItem, TPatch> {`;

const newColl = `/// @emoji 🧺️ Generic ordered-collection operation (add/remove/move/patch) with mechanical pre-state inverses.
///
/// 🎞️ \`crate::os_spr::command::CollectionOperation\` is the frozen-contract twin (\`Add { id, item, at }\`,
/// \`Move { id, to }\`). This VCS shape keeps \`index\`/\`to_index\` for \`apply_collection_operation\` below —
/// schemas differ, so these are NOT \`pub use\` aliases of spr.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CollectionOperation<TId, TItem, TPatch> {`;

if (!vcs.includes(oldColl)) {
  console.error("collection comment block missing");
  process.exit(1);
}
vcs = vcs.replace(oldColl, newColl);

const oldItem = `/// @emoji 🧩️ Sparse collection patch entry (mirrors semio_compose_rs \`XModified\`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {`;

const newItem = `/// @emoji 🧩️ Sparse collection patch entry (mirrors semio_compose_rs \`XModified\`).
///
/// 🎞️ Field-identical to \`crate::os_spr::command::ItemPatch\`, but kept local because the surrounding
/// VCS \`CollectionOperation\` schema still diverges from spr (\`index\` vs \`at\`) — see that enum's note.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemPatch<TId, TPatch> {`;

if (!vcs.includes(oldItem)) {
  console.error("ItemPatch doc missing");
  process.exit(1);
}
vcs = vcs.replace(oldItem, newItem);

const testMarker = `        let id_different_timestamp = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:02Z");
        assert_ne!(id_a, id_different_timestamp, "a different timestamp must change the id");
    }
}
//#endregion 🧪️Tests`;

const testInsert = `        let id_different_timestamp = content_addressed_checkpoint_id(None, &change_ids, &changes, Some("root"), &authors, "2026-07-27T00:00:02Z");
        assert_ne!(id_a, id_different_timestamp, "a different timestamp must change the id");
    }

    //#region 🆔️Ids
    #[test]
    fn content_addressed_entity_and_mint_helpers_are_deterministic() {
        assert_eq!(content_addressed_entity_id("x", b"payload"), content_addressed_entity_id("x", b"payload"));
        assert_ne!(content_addressed_entity_id("x", b"a"), content_addressed_entity_id("x", b"b"));
        assert_eq!(edit_scoped_id("edit-1", 0), edit_scoped_id("edit-1", 0));
        assert_ne!(edit_scoped_id("edit-1", 0), edit_scoped_id("edit-1", 1));
        assert!(edit_scoped_id("edit-1", 0).starts_with("scoped-"));
        assert_eq!(mint_edit_id(Some("alice"), 3, b"fwd"), mint_edit_id(Some("alice"), 3, b"fwd"));
        assert_ne!(mint_edit_id(Some("alice"), 3, b"fwd"), mint_edit_id(Some("bob"), 3, b"fwd"));
        assert_eq!(mint_change_id(&["e1".into(), "e2".into()], Some("msg")), mint_change_id(&["e1".into(), "e2".into()], Some("msg")));
        assert_eq!(mint_alternative_id("main", &["ck1".into()]), mint_alternative_id("main", &["ck1".into()]));
        assert_eq!(mint_operation_id(b"op-bytes"), mint_operation_id(b"op-bytes"));
        assert_eq!(create_document_vcs_id("draft"), create_document_vcs_id("draft"));
        assert!(create_document_vcs_id("draft").starts_with("draft-"));
    }
    //#endregion 🆔️Ids
}
//#endregion 🧪️Tests`;

if (!vcs.includes(testMarker)) {
  console.error("test marker missing");
  process.exit(1);
}
vcs = vcs.replace(testMarker, testInsert);
fs.writeFileSync(VCS, vcs);
console.log("vcs patched ok");

// ---- store call sites ----
let store = fs.readFileSync(STORE, "utf8");

const reexportOld = `    apply_collection_operation, apply_operation, collection_diff_from_operation, content_addressed_checkpoint_id, content_addressed_entity_id, create_document_vcs_id, edit_scoped_id, invert_collection_operation, Alternative, Author, Change, Checkpoint, CollectionDiff, CollectionOperation,
    DocumentVcs, Identified, ItemPatch, Patchable, VcsError,`;

const reexportNew = `    apply_collection_operation, apply_operation, collection_diff_from_operation, content_addressed_checkpoint_id, content_addressed_entity_id, create_document_vcs_id, edit_scoped_id, invert_collection_operation, mint_alternative_id, mint_change_id, mint_edit_id, mint_operation_id, Alternative, Author, Change, Checkpoint, CollectionDiff, CollectionOperation,
    DocumentVcs, Identified, ItemPatch, Patchable, VcsError,`;

if (!store.includes(reexportOld)) {
  console.error("reexport mismatch");
  const i = store.indexOf("content_addressed_checkpoint_id");
  console.log(JSON.stringify(store.slice(i, i + 350)));
  process.exit(1);
}
store = store.replace(reexportOld, reexportNew);

const replacements = [
  [
    `let alternative_id = content_addressed_entity_id("alternative", format!("{alternative_name}:{checkpoint_id}").as_bytes());
    envelope.vcs.alternatives.push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] });
    if let Some(message) = checkpoint_message {
        let change = Change { id: content_addressed_entity_id("change", format!("reconcile:{message}").as_bytes()), edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };`,
    `let alternative_id = mint_alternative_id(alternative_name, &[checkpoint_id.clone()]);
    envelope.vcs.alternatives.push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] });
    if let Some(message) = checkpoint_message {
        let change = Change { id: mint_change_id(&[], Some(&message)), edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };`,
  ],
  [
    `operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(content_addressed_entity_id("operation", format!("{}", now_ms()).as_bytes())))),
                    dependencies: operation.dependencies(),
                    base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                    author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                    timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                    undo_policy: operation.undo_policy(),
                    payload_hash: None,`,
    `operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(mint_operation_id(&operation.encode_op().unwrap_or_default())))),
                    dependencies: operation.dependencies(),
                    base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                    author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                    timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                    undo_policy: operation.undo_policy(),
                    payload_hash: None,`,
  ],
  [
    `operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(content_addressed_entity_id("operation", format!("{}", now_ms()).as_bytes())))),
                dependencies: operation.dependencies(),
                base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                payload_hash: None,`,
    `operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(mint_operation_id(&serde_json::to_vec(operation).unwrap_or_default())))),
                dependencies: operation.dependencies(),
                base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                payload_hash: None,`,
  ],
  [
    `let change = Change { id: content_addressed_entity_id("change", format!("{:?}:{}", pending, message.as_deref().unwrap_or("")).as_bytes()), edit_ids: pending, description: message.clone(), saved_at: now_iso() };`,
    `let change = Change { id: mint_change_id(&pending, message.as_deref()), edit_ids: pending, description: message.clone(), saved_at: now_iso() };`,
  ],
  [
    `let alt_id = content_addressed_entity_id("alternative", format!("{name}:{checkpoint_id}").as_bytes());
                self.envelope.vcs.alternatives.push(Alternative { id: alt_id.clone(), name, checkpoint_ids: vec![checkpoint_id.clone()] });`,
    `let alt_id = mint_alternative_id(&name, &[checkpoint_id.clone()]);
                self.envelope.vcs.alternatives.push(Alternative { id: alt_id.clone(), name, checkpoint_ids: vec![checkpoint_id.clone()] });`,
  ],
  [
    `let edit = Edit { id: content_addressed_entity_id("edit", format!("{}:{}:{}", self.edit_sequence, started_at, self.envelope.id).as_bytes()), actor, forwards, backwards, operation_meta, description, coalesce_key: None, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`,
    `let forwards_fingerprint = serde_json::to_vec(&forwards).unwrap_or_default();
                let edit = Edit { id: mint_edit_id(actor.as_deref(), self.edit_sequence, &forwards_fingerprint), actor, forwards, backwards, operation_meta, description, coalesce_key: None, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`,
  ],
  [
    `let edit_id = content_addressed_entity_id("edit", format!("amend:{}:{}:{}", self.edit_sequence, started_at, self.envelope.id).as_bytes());
                    let edit = Edit { id: edit_id.clone(), actor, forwards, backwards, operation_meta, description: None, coalesce_key, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`,
    `let forwards_fingerprint = serde_json::to_vec(&forwards).unwrap_or_default();
                    let edit_id = mint_edit_id(actor.as_deref(), self.edit_sequence, &forwards_fingerprint);
                    let edit = Edit { id: edit_id.clone(), actor, forwards, backwards, operation_meta, description: None, coalesce_key, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`,
  ],
  [
    `operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(content_addressed_entity_id("operation", format!("{}", now_ms()).as_bytes())))),
                dependencies: operation.dependencies(),
                base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                // 🎞️ CW3: direct blake3 (same primitive \`crate::os_pack::core::ContentHash\` uses) replaces the
                // old \`framework_hash::hash_bytes\` String hash — \`crate::os_spr::core::PayloadHash\` is
                // now \`[u8; 32]\`, not a hex string. NOT \`crate::os_pack::content_hash\`, which reads a pack
                // FILE's footer rather than hashing arbitrary bytes. 🎯️ B2: hashes the real
                // \`OpBinary\` encoding, not a JSON serialization — two ops that encode identically
                // via \`encode_op()\` but differ in JSON shape (or vice versa) must hash identically.
                payload_hash: Some(crate::os_spr::PayloadHash(*blake3::hash(&operation.encode_op().unwrap_or_default()).as_bytes())),`,
    `operation_id: Some(operation.operation_id().unwrap_or_else(|| OperationId(mint_operation_id(&operation.encode_op().unwrap_or_default())))),
                dependencies: operation.dependencies(),
                base_version: operation.base_version().map(|version| version.0).unwrap_or(0),
                author_id: Some(operation.author_id().unwrap_or_else(|| ActorId("local".into()))),
                timestamp: operation.timestamp().unwrap_or_else(|| HybridLogicalTimestamp::new(0, now_ms())),
                undo_policy: operation.undo_policy(),
                // 🎞️ CW3: direct blake3 (same primitive \`crate::os_pack::core::ContentHash\` uses) replaces the
                // old \`framework_hash::hash_bytes\` String hash — \`crate::os_spr::core::PayloadHash\` is
                // now \`[u8; 32]\`, not a hex string. NOT \`crate::os_pack::content_hash\`, which reads a pack
                // FILE's footer rather than hashing arbitrary bytes. 🎯️ B2: hashes the real
                // \`OpBinary\` encoding, not a JSON serialization — two ops that encode identically
                // via \`encode_op()\` but differ in JSON shape (or vice versa) must hash identically.
                payload_hash: Some(crate::os_spr::PayloadHash(*blake3::hash(&operation.encode_op().unwrap_or_default()).as_bytes())),`,
  ],
  [
    `let checkpoint_id = content_addressed_entity_id("space-checkpoint", format!("{message}:{:?}", document_ids).as_bytes());
        let parent_id = self.meta.projection()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let checkpoint = SpaceCheckpoint { id: checkpoint_id.clone(), parent_id, message: message.clone(), authors, timestamp: HybridLogicalTimestamp::new(0, now_ms()), members: pins };`,
    `let pins_fingerprint = serde_json::to_vec(&pins).unwrap_or_default();
        let mut space_checkpoint_payload = message.as_bytes().to_vec();
        space_checkpoint_payload.push(0);
        space_checkpoint_payload.extend_from_slice(&pins_fingerprint);
        let checkpoint_id = content_addressed_entity_id("space-checkpoint", &space_checkpoint_payload);
        let parent_id = self.meta.projection()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let checkpoint = SpaceCheckpoint { id: checkpoint_id.clone(), parent_id, message: message.clone(), authors, timestamp: HybridLogicalTimestamp::new(0, now_ms()), members: pins };`,
  ],
  [
    `let checkpoint_id = self.meta.projection()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let alternative_id = content_addressed_entity_id("space-alternative", name.as_bytes());
        let alternative = SpaceAlternative { id: alternative_id.clone(), name, checkpoint_ids: checkpoint_id.into_iter().collect() };`,
    `let checkpoint_ids: Vec<String> = self.meta.projection()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).into_iter().collect();
        let alternative_id = content_addressed_entity_id("space-alternative", mint_alternative_id(&name, &checkpoint_ids).as_bytes());
        let alternative = SpaceAlternative { id: alternative_id.clone(), name, checkpoint_ids };`,
  ],
];

for (const [from, to] of replacements) {
  if (!store.includes(from)) {
    console.error("store replacement missing:");
    console.error(from.slice(0, 120));
    process.exit(1);
  }
  store = store.replace(from, to);
}

// Remaining time-based operation mints?
const leftover = [...store.matchAll(/content_addressed_entity_id\("operation"[^\n]*/g)].map((m) => m[0]);
if (leftover.length) {
  console.error("leftover operation mints", leftover);
  process.exit(1);
}
const leftoverTime = [...store.matchAll(/content_addressed_entity_id\("edit"[^\n]*/g)].map((m) => m[0]);
if (leftoverTime.length) {
  console.error("leftover edit mints", leftoverTime);
  process.exit(1);
}

fs.writeFileSync(STORE, store);
console.log("store patched ok");
