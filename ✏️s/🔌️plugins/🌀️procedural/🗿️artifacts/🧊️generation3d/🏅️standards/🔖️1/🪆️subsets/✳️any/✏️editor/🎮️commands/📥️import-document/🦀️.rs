//! 📥️ Generation3d play app commands command — `import-document`: the verb the shell re-dispatches
//! for every CHUNK of every file picked by `📂️import-document-request`.
//!
//! 🐛️ Until ticket 26/09/09/PROCEDURAL-3D-END-TO-END's io-surface lane the nine import leaves were
//! round-trip tested and unreachable from any command, menu or button
//! (`📓️audit-user-journey-gaps-2026-09-13.md` §6, P0 #1).
//!
//! 🧬️ Importing REPLACES the whole document, and it does so the way `🎨️set-active-example` does: as
//! an ordered batch of real `Generation3dMutation`s built by `generation3d_fixture_operations`, never
//! an `Effect::LoadDocument`. That keeps the import event-sourced and point-invertible — one `mod+z`
//! puts the previous graph back — where a whole-document replace effect would be a CRUD write with
//! no inverse.
//!
//! 📷️ The camera rides the CONFIG lane (`config_after_document_load`), exactly as an example switch
//! does, because `generation3d_fixture_operations` deliberately ignores it
//! (`mutations::tests::fixture_ops_ignore_camera`).
//!
//! ⏳️ Progress and cancellation: `dispatchOpenedFiles` (`🛠️ShellHelpers/🟦️.tsx`) sends one
//! invocation per [`importPayloadChunks`] chunk, IN ORDER, awaiting each — so a chunk that lands is
//! `Staged { next_chunk, chunk_count }`, which is a real "n of N" the surface can report and which
//! costs no document edit and no history row. Cancelling is dropping the run: an abandoned run holds
//! one slot until [`Generation3dImportStaging::retire_abandoned`] sweeps it, never unbounded memory,
//! and the next pick re-opens at chunk 0 rather than resuming into bytes nobody can account for.
//!
//! @see ../../../🚪️io/🦀️.rs — `document_io::import_document_bytes`, the composition point this calls.
//! @see ../🎨️set-active-example/🦀️.rs — the same whole-document replacement, from a bundled example.
//! @see ../../../../../../../🧩️puzzle/🗿️artifacts/🧊️3d/…/🎮️commands/📥️import-fixture/🦀️.rs — the
//!      repo's other chunked inbound lane, whose run/gap/capacity vocabulary this mirrors.

use crate::editor::generation3d::commands::set_active_example::config_after_document_load;
use crate::editor::generation3d::config::{Generation3dConfig, Generation3dConfigMutation};
use crate::standards::v1::subsets::any::io::document_io;
use crate::standards::v1::subsets::any::schema::mutations::text::{generation3d_fixture_operations, generation_mutation_to_generation3d, Generation3dMutation};
use crate::Generation3dSnapshot;
use semio_framework_artifact_playbook_playbook::GenerationMutation;
use semio_framework_os_flow::FlowEvalSession;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 📏️Bounds
/// 📏️ Bytes ONE inbound chunk may carry.
///
/// 🧊️ Derived from the bound that actually BINDS, never a literal: a chunk crosses as one `payload`
/// STRING inside one public invocation, and `validate_public_json_envelope`
/// (`🧰️framework/…/🔌️plugin/🦀️.rs:33383`) refuses any string above
/// `PUBLIC_INVOCATION_STRING_BYTES` **before** this tool's own contract is ever consulted — so no
/// tool contract can widen it and a wider chunk is rejected by the host, not by this module. The
/// payload a `read_as: "dataUrl"` pick carries is base64, whose alphabet costs exactly one wire byte
/// per character (`public_invocation_char_cost`), so the string cap IS the chunk extent with no
/// escape-factor headroom to reserve.
pub const GENERATION3D_IMPORT_CHUNK_BYTES: usize = semio_framework::PUBLIC_INVOCATION_STRING_BYTES;

/// 📏️ Largest payload ONE import may reassemble.
///
/// 🧾️ Derived, never a literal: an imported file is planted in the graph as an `InputNote`'s text
/// (`mesh_bridge::import_document`), so the whole reassembled payload has to fit ONE Artifact-lane
/// edit — `GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES`. A run allowed to grow past it would stage
/// megabytes only to be refused by the store at the very last chunk.
pub const GENERATION3D_IMPORT_TOTAL_BYTES: usize = crate::editor::generation3d::GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES;

/// 📦️ Chunks one import may stage — the whole budget at the chunk extent, never a second literal.
pub const GENERATION3D_IMPORT_MAXIMUM_CHUNKS: usize = GENERATION3D_IMPORT_TOTAL_BYTES.div_ceil(GENERATION3D_IMPORT_CHUNK_BYTES);

/// 📦️ Import runs one app instance stages at once. One open run per file a user is picking into this
/// document; an abandoned run costs one slot until the next sweep.
pub const GENERATION3D_IMPORT_SLOTS: usize = 2;
//#endregion 📏️Bounds

