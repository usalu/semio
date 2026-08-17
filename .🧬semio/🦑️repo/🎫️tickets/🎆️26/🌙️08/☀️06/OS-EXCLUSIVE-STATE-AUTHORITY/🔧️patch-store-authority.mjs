import fs from "fs";
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs";
let text = fs.readFileSync(path, "utf8");

text = text.replace(
  "content_addressed_checkpoint_id, create_document_vcs_id, invert_collection_operation,",
  "content_addressed_checkpoint_id, content_addressed_entity_id, create_document_vcs_id, edit_scoped_id, invert_collection_operation,"
);

const amendClose = "        coalesce_key: Option<String>,\n    },\n}\n//#endregion";
if (!text.includes(amendClose)) {
  console.error("amend close missing");
  process.exit(1);
}
text = text.replace(
  amendClose,
  `        coalesce_key: Option<String>,
    },
    /// @emoji 🕸️ Feeds a remote OperationEnvelope through the causal DAG into the edit timeline.
    IngestRemote {
        envelope: crate::os_spr::OperationEnvelope,
    },
    /// @emoji 🧹 Clears volatile draft-lane history that must never enter a Change/Checkpoint.
    PruneDrafts,
}
//#endregion`
);

const afterSchemas = "//#endregion 🔖️Schemas\n\n//#region 🔖️Text";
const inject = `//#endregion 🔖️Schemas

//#region 🔖️Authority
/// @emoji 🧾 Receipt from the sole store write gate (\`dispatch\` / \`reset\`).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandReceipt {
    pub edit_ids: Vec<String>,
    pub generation: u64,
}

/// @emoji 👁️ Read-only view over a document envelope — mutation is sealed through \`dispatch\`/\`reset\`.
#[derive(Clone, Copy, Debug)]
pub struct DocumentEnvelopeView<'a, P, Operation> {
    envelope: &'a DocumentEnvelope<P, Operation>,
}

impl<'a, P, Operation> DocumentEnvelopeView<'a, P, Operation> {
    pub fn schema(&self) -> &str { &self.envelope.schema }
    pub fn id(&self) -> &str { &self.envelope.id }
    pub fn vcs(&self) -> &DocumentVcs<P, Operation> { &self.envelope.vcs }
    pub fn backbone(&self) -> Option<&DocumentBackboneRef> { self.envelope.backbone.as_ref() }
    pub fn active_alternative_id(&self) -> Option<&str> { self.envelope.active_alternative_id.as_deref() }
    pub fn cursor(&self) -> Option<&DocumentCursor> { self.envelope.cursor.as_ref() }
    pub fn inner(&self) -> &'a DocumentEnvelope<P, Operation> { self.envelope }
}

/// @emoji 📝 Draft-lane store alias — same algebra as DocumentStore; PruneDrafts never enters a Change.
pub type DraftStore<P, Operation> = DocumentStore<P, Operation>;
//#endregion 🔖️Authority

//#region 🔖️Text`;
if (!text.includes(afterSchemas)) {
  console.error("afterSchemas missing");
  process.exit(1);
}
text = text.replace(afterSchemas, inject);
fs.writeFileSync(path, text);
console.log("pass1 ok");
