#!/usr/bin/env bun
import fs from "fs";
import path from "path";

function discover(...preds) {
  let cur = ".";
  for (const pred of preds) {
    const names = fs.readdirSync(cur);
    const hit = typeof pred === "string" ? names.find((n) => n.includes(pred)) : names.find(pred);
    if (!hit) throw new Error("missing " + pred + " in " + cur);
    cur = path.join(cur, hit);
  }
  return cur;
}

function mustReplace(text, from, to, label) {
  if (!text.includes(from)) throw new Error("MISSING " + label + ": " + JSON.stringify(from).slice(0, 240));
  return text.replace(from, to);
}

function mustReplaceAll(text, from, to, label) {
  const count = text.split(from).length - 1;
  if (count === 0) throw new Error("MISSING " + label + ": " + JSON.stringify(from).slice(0, 240));
  return text.replaceAll(from, to);
}

const store = discover("framework", "products", "os", "modules", (n) => n.includes("store"), (n) => n.includes("component") && n.endsWith(".rs"));
const sync = discover("framework", "products", "os", "modules", (n) => n.includes("store"), (n) => n.includes("sync"), (n) => n.includes("component") && n.endsWith(".rs"));
const plugin = discover("framework", "products", "os", "modules", (n) => n.includes("plugin"), (n) => n.includes("component") && n.endsWith(".rs"));
const osComp = discover("framework", "products", "os", (n) => n.includes("component") && n.endsWith(".rs"));
const hostComp = discover("framework", "products", "os", (n) => n.includes("host"), (n) => n.includes("component") && n.endsWith(".rs"));

console.log({ store, sync, plugin, osComp, hostComp });

let text = fs.readFileSync(store, "utf8");

if (!text.includes("operation_envelope_serde")) {
  text = mustReplace(
    text,
    "    IngestRemote {\n        envelope: crate::os_spr::OperationEnvelope,\n    },",
    `    IngestRemote {
        #[serde(with = "operation_envelope_serde")]
        envelope: crate::os_spr::OperationEnvelope,
    },`,
    "ingest serde attr"
  );
  text = mustReplace(
    text,
    "//#region 🔖️CommandFormat\n",
    `//#region 🔖️CommandFormat
mod operation_envelope_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(envelope: &crate::os_spr::OperationEnvelope, serializer: S) -> Result<S::Ok, S::Error> {
        let mut bytes = Vec::new();
        crate::os_spr::encode_envelope(envelope, &mut bytes);
        bytes.serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<crate::os_spr::OperationEnvelope, D::Error> {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        let mut pos = 0;
        crate::os_spr::decode_envelope(&bytes, &mut pos).map_err(serde::de::Error::custom)
    }
}

`,
    "serde module"
  );
}

if (text.includes("Draft lane: drop redo stack")) {
  text = mustReplace(
    text,
    `            DocumentCommand::PruneDrafts => {
                // Draft lane: drop redo stack and any applied edits that are not yet in a Change.
                let committed: std::collections::HashSet<String> = self.envelope.vcs.changes.iter().flat_map(|change| change.edit_ids.iter().cloned()).collect();
                self.applied_edit_ids.retain(|id| committed.contains(id));
                self.envelope.vcs.edits.retain(|edit| committed.contains(&edit.id));
                self.redo_edit_ids.clear();
                self.tail_undo_cache = None;
                self.current = self.fold_current().expect("prune: fold_current");
                self.bump();
                Ok(())
            }`,
    `            DocumentCommand::PruneDrafts => {
                // Reserved for draft-lane stores ({@link DraftStore}): real prune lands with draft ops.
                Ok(())
            }`,
    "prune stub"
  );
}