//#region 🧵️Staging
/// 🚫️ Why one chunk was refused. Every arm carries a stable code and becomes a typed `Fault` — never
/// a silent no-op, which is exactly what an unreachable import already was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Generation3dImportFault {
    Envelope,
    Chunk,
    Gap,
    Capacity,
}

impl Generation3dImportFault {
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "generation3d-import-envelope",
            Self::Chunk => "generation3d-import-chunk",
            Self::Gap => "generation3d-import-gap",
            Self::Capacity => "generation3d-import-capacity",
        }
    }

    pub fn message(self) -> &'static str {
        match self {
            Self::Envelope => "import chunk envelope is out of range",
            Self::Chunk => "import chunk is larger than one public invocation string",
            Self::Gap => "import chunk does not continue its staged run",
            Self::Capacity => "import run exceeds one artifact-lane edit",
        }
    }
}

/// 🧱️ What one accepted chunk did to its run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Generation3dImportStep {
    /// 🧱️ The chunk landed and the run is still open — no document edit, no history row. `next_chunk`
    /// of `chunk_count` is the progress a surface reports.
    Staged { next_chunk: usize, chunk_count: usize },
    /// ✅️ The chunk closed the run; the pages are the whole payload in arrival order.
    Complete(Vec<String>),
}

/// 📥️ The chunk envelope one `importDocument` invocation carries. An invocation that declares none is
/// one whole-payload chunk — the only shape that can fit [`GENERATION3D_IMPORT_CHUNK_BYTES`] alone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Generation3dImportEnvelope {
    pub name: String,
    pub chunk: usize,
    pub chunk_count: usize,
}

struct Generation3dImportSlot {
    name: String,
    chunk_count: usize,
    next_chunk: usize,
    bytes: usize,
    pages: Vec<String>,
    touched: u64,
}

/// 🧵️ One app INSTANCE's open chunk runs. Instance-scoped, not process-global: two users importing
/// into two documents of the same component must never see each other's staged bytes, and an
/// instance that closes takes its runs with it.
#[derive(Default)]
pub struct Generation3dImportStaging {
    slots: Vec<Generation3dImportSlot>,
    sequence: u64,
    swept: u64,
}

impl Generation3dImportStaging {
    /// 🧩️ Admits one chunk, and hands back every page the moment the run closes. A run is keyed by
    /// `(name, chunk_count)`: a re-pick opens a different run instead of corrupting the staged one,
    /// and a chunk the run already admitted is a RETRANSMISSION acknowledged at the cursor it stands
    /// on rather than a gap that costs the whole upload again.
    pub fn admit(&mut self, envelope: &Generation3dImportEnvelope, text: &str) -> Result<Generation3dImportStep, Generation3dImportFault> {
        if envelope.chunk_count == 0 || envelope.chunk_count > GENERATION3D_IMPORT_MAXIMUM_CHUNKS || envelope.chunk >= envelope.chunk_count {
            return Err(Generation3dImportFault::Envelope);
        }
        if text.len() > GENERATION3D_IMPORT_CHUNK_BYTES {
            return Err(Generation3dImportFault::Chunk);
        }
        if envelope.chunk_count == 1 {
            return Ok(Generation3dImportStep::Complete(vec![text.to_string()]));
        }
        self.sequence = self.sequence.saturating_add(1);
        let sequence = self.sequence;
        let held = self.slots.iter().position(|slot| slot.name == envelope.name && slot.chunk_count == envelope.chunk_count);
        let index = match held {
            Some(index) if self.slots[index].next_chunk == envelope.chunk => index,
            Some(index) if envelope.chunk != 0 && envelope.chunk < self.slots[index].next_chunk => {
                let slot = &mut self.slots[index];
                slot.touched = sequence;
                return Ok(Generation3dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
            }
            Some(index) if envelope.chunk != 0 => {
                self.slots.swap_remove(index);
                return Err(Generation3dImportFault::Gap);
            }
            Some(index) => {
                self.slots[index] = Generation3dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence };
                index
            }
            None if envelope.chunk != 0 => return Err(Generation3dImportFault::Gap),
            None => {
                if self.slots.len() >= GENERATION3D_IMPORT_SLOTS {
                    let stale = self.slots.iter().enumerate().min_by_key(|(_, slot)| slot.touched).map(|(index, _)| index).ok_or(Generation3dImportFault::Capacity)?;
                    self.slots.swap_remove(stale);
                }
                self.slots.push(Generation3dImportSlot { name: envelope.name.clone(), chunk_count: envelope.chunk_count, next_chunk: 0, bytes: 0, pages: Vec::new(), touched: sequence });
                self.slots.len() - 1
            }
        };
        let slot = &mut self.slots[index];
        if slot.bytes.saturating_add(text.len()) > GENERATION3D_IMPORT_TOTAL_BYTES {
            self.slots.swap_remove(index);
            return Err(Generation3dImportFault::Capacity);
        }
        slot.bytes = slot.bytes.saturating_add(text.len());
        slot.pages.push(text.to_string());
        slot.next_chunk = envelope.chunk.saturating_add(1);
        slot.touched = sequence;
        if slot.next_chunk < slot.chunk_count {
            return Ok(Generation3dImportStep::Staged { next_chunk: slot.next_chunk, chunk_count: slot.chunk_count });
        }
        let closed = self.slots.swap_remove(index);
        Ok(Generation3dImportStep::Complete(closed.pages))
    }

