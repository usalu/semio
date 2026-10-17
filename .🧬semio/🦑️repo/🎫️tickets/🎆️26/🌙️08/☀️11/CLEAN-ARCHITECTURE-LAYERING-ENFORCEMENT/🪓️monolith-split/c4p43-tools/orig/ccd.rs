//! 🔣️ Binary codec: option and vec combinators, pack readers and writers, and the `AppCommand` and `AppFrame` tag dispatch.

use ::protocol::wire::command_ingress::{FixedCommandPage, CommandPageSet, PagedCommand, COMMAND_PAGE_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, INVOCATION_RESULT_PACK_MAXIMUM_BYTES};
use super::{AppCommand, AppDocumentIdentity, AppFrame, ChildHeadPackEntry, ChildPackEntry, DOCUMENT_ARCHIVE_MAXIMUM_BYTES, DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, DOCUMENT_ARCHIVE_VERSION, DocumentArchiveArtifactRef, DocumentArchiveLoadState, DocumentArchiveLoadStatus, DocumentArchiveOwnerRef, DocumentArchivePack, MediaExportHandleWire, MediaExportStateWire, OwnedDocumentMemberPackEntry, PresenceRosterWire, TRANSACTION_PREPARED_OPS_MAXIMUM, WindowConfigPackEntry};

//#region 🔖️Codec
// Hand-rolled binary frame encode/decode: `tag: u8 | fields...` — see the module-level docstring.
// `protocol::wire::🔖️WireCodec` supplies the primitives; this crate adds only the option/vec
// combinators the frame shapes need plus the tag-dispatch match arms below.

// 🚫️async: R9 pure accessor — most call sites are inside `.ok_or_else`'s sync closure
// (Option::ok_or_else requires sync FnOnce); no suspension point exists in the body either.
pub(super) fn malformed(what: &'static str, offset: u64, detail: &str) -> protocol::ProtocolError {
    protocol::ProtocolError::Malformed { what, offset, detail: detail.to_string() }
}

//#region 🔖️Combinators
async fn write_opt_u64(out: &mut Vec<u8>, value: &Option<u64>) {
    protocol::write_bool(out, value.is_some());
    if let Some(v) = value {
        protocol::write_varint_u64(out, *v);
    }
}

async fn read_opt_u64(bytes: &[u8], pos: &mut usize) -> Result<Option<u64>, protocol::ProtocolError> {
    if protocol::read_bool(bytes, pos)? {
        Ok(Some(protocol::read_varint_u64(bytes, pos)?))
    } else {
        Ok(None)
    }
}

fn read_invocation_result_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<u8>, protocol::ProtocolError> {
    let start = *pos;
    let length = usize::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("channel invocation result pack", start as u64, "packed mutations or inverse group length does not fit this host"))?;
    if length > INVOCATION_RESULT_PACK_MAXIMUM_BYTES {
        return Err(malformed("channel invocation result pack", start as u64, "packed mutations or inverse group exceed command transport authority"));
    }
    let end = pos.checked_add(length).ok_or_else(|| malformed("channel invocation result pack", start as u64, "packed mutations or inverse group length overflow"))?;
    let value = bytes.get(*pos..end).ok_or_else(|| malformed("channel invocation result pack", start as u64, "packed mutations or inverse group are truncated"))?.to_vec();
    *pos = end;
    Ok(value)
}

/// 🎞️ `presence u8 | byte` — an `Option<u8>` (`AppCommand::Presence.own_color`), the same
/// presence-byte convention as {@link write_opt_u64} above.
async fn write_opt_u8(out: &mut Vec<u8>, value: &Option<u8>) {
    protocol::write_bool(out, value.is_some());
    if let Some(v) = value {
        out.push(*v);
    }
}

async fn read_opt_u8(bytes: &[u8], pos: &mut usize) -> Result<Option<u8>, protocol::ProtocolError> {
    if protocol::read_bool(bytes, pos)? {
        let byte = *bytes.get(*pos).ok_or_else(|| malformed("channel app-command opt-u8", *pos as u64, "truncated"))?;
        *pos += 1;
        Ok(Some(byte))
    } else {
        Ok(None)
    }
}

async fn write_vec_bytes(out: &mut Vec<u8>, values: &[Vec<u8>]) {
    protocol::write_varint_u64(out, values.len() as u64);
    for value in values {
        protocol::write_bytes(out, value);
    }
}

async fn read_vec_bytes(bytes: &[u8], pos: &mut usize) -> Result<Vec<Vec<u8>>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    // 🚫️async: R10 shape 2 — `read_bytes` is async but `Iterator::map`'s closure is sync; hoisted
    // into a plain loop so each element can be awaited.
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(protocol::read_bytes(bytes, pos)?);
    }
    Ok(out)
}

async fn write_presence_roster(out: &mut Vec<u8>, roster: &PresenceRosterWire) {
    protocol::write_varint_u64(out, roster.len() as u64);
    for entry in roster.iter() {
        protocol::write_bytes(out, entry);
    }
}

async fn write_vec_envelope(out: &mut Vec<u8>, values: &[protocol::causal::MutationEnvelope]) {
    protocol::write_varint_u64(out, values.len() as u64);
    for value in values {
        protocol::io::binary::causal::encode_envelope(value, out);
    }
}

async fn read_vec_envelope(bytes: &[u8], pos: &mut usize) -> Result<Vec<protocol::causal::MutationEnvelope>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    // 🚫️async: R10 shape 2 — `decode_envelope` is async but `Iterator::map`'s closure is sync;
    // hoisted into a plain loop so each element can be awaited.
    let mut out = Vec::with_capacity(count as usize);
    for _ in 0..count {
        out.push(protocol::io::binary::causal::decode_envelope(bytes, pos)?);
    }
    Ok(out)
}
//#endregion 🔖️Combinators

/// 📤️ Encodes one `AppCommand`: `tag u8 | fields`.

struct CommandPageWriter {
    pages: CommandPageSet,
    current: [u8; COMMAND_PAGE_MAXIMUM_BYTES],
    current_len: usize,
    pub(super) bytes: usize,
}

impl CommandPageWriter {
    /// ✍️ The generic encoder is the ONE authority that cannot declare its page count up front — it
    /// discovers it while writing — so it declares the whole 64-page ceiling. It runs on a host, not
    /// on the guest's fixed linear memory: the guest never encodes, it only admits pages.
    pub(super) fn try_new() -> Result<Self, semio_framework_diagnostic::Fault> {
        Ok(Self { pages: CommandPageSet::try_new(COMMAND_MAXIMUM_PAGES)?, current: [0; COMMAND_PAGE_MAXIMUM_BYTES], current_len: 0, bytes: 0 })
    }

    fn flush(&mut self) -> Result<(), semio_framework_diagnostic::Fault> {
        let current = std::mem::replace(&mut self.current, [0; COMMAND_PAGE_MAXIMUM_BYTES]);
        let page = FixedCommandPage::try_from_array(current, self.current_len as u32)?;
        self.current_len = 0;
        self.pages.try_push(page).map_err(|(fault, _page)| fault)
    }

