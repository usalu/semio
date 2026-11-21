/** 🧬 Extracts the live reconciliation selector for both conventional and incremental Save. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {join} from "node:path";

const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const exportPath = join(base, "🚪️io/📤️export/🧬️encode-and-reconcile/🦀️.rs");
const writerPath = join(base, "🔨️modules/📝️writer/🦀️.rs");
const output = join(import.meta.dir, "🗑️generated/classic-graph");
const quoteFile = "🧮️graph-quote.rs";
const writerFile = "🌊️graph-writer.rs";
const flateFile = "🗜️stored-flate.rs";
const stages = new Map<string, [string, string]>();
const replace = (value: string, old: string, changed: string) => {
    assert.equal(value.split(old).length, 2, old);
    return value.replace(old, () => changed);
};
let source = await Bun.file(exportPath).text();
const originalSource = source;
source = replace(source, `pub fn encode_pdf_with(snapshot: &PdfSnapshot, options: &EncodeOptions) -> PResult<Vec<u8>> {
    let version`, `pub fn encode_pdf_with(snapshot: &PdfSnapshot, options: &EncodeOptions) -> PResult<Vec<u8>> {
    let graph = PreparedPdfGraph::from_snapshot(snapshot, options)?;
    Ok(serialize_document(&graph.version, &graph.objects, &graph.trailer, &graph.write))
}

struct PreparedPdfGraph<'a> {
    version: String,
    objects: Cow<'a, [PdfIndirectObject]>,
    trailer: DocumentTrailer,
    write: WriteOptions,
}

impl<'a> PreparedPdfGraph<'a> {
fn from_snapshot(snapshot: &'a PdfSnapshot, options: &EncodeOptions) -> PResult<Self> {
    let version`);
source = replace(source, "    Ok(serialize_document(&version, &objects, &trailer, &write))\n}", `    Ok(Self { version, objects, trailer, write })
}
}

use crate::standards::v1_7::subsets::base::modules::writer::classic_graph_quote::{GraphLoan, GraphRefusal};
use super::ordinary_graft::OrdinaryRows;

/// 📌️ Owns the exact prepared current COS graph, or reloans the pinned original when reconciliation borrows it.
pub struct PinnedPdfGraph {
    source: usize,
    revision: [u8; 32],
    version: String,
    reconciled: Option<Vec<PdfIndirectObject>>,
    ordinary: Option<OrdinaryRows>,
    graft_pending: bool,
    trailer: DocumentTrailer,
}

impl PinnedPdfGraph {
    /// 🧬️ The reference planner uses the actual encode selector and never omits trailer extras.
    /// A retained Original job must receive this owner after controlled reconciliation rather than call this cold path.
    pub fn prepare_reference(snapshot: &PdfSnapshot, revision: [u8; 32]) -> PResult<Self> {
        let graph = PreparedPdfGraph::from_snapshot(snapshot, &EncodeOptions::default())?;
        if graph.write.encryption.is_some() || graph.write.xref_stream {
            return Err(PdfEngineError::Unsupported("segmented Save requires a clear classic PDF graph".into()));
        }
        let reconciled = match graph.objects {
            Cow::Borrowed(objects) => {
                if !std::ptr::eq(objects, snapshot.objects.as_slice()) {
                    return Err(PdfEngineError::Malformed("reconciliation returned a foreign borrowed graph".into()));
                }
                None
            }
            Cow::Owned(objects) => Some(objects),
        };
        Ok(Self { source: std::ptr::from_ref(snapshot) as usize, revision, version: graph.version, reconciled, ordinary: None, graft_pending: false, trailer: graph.trailer })
    }


    /// 🧮️ Quotes the retained candidate's entire shallow control allocation before the graph is reloaned.
    pub fn retained_control_extent(snapshot: &PdfSnapshot) -> Option<usize> {
        if snapshot.encryption.is_some() || snapshot.objects.is_empty() || snapshot.trailer.len() > 16
            || snapshot.declared_version.len() > 16
            || dict_get(&snapshot.trailer, "Root").and_then(PdfObject::as_ref).is_none()
        { return None; }
        let mut extent = snapshot.declared_version.len().max(3);
        if let Some([first, second]) = &snapshot.document_id {
            if first.len() > 128 || second.len() > 128 { return None; }
            extent += first.len() + second.len();
        }
        for entry in snapshot.trailer.iter().filter(|entry| !matches!(entry.key.as_str(), "Root" | "Info" | "ID")) {
            let body = match &entry.value {
                PdfObject::Null | PdfObject::Bool(_) | PdfObject::Int(_) | PdfObject::Ref(_) => 0,
                PdfObject::Real(decimal) if decimal.scale <= 384 && decimal.coefficient.bytes().all(|byte| byte.is_ascii_digit()) => decimal.coefficient.len(),
                PdfObject::Name(text) | PdfObject::Text(text) => text.len(),
                PdfObject::Str(bytes) => bytes.len(),
                _ => return None,
            };
            if entry.key.len() > 128 || body > 128 { return None; }
            extent += size_of::<PdfDictEntry>() + entry.key.len() + body;
        }
        Some(extent)
    }

    /// 🌱️ Builds only the retained control trailer. The graph remains private pending the controlled semantic check.
    pub fn retained_candidate(snapshot: &PdfSnapshot, revision: [u8; 32]) -> PResult<Self> {
        Self::retained_control_extent(snapshot).ok_or_else(|| PdfEngineError::Unsupported("retained PDF control trailer exceeds the Save qualification".into()))?;
        let root = dict_get(&snapshot.trailer, "Root").and_then(PdfObject::as_ref).ok_or_else(||
            PdfEngineError::Malformed("retained PDF has no root".into()))?;
        let info = dict_get(&snapshot.trailer, "Info").and_then(PdfObject::as_ref);
        let extra = |entry: &&PdfDictEntry| !matches!(entry.key.as_str(), "Root" | "Info" | "ID");
        let mut entries = Vec::with_capacity(snapshot.trailer.iter().filter(extra).count());
        entries.extend(snapshot.trailer.iter().filter(extra).cloned());
        let version = if snapshot.declared_version.is_empty() { "1.7".to_owned() } else { snapshot.declared_version.clone() };
        let trailer = DocumentTrailer { root, info, id: snapshot.document_id.clone(), extra: entries };
        Ok(Self { source: std::ptr::from_ref(snapshot) as usize, revision, version, reconciled: None, ordinary: None, graft_pending: true, trailer })
    }


    /// 📏️ Counts the capacities actually owned by the shallow retained-control candidate.
    pub fn retained_control_capacity(&self) -> usize {
        let ids = self.trailer.id.as_ref().map_or(0, |[a, b]| a.capacity() + b.capacity());
        self.version.capacity() + ids + self.trailer.extra.capacity() * size_of::<PdfDictEntry>()
            + self.trailer.extra.iter().map(|entry| entry.key.capacity() + match &entry.value {
                PdfObject::Name(text) | PdfObject::Text(text) => text.capacity(),
                PdfObject::Str(bytes) => bytes.capacity(),
                PdfObject::Real(decimal) => decimal.coefficient.capacity(),
                _ => 0,
            }).sum::<usize>()
    }

    pub fn needs_ordinary_graft(&self) -> bool { self.graft_pending }

    pub fn install_ordinary_graft(&mut self, rows: OrdinaryRows) {
        assert!(self.graft_pending && self.ordinary.is_none());
        self.trailer.info = rows.info_ref;
        self.ordinary = Some(rows);
        self.graft_pending = false;
    }

    /// 🔐️ A graph quote is only possible for the exact lifetime-pinned snapshot and canonical base.
    pub fn loan<'a>(&'a self, original: &'a PdfSnapshot, revision: [u8; 32]) -> Result<GraphLoan<'a>, GraphRefusal> {
        if self.source != std::ptr::from_ref(original) as usize || self.revision != revision {
            return Err(GraphRefusal::SourceChanged);
        }
        Ok(GraphLoan { version: &self.version, objects: self.reconciled.as_deref().unwrap_or(&original.objects), trailer: &self.trailer, overlay: self.ordinary.as_ref().map_or_else(Default::default, OrdinaryRows::overlay) })
    }
}`);
const start = source.indexOf("fn from_snapshot");
const stop = source.indexOf("\n}\n}", start);
source = source.slice(0, start) + source.slice(start, stop + 2).split("\n").map(line => line.length === 0 ? line : "    " + line).join("\n") + source.slice(stop + 2);
stages.set(exportPath, [originalSource, source]);
const originalWriter = await Bun.file(writerPath).text();
const moduleAnchor = "use std::collections::BTreeMap;";
const stagedWriter = replace(originalWriter, moduleAnchor, moduleAnchor + `\n\n#[path = "${quoteFile}"]\npub mod classic_graph_quote;\n#[path = "${writerFile}"]\npub mod classic_graph_writer;\n#[path = "${flateFile}"]\npub mod classic_stored_flate;`);
stages.set(writerPath, [originalWriter, stagedWriter]);
for (const [name, file] of [[quoteFile, "pdf-classic-graph-quote.rs"], [writerFile, "pdf-classic-graph-writer.rs"], [flateFile, "pdf-classic-stored-flate.rs"]]) {
    stages.set(join(base, "🔨️modules/📝️writer", name), ["", await Bun.file(join(import.meta.dir, file)).text()]);
}
const liftPath = join(base, "🔨️modules/⬇️lift/🧩️lifter-and-document/🧭️document-lift/🧭️controlled/🦀️.rs");
const lift = await Bun.file(liftPath).text();
const qualified = replace(lift,
    'let text = if entry.key == "Trapped" { entry.value.as_name() } else { as_native_text(&entry.value) };',
    `let text = if entry.key == "Trapped" {
            entry.value.as_name()
        } else if let PdfObject::Str(bytes) = &entry.value {
            if bytes.len() > MAX_COPY || !bytes.iter().all(|byte| (0x20..=0x7E).contains(byte)) {
                return self.defer(Barrier::Info);
            }
            std::str::from_utf8(bytes).ok()
        } else {
            as_native_text(&entry.value)
        };`);
stages.set(liftPath, [lift, qualified]);
const exportRoot = join(base, "🚪️io/📤️export/🦀️.rs");
const exportRootSource = await Bun.file(exportRoot).text();
const graphProof = join(base, "🚪️io/📤️export/🔏️graph-proof.rs");
stages.set(exportRoot, [exportRootSource, replace(exportRootSource, "pub use encode_and_reconcile::*;",
    'pub use encode_and_reconcile::*;\n\n#[path = "🔏️graph-proof.rs"]\npub mod exact_graph_proof;')]);
stages.set(graphProof, ["", await Bun.file(join(import.meta.dir, "pdf-exact-graph-proof.rs")).text()]);
const graftPath = join(base, "🚪️io/📤️export/🪡️ordinary-graft.rs");
const exportRootStaged = stages.get(exportRoot)![1];
stages.set(exportRoot, [exportRootSource, replace(exportRootStaged, "pub mod exact_graph_proof;",
    'pub mod exact_graph_proof;\n\n#[path = "🪡️ordinary-graft.rs"]\npub mod ordinary_graft;')]);
stages.set(graftPath, ["", await Bun.file(join(import.meta.dir, "pdf-paid-ordinary-graft.rs")).text()]);
const lowerPath = join(base, "🔨️modules/⬆️lower/📚️catalog-and-info/🦀️.rs");
const lower = await Bun.file(lowerPath).text();
const stagedLower = replace(lower,
    'pub fn lower_info(info: &PdfInfo) -> Vec<PdfDictEntry> {\n    let text = |value: &Option<String>| value.as_ref().map(|v| PdfObject::Str(encode_text_string(v)));',
    'pub fn lower_info(info: &PdfInfo) -> Vec<PdfDictEntry> { lower_info_with(info, encode_text_string) }\n\npub fn lower_info_with(info: &PdfInfo, encode_text: impl Fn(&str) -> Vec<u8>) -> Vec<PdfDictEntry> {\n    let text = |value: &Option<String>| value.as_ref().map(|v| PdfObject::Str(encode_text(v)));');
stages.set(lowerPath, [lower, stagedLower]);
const hash = (body: string) => createHash("sha256").update(body).digest("hex");
await Bun.write(join(output, "pinned-graph-overlay.json"), JSON.stringify([...stages].map(([file, [before, after]]) =>
    [file, before.length ? hash(before) : null, after])));
console.log("[DEBUG] Staged " + stages.size + " current-graph source paths");