if (text.includes("DocumentCommand::Apply { .. } | DocumentCommand::IngestRemote")) {
  text = mustReplace(
    text,
    `    pub fn dispatch(&mut self, command: DocumentCommand<Operation>) -> Result<CommandReceipt, VcsError> {
        self.pump()?;
        let is_apply = matches!(command, DocumentCommand::Apply { .. } | DocumentCommand::IngestRemote { .. });
        let before = self.applied_edit_ids.len();
        self.dispatch_inner(command)?;
        self.flush_outbound(is_apply)?;
        Ok(CommandReceipt {
            edit_ids: self.applied_edit_ids[before..].to_vec(),
            generation: self.generation(),
        })
    }`,
    `    pub fn dispatch(&mut self, command: DocumentCommand<Operation>) -> Result<CommandReceipt, VcsError> {
        self.pump()?;
        let skip_flush = matches!(command, DocumentCommand::IngestRemote { .. } | DocumentCommand::PruneDrafts);
        let is_apply = matches!(command, DocumentCommand::Apply { .. });
        let before = self.applied_edit_ids.len();
        self.dispatch_inner(command)?;
        if !skip_flush {
            self.flush_outbound(is_apply)?;
        }
        Ok(CommandReceipt {
            edit_ids: self.applied_edit_ids[before..].to_vec(),
            generation: self.generation(),
        })
    }`,
    "dispatch flush"
  );
}

if (!text.includes("fn envelope_view(")) {
  text = mustReplace(
    text,
    `    pub fn envelope(&self) -> &DocumentEnvelope<P, Operation> {
        &self.envelope
    }

    pub fn applied_edit_ids(&self) -> &[String] {`,
    `    pub fn envelope(&self) -> &DocumentEnvelope<P, Operation> {
        &self.envelope
    }

    /// @emoji 👁️ Read-only envelope view — prefer this over mutating through public fields.
    pub fn envelope_view(&self) -> DocumentEnvelopeView<'_, P, Operation> {
        DocumentEnvelopeView { envelope: &self.envelope }
    }

    pub fn applied_edit_ids(&self) -> &[String] {`,
    "envelope_view"
  );
}