    fn write(&mut self, mut bytes: &[u8]) -> Result<(), semio_framework_diagnostic::Fault> {
        while !bytes.is_empty() {
            if self.current_len == COMMAND_PAGE_MAXIMUM_BYTES {
                self.flush()?;
            }
            let take = bytes.len().min(COMMAND_PAGE_MAXIMUM_BYTES - self.current_len);
            self.current[self.current_len..self.current_len + take].copy_from_slice(&bytes[..take]);
            self.current_len += take;
            bytes = &bytes[take..];
            self.bytes += take;
        }
        Ok(())
    }

    fn byte(&mut self, byte: u8) -> Result<(), semio_framework_diagnostic::Fault> {
        self.write(&[byte])
    }

    pub(super) fn varint(&mut self, mut value: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.byte(byte)?;
            if value == 0 {
                return Ok(());
            }
        }
    }

    pub(super) fn bytes(&mut self, bytes: &[u8]) -> Result<(), semio_framework_diagnostic::Fault> {
        self.varint(bytes.len() as u64)?;
        self.write(bytes)
    }

    pub(super) fn string(&mut self, value: &str) -> Result<(), semio_framework_diagnostic::Fault> {
        self.bytes(value.as_bytes())
    }

    fn envelope(&mut self, value: &protocol::causal::MutationEnvelope) -> Result<(), semio_framework_diagnostic::Fault> {
        self.string(&value.mutation_id.0)?;
        self.string(&value.document_id.0)?;
        self.string(&value.actor.0)?;
        self.varint(value.dependencies.len() as u64)?;
        for dependency in &value.dependencies {
            self.string(&dependency.0)?;
        }
        match &value.observed {
            Some(observed) => {
                self.varint(1)?;
                self.string(&observed.0)?;
            }
            None => self.varint(0)?,
        }
        self.varint(value.target.len() as u64)?;
        for segment in &value.target {
            self.string(segment)?;
        }
        self.string(&value.diff.schema.0)?;
        self.bytes(&value.diff.payload)?;
        self.string(&value.inverse.schema.0)?;
        self.bytes(&value.inverse.payload)?;
        self.varint(value.timestamp.actor)?;
        self.varint(value.timestamp.physical_ms)?;
        self.varint(value.timestamp.logical)?;
        match &value.transaction {
            Some(transaction) => {
                self.varint(1)?;
                self.string(&transaction.id)?;
                self.string(&transaction.tool)
            }
            None => self.varint(0),
        }
    }

    pub(super) fn finish(mut self) -> Result<PagedCommand, semio_framework_diagnostic::Fault> {
        if self.current_len != 0 {
            self.flush()?;
        }
        PagedCommand::try_from_pages(self.pages).map_err(|(fault, _pages)| fault)
    }
}

fn write_media_export_handle_paged(out: &mut CommandPageWriter, handle: &MediaExportHandleWire) -> Result<(), semio_framework_diagnostic::Fault> {
    out.varint(u64::from(handle.app_instance_id))?;
    out.string(&handle.parent_document_id)?;
    out.varint(handle.operation_id)?;
    out.varint(handle.base_revision)?;
    out.varint(handle.generation)
}

fn write_media_export_handle(out: &mut Vec<u8>, handle: &MediaExportHandleWire) {
    protocol::write_varint_u64(out, u64::from(handle.app_instance_id));
    protocol::write_str(out, &handle.parent_document_id);
    protocol::write_varint_u64(out, handle.operation_id);
    protocol::write_varint_u64(out, handle.base_revision);
    protocol::write_varint_u64(out, handle.generation);
}

fn read_media_export_handle(bytes: &[u8], pos: &mut usize) -> Result<MediaExportHandleWire, protocol::ProtocolError> {
    let app_instance_id = u32::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("media export app instance", *pos as u64, "app instance exceeds u32"))?;
    Ok(MediaExportHandleWire {
        app_instance_id,
        parent_document_id: protocol::read_str(bytes, pos)?,
        operation_id: protocol::read_varint_u64(bytes, pos)?,
        base_revision: protocol::read_varint_u64(bytes, pos)?,
        generation: protocol::read_varint_u64(bytes, pos)?,
    })
}