    /// 🧹️ Drops every run that did not advance across a whole retirement cycle — the cancellation an
    /// abandoned pick gets for free.
    pub fn retire_abandoned(&mut self) {
        let watermark = self.swept;
        self.slots.retain(|slot| slot.touched > watermark);
        self.swept = self.sequence;
    }

    /// 🔎️ Open runs as `(name, next_chunk, chunk_count, bytes)` — the progress census the laws read.
    pub fn open_runs(&self) -> Vec<(String, usize, usize, usize)> {
        self.slots.iter().map(|slot| (slot.name.clone(), slot.next_chunk, slot.chunk_count, slot.bytes)).collect()
    }
}

/// 📥️ Slices a payload into the chunks the host sends, so the guest's own laws exercise the wire the
/// renderer builds rather than a shape only tests use. Mirrors `importPayloadChunks`
/// (`🛠️ShellHelpers/🟦️.tsx`), sliced by UTF-8 extent so no slice ever splits a code point.
pub fn generation3d_import_chunks(payload: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut page = String::new();
    for character in payload.chars() {
        if page.len().saturating_add(character.len_utf8()) > GENERATION3D_IMPORT_CHUNK_BYTES {
            chunks.push(std::mem::take(&mut page));
        }
        page.push(character);
    }
    if !page.is_empty() || chunks.is_empty() {
        chunks.push(page);
    }
    chunks
}
//#endregion 🧵️Staging

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "import-document")]
pub struct ImportDocument {
    pub name: String,
    pub payload: String,
    pub chunk: u32,
    pub chunk_count: u32,
}

impl ImportDocument {
    pub fn envelope(&self) -> Generation3dImportEnvelope {
        Generation3dImportEnvelope { name: self.name.clone(), chunk: self.chunk as usize, chunk_count: self.chunk_count.max(1) as usize }
    }
}

fn import_fault(code: &str, message: impl Into<String>) -> Fault {
    Fault::new(FaultOrigin::App, FaultCode::new(code), message.into())
}

/// 📥️ Admits one chunk. A run that is still open emits NOTHING — no document edit, no history row —
/// and the surface reads its progress off [`Generation3dImportStaging::open_runs`].
pub fn emit(
    payload: &ImportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    staging: &mut Generation3dImportStaging,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let envelope = payload.envelope();
    let pages = match staging.admit(&envelope, &payload.payload).map_err(|fault| import_fault(fault.code(), fault.message()))? {
        Generation3dImportStep::Staged { .. } => return Ok(Emit::default()),
        Generation3dImportStep::Complete(pages) => pages,
    };
    apply_complete_payload(&envelope.name, &pages.concat(), doc, cfg)
}

/// 📥️ Decodes one fully reassembled payload and replaces the document with what it holds.
///
/// 🚨️ Bytes that are not the format their name claims are a TYPED fault carrying why. They are never
/// an empty document reported as success — the exact failure mode ticket
/// 26/09/09/PROCEDURAL-3D-END-TO-END removed from the leaves themselves
/// (`📓️io-codecs-2026-09-09.md` §1), which a silent `Ok(Emit::default())` here would have
/// reintroduced one layer up.
pub fn apply_complete_payload(
    name: &str,
    payload: &str,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    let imported = document_io::import_document(name, payload).map_err(|error| import_fault("generation3d.io.import", error.to_string()))?;
    let mut operations: Vec<Generation3dMutation> = doc.snapshot.generation.generations.iter().map(|generation| generation_mutation_to_generation3d(GenerationMutation::Remove { id: generation.id.clone() })).collect();
    operations.extend(generation3d_fixture_operations(&doc.snapshot.fixture, &imported.fixture));
    let config = config_after_document_load(cfg.snapshot, &imported.fixture.camera);
    imported.retire_cold();
    Ok(Emit { artifact_mutations: operations, config_mutations: vec![Generation3dConfigMutation::SetSnapshot(crate::editor::generation3d::config::SetSnapshot { config })], ..Default::default() })
}

/// 🧵️ The session-free entry point: a dispatch that reaches this app without its retained instance
/// owner can still import a single-chunk pick, which is every file below one public invocation
/// string. A chunked run needs the instance's staging and says so instead of staging into a
/// throwaway nobody will ever hand the next chunk to.
pub fn handle(
    payload: &ImportDocument,
    doc: &ArtifactView<'_, Generation3dSnapshot>,
    cfg: &ConfigView<'_, Generation3dConfig>,
    _session: &mut FlowEvalSession,
) -> Result<Emit<Generation3dMutation, Generation3dConfigMutation>, Fault> {
    if payload.chunk_count > 1 {
        return Err(import_fault("generation3d-import-unstaged", "a chunked import needs this app instance's retained staging"));
    }
    let mut staging = Generation3dImportStaging::default();
    emit(payload, doc, cfg, &mut staging)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
