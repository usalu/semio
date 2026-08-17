import fs from "fs";
import path from "path";

const fw = fs.readdirSync(".").find((n) => n.endsWith("framework"));
const os = fs.readdirSync(path.join(fw, "🛍️products")).find((n) => n.endsWith("os"));
const storePath = path.join(fw, "🛍️products", os, "🔨️modules", "🏪️store", "🦀️component.rs");
const hostPath = path.join(fw, "🛍️products", os, "🔨️modules", "🔌️plugin", "🖥️host", "🦀️component.rs");

// --- DocumentCodec: add apply_ops_binary ---
let store = fs.readFileSync(storePath, "utf8");
if (!store.includes("apply_ops_binary:")) {
  const fieldNeedle = `    pub edit_text_from_envelope: fn(&crate::os_spr::OperationEnvelope) -> Result<String, VcsError>,
}`;
  const fieldAdd = `    pub edit_text_from_envelope: fn(&crate::os_spr::OperationEnvelope) -> Result<String, VcsError>,
    /// 🧾 `(pack, spr, encode_ops_vec bytes) -> (pack, spr, ops text)` — host-authoritative Emit apply.
    pub apply_ops_binary: fn(&[u8], &[u8], &[u8]) -> Result<(Vec<u8>, Vec<u8>, String), VcsError>,
}`;
  if (!store.includes(fieldNeedle)) throw new Error("codec fields needle missing");
  store = store.replace(fieldNeedle, fieldAdd);

  // Insert apply_ops_binary_impl inside of() before Self { 
  const ofSelf = `        Self {
            schema: schema.into(),
            extension: P::EXTENSION,
            pack_schema_hash: P::pack_schema_hash(),
            compile_dsl: compile_dsl_impl::<P, Operation>,
            print_mirror: print_mirror_impl::<P, Operation>,
            edit_text_from_envelope: edit_text_from_envelope_impl::<P, Operation>,
        }`;
  // find exact - may vary
  if (!store.includes("edit_text_from_envelope: edit_text_from_envelope_impl::<P, Operation>,")) {
    throw new Error("of() Self needle missing");
  }

  const applyImpl = `
        fn apply_ops_binary_impl<P, Operation>(pack: &[u8], spr: &[u8], ops_vec: &[u8]) -> Result<(Vec<u8>, Vec<u8>, String), VcsError>
        where
            P: Clone + DocumentDsl + DocumentPack,
            Operation: OpText + OpBinary + self::Operation<P>,
        {
            let mut parsed: ParsedDocumentText<P, Operation> = parse_document_pack(pack, spr).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let frames = crate::os_spr::decode_ops_vec(ops_vec).map_err(|error| VcsError::Deserialize(error.to_string()))?;
            for frame in frames {
                let op = Operation::decode_op(&frame).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                op.apply(&mut parsed.envelope.vcs.initial_projection);
            }
            let pack_files = print_document_pack(&parsed.envelope)?;
            let text = print_document_text(&parsed.envelope)?;
            Ok((pack_files.pack, pack_files.spr, text.ops))
        }
`;
  // Insert before edit_text_from_envelope_impl or before Self
  const editImplMark = "fn edit_text_from_envelope_impl";
  const editIdx = store.indexOf(editImplMark);
  if (editIdx < 0) throw new Error("edit_text_from_envelope_impl missing");
  // insert apply impl before edit impl
  if (!store.includes("fn apply_ops_binary_impl")) {
    store = store.slice(0, editIdx) + applyImpl + "\n        " + store.slice(editIdx);
  }
  store = store.replace(
    "edit_text_from_envelope: edit_text_from_envelope_impl::<P, Operation>,\n        }",
    "edit_text_from_envelope: edit_text_from_envelope_impl::<P, Operation>,\n            apply_ops_binary: apply_ops_binary_impl::<P, Operation>,\n        }",
  );
  fs.writeFileSync(storePath, store);
  console.log("store DocumentCodec updated");
} else {
  console.log("store already has apply_ops_binary");
}