/// 📄️ Produces the exact pre-admitted page owner consumed by reactor command batches.
pub async fn encode_app_command(command: &AppCommand) -> Result<PagedCommand, semio_framework_diagnostic::Fault> {
    if let AppCommand::Presence { own_color, peers, .. } = command {
        let mut pages = CommandPageSet::try_new(peers.len().max(1))?;
        for peer in peers.iter() {
            pages.try_push(FixedCommandPage::try_copy_from(peer)?).map_err(|(fault, _page)| fault)?;
        }
        if pages.is_empty() {
            pages.try_push(FixedCommandPage::try_copy_from(&[])?).map_err(|(fault, _page)| fault)?;
        }
        return PagedCommand::try_from_presence_pages(*own_color, pages, peers.len()).map_err(|(fault, _pages)| fault);
    }
    let mut out = CommandPageWriter::try_new()?;
    match command {
        AppCommand::ConfigCommand { seq, command } => {
            out.byte(0)?;
            out.varint(*seq)?;
            out.bytes(command)?;
        }
        AppCommand::Command { seq, command, view_state } => {
            out.byte(1)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(view_state)?;
        }
        AppCommand::CommandText { seq, line } => {
            out.byte(2)?;
            out.varint(*seq)?;
            out.string(line)?;
        }
        AppCommand::ContextMenu { seq, request } => {
            out.byte(3)?;
            out.varint(*seq)?;
            out.bytes(request)?;
        }
        AppCommand::ArtifactCommand { seq, command } => {
            out.byte(4)?;
            out.varint(*seq)?;
            out.bytes(command)?;
        }
        AppCommand::ApplyEnvelopes { seq, envelopes } => {
            out.byte(5)?;
            out.varint(*seq)?;
            out.varint(envelopes.len() as u64)?;
            for envelope in envelopes {
                out.envelope(envelope)?;
            }
        }
        AppCommand::ReadDocument { seq } => {
            out.byte(7)?;
            out.varint(*seq)?;
        }
        AppCommand::LoadConfig { seq, pack, spr } => {
            out.byte(8)?;
            out.varint(*seq)?;
            out.bytes(pack)?;
            out.bytes(spr)?;
        }
        AppCommand::ReadConfig { seq } => {
            out.byte(9)?;
            out.varint(*seq)?;
        }
        AppCommand::LoadWindowConfig { seq, entry } => {
            out.byte(30)?;
            out.varint(*seq)?;
            out.string(&entry.window_id)?;
            out.string(&entry.window_kind_id)?;
            out.bytes(&entry.envelope_pack)?;
        }
        AppCommand::ReadWindowConfigs { seq } => {
            out.byte(31)?;
            out.varint(*seq)?;
        }
        AppCommand::MediaIn { seq, port, descriptor, data } => {
            out.byte(10)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.bytes(descriptor)?;
            out.bytes(data)?;
        }
        AppCommand::MediaOut { seq, port, request } => {
            out.byte(11)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.bytes(request)?;
        }
        AppCommand::MediaFingerprint { seq, port } => {
            out.byte(12)?;
            out.varint(*seq)?;
            out.string(port)?;
        }
        AppCommand::PureCommand { seq, command, head } => {
            out.byte(13)?;
            out.varint(*seq)?;
            out.bytes(command)?;
            out.bytes(head)?;
        }
        AppCommand::LoadChildren { seq, entries } => {
            out.byte(14)?;
            out.varint(*seq)?;
            out.varint(entries.len() as u64)?;
            for entry in entries {
                out.string(&entry.slot)?;
                out.string(&entry.child_id)?;
                out.string(&entry.dialect)?;
                out.bytes(&entry.envelope_pack)?;
                out.string(&entry.owner)?;
            }
        }
        AppCommand::ReadChildren { seq } => {
            out.byte(15)?;
            out.varint(*seq)?;
        }
        AppCommand::ReadHistory { seq } => {
            out.byte(16)?;
            out.varint(*seq)?;
        }
        AppCommand::TransactionPrepare { seq, txn_id, mutation_id, payload, prepared_ops, origin, prepared_child_ops } => {
            if prepared_ops.len() > TRANSACTION_PREPARED_OPS_MAXIMUM {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.transaction-prepared-ops"), "transaction prepare exceeds its fixed 1024 prepared-op authority"));
            }
            out.byte(17)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
            out.string(mutation_id)?;
            out.bytes(payload)?;
            out.varint(prepared_ops.len() as u64)?;
            for op in prepared_ops {
                out.bytes(op)?;
            }
            out.bytes(origin)?;
            out.bytes(prepared_child_ops)?;
        }
        AppCommand::TransactionCommit { seq, txn_id } => {
            out.byte(18)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
        }
        AppCommand::TransactionRollback { seq, txn_id } => {
            out.byte(19)?;
            out.varint(*seq)?;
            out.string(txn_id)?;
        }
        AppCommand::TransactionUndo { seq, group_id } => {
            out.byte(20)?;
            out.varint(*seq)?;
            out.string(group_id)?;
        }
        AppCommand::TransactionRedo { seq, group_id } => {
            out.byte(21)?;
            out.varint(*seq)?;
            out.string(group_id)?;
        }
        AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => {
            out.byte(22)?;
            out.varint(*seq)?;
            out.string(artifact_ref)?;
            out.byte(*role)?;
            out.string(plugin_id)?;
            out.string(app_id)?;
        }
        AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => {
            out.byte(23)?;
            out.varint(*seq)?;
            out.string(artifact_kind)?;
            out.string(standard)?;
            out.string(subset)?;
            out.byte(*role)?;
            out.string(plugin_id)?;
            out.string(app_id)?;
        }
        AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => {
            out.byte(24)?;
            out.varint(*seq)?;
            out.string(artifact_kind)?;
            out.string(standard)?;
            out.string(subset)?;
            out.byte(*role)?;
        }
        AppCommand::SetMergePolicy { seq, policy } => {
            out.byte(25)?;
            out.varint(*seq)?;
            out.byte(*policy)?;
        }
        AppCommand::ResolveConflict { seq, conflict_id, resolution } => {
            out.byte(26)?;
            out.varint(*seq)?;
            out.string(conflict_id)?;
            out.byte(*resolution)?;
        }
        AppCommand::ReadConflicts { seq } => {
            out.byte(27)?;
            out.varint(*seq)?;
        }
        AppCommand::LocalInteractionQuery { seq, command } => {
            out.byte(29)?;
            out.varint(*seq)?;
            out.bytes(&protocol::encode_local_interaction_query_command(command))?;
        }
        AppCommand::LoadDocumentArchive { seq, archive } | AppCommand::MergeDocumentArchive { seq, archive } => {
            if archive.members.len() > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.document-archive-members"), "document archive exceeds its fixed 1024-member authority"));
            }
            out.byte(if matches!(command, AppCommand::MergeDocumentArchive { .. }) { 43 } else { 32 })?;
            out.varint(*seq)?;
            out.bytes(&archive.parent_pack)?;
            out.bytes(&archive.parent_spr)?;
            out.varint(archive.members.len() as u64)?;
            for entry in &archive.members {
                out.varint(u64::from(entry.ordinal))?;
                out.string(&entry.reference.artifact_id)?;
                out.string(&entry.reference.artifact_kind)?;
                out.string(&entry.reference.standard)?;
                out.string(&entry.reference.subset)?;
                out.string(&entry.owner.parent.artifact_id)?;
                out.string(&entry.owner.parent.artifact_kind)?;
                out.string(&entry.owner.parent.standard)?;
                out.string(&entry.owner.parent.subset)?;
                out.string(&entry.owner.slot)?;
                out.string(&entry.owner.child_id)?;
                out.bytes(&entry.envelope_pack)?;
            }
        }
        AppCommand::ReadDocumentArchive { seq } => {
            out.byte(33)?;
            out.varint(*seq)?;
        }
        AppCommand::PollDocumentArchiveLoad { seq, operation } => {
            out.byte(34)?;
            out.varint(*seq)?;
            out.varint(*operation)?;
        }
        AppCommand::CancelDocumentArchiveLoad { seq, operation } => {
            out.byte(35)?;
            out.varint(*seq)?;
            out.varint(*operation)?;
        }
        AppCommand::AcknowledgeDocumentArchiveLoad { seq, operation } => {
            out.byte(36)?;
            out.varint(*seq)?;
            out.varint(*operation)?;
        }
        AppCommand::SubmitMediaExport { seq, port, expected_parent_document_id, expected_base_revision } => {
            out.byte(37)?;
            out.varint(*seq)?;
            out.string(port)?;
            out.string(expected_parent_document_id)?;
            out.varint(*expected_base_revision)?;
        }
        AppCommand::PollMediaExport { seq, handle } | AppCommand::CancelMediaExport { seq, handle } | AppCommand::TakeMediaExportChunk { seq, handle } => {
            let tag = match command {
                AppCommand::PollMediaExport { .. } => 38,
                AppCommand::CancelMediaExport { .. } => 39,
                AppCommand::TakeMediaExportChunk { .. } => 40,
                _ => unreachable!(),
            };
            out.byte(tag)?;
            out.varint(*seq)?;
            write_media_export_handle_paged(&mut out, handle)?;
        }
        AppCommand::ReadDocumentIdentity { seq } => {
            out.byte(41)?;
            out.varint(*seq)?;
        }
        AppCommand::ReadChildHeads { seq } => {
            out.byte(42)?;
            out.varint(*seq)?;
        }
        AppCommand::Presence { .. } => unreachable!(),
    }
    out.finish()
}

/// 🧸️ `count varint | (slot, child_id, dialect, envelope_pack, owner)*` — the shared list codec for
/// both `AppCommand::LoadChildren` and `AppFrame::Children`.
async fn write_vec_child_pack(out: &mut Vec<u8>, entries: &[ChildPackEntry]) {
    protocol::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        protocol::write_str(out, &entry.slot);
        protocol::write_str(out, &entry.child_id);
        protocol::write_str(out, &entry.dialect);
        protocol::write_bytes(out, &entry.envelope_pack);
        protocol::write_str(out, &entry.owner);
    }
}

