import fs from "fs";
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs";
let text = fs.readFileSync(path, "utf8");

// Seal set_envelope / set_state as pub(crate)
text = text.replace(
  "    pub fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>) {",
  "    pub(crate) fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>) {"
);
text = text.replace(
  "    pub fn set_state(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>) {",
  "    pub(crate) fn set_state(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>) {"
);

// Add public reset before set_envelope
if (!text.includes("pub fn reset(&mut self,")) {
  text = text.replace(
    "    pub(crate) fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>) {",
    `    /// @emoji ♻️ Sole public reload API — replaces the former public \`set_state\`/\`set_envelope\` escape hatches.
    pub fn reset(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>, redo_edit_ids: Vec<String>) -> Result<CommandReceipt, VcsError> {
        self.set_state(envelope, applied_edit_ids, redo_edit_ids);
        Ok(CommandReceipt { edit_ids: self.applied_edit_ids.clone(), generation: self.generation() })
    }

    pub(crate) fn set_envelope(&mut self, envelope: DocumentEnvelope<P, Operation>, applied_edit_ids: Vec<String>) {`
  );
}

// Change dispatch signature and body to return CommandReceipt
const oldDispatch = `    pub fn dispatch(&mut self, command: DocumentCommand<Operation>) -> Result<(), VcsError> {
        self.pump()?;
        let is_apply = matches!(command, DocumentCommand::Apply { .. });
        self.dispatch_inner(command)?;
        self.flush_outbound(is_apply)
    }`;
const newDispatch = `    pub fn dispatch(&mut self, command: DocumentCommand<Operation>) -> Result<CommandReceipt, VcsError> {
        self.pump()?;
        let is_apply = matches!(command, DocumentCommand::Apply { .. } | DocumentCommand::IngestRemote { .. });
        let before = self.applied_edit_ids.len();
        self.dispatch_inner(command)?;
        self.flush_outbound(is_apply)?;
        Ok(CommandReceipt {
            edit_ids: self.applied_edit_ids[before..].to_vec(),
            generation: self.generation(),
        })
    }`;
if (!text.includes(oldDispatch)) {
  console.error("dispatch block missing");
  process.exit(1);
}
text = text.replace(oldDispatch, newDispatch);

// Seal ingest_remote and keep as pub(crate); public path is DocumentCommand::IngestRemote
text = text.replace(
  "    pub fn ingest_remote(&mut self, envelope: crate::os_spr::OperationEnvelope) -> Result<(), VcsError> {",
  "    pub(crate) fn ingest_remote(&mut self, envelope: crate::os_spr::OperationEnvelope) -> Result<(), VcsError> {"
);

// Add match arms for IngestRemote and PruneDrafts before the closing of dispatch_inner's match
// Find AmendLast arm end - the match closes with `        }\n    }\n\n    /// @emoji 🔂️ Replays`
const replayMarker = "    /// @emoji 🔂️ Replays `operations` over `pre_projection`";
const idx = text.indexOf(replayMarker);
if (idx < 0) {
  console.error("replay marker missing");
  process.exit(1);
}
// Walk backwards to find the end of AmendLast match arm - look for last `        }\n    }` before replayMarker
const before = text.slice(0, idx);
const insertAt = before.lastIndexOf("            }\n        }\n    }\n\n");
if (insertAt < 0) {
  console.error("match close not found");
  // try alternate
  console.log(JSON.stringify(before.slice(-200)));
  process.exit(1);
}
const arms = `            }
            DocumentCommand::IngestRemote { envelope } => {
                self.ingest_remote(envelope)?;
                Ok(())
            }
            DocumentCommand::PruneDrafts => {
                // Draft lane: drop redo stack and any applied edits that are not yet in a Change.
                let committed: std::collections::HashSet<String> = self.envelope.vcs.changes.iter().flat_map(|change| change.edit_ids.iter().cloned()).collect();
                self.applied_edit_ids.retain(|id| committed.contains(id));
                self.envelope.vcs.edits.retain(|edit| committed.contains(&edit.id));
                self.redo_edit_ids.clear();
                self.tail_undo_cache = None;
                self.current = self.fold_current().expect("prune: fold_current");
                self.bump();
                Ok(())
            }
        }
    }

`;
// Replace the closing pattern once at insertAt
const pattern = "            }\n        }\n    }\n\n";
const endPat = before.lastIndexOf(pattern);
text = text.slice(0, endPat) + arms + text.slice(endPat + pattern.length);