// --- Host SessionLanePack apply ---
let host = fs.readFileSync(hostPath, "utf8");
if (!host.includes("fn apply_emit_ops")) {
  const oldRecord = `    /// 🧾 Records Emit op bytes for this lane (host-authoritative apply pending).
    pub fn record_emit_ops(&mut self, ops: Vec<u8>) {
        if !ops.is_empty() {
            self.pending_binary_ops = ops;
        }
    }`;
  const neu = `    /// 🧾 Records Emit op bytes for this lane (legacy alias — prefer {@link Self::apply_emit_ops}).
    pub fn record_emit_ops(&mut self, ops: Vec<u8>) {
        self.apply_emit_ops(None, ops);
    }

    /// 🧾 Host-authoritative Emit apply: merges op frames into the pending journal and, when a
    /// document schema codec is registered, folds them into pack+spr+ops text.
    pub fn apply_emit_ops(&mut self, schema: Option<&str>, ops: Vec<u8>) {
        if ops.is_empty() {
            return;
        }
        let mut frames = match store::protocol_decode_ops_vec(&self.pending_binary_ops) {
            Ok(existing) if !self.pending_binary_ops.is_empty() => existing,
            _ => Vec::new(),
        };
        match store::protocol_decode_ops_vec(&ops) {
            Ok(incoming) => frames.extend(incoming),
            Err(_) => frames.push(ops.clone()),
        }
        self.pending_binary_ops = store::protocol_encode_ops_vec(&frames);
        if !self.ops.is_empty() && !self.ops.ends_with('\\n') {
            self.ops.push('\\n');
        }
        self.ops.push_str(&format!("# emit-binary {}\\n", hex::encode(blake3::hash(&ops).as_bytes())));
        if let Some(schema) = schema {
            if let Some(codec) = store::document_codec(schema) {
                if !self.pack.is_empty() || !self.spr.is_empty() {
                    match (codec.apply_ops_binary)(&self.pack, &self.spr, &self.pending_binary_ops) {
                        Ok((pack, spr, ops_text)) => {
                            self.pack = pack;
                            self.spr = spr;
                            self.ops = ops_text;
                            self.pending_binary_ops.clear();
                            eprintln!("[DEBUG] host apply_emit_ops folded via DocumentCodec schema={schema}");
                        }
                        Err(error) => {
                            eprintln!("[DEBUG] host apply_emit_ops codec fold failed schema={schema}: {error}");
                        }
                    }
                }
            }
        }
    }`;
  if (!host.includes(oldRecord)) {
    // try without doc comment exact
    const alt = `    pub fn record_emit_ops(&mut self, ops: Vec<u8>) {
        if !ops.is_empty() {
            self.pending_binary_ops = ops;
        }
    }`;
    if (!host.includes(alt)) throw new Error("record_emit_ops missing");
    host = host.replace(alt, neu.replace(oldRecord, alt).includes("apply_emit_ops") ? neu.slice(neu.indexOf("pub fn record_emit_ops")) : neu);
  } else {
    host = host.replace(oldRecord, neu);
  }

  // DocumentSession schemas
  if (!host.includes("pub document_schema:")) {
    host = host.replace(
      `pub struct DocumentSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub document: SessionLanePack,
    pub config: SessionLanePack,
    pub draft: SessionLanePack,
}`,
      `pub struct DocumentSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub document_schema: Option<String>,
    pub config_schema: Option<String>,
    pub draft_schema: Option<String>,
    pub document: SessionLanePack,
    pub config: SessionLanePack,
    pub draft: SessionLanePack,
}`,
    );
  }

  // Emit arm
  const emitOld = `                AppFrame::Emit { document_ops, config_ops, draft_ops, .. } => {
                    let session = host.ensure_session(instance_id);
                    session.document.record_emit_ops(document_ops);
                    session.config.record_emit_ops(config_ops);
                    session.draft.record_emit_ops(draft_ops);
                    session.command_log_len = session.command_log_len.saturating_add(1);
                    session.generation = session.generation.saturating_add(1);
                    eprintln!(
                        "[DEBUG] host Emit recorded pending ops doc={} cfg={} draft={} gen={}",
                        session.document.pending_binary_ops.len(),
                        session.config.pending_binary_ops.len(),
                        session.draft.pending_binary_ops.len(),
                        session.generation
                    );
                }`;
  const emitNew = `                AppFrame::Emit { document_ops, config_ops, draft_ops, .. } => {
                    let session = host.ensure_session(instance_id);
                    let document_schema = session.document_schema.clone();
                    let config_schema = session.config_schema.clone();
                    let draft_schema = session.draft_schema.clone();
                    session.document.apply_emit_ops(document_schema.as_deref(), document_ops);
                    session.config.apply_emit_ops(config_schema.as_deref(), config_ops);
                    session.draft.apply_emit_ops(draft_schema.as_deref(), draft_ops);
                    session.command_log_len = session.command_log_len.saturating_add(1);
                    session.generation = session.generation.saturating_add(1);
                    eprintln!(
                        "[DEBUG] host Emit applied ops doc_pending={} cfg_pending={} draft_pending={} gen={}",
                        session.document.pending_binary_ops.len(),
                        session.config.pending_binary_ops.len(),
                        session.draft.pending_binary_ops.len(),
                        session.generation
                    );
                }`;
  if (!host.includes(emitOld)) throw new Error("Emit arm missing");
  host = host.replace(emitOld, emitNew);

  // bind_session_schemas API on WasmPluginRuntime
  if (!host.includes("fn bind_session_schemas")) {
    const destroy = `    pub fn destroy_app(&self, instance_id: u32) {
        if let Ok(mut store) = self.store_guard() {
            store.data_mut().sessions.remove(&instance_id);
        }
    }`;
    const bind = `    pub fn destroy_app(&self, instance_id: u32) {
        if let Ok(mut store) = self.store_guard() {
            store.data_mut().sessions.remove(&instance_id);
        }
    }

    /// 🧬️ Bind document/config/draft schema ids so Emit can fold through {@link store::DocumentCodec}.
    pub fn bind_session_schemas(
        &self,
        instance_id: u32,
        document_schema: impl Into<Option<String>>,
        config_schema: impl Into<Option<String>>,
        draft_schema: impl Into<Option<String>>,
    ) {
        if let Ok(mut store) = self.store_guard() {
            let session = store.data_mut().ensure_session(instance_id);
            session.document_schema = document_schema.into();
            session.config_schema = config_schema.into();
            session.draft_schema = draft_schema.into();
        }
    }`;
    if (host.includes(destroy)) host = host.replace(destroy, bind);
  }

  fs.writeFileSync(hostPath, host);
  console.log("host updated");
} else {
  console.log("host already has apply_emit_ops");
}