if (!text.match(/enum CommandHeaderLine \{[\s\S]*?PruneDrafts,/)) {
  text = mustReplace(
    text,
    `    Amend {
        key: Option<String>,
    },
}`,
    `    Amend {
        key: Option<String>,
    },
    PruneDrafts,
}`,
    "header prune"
  );
}

const printSlice = text.slice(text.indexOf("pub fn print_command"), text.indexOf("pub fn parse_command"));
if (!printSlice.includes("PruneDrafts")) {
  text = mustReplace(
    text,
    `        DocumentCommand::AmendLast { operations, coalesce_key } => {
            out.push_str(&CommandHeaderLine::Amend { key: coalesce_key.clone() }.print_op());
            out.push('\\n');
            print_indented_ops(&mut out, operations)?;
        }
    }
    Ok(out)
}`,
    `        DocumentCommand::AmendLast { operations, coalesce_key } => {
            out.push_str(&CommandHeaderLine::Amend { key: coalesce_key.clone() }.print_op());
            out.push('\\n');
            print_indented_ops(&mut out, operations)?;
        }
        DocumentCommand::IngestRemote { .. } => {
            return Err(VcsError::Serialize("IngestRemote has no text command form".into()));
        }
        DocumentCommand::PruneDrafts => {
            out.push_str(&CommandHeaderLine::PruneDrafts.print_op());
            out.push('\\n');
        }
    }
    Ok(out)
}`,
    "print arms"
  );
}

const parseSlice = text.slice(text.indexOf("pub fn parse_command"), text.indexOf("COMMAND_BINARY_FORMAT"));
if (!parseSlice.includes("PruneDrafts")) {
  text = mustReplace(
    text,
    `        CommandHeaderLine::Amend { key } => {
            let operations = parse_indented_ops(&body_lines)?;
            if operations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("amend requires at least one operation line"));
            }
            Ok(DocumentCommand::AmendLast { operations, coalesce_key: key })
        }
    }
}`,
    `        CommandHeaderLine::Amend { key } => {
            let operations = parse_indented_ops(&body_lines)?;
            if operations.is_empty() {
                return Err(crate::os_dsl::__rt::field_error("amend requires at least one operation line"));
            }
            Ok(DocumentCommand::AmendLast { operations, coalesce_key: key })
        }
        CommandHeaderLine::PruneDrafts => Ok(DocumentCommand::PruneDrafts),
    }
}`,
    "parse arm"
  );
}

if (!text.includes("write_varint_u64(&mut out, 9)")) {
  text = mustReplace(
    text,
    `            DocumentCommand::AmendLast { operations, coalesce_key } => {
                crate::os_pack::write_varint_u64(&mut out, 8);
                out.push(if coalesce_key.is_some() { 0b01 } else { 0 });
                if let Some(key) = coalesce_key {
                    write_command_str(&mut out, key);
                }
                write_command_ops(&mut out, operations)?;
            }
        }
        Ok(out)
    }`,
    `            DocumentCommand::AmendLast { operations, coalesce_key } => {
                crate::os_pack::write_varint_u64(&mut out, 8);
                out.push(if coalesce_key.is_some() { 0b01 } else { 0 });
                if let Some(key) = coalesce_key {
                    write_command_str(&mut out, key);
                }
                write_command_ops(&mut out, operations)?;
            }
            DocumentCommand::IngestRemote { envelope } => {
                crate::os_pack::write_varint_u64(&mut out, 9);
                let mut bytes = Vec::new();
                crate::os_spr::encode_envelope(envelope, &mut bytes);
                crate::os_pack::write_varint_u64(&mut out, bytes.len() as u64);
                out.extend_from_slice(&bytes);
            }
            DocumentCommand::PruneDrafts => crate::os_pack::write_varint_u64(&mut out, 10),
        }
        Ok(out)
    }`,
    "opbinary encode"
  );

  text = mustReplace(
    text,
    `            8 => {
                let presence = reader.read_u8()?;
                let coalesce_key = if presence & 0b01 != 0 { Some(read_command_str(&mut reader)?) } else { None };
                let operations = read_command_ops(&mut reader)?;
                Ok(DocumentCommand::AmendLast { operations, coalesce_key })
            }
            other => Err(crate::os_spr::ProtocolError::Malformed { what: "command variant", offset: 1, detail: format!("unknown command ordinal {other}") }),`,
    `            8 => {
                let presence = reader.read_u8()?;
                let coalesce_key = if presence & 0b01 != 0 { Some(read_command_str(&mut reader)?) } else { None };
                let operations = read_command_ops(&mut reader)?;
                Ok(DocumentCommand::AmendLast { operations, coalesce_key })
            }
            9 => {
                let len = reader.read_varint_u64()?;
                let bytes = reader.read_bytes(len as usize)?;
                let mut pos = 0;
                let envelope = crate::os_spr::decode_envelope(bytes, &mut pos).map_err(|error| crate::os_spr::ProtocolError::Malformed { what: "ingest envelope", offset: 0, detail: error.to_string() })?;
                Ok(DocumentCommand::IngestRemote { envelope })
            }
            10 => Ok(DocumentCommand::PruneDrafts),
            other => Err(crate::os_spr::ProtocolError::Malformed { what: "command variant", offset: 1, detail: format!("unknown command ordinal {other}") }),`,
    "opbinary decode"
  );
}

if (text.includes("return self.dispatch(DocumentCommand::SwitchAlternative { alternative_id: alternative_id.to_string() });")) {
  text = mustReplace(
    text,
    `                return self.dispatch(DocumentCommand::SwitchAlternative { alternative_id: alternative_id.to_string() });
            }
        }
        self.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.to_string() })
    }`,
    `                return self.dispatch(DocumentCommand::SwitchAlternative { alternative_id: alternative_id.to_string() }).map(|_| ());
            }
        }
        self.dispatch(DocumentCommand::CheckoutCheckpoint { checkpoint_id: checkpoint_id.to_string() }).map(|_| ())
    }`,
    "checkout map"
  );
}

if (text.includes("self.dispatch(DocumentCommand::Undo)\n    }")) {
  text = mustReplace(
    text,
    `    fn undo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Undo)
    }

    fn redo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Redo)
    }`,
    `    fn undo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Undo).map(|_| ())
    }

    fn redo(&mut self) -> Result<(), VcsError> {
        self.dispatch(DocumentCommand::Redo).map(|_| ())
    }`,
    "undo/redo map"
  );
}

// Tests inside store: set_state -> reset (with expect), ingest_remote -> dispatch
text = text.replace(/\bstore\.set_state\(/g, "store.reset(");
text = text.replace(/\bremote_store\.set_state\(/g, "remote_store.reset(");
text = text.replace(/store\.reset\(([^;]+)\);/g, (m, args) => (m.includes("expect") || m.includes("?") ? m : `store.reset(${args}).expect("reset");`));
text = text.replace(/remote_store\.reset\(([^;]+)\);/g, (m, args) => (m.includes("expect") || m.includes("?") ? m : `remote_store.reset(${args}).expect("reset");`));

if (text.includes("store.ingest_remote(foreign_operation_envelope(")) {
  text = text.replace(
    /store\.ingest_remote\(foreign_operation_envelope\(([^)]*)\)\)\.expect\("ingest foreign"\);/g,
    'store.dispatch(DocumentCommand::IngestRemote { envelope: foreign_operation_envelope($1) }).expect("ingest foreign");'
  );
}
if (text.includes('store.ingest_remote(foreign).expect("ingest foreign");')) {
  text = mustReplace(
    text,
    'store.ingest_remote(foreign).expect("ingest foreign");',
    'store.dispatch(DocumentCommand::IngestRemote { envelope: foreign }).expect("ingest foreign");',
    "test ingest2"
  );
}

fs.writeFileSync(store, text);
console.log("store patched");

let syncText = fs.readFileSync(sync, "utf8");
if (syncText.includes("self.store.ingest_remote(envelope)")) {
  syncText = mustReplace(
    syncText,
    `        self.store.ingest_remote(envelope).map_err(|error| SyncError::Vcs(error.to_string()))`,
    `        self.store.dispatch(crate::os_store::DocumentCommand::IngestRemote { envelope }).map(|_| ()).map_err(|error| SyncError::Vcs(error.to_string()))`,
    "sync receive"
  );
}
if (syncText.includes("self.store.set_state(envelope, applied, redo);")) {
  syncText = mustReplace(
    syncText,
    `        self.store.set_state(envelope, applied, redo);`,
    `        self.store.reset(envelope, applied, redo).map_err(|error| SyncError::Vcs(error.to_string()))?;`,
    "sync reconcile"
  );
}
fs.writeFileSync(sync, syncText);
console.log("sync patched");

let pluginText = fs.readFileSync(plugin, "utf8");
pluginText = pluginText.replaceAll("self.config_store.set_state(", "self.config_store.reset(");
pluginText = pluginText.replaceAll("self.store.set_envelope(", "TEMP_RESET_TWO_ARG(");
pluginText = pluginText.replaceAll("self.store.set_state(", "self.store.reset(");
pluginText = pluginText.replaceAll("TEMP_RESET_TWO_ARG(", "self.store.reset(");
pluginText = pluginText.replace(
  /self\.store\.reset\((parsed\.envelope), (applied)\);/g,
  "self.store.reset($1, $2, Vec::new()).map_err(|error| error.into_fault())?;"
);
pluginText = pluginText.replace(
  /self\.(config_store|store)\.reset\((parsed\.envelope), (applied), (redo)\);/g,
  "self.$1.reset($2, $3, $4).map_err(|error| error.into_fault())?;"
);
if (pluginText.includes("self.store.ingest_remote(envelope)")) {
  pluginText = mustReplace(
    pluginText,
    `                self.store.ingest_remote(envelope).map_err(|error| error.into_fault())?;`,
    `                self.store.dispatch(DocumentCommand::IngestRemote { envelope }).map_err(|error| error.into_fault())?;`,
    "plugin ingest"
  );
}
fs.writeFileSync(plugin, pluginText);
console.log("plugin patched");

for (const [label, p] of [
  ["os", osComp],
  ["host", hostComp],
]) {
  let t = fs.readFileSync(p, "utf8");
  if (!t.includes("inner.set_envelope(")) {
    console.log(label, "already patched or missing");
    continue;
  }
  t = mustReplace(
    t,
    `                inner.set_envelope(snapshot, applied_edit_ids);`,
    `                inner.reset(snapshot, applied_edit_ids, Vec::new()).expect("reset applied edits");`,
    label + " reset"
  );
  fs.writeFileSync(p, t);
  console.log(label, "patched");
}

console.log("done");