/// 🪆️ `count varint | (slot, child_id, dialect, head_pack, owner)*` — `AppFrame::ChildHeads`' list codec.
fn write_vec_child_head(out: &mut Vec<u8>, entries: &[ChildHeadPackEntry]) {
    protocol::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        protocol::write_str(out, &entry.slot);
        protocol::write_str(out, &entry.child_id);
        protocol::write_str(out, &entry.dialect);
        protocol::write_bytes(out, &entry.head_pack);
        protocol::write_str(out, &entry.owner);
    }
}

/// 🪆️ Inverse of [`write_vec_child_head`]; the declared count never reserves past the archive's member authority.
fn read_vec_child_head(bytes: &[u8], pos: &mut usize) -> Result<Vec<ChildHeadPackEntry>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    if count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS as u64 {
        return Err(malformed("channel child heads", *pos as u64, "child head count exceeds the member authority"));
    }
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(ChildHeadPackEntry { slot: protocol::read_str(bytes, pos)?, child_id: protocol::read_str(bytes, pos)?, dialect: protocol::read_str(bytes, pos)?, head_pack: protocol::read_bytes(bytes, pos)?, owner: protocol::read_str(bytes, pos)? });
    }
    Ok(entries)
}

/// 🧸️ Inverse of [`write_vec_child_pack`].
async fn read_vec_child_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<ChildPackEntry>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(ChildPackEntry { slot: protocol::read_str(bytes, pos)?, child_id: protocol::read_str(bytes, pos)?, dialect: protocol::read_str(bytes, pos)?, envelope_pack: protocol::read_bytes(bytes, pos)?, owner: protocol::read_str(bytes, pos)? });
    }
    Ok(entries)
}

fn write_document_archive(out: &mut Vec<u8>, archive: &DocumentArchivePack) {
    protocol::write_bytes(out, &archive.parent_pack);
    protocol::write_bytes(out, &archive.parent_spr);
    protocol::write_varint_u64(out, archive.members.len() as u64);
    for entry in &archive.members {
        protocol::write_varint_u64(out, u64::from(entry.ordinal));
        protocol::write_str(out, &entry.reference.artifact_id);
        protocol::write_str(out, &entry.reference.artifact_kind);
        protocol::write_str(out, &entry.reference.standard);
        protocol::write_str(out, &entry.reference.subset);
        protocol::write_str(out, &entry.owner.parent.artifact_id);
        protocol::write_str(out, &entry.owner.parent.artifact_kind);
        protocol::write_str(out, &entry.owner.parent.standard);
        protocol::write_str(out, &entry.owner.parent.subset);
        protocol::write_str(out, &entry.owner.slot);
        protocol::write_str(out, &entry.owner.child_id);
        protocol::write_bytes(out, &entry.envelope_pack);
    }
}

/// 🗃️ Encodes one exact persisted recursive-document archive with an explicit version owner.
pub fn encode_document_archive_bytes(archive: &DocumentArchivePack) -> Result<Vec<u8>, protocol::ProtocolError> {
    if archive.members.len() > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
        return Err(malformed("document archive member count", 0, "member count exceeds 1024"));
    }
    let mut bytes = vec![DOCUMENT_ARCHIVE_VERSION];
    write_document_archive(&mut bytes, archive);
    if bytes.len() > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
        return Err(malformed("document archive bytes", 0, "archive exceeds its fixed byte authority"));
    }
    Ok(bytes)
}

pub(super) fn document_archive_into_fields(mut archive: DocumentArchivePack) -> Vec<Vec<u8>> {
    let mut fields = Vec::with_capacity(archive.members.len().saturating_mul(11).saturating_add(2));
    fields.push(std::mem::take(&mut archive.parent_pack));
    fields.push(std::mem::take(&mut archive.parent_spr));
    for mut entry in archive.members.drain(..) {
        fields.push(std::mem::take(&mut entry.reference.artifact_id).into_bytes());
        fields.push(std::mem::take(&mut entry.reference.artifact_kind).into_bytes());
        fields.push(std::mem::take(&mut entry.reference.standard).into_bytes());
        fields.push(std::mem::take(&mut entry.reference.subset).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.artifact_id).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.artifact_kind).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.standard).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.parent.subset).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.slot).into_bytes());
        fields.push(std::mem::take(&mut entry.owner.child_id).into_bytes());
        fields.push(std::mem::take(&mut entry.envelope_pack));
    }
    fields
}

async fn read_document_archive(bytes: &[u8], pos: &mut usize) -> Result<DocumentArchivePack, protocol::ProtocolError> {
    let parent_pack = protocol::read_bytes(bytes, pos)?;
    let parent_spr = protocol::read_bytes(bytes, pos)?;
    let count = usize::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("document archive member count", *pos as u64, "member count is not representable"))?;
    if count > DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
        return Err(malformed("document archive member count", *pos as u64, "member count exceeds 1024"));
    }
    let mut members = Vec::new();
    members.try_reserve_exact(count).map_err(|_| malformed("document archive member count", *pos as u64, "member roster allocation failed"))?;
    for _ in 0..count {
        let ordinal = u32::try_from(protocol::read_varint_u64(bytes, pos)?).map_err(|_| malformed("document archive member ordinal", *pos as u64, "ordinal exceeds u32"))?;
        let reference = DocumentArchiveArtifactRef {
            artifact_id: protocol::read_str(bytes, pos)?,
            artifact_kind: protocol::read_str(bytes, pos)?,
            standard: protocol::read_str(bytes, pos)?,
            subset: protocol::read_str(bytes, pos)?,
        };
        let parent = DocumentArchiveArtifactRef {
            artifact_id: protocol::read_str(bytes, pos)?,
            artifact_kind: protocol::read_str(bytes, pos)?,
            standard: protocol::read_str(bytes, pos)?,
            subset: protocol::read_str(bytes, pos)?,
        };
        let owner = DocumentArchiveOwnerRef { parent, slot: protocol::read_str(bytes, pos)?, child_id: protocol::read_str(bytes, pos)? };
        let envelope_pack = protocol::read_bytes(bytes, pos)?;
        members.push(OwnedDocumentMemberPackEntry { ordinal, reference, owner, envelope_pack });
    }
    Ok(DocumentArchivePack { parent_pack, parent_spr, members })
}

/// 🗃️ Decodes one exact persisted recursive-document archive and rejects trailing ownership.
pub async fn decode_document_archive_bytes(bytes: &[u8]) -> Result<DocumentArchivePack, protocol::ProtocolError> {
    if bytes.is_empty() || bytes.len() > DOCUMENT_ARCHIVE_MAXIMUM_BYTES {
        return Err(malformed("document archive bytes", 0, "archive exceeds its fixed byte authority"));
    }
    if bytes[0] != DOCUMENT_ARCHIVE_VERSION {
        return Err(malformed("document archive version", 0, "unsupported or missing version"));
    }
    let mut position = 1;
    let archive = read_document_archive(bytes, &mut position).await?;
    if position != bytes.len() {
        return Err(malformed("document archive bytes", position as u64, "trailing bytes"));
    }
    Ok(archive)
}