// Fix id minting call sites to be content-addressed with distinguishing payloads
text = text.replace(
  `let alternative_id = create_document_vcs_id("alternative");
    envelope.vcs.alternatives.push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] });
    if let Some(message) = checkpoint_message {
        let change = Change { id: create_document_vcs_id("change"), edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };`,
  `let alternative_id = content_addressed_entity_id("alternative", format!("{alternative_name}:{checkpoint_id}").as_bytes());
    envelope.vcs.alternatives.push(Alternative { id: alternative_id.clone(), name: alternative_name.to_string(), checkpoint_ids: vec![checkpoint_id] });
    if let Some(message) = checkpoint_message {
        let change = Change { id: content_addressed_entity_id("change", format!("reconcile:{message}").as_bytes()), edit_ids: Vec::new(), description: Some(message), saved_at: now_iso() };`
);

text = text.replace(
  `let change = Change { id: create_document_vcs_id("change"), edit_ids: pending, description: message.clone(), saved_at: now_iso() };`,
  `let change = Change { id: content_addressed_entity_id("change", format!("{:?}:{}", pending, message.as_deref().unwrap_or("")).as_bytes()), edit_ids: pending, description: message.clone(), saved_at: now_iso() };`
);

text = text.replace(
  `let alt_id = create_document_vcs_id("alternative");
                self.envelope.vcs.alternatives.push(Alternative { id: alt_id.clone(), name, checkpoint_ids: vec![checkpoint_id.clone()] });`,
  `let alt_id = content_addressed_entity_id("alternative", format!("{name}:{checkpoint_id}").as_bytes());
                self.envelope.vcs.alternatives.push(Alternative { id: alt_id.clone(), name, checkpoint_ids: vec![checkpoint_id.clone()] });`
);

text = text.replace(
  `let edit = Edit { id: create_document_vcs_id("edit"), actor, forwards, backwards, operation_meta, description, coalesce_key: None, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`,
  `let edit = Edit { id: content_addressed_entity_id("edit", format!("{}:{}:{}", self.edit_sequence, started_at, self.envelope.id).as_bytes()), actor, forwards, backwards, operation_meta, description, coalesce_key: None, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`
);

text = text.replace(
  `let edit_id = create_document_vcs_id("edit");
                    let edit = Edit { id: edit_id.clone(), actor, forwards, backwards, operation_meta, description: None, coalesce_key, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`,
  `let edit_id = content_addressed_entity_id("edit", format!("amend:{}:{}:{}", self.edit_sequence, started_at, self.envelope.id).as_bytes());
                    let edit = Edit { id: edit_id.clone(), actor, forwards, backwards, operation_meta, description: None, coalesce_key, sequence_number: self.edit_sequence, started_at, finished_at: Some(now_iso()) };`
);

text = text.replace(
  `let checkpoint_id = create_document_vcs_id("space-checkpoint");`,
  `let checkpoint_id = content_addressed_entity_id("space-checkpoint", format!("{message}:{:?}", document_ids).as_bytes());`
);

text = text.replace(
  `let alternative_id = create_document_vcs_id("space-alternative");`,
  `let alternative_id = content_addressed_entity_id("space-alternative", name.as_bytes());`
);

// operation id fallbacks — use edit_scoped via sequence-ish payload from timestamp
text = text.replaceAll(
  `OperationId(create_document_vcs_id("operation"))`,
  `OperationId(content_addressed_entity_id("operation", format!("{}", now_ms()).as_bytes()))`
);

fs.writeFileSync(path, text);
console.log("pass2 seal ok");