/// 🪟️ Writes the exact concrete-window config pack roster.
async fn write_vec_window_config_pack(out: &mut Vec<u8>, entries: &[WindowConfigPackEntry]) {
    protocol::write_varint_u64(out, entries.len() as u64);
    for entry in entries {
        protocol::write_str(out, &entry.window_id);
        protocol::write_str(out, &entry.window_kind_id);
        protocol::write_bytes(out, &entry.envelope_pack);
    }
}

/// 🪟️ Reads the exact concrete-window config pack roster.
async fn read_vec_window_config_pack(bytes: &[u8], pos: &mut usize) -> Result<Vec<WindowConfigPackEntry>, protocol::ProtocolError> {
    let count = protocol::read_varint_u64(bytes, pos)?;
    let mut entries = Vec::with_capacity(count as usize);
    for _ in 0..count {
        entries.push(WindowConfigPackEntry {
            window_id: protocol::read_str(bytes, pos)?,
            window_kind_id: protocol::read_str(bytes, pos)?,
            envelope_pack: protocol::read_bytes(bytes, pos)?,
        });
    }
    Ok(entries)
}

/// 📥️ Decodes one `AppCommand`, the inverse of [`encode_app_command`].
pub async fn decode_app_command(bytes: &[u8]) -> Result<AppCommand, protocol::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-command tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let command = match tag {
        0 => AppCommand::ConfigCommand { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)? },
        1 => AppCommand::Command { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)?, view_state: protocol::read_bytes(bytes, &mut pos)? },
        2 => AppCommand::CommandText { seq: protocol::read_varint_u64(bytes, &mut pos)?, line: protocol::read_str(bytes, &mut pos)? },
        3 => AppCommand::ContextMenu { seq: protocol::read_varint_u64(bytes, &mut pos)?, request: protocol::read_bytes(bytes, &mut pos)? },
        4 => AppCommand::ArtifactCommand { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)? },
        5 => AppCommand::ApplyEnvelopes { seq: protocol::read_varint_u64(bytes, &mut pos)?, envelopes: read_vec_envelope(bytes, &mut pos).await? },
        7 => AppCommand::ReadDocument { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        8 => AppCommand::LoadConfig { seq: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)? },
        9 => AppCommand::ReadConfig { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        10 => AppCommand::MediaIn { seq: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, descriptor: protocol::read_bytes(bytes, &mut pos)?, data: protocol::read_bytes(bytes, &mut pos)? },
        11 => AppCommand::MediaOut { seq: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, request: protocol::read_bytes(bytes, &mut pos)? },
        12 => AppCommand::MediaFingerprint { seq: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)? },
        13 => AppCommand::PureCommand { seq: protocol::read_varint_u64(bytes, &mut pos)?, command: protocol::read_bytes(bytes, &mut pos)?, head: protocol::read_bytes(bytes, &mut pos)? },
        14 => AppCommand::LoadChildren { seq: protocol::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos).await? },
        15 => AppCommand::ReadChildren { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        16 => AppCommand::ReadHistory { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        17 => AppCommand::TransactionPrepare {
            seq: protocol::read_varint_u64(bytes, &mut pos)?,
            txn_id: protocol::read_str(bytes, &mut pos)?,
            mutation_id: protocol::read_str(bytes, &mut pos)?,
            payload: protocol::read_bytes(bytes, &mut pos)?,
            prepared_ops: read_vec_bytes(bytes, &mut pos).await?,
            origin: protocol::read_bytes(bytes, &mut pos)?,
            prepared_child_ops: protocol::read_bytes(bytes, &mut pos)?,
        },
        18 => AppCommand::TransactionCommit { seq: protocol::read_varint_u64(bytes, &mut pos)?, txn_id: protocol::read_str(bytes, &mut pos)? },
        19 => AppCommand::TransactionRollback { seq: protocol::read_varint_u64(bytes, &mut pos)?, txn_id: protocol::read_str(bytes, &mut pos)? },
        20 => AppCommand::TransactionUndo { seq: protocol::read_varint_u64(bytes, &mut pos)?, group_id: protocol::read_str(bytes, &mut pos)? },
        21 => AppCommand::TransactionRedo { seq: protocol::read_varint_u64(bytes, &mut pos)?, group_id: protocol::read_str(bytes, &mut pos)? },
        22 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let artifact_ref = protocol::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command OpenArtifact.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = protocol::read_str(bytes, &mut pos)?;
            let app_id = protocol::read_str(bytes, &mut pos)?;
            AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id }
        }
        23 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = protocol::read_str(bytes, &mut pos)?;
            let standard = protocol::read_str(bytes, &mut pos)?;
            let subset = protocol::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetDefaultApp.role", pos as u64, "truncated"))?;
            pos += 1;
            let plugin_id = protocol::read_str(bytes, &mut pos)?;
            let app_id = protocol::read_str(bytes, &mut pos)?;
            AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id }
        }
        24 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let artifact_kind = protocol::read_str(bytes, &mut pos)?;
            let standard = protocol::read_str(bytes, &mut pos)?;
            let subset = protocol::read_str(bytes, &mut pos)?;
            let role = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ClearDefaultApp.role", pos as u64, "truncated"))?;
            AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role }
        }
        25 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let policy = *bytes.get(pos).ok_or_else(|| malformed("channel app-command SetMergePolicy.policy", pos as u64, "truncated"))?;
            AppCommand::SetMergePolicy { seq, policy }
        }
        26 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let conflict_id = protocol::read_str(bytes, &mut pos)?;
            let resolution = *bytes.get(pos).ok_or_else(|| malformed("channel app-command ResolveConflict.resolution", pos as u64, "truncated"))?;
            AppCommand::ResolveConflict { seq, conflict_id, resolution }
        }
        27 => AppCommand::ReadConflicts { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        28 => return Err(malformed("channel presence command", pos as u64, "Presence requires reserve-before-decode PresenceCommandCursor admission")),
        29 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let length = protocol::read_varint_u64(bytes, &mut pos)?;
            if length > 142 || length as usize != bytes.len().saturating_sub(pos) {
                return Err(malformed("local interaction command", pos as u64, "invalid exact bounded envelope"));
            }
            let command = protocol::decode_local_interaction_query_command(&bytes[pos..]).map_err(|reason| malformed("local interaction command", pos as u64, reason))?;
            AppCommand::LocalInteractionQuery { seq, command }
        }
        30 => AppCommand::LoadWindowConfig {
            seq: protocol::read_varint_u64(bytes, &mut pos)?,
            entry: WindowConfigPackEntry {
                window_id: protocol::read_str(bytes, &mut pos)?,
                window_kind_id: protocol::read_str(bytes, &mut pos)?,
                envelope_pack: protocol::read_bytes(bytes, &mut pos)?,
            },
        },
        31 => AppCommand::ReadWindowConfigs { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        32 => AppCommand::LoadDocumentArchive { seq: protocol::read_varint_u64(bytes, &mut pos)?, archive: read_document_archive(bytes, &mut pos).await? },
        33 => AppCommand::ReadDocumentArchive { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        34 => AppCommand::PollDocumentArchiveLoad { seq: protocol::read_varint_u64(bytes, &mut pos)?, operation: protocol::read_varint_u64(bytes, &mut pos)? },
        35 => AppCommand::CancelDocumentArchiveLoad { seq: protocol::read_varint_u64(bytes, &mut pos)?, operation: protocol::read_varint_u64(bytes, &mut pos)? },
        36 => AppCommand::AcknowledgeDocumentArchiveLoad { seq: protocol::read_varint_u64(bytes, &mut pos)?, operation: protocol::read_varint_u64(bytes, &mut pos)? },
        37 => AppCommand::SubmitMediaExport {
            seq: protocol::read_varint_u64(bytes, &mut pos)?,
            port: protocol::read_str(bytes, &mut pos)?,
            expected_parent_document_id: protocol::read_str(bytes, &mut pos)?,
            expected_base_revision: protocol::read_varint_u64(bytes, &mut pos)?,
        },
        38..=40 => {
            let seq = protocol::read_varint_u64(bytes, &mut pos)?;
            let handle = read_media_export_handle(bytes, &mut pos)?;
            match tag {
                38 => AppCommand::PollMediaExport { seq, handle },
                39 => AppCommand::CancelMediaExport { seq, handle },
                40 => AppCommand::TakeMediaExportChunk { seq, handle },
                _ => unreachable!(),
            }
        }
        41 => AppCommand::ReadDocumentIdentity { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        42 => AppCommand::ReadChildHeads { seq: protocol::read_varint_u64(bytes, &mut pos)? },
        43 => AppCommand::MergeDocumentArchive { seq: protocol::read_varint_u64(bytes, &mut pos)?, archive: read_document_archive(bytes, &mut pos).await? },
        other => return Err(malformed("channel app-command tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    Ok(command)
}

/// 🪪 Reads only the fixed Presence tag and sequence so the app can reserve its exact
/// item/byte owner before the roster decoder allocates or copies any entry.
pub fn presence_command_sequence(bytes: &[u8]) -> Result<Option<u64>, protocol::ProtocolError> {
    let Some(tag) = bytes.first().copied() else {
        return Err(malformed("channel app-command tag", 0, "empty frame"));
    };
    if tag != 28 {
        return Ok(None);
    }
    let mut pos = 1usize;
    Ok(Some(protocol::read_varint_u64(bytes, &mut pos)?))
}

/// 📤️ Encodes one `AppFrame`: `tag u8 | fields`.
pub async fn encode_app_frame(frame: &AppFrame) -> Vec<u8> {
    let mut out = Vec::new();
    match frame {
        AppFrame::LocalInteractionQuery { reply } => {
            out.reserve_exact(4260);
            encode_local_interaction_query_frame_into(reply, &mut out).expect("typed local query frame fits admitted wire extent");
        }
        AppFrame::Done { in_reply_to } => {
            out.push(0);
            protocol::write_varint_u64(&mut out, *in_reply_to);
        }
        AppFrame::Invocation { in_reply_to, output, diagnostics, ui_scope, history_patch, messages, mutations, inverse_group } => {
            assert!(mutations.len() <= INVOCATION_RESULT_PACK_MAXIMUM_BYTES && inverse_group.len() <= INVOCATION_RESULT_PACK_MAXIMUM_BYTES, "invocation result pack exceeds command transport authority");
            out.push(1);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, output);
            protocol::write_bytes(&mut out, diagnostics);
            protocol::write_bytes(&mut out, ui_scope);
            protocol::write_bytes(&mut out, history_patch);
            protocol::write_bytes(&mut out, messages);
            protocol::write_bytes(&mut out, mutations);
            protocol::write_bytes(&mut out, inverse_group);
        }
        AppFrame::DocumentChanged { envelopes, origin } => {
            out.push(2);
            write_vec_envelope(&mut out, envelopes).await;
            protocol::write_str(&mut out, origin);
        }
        AppFrame::Document { in_reply_to, pack, spr, ops } => {
            out.push(3);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, pack);
            protocol::write_bytes(&mut out, spr);
            protocol::write_str(&mut out, ops);
        }
        AppFrame::Config { in_reply_to, pack, spr, ops } => {
            out.push(4);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, pack);
            protocol::write_bytes(&mut out, spr);
            protocol::write_str(&mut out, ops);
        }
        AppFrame::WindowConfigs { in_reply_to, entries } => {
            out.push(24);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_vec_window_config_pack(&mut out, entries).await;
        }
        AppFrame::ConfigChanged { envelopes, origin } => {
            out.push(5);
            write_vec_envelope(&mut out, envelopes).await;
            protocol::write_str(&mut out, origin);
        }
        AppFrame::ContextMenu { in_reply_to, items } => {
            out.push(6);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, items);
        }
        AppFrame::Media { in_reply_to, port, descriptor, data } => {
            out.push(7);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_str(&mut out, port);
            protocol::write_bytes(&mut out, descriptor);
            protocol::write_bytes(&mut out, data);
        }
        AppFrame::MediaFingerprint { in_reply_to, port, fingerprint } => {
            out.push(8);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_str(&mut out, port);
            protocol::write_bytes(&mut out, fingerprint);
        }
        AppFrame::Error { in_reply_to, fault, report } => {
            out.push(9);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_bytes(&mut out, fault);
            protocol::write_bytes(&mut out, report);
        }
        AppFrame::Emit { in_reply_to, document_ops, config_ops, draft_ops, output, diagnostics, child_ops } => {
            out.push(10);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, document_ops);
            protocol::write_bytes(&mut out, config_ops);
            protocol::write_bytes(&mut out, draft_ops);
            protocol::write_bytes(&mut out, output);
            protocol::write_bytes(&mut out, diagnostics);
            protocol::write_bytes(&mut out, child_ops);
        }
        AppFrame::Draft { in_reply_to, pack, spr, ops } => {
            out.push(11);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, pack);
            protocol::write_bytes(&mut out, spr);
            protocol::write_str(&mut out, ops);
        }
        AppFrame::Children { in_reply_to, entries } => {
            out.push(12);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_pack(&mut out, entries).await;
        }
        AppFrame::Ephemeral { presence, presence_generation, transient_generation, interaction, tool_run, history_edit } => {
            out.push(13);
            protocol::write_bytes(&mut out, presence);
            protocol::write_varint_u64(&mut out, *presence_generation);
            protocol::write_varint_u64(&mut out, *transient_generation);
            protocol::write_bytes(&mut out, interaction);
            protocol::write_bytes(&mut out, tool_run);
            protocol::write_bytes(&mut out, history_edit);
        }
        AppFrame::HistorySnapshot { in_reply_to, history_patch } => {
            out.push(14);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_bytes(&mut out, history_patch);
        }
        AppFrame::TransactionProposal { in_reply_to, proposal_id, local_ops, foreign } => {
            out.push(15);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_str(&mut out, proposal_id);
            write_vec_bytes(&mut out, local_ops).await;
            write_vec_bytes(&mut out, foreign).await;
        }
        AppFrame::TransactionPrepared { txn_id, foreign, rejection } => {
            out.push(16);
            protocol::write_str(&mut out, txn_id);
            write_vec_bytes(&mut out, foreign).await;
            protocol::write_bytes(&mut out, rejection);
        }
        AppFrame::TransactionCommitted { txn_id, edit_id } => {
            out.push(17);
            protocol::write_str(&mut out, txn_id);
            protocol::write_str(&mut out, edit_id);
        }
        AppFrame::TransactionRolledBack { txn_id } => {
            out.push(18);
            protocol::write_str(&mut out, txn_id);
        }
        AppFrame::MergeReport { in_reply_to, report } => {
            out.push(19);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_bytes(&mut out, report);
        }
        AppFrame::Conflicts { in_reply_to, conflicts } => {
            out.push(20);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_bytes(&mut out, conflicts);
        }
        AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops } => {
            out.push(21);
            write_opt_u64(&mut out, in_reply_to).await;
            protocol::write_str(&mut out, surface);
            protocol::write_str(&mut out, kind);
            protocol::write_varint_u64(&mut out, *revision);
            protocol::write_varint_u64(&mut out, *base_revision);
            protocol::write_bytes(&mut out, ops);
        }
        AppFrame::UiSnapshotEnd { revision } => {
            out.push(22);
            protocol::write_varint_u64(&mut out, *revision);
        }
        AppFrame::OperationCompleted { operation, revision, ui_scope, history_patch } => {
            out.push(25);
            protocol::write_varint_u64(&mut out, *operation);
            protocol::write_varint_u64(&mut out, *revision);
            protocol::write_bytes(&mut out, ui_scope);
            protocol::write_bytes(&mut out, history_patch);
        }
        AppFrame::DocumentArchive { in_reply_to, archive } => {
            assert!(archive.members.len() <= DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS, "document archive exceeds its fixed 1024-member authority");
            out.push(26);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_document_archive(&mut out, archive);
        }
        AppFrame::DocumentArchiveLoad { in_reply_to, status } => {
            out.push(27);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_varint_u64(&mut out, status.operation);
            out.push(status.state.wire());
            protocol::write_varint_u64(&mut out, status.completed);
            protocol::write_varint_u64(&mut out, status.total);
            protocol::write_varint_u64(&mut out, status.ahead);
            protocol::write_bytes(&mut out, &status.fault);
        }
        AppFrame::MediaExportSubmitted { in_reply_to, handle } => {
            out.push(28);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_media_export_handle(&mut out, handle);
        }
        AppFrame::MediaExportStatus { in_reply_to, handle, state, applied_progress, checkpoint_available, mime_type, total_bytes, detail } => {
            out.push(29);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_media_export_handle(&mut out, handle);
            out.push(state.wire());
            protocol::write_varint_u64(&mut out, *applied_progress);
            protocol::write_bool(&mut out, *checkpoint_available);
            protocol::write_str(&mut out, mime_type);
            protocol::write_varint_u64(&mut out, *total_bytes);
            protocol::write_str(&mut out, detail);
        }
        AppFrame::MediaExportChunk { in_reply_to, handle, data, terminal } => {
            out.push(30);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_media_export_handle(&mut out, handle);
            protocol::write_bytes(&mut out, data);
            protocol::write_bool(&mut out, *terminal);
        }
        AppFrame::DocumentIdentity { in_reply_to, identity } => {
            out.push(31);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            protocol::write_varint_u64(&mut out, u64::from(identity.app_instance_id));
            protocol::write_bool(&mut out, identity.parent_document_id.is_some());
            if let Some(id) = &identity.parent_document_id {
                protocol::write_str(&mut out, id);
            }
        }
        AppFrame::ChildHeads { in_reply_to, entries } => {
            out.push(32);
            protocol::write_varint_u64(&mut out, *in_reply_to);
            write_vec_child_head(&mut out, entries);
        }
    }
    out
}

/// 📥️ Decodes one `AppFrame`, the inverse of [`encode_app_frame`].
pub async fn decode_app_frame(bytes: &[u8]) -> Result<AppFrame, protocol::ProtocolError> {
    let tag = *bytes.first().ok_or_else(|| malformed("channel app-frame tag", 0, "empty frame"))?;
    let mut pos = 1usize;
    let frame = match tag {
        0 => AppFrame::Done { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)? },
        1 => AppFrame::Invocation {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            output: protocol::read_bytes(bytes, &mut pos)?,
            diagnostics: protocol::read_bytes(bytes, &mut pos)?,
            ui_scope: protocol::read_bytes(bytes, &mut pos)?,
            history_patch: protocol::read_bytes(bytes, &mut pos)?,
            messages: protocol::read_bytes(bytes, &mut pos)?,
            mutations: read_invocation_result_pack(bytes, &mut pos)?,
            inverse_group: read_invocation_result_pack(bytes, &mut pos)?,
        },
        2 => AppFrame::DocumentChanged { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: protocol::read_str(bytes, &mut pos)? },
        3 => AppFrame::Document { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)?, ops: protocol::read_str(bytes, &mut pos)? },
        4 => AppFrame::Config { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)?, ops: protocol::read_str(bytes, &mut pos)? },
        5 => AppFrame::ConfigChanged { envelopes: read_vec_envelope(bytes, &mut pos).await?, origin: protocol::read_str(bytes, &mut pos)? },
        6 => AppFrame::ContextMenu { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, items: protocol::read_bytes(bytes, &mut pos)? },
        7 => {
            AppFrame::Media { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, descriptor: protocol::read_bytes(bytes, &mut pos)?, data: protocol::read_bytes(bytes, &mut pos)? }
        }
        8 => AppFrame::MediaFingerprint { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, port: protocol::read_str(bytes, &mut pos)?, fingerprint: protocol::read_bytes(bytes, &mut pos)? },
        9 => AppFrame::Error { in_reply_to: read_opt_u64(bytes, &mut pos).await?, fault: protocol::read_bytes(bytes, &mut pos)?, report: protocol::read_bytes(bytes, &mut pos)? },
        10 => AppFrame::Emit {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            document_ops: protocol::read_bytes(bytes, &mut pos)?,
            config_ops: protocol::read_bytes(bytes, &mut pos)?,
            draft_ops: protocol::read_bytes(bytes, &mut pos)?,
            output: protocol::read_bytes(bytes, &mut pos)?,
            diagnostics: protocol::read_bytes(bytes, &mut pos)?,
            child_ops: protocol::read_bytes(bytes, &mut pos)?,
        },
        11 => AppFrame::Draft { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, pack: protocol::read_bytes(bytes, &mut pos)?, spr: protocol::read_bytes(bytes, &mut pos)?, ops: protocol::read_str(bytes, &mut pos)? },
        12 => AppFrame::Children { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_pack(bytes, &mut pos).await? },
        13 => AppFrame::Ephemeral {
            presence: protocol::read_bytes(bytes, &mut pos)?,
            presence_generation: protocol::read_varint_u64(bytes, &mut pos)?,
            transient_generation: protocol::read_varint_u64(bytes, &mut pos)?,
            interaction: protocol::read_bytes(bytes, &mut pos)?,
            tool_run: protocol::read_bytes(bytes, &mut pos)?,
            history_edit: protocol::read_bytes(bytes, &mut pos)?,
        },
        14 => AppFrame::HistorySnapshot { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, history_patch: protocol::read_bytes(bytes, &mut pos)? },
        15 => AppFrame::TransactionProposal {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            proposal_id: protocol::read_str(bytes, &mut pos)?,
            local_ops: read_vec_bytes(bytes, &mut pos).await?,
            foreign: read_vec_bytes(bytes, &mut pos).await?,
        },
        16 => AppFrame::TransactionPrepared { txn_id: protocol::read_str(bytes, &mut pos)?, foreign: read_vec_bytes(bytes, &mut pos).await?, rejection: protocol::read_bytes(bytes, &mut pos)? },
        17 => AppFrame::TransactionCommitted { txn_id: protocol::read_str(bytes, &mut pos)?, edit_id: protocol::read_str(bytes, &mut pos)? },
        18 => AppFrame::TransactionRolledBack { txn_id: protocol::read_str(bytes, &mut pos)? },
        19 => AppFrame::MergeReport { in_reply_to: read_opt_u64(bytes, &mut pos).await?, report: protocol::read_bytes(bytes, &mut pos)? },
        20 => AppFrame::Conflicts { in_reply_to: read_opt_u64(bytes, &mut pos).await?, conflicts: protocol::read_bytes(bytes, &mut pos)? },
        21 => {
            let in_reply_to = read_opt_u64(bytes, &mut pos).await?;
            let surface = protocol::read_str(bytes, &mut pos)?;
            let kind = protocol::read_str(bytes, &mut pos)?;
            let revision = protocol::read_varint_u64(bytes, &mut pos)?;
            let base_revision = protocol::read_varint_u64(bytes, &mut pos)?;
            let ops = protocol::read_bytes(bytes, &mut pos)?;
            AppFrame::UiPatch { in_reply_to, surface, kind, revision, base_revision, ops }
        }
        22 => AppFrame::UiSnapshotEnd { revision: protocol::read_varint_u64(bytes, &mut pos)? },
        23 => {
            let length = protocol::read_varint_u64(bytes, &mut pos)?;
            if length > 4256 || length as usize != bytes.len().saturating_sub(pos) {
                return Err(malformed("local interaction reply", pos as u64, "invalid exact bounded envelope"));
            }
            let reply = protocol::decode_local_interaction_query_reply(&bytes[pos..]).map_err(|reason| malformed("local interaction reply", pos as u64, reason))?;
            pos = bytes.len();
            AppFrame::LocalInteractionQuery { reply }
        }
        24 => AppFrame::WindowConfigs {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            entries: read_vec_window_config_pack(bytes, &mut pos).await?,
        },
        25 => AppFrame::OperationCompleted {
            operation: protocol::read_varint_u64(bytes, &mut pos)?,
            revision: protocol::read_varint_u64(bytes, &mut pos)?,
            ui_scope: protocol::read_bytes(bytes, &mut pos)?,
            history_patch: protocol::read_bytes(bytes, &mut pos)?,
        },
        26 => AppFrame::DocumentArchive { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, archive: read_document_archive(bytes, &mut pos).await? },
        27 => {
            let in_reply_to = protocol::read_varint_u64(bytes, &mut pos)?;
            let operation = protocol::read_varint_u64(bytes, &mut pos)?;
            let state = DocumentArchiveLoadState::from_wire(*bytes.get(pos).ok_or_else(|| malformed("document archive load state", pos as u64, "truncated"))?)?;
            pos += 1;
            let completed = protocol::read_varint_u64(bytes, &mut pos)?;
            let total = protocol::read_varint_u64(bytes, &mut pos)?;
            let ahead = protocol::read_varint_u64(bytes, &mut pos)?;
            let fault = protocol::read_bytes(bytes, &mut pos)?;
            AppFrame::DocumentArchiveLoad { in_reply_to, status: DocumentArchiveLoadStatus { operation, state, completed, total, ahead, fault } }
        }
        28 => AppFrame::MediaExportSubmitted {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            handle: read_media_export_handle(bytes, &mut pos)?,
        },
        29 => {
            let in_reply_to = protocol::read_varint_u64(bytes, &mut pos)?;
            let handle = read_media_export_handle(bytes, &mut pos)?;
            let state = MediaExportStateWire::from_wire(*bytes.get(pos).ok_or_else(|| malformed("media export state", pos as u64, "truncated"))?)?;
            pos += 1;
            let applied_progress = protocol::read_varint_u64(bytes, &mut pos)?;
            let checkpoint_available = protocol::read_bool(bytes, &mut pos)?;
            let mime_type = protocol::read_str(bytes, &mut pos)?;
            let total_bytes = protocol::read_varint_u64(bytes, &mut pos)?;
            let detail = protocol::read_str(bytes, &mut pos)?;
            AppFrame::MediaExportStatus { in_reply_to, handle, state, applied_progress, checkpoint_available, mime_type, total_bytes, detail }
        }
        30 => AppFrame::MediaExportChunk {
            in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?,
            handle: read_media_export_handle(bytes, &mut pos)?,
            data: protocol::read_bytes(bytes, &mut pos)?,
            terminal: protocol::read_bool(bytes, &mut pos)?,
        },
        31 => {
            if bytes.len() > 2_070 {
                return Err(malformed("channel document identity", pos as u64, "identity frame byte budget exceeded"));
            }
            let in_reply_to = protocol::read_varint_u64(bytes, &mut pos)?;
            let app_instance_id = u32::try_from(protocol::read_varint_u64(bytes, &mut pos)?).map_err(|_| malformed("channel document identity", pos as u64, "app instance exceeds u32"))?;
            let presence = *bytes.get(pos).ok_or_else(|| malformed("channel document identity", pos as u64, "truncated document presence"))?;
            pos += 1;
            if presence > 1 {
                return Err(malformed("channel document identity", pos as u64, "document presence must be zero or one"));
            }
            let parent_document_id = if presence == 1 {
                let id = protocol::read_str(bytes, &mut pos)?;
                if !(1..=512).contains(&id.chars().count()) {
                    return Err(malformed("channel document identity", pos as u64, "document id length refused"));
                }
                Some(id)
            } else { None };
            AppFrame::DocumentIdentity { in_reply_to, identity: AppDocumentIdentity { app_instance_id, parent_document_id } }
        }
        32 => AppFrame::ChildHeads { in_reply_to: protocol::read_varint_u64(bytes, &mut pos)?, entries: read_vec_child_head(bytes, &mut pos)? },
        other => return Err(malformed("channel app-frame tag", pos as u64, &format!("unknown tag {other:#x}"))),
    };
    if pos != bytes.len() {
        return Err(malformed("channel app-frame", pos as u64, "trailing bytes after the frame"));
    }
    Ok(frame)
}
//#endregion 🔖️Codec

/// 📤️ Atomic bounded frame encoding into caller-admitted storage; failure never modifies output.
pub fn encode_local_interaction_query_frame_into(reply: &protocol::LocalInteractionQueryReply, out: &mut Vec<u8>) -> Result<(), &'static str> {
    let length = protocol::local_interaction_query_reply_encoded_len(reply)?;
    let prefix = 1 + ((usize::BITS - length.leading_zeros()).max(1) as usize).div_ceil(7);
    if out.capacity() - out.len() < prefix + length {
        return Err("local-interaction.frame-not-admitted");
    }
    out.push(23);
    protocol::write_varint_u64(out, length as u64);
    protocol::encode_local_interaction_query_reply_into(reply, out)
}
