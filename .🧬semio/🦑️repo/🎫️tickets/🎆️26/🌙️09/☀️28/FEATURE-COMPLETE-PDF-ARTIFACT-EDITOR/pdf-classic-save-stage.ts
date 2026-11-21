/** 📦 Consolidates the native PDF Save producer and its authenticated reference qualification. */
import assert from "node:assert/strict";
import {createHash} from "node:crypto";
import {join, relative} from "node:path";

const base = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base";
const os = "🧰️framework/🛍️products/💻️os/🔨️modules";
const plugin = join(os, "🔌️plugin");
const editor = join(base, "✏️editor");
const ui = join(os, "📺️renderer/🧑‍🎨engine");
const output = join(import.meta.dir, "🗑️generated/classic-graph");
const hash = (body: string) => createHash("sha256").update(body).digest("hex");
type Row = [string, string | null, string];
const graph = await Bun.file(join(output, "pinned-graph-overlay.json")).json() as Row[];
const stage = new Map(graph.map(([file, digest, body]) => [file, {digest, body}]));

for (const relative of ["artifact-output-queue/overlay.json", "media-drain-bounds.json"]) {
    const dependency = Bun.file(join(import.meta.dir, "🗑️generated", relative));
    assert(await dependency.exists(), "Missing verified Save dependency stage " + relative);
    const rows = await dependency.json() as {path: string; before: string | null; after: string}[];
    for (const {path, before, after} of rows) {
        assert(!stage.has(path));
        const file = Bun.file(path);
        const current = await file.exists() ? hash(await file.text()) : null;
        if (current === hash(after)) continue;
        assert.equal(current, before, path + " differs from the verified dependency stage");
        stage.set(path, {digest: before, body: after});
    }
}

async function edit(file: string, transform: (source: string) => string) {
    const live = await Bun.file(file).text();
    const previous = stage.get(file);
    if (previous) assert.equal(previous.digest, hash(live), file + " changed after graph staging");
    stage.set(file, {digest: hash(live), body: transform(previous?.body ?? live)});
}
const sub = (source: string, old: string, next: string) => {
    assert.equal(source.split(old).length, 2, old);
    return source.replace(old, () => next);
};
const copy = (source: string, root: string, anchor: string, next: string) => edit(root, text => sub(text, anchor, next));

const exportFile = join(base, "🚪️io/📤️export/🧬️encode-and-reconcile/🦀️.rs");
const writer = join(base, "🔨️modules/📝️writer/🦀️.rs");
await copy("graph", exportFile, "pub struct PinnedPdfGraph {", "#[derive(semio_framework_value::RetireOwned)]\npub struct PinnedPdfGraph {");
await copy("trailer", writer, "#[derive(Clone, Debug, PartialEq)]\npub struct DocumentTrailer", "#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]\npub struct DocumentTrailer");
const graft = join(base, "🚪️io/📤️export/🪡️ordinary-graft.rs");
const graftRow = stage.get(graft)!;
graftRow.body = sub(graftRow.body, "#[derive(Default)]\npub struct OrdinaryRows", "#[derive(Default, semio_framework_value::RetireOwned)]\npub struct OrdinaryRows");

const sources = [
    ["🏭️factory.rs", "pdf-registered-save-factory.rs", "factory"],
    ["📤️producer.rs", "pdf-registered-save-producer.rs", "producer"],
    ["🧱️program.rs", "pdf-classic-save-program.rs", "program"],
    ["♻️close.rs", "pdf-classic-save-close.rs", "close"],
    ["🧾️job.rs", "pdf-classic-save-job.rs", "job"],
    ["📄️page.rs", "pdf-segmented-save-page.rs", "page"],
    ["🪟️window.rs", "pdf-registered-save-window.rs", "window"],
] as const;
const directory = join(editor, "💾️native-save");
for (const [target, source, kind] of sources) {
    let body = await Bun.file(join(import.meta.dir, source)).text();
    if (kind === "page" || kind === "window") {
        body = body.replaceAll("semio_framework_job::", "semio_plugin::__job::")
            .replaceAll("semio_plugin_decl_media::", "semio_plugin::")
            .replaceAll("semio_plugin_decl_app::media_contract::MediaConstruction", "semio_plugin::MediaConstruction");
    }
    if (kind === "close") body = body
        .replace("exact_graph_proof::ProjectionTrailer, ", "")
        .replace("schema::snapshot::{PdfPage, PdfSnapshot}", "schema::snapshot::{PdfDictEntry, PdfPage, PdfSnapshot}")
        .replace("control_trailer: Option<ProjectionTrailer>", "control_trailer: Option<Vec<PdfDictEntry>>")
        .replace("control_trailer: active.control_trailer,", "control_trailer: active.control_trailer.map(|trailer| trailer.into_entries()),");
    if (kind === "factory") body = body.replace("    media_file_export_action_definition(\n",
        "    let mut action = media_file_export_action_definition(\n").replace("        PORT, \"document.pdf\", MIME, FORMAT,\n    )", "        PORT, \"document.pdf\", MIME, FORMAT,\n    );\n    action.in_palette = false;\n    action");
    stage.set(join(directory, target), {digest: null, body});
}
stage.set(join(directory, "🦀️.rs"), {digest: null,
    body: "//! 💾️ Native PDF Save qualification, publication and close.\n" + sources.map(([file, , name]) => `#[path = "${file}"]\npub mod ${name};`).join("\n") + "\n"});

const macro = "✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs";
await edit(macro, source => {
    const marker = source.indexOf("macro_rules! snapshot_details_editor_support");
    assert(marker >= 0);
    let tail = source.slice(marker);
    tail = sub(tail, "        $(, native_snapshot_pack: $native_snapshot_pack:tt)?",
        "        $(, native_snapshot_pack: $native_snapshot_pack:tt)?\n        $(, native_media_export_factory: $native_media_export_factory:ty)?");
    tail = sub(tail, "            $crate::register_snapshot_edit_tool_factory::<Self>(registry)",
        `            $(
                let controller = registry.controller_id().to_string();
                registry.register(<$native_media_export_factory>::new(&controller))?;
            )?
            $crate::register_snapshot_edit_tool_factory::<Self>(registry)`);
    return source.slice(0, marker) + tail;
});

await edit(join(editor, "🦀️.rs"), source => {
    source = sub(source, "mod image_import;", "mod image_import;\n\n#[path = \"💾️native-save/🦀️.rs\"]\nmod native_save;");
    source = sub(source, "    fn encode_natural_file(", `    fn io() -> Option<semio_plugin::AppIo> {
        Some(native_save::factory::app_io())
    }

    fn build_media_export_job(
        request: &mut Option<semio_plugin::ArtifactMediaExportJobRequest<semio_plugin::EditorApp<Self>>>,
        body: &mut Option<Box<dyn semio_plugin::ArtifactReservedJob + Send>>,
        construction: &mut semio_plugin::MediaConstruction, cx: &mut semio_plugin::__job::StepContext<'_>,
    ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
        native_save::job::PdfGraphSaveJob::admit_original(request, body, construction, cx)
    }

    fn encode_natural_file(`);
    source = sub(source, "        bounded_native: true,\n        native:", "        bounded_native: true,\n        native_media_export_factory: native_save::factory::PdfNativeExportFactory,\n        native:");
    return sub(source, "    definition.actions.extend(image_import::actions());", "    definition.actions.extend(image_import::actions());\n    definition.actions.push(native_save::factory::export_action());");
});

const docs = join(plugin, "🧵️instance/📄️docs/📥️staging/🖼️render/🦀️.rs");
await copy("broker", docs,
    "let result=match active.retained_result.as_ref(){Some(Ok(result))=>result,Some(Err(error))=>",
    "let result=match active.retained_result.as_ref(){Some(Ok(result))=>result,Some(Err(MediaError::NotImplemented)) if active.port==semio_plugin_decl_app::NATURAL_FILE_PORT&&active.output_chunks.bytes()==0&&active.output_credit.bytes()==0=>return self.finish_media_poll(operation_id,active,ArtifactMediaExportPoll::Failed(\"media-export.reference-required\".into())),Some(Err(error))=>");

const lifecycle = join(ui, "🎬️media/🚚️lifecycle/🟦️.ts");
await edit(lifecycle, source => sub(
    sub(source, "async function receiveExport(", `/** 🧭️ The authenticated producer declined before offering a descriptor or a byte page. */
export class MediaExportReferenceRequired extends Error {}

async function receiveExport(`),
    '  if (status.state === "failed") throw new Error(status.detail || "media-export.failed");',
    `  if (status.state === "failed") {
    if (status.detail === "media-export.reference-required" && status.total_bytes === 0n && status.descriptor.length === 0) throw new MediaExportReferenceRequired(status.detail);
    throw new Error(status.detail || "media-export.failed");
  }`));
const file = join(ui, "🎬️media/📤️file/🟦️.ts");
await edit(file, source => sub(source,
    'import { receiveMediaExportBytes, type MediaTransportPort } from "../🚚️lifecycle/🟦️.ts";',
    'import { MediaExportReferenceRequired, receiveMediaExportBytes, type MediaTransportPort } from "../🚚️lifecycle/🟦️.ts";')
    + `
/** 🔀️ Selects the prepared segmented producer or the same-revision native reference producer. */
export async function deliverNaturalMediaFileV1(
  segmented: (() => Promise<boolean>) | null,
  stillCurrent: () => Promise<void>,
  reference: () => Promise<void>,
): Promise<void> {
  if (segmented !== null) {
    try {
      if (!await segmented()) throw new Error("natural-file.export-owner-retired");
      return;
    } catch (reason) {
      if (!(reason instanceof MediaExportReferenceRequired)) throw reason;
      await stillCurrent();
    }
  }
  await reference();
}
`);
const shell = join(ui, "🧱️elements/🏛️ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx");
await edit(shell, source => {
    source = sub(source, "downloadDeclaredMediaFileV1, type MediaFileExportActionIdV1", "downloadDeclaredMediaFileV1, deliverNaturalMediaFileV1, type MediaFileExportActionIdV1");
    source = sub(source, `      if (declaration !== null) {
        const delivered = await downloadDeclaredMediaFileV1({ instanceId: owned.target.instanceId, declaration, port: owned.handle, signal: task.signal, current: heldArchive, progress: task.progress, sinkFactory: shellSegmentedDownloadSinkFactory, beforeDelivery: verifyDelivery });
        if (!delivered) throw task.signal.reason ?? new Error("natural-file.export-owner-retired");
      } else {
        const artifact = await exportMedia!(owned.target.instanceId, NATURAL_FILE_PORT_ID);`, `      await deliverNaturalMediaFileV1(declaration === null ? null : () =>
        downloadDeclaredMediaFileV1({ instanceId: owned.target.instanceId, declaration, port: owned.handle, signal: task.signal, current: heldArchive, progress: task.progress, sinkFactory: shellSegmentedDownloadSinkFactory, beforeDelivery: verifyDelivery }),
      verifyDelivery, async () => {
        if (exportMedia === undefined) throw new Error("natural-file.exporter-unavailable");
        const artifact = await exportMedia(owned.target.instanceId, NATURAL_FILE_PORT_ID);`);
    return sub(source, "        downloadMediaExportBytes(file, format.mediaType, artifact.data);\n      }", "        downloadMediaExportBytes(file, format.mediaType, artifact.data);\n      });");
});

// Keep the original media job independent of the nearly full controller and reconciliation crates.
const packageName = "semio-s-artifact-stdio-pdf-native-save";
const crateName = packageName.replaceAll("-", "_");
const setup = join(directory, "📦️packages/🦀️rust");
const schema4 = join(base, "🧬️schema/📦️packages/🦀️rust");
const schema5 = join(base, "../🧾️vt/🧬️schema/📦️packages/🦀️rust");
const editorSource = join(editor, "🦀️.rs");
const exportRoot = join(base, "🚪️io/📤️export/🦀️.rs");
const graphProof = join(base, "🚪️io/📤️export/🔏️graph-proof.rs");
const graftPath = join(base, "🚪️io/📤️export/🪡️ordinary-graft.rs");
const stagedExport = stage.get(exportFile)!;
const graphStart = stagedExport.body.indexOf("use crate::standards::v1_7::subsets::base::modules::writer::classic_graph_quote");
const graphEnd = stagedExport.body.indexOf("/// 🧾 Trailer entries beyond", graphStart);
assert(graphStart >= 0 && graphEnd > graphStart);
const preparedGraph = stagedExport.body.slice(graphStart, graphEnd)
    .replace("use super::ordinary_graft::OrdinaryRows;", "use super::ordinary_graft::OrdinaryRows;")
    .replace("let graph = PreparedPdfGraph::from_snapshot(snapshot, &EncodeOptions::default())?;",
        "let graph = PreparedPdfGraph::from_snapshot(snapshot, &EncodeOptions::default())?;");
const graphImports = `use std::borrow::Cow;
use crate::standards::v1_7::subsets::base::{
    io::{error::{PResult, PdfEngineError}, export::{EncodeOptions, PreparedPdfGraph}},
    modules::{lexer::dict_get, writer::DocumentTrailer},
    schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot},
};
`;
stagedExport.body = stagedExport.body.slice(0, graphStart) + stagedExport.body.slice(graphEnd);
stagedExport.body = stagedExport.body.replace("struct PreparedPdfGraph<'a> {", "pub struct PreparedPdfGraph<'a> {")
    .replace("    fn from_snapshot(", "    pub fn from_snapshot(")
    .replace("    version: String,\n    objects: Cow<'a, [PdfIndirectObject]>,\n    trailer: DocumentTrailer,\n    write: WriteOptions,",
        "    pub version: String,\n    pub objects: Cow<'a, [PdfIndirectObject]>,\n    pub trailer: DocumentTrailer,\n    pub write: WriteOptions,");
stage.set(join(directory, "🧬️graph.rs"), {digest: null, body: graphImports + preparedGraph});
stage.set(join(directory, "🔏️proof.rs"), stage.get(graphProof)!);
stage.set(join(directory, "🪡️graft.rs"), stage.get(graftPath)!);
stage.delete(graphProof);
stage.delete(graftPath);
stage.delete(exportRoot);
const programFile = stage.get(join(directory, "🧱️program.rs"))!;
const programStart = programFile.body.indexOf("    io::export::{PinnedPdfGraph,");
const programEnd = programFile.body.indexOf("    modules::{lexer::CosGrant", programStart);
assert(programStart >= 0 && programEnd > programStart);
programFile.body = programFile.body.slice(0, programStart) + programFile.body.slice(programEnd);
programFile.body = sub(programFile.body, "use super::producer", `use super::{graph::PinnedPdfGraph, exact_graph_proof::{COMPARE_WORK, ExactGraphProof, ExactGraphRefusal,
    ExactGraphTurn, ProjectionTrailer, projection_trailer, projection_trailer_extent},
    ordinary_graft::{GRAFT_WORK, OrdinaryGraft, OrdinaryRows}};
use super::producer`);
const closeFile = stage.get(join(directory, "♻️close.rs"))!;
closeFile.body = sub(closeFile.body, "    io::export::{PinnedPdfGraph, ordinary_graft::OrdinaryRows},\n", "")
    .replace("use super::program", "use super::{graph::PinnedPdfGraph, ordinary_graft::OrdinaryRows};\nuse super::program");
const job = stage.get(join(directory, "🧾️job.rs"))!;
job.body = job.body.replace("use std::{mem::ManuallyDrop, ptr::NonNull};", "use std::{mem::ManuallyDrop, ptr::NonNull};\nuse semio_plugin::ArtifactApp;")
    .replace("ArtifactReservedJob, EditorApp, MediaConstruction", "ArtifactReservedJob, MediaConstruction")
    .replace("use crate::{editor::pdf17::Pdf17Editor, standards::v1_7::subsets::base::io::export::PinnedPdfGraph};",
        "use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;\nuse super::graph::PinnedPdfGraph;")
    .replace("type App = EditorApp<Pdf17Editor>;\ntype Request = ArtifactMediaExportJobRequest<App>;\ntype RequestClose = ArtifactMediaExportRequestRetirement<App>;",
        "type Request<A> = ArtifactMediaExportJobRequest<A>;\ntype RequestClose<A> = ArtifactMediaExportRequestRetirement<A>;")
    .replaceAll("Option<Request>", "Option<Request<A>>").replaceAll("Option<RequestClose>", "Option<RequestClose<A>>")
    .replaceAll("size_of::<Request>()", "size_of::<Request<A>>()").replaceAll("size_of::<RequestClose>()", "size_of::<RequestClose<A>>()")
    .replace("pub struct PdfGraphSaveJob {", "pub struct PdfGraphSaveJob<A: ArtifactApp<Snapshot = PdfSnapshot>> {")
    .replace("impl PdfGraphSaveJob {", "impl<A: ArtifactApp<Snapshot = PdfSnapshot>> PdfGraphSaveJob<A> {")
    .replace("impl InteractiveJob for PdfGraphSaveJob {", "impl<A: ArtifactApp<Snapshot = PdfSnapshot>> InteractiveJob for PdfGraphSaveJob<A> {")
    .replace("impl ArtifactReservedJob for PdfGraphSaveJob {}", "impl<A: ArtifactApp<Snapshot = PdfSnapshot>> ArtifactReservedJob for PdfGraphSaveJob<A> {}")
    .replace("impl Drop for PdfGraphSaveJob {", "impl<A: ArtifactApp<Snapshot = PdfSnapshot>> Drop for PdfGraphSaveJob<A> {");
const factory = stage.get(join(directory, "🏭️factory.rs"))!;
factory.body = factory.body.replace("ArtifactToolPublicationLane, EditorApp, LocalizedLabel", "ArtifactToolPublicationLane, LocalizedLabel, ArtifactApp")
    .replace("use crate::editor::pdf17::Pdf17Editor;", "use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;")
    .replace("pub struct PdfNativeExportFactory {", "pub struct PdfNativeExportFactory<A: ArtifactApp<Snapshot = PdfSnapshot>> {\n    owner: std::marker::PhantomData<fn() -> A>,")
    .replace("impl PdfNativeExportFactory {", "impl<A: ArtifactApp<Snapshot = PdfSnapshot>> PdfNativeExportFactory<A> {")
    .replace("Self { keys:", "Self { owner: std::marker::PhantomData, keys:")
    .replace("impl ToolJobFactory for PdfNativeExportFactory {", "impl<A: ArtifactApp<Snapshot = PdfSnapshot>> ToolJobFactory for PdfNativeExportFactory<A> {")
    .replace("impl ArtifactOwnedToolJobFactory for PdfNativeExportFactory {\n    type Owner = EditorApp<Pdf17Editor>;",
        "impl<A: ArtifactApp<Snapshot = PdfSnapshot>> ArtifactOwnedToolJobFactory for PdfNativeExportFactory<A> {\n    type Owner = A;");
const rootModule = stage.get(join(directory, "🦀️.rs"))!;
rootModule.body += '\n#[path = "🧬️graph.rs"]\npub mod graph;\n#[path = "🔏️proof.rs"]\npub mod exact_graph_proof;\n#[path = "🪡️graft.rs"]\npub mod ordinary_graft;\n';
stage.set(join(setup, "🦀️.rs"), {digest: null, body: `//! 🎫️ Owns the Original media Save program for the qualified PDF COS profile.
pub use semio_s_artifact_stdio_pdf_schema_4::standards;

#[path = "../../🦀️.rs"]
mod save;
pub use save::*;
`});

const manifest = await Bun.file(join(editor, "📄️document/📦️packages/🦀️rust/Cargo.toml")).text();
const workspaceField = manifest.split("\n").find(line => line.startsWith("workspace = "));
const frameworkValue = 'semio-framework-value = { path = "' + relative(setup, "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust").replaceAll("\\", "/") + '" }';
const frameworkPublication = 'semio-framework-job-publication = { path = "' + relative(setup, "🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/📦️packages/🦀️rust").replaceAll("\\", "/") + '" }';
assert(workspaceField && frameworkValue);
stage.set(join(setup, "Cargo.toml"), {digest: null, body: `[package]
${workspaceField}
name = "${packageName}"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[package.metadata.semio]
role = "library"

[lints]
workspace = true

[lib]
path = "🦀️.rs"

[dependencies]
${frameworkPublication}
${frameworkValue}
semio-os-kernel = { workspace = true }
semio-plugin = { workspace = true }
semio-s-artifact-stdio-pdf-schema-4 = { path = "../../../../🧬️schema/📦️packages/🦀️rust" }
`});
const workspace = join(setup, JSON.parse(workspaceField.split(" = ")[1]));
const member = relative(workspace, setup).replaceAll("\\", "/");
await edit(join(workspace, "Cargo.toml"), source => sub(source, 'resolver = "2"\nmembers = [', 'resolver = "2"\nmembers = [\n    "' + member + '",'));
await edit(join(schema5, "Cargo.toml"), source => sub(source, "[dependencies]\n", `[dependencies]\n${packageName} = { path = "../../../../🧱️base/✏️editor/💾️native-save/📦️packages/🦀️rust" }\n`));

await edit(macro, source => {
    source = sub(source, "$(, native_media_export_factory: $native_media_export_factory:ty)?",
        "$(, native_media_export: { factory: $native_media_export_factory:ty, io: $native_media_export_io:path, build: $native_media_export_build:path })?");
    const marker = source.indexOf("macro_rules! snapshot_details_editor_support");
    return source.slice(0, marker) + sub(source.slice(marker), "        fn build_tool_job(\n", `        $(
            fn io() -> Option<semio_plugin::AppIo> { Some($native_media_export_io()) }

            fn build_media_export_job(
                request: &mut Option<semio_plugin::ArtifactMediaExportJobRequest<semio_plugin::EditorApp<Self>>>,
                body: &mut Option<Box<dyn semio_plugin::ArtifactReservedJob + Send>>,
                construction: &mut semio_plugin::MediaConstruction, cx: &mut semio_plugin::__job::StepContext<'_>,
            ) -> Result<bool, semio_plugin::__job::OriginalTurnFailure> {
                $native_media_export_build(request, body, construction, cx)
            }
        )?

        fn build_tool_job(
`);
});
await edit(editorSource, source => {
    source = sub(source, '\n#[path = "💾️native-save/🦀️.rs"]\nmod native_save;\n', "");
    const start = source.indexOf("    fn io() -> Option<semio_plugin::AppIo>");
    const end = source.indexOf("    fn encode_natural_file", start);
    assert(start >= 0 && end > start);
    source = source.slice(0, start) + source.slice(end);
    source = sub(source, "        native_media_export_factory: native_save::factory::PdfNativeExportFactory,",
          `        native_media_export: { factory: ${crateName}::factory::PdfNativeExportFactory<semio_plugin::EditorApp<Self>>, io: ${crateName}::factory::app_io, build: ${crateName}::job::PdfGraphSaveJob::<semio_plugin::EditorApp<Self>>::admit_original },`);
    return sub(source, "    definition.actions.extend(image_import::actions());\n    definition.actions.push(native_save::factory::export_action());",
        `    definition.actions.extend(image_import::actions().into_iter().chain(std::iter::once(${crateName}::factory::export_action())));`);
});

const lockDependency = "semio-s-artifact-stdio-pdf-schema-5";
const locks = [...new Bun.Glob("**/Cargo.lock").scanSync({cwd: "✏️s", dot: false})].map(lock => join("✏️s", lock));
locks.push(join("🌎️hub", "Cargo.lock"));
for (const path of locks) {
    const existing = await Bun.file(path).text();
    if (!existing.includes('name = "' + lockDependency + '"')) continue;
    await edit(path, source => {
        const packs = source.split("[[package]]\n");
        const found = packs.findIndex(row => row.startsWith('name = "' + lockDependency + '"\n'));
        assert(found >= 0 && !source.includes('name = "' + packageName + '"\n'));
        const current = packs[found];
        packs[found] = sub(current, "dependencies = [\n", 'dependencies = [\n "' + packageName + '",\n');
        const slot = packs.findIndex(row => /^name = "([^"]+)"/m.exec(row)?.[1] > packageName);
        assert(slot >= 0, path);
        const version = /^version = "([^"]+)"$/m.exec(current)?.[1];
        assert(version !== undefined);
        packs.splice(slot, 0, 'name = "' + packageName + '"\nversion = "' + version + '"\ndependencies = [\n "semio-framework-job-publication",\n "semio-framework-value",\n "semio-os-kernel",\n "semio-plugin",\n "semio-s-artifact-stdio-pdf-schema-4",\n]\n\n');
        const updated = packs.join("[[package]]\n");
        Bun.TOML.parse(updated);
        return updated;
    });
}

const classic = [
    ["🧮️graph-quote.rs", "classic_graph_quote"],
    ["🌊️graph-writer.rs", "classic_graph_writer"],
    ["🗜️stored-flate.rs", "classic_stored_flate"],
] as const;
for (const [name, module] of classic) {
    const member = join(base, "🔨️modules/📝️writer", name);
    const row = stage.get(member)!;
    if (module === "classic_graph_quote") {
        row.body = sub(row.body, "use super::{DocumentTrailer, ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, WRITER_TRAILER_KEYS};",
            `use crate::standards::v1_7::subsets::base::{
    modules::{writer::DocumentTrailer, xref::WRITER_TRAILER_KEYS},
    schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject},
};`);
    }
    if (module === "classic_graph_writer") {
        row.body = sub(row.body, "use super::{PdfObject, encode_stream};",
            `use crate::standards::v1_7::subsets::base::{
    modules::filters::encode_stream, schema::snapshot::PdfObject,
};`);
    }
    stage.set(join(directory, name), row);
    stage.delete(member);
    rootModule.body += '\n#[path = "' + name + '"]\npub mod ' + module + ';\n';
}
await edit(writer, source => {
    const begin = source.indexOf('#[path = "🧮️graph-quote.rs"]');
    const end = source.indexOf("pub mod classic_stored_flate;", begin) + "pub mod classic_stored_flate;".length;
    assert(begin >= 0 && end > begin);
    return source.slice(0, begin) + source.slice(end).replace(/^\n{1,2}/, "");
});
const schema4Mirror = join(schema4, "🪞️mirror/🦀️.rs");
await edit(schema4Mirror, source => sub(source,
    "pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::writer;",
    "pub use semio_s_artifact_stdio_pdf_conformance_support::standards::v1_7::subsets::base::modules::{filters, writer, xref};"));
for (const name of ["🧬️graph.rs", "🔏️proof.rs", "🪡️graft.rs", "🧱️program.rs"]) {
    const row = stage.get(join(directory, name))!;
    row.body = row.body.replaceAll("crate::standards::v1_7::subsets::base::modules::writer::classic_graph_quote", "super::classic_graph_quote");
    if (name === "🧱️program.rs") {
        row.body = row.body.replace("    modules::writer::{\n        classic_graph_quote::{GraphQuote, MAX_PREFIX, QUOTE_WORK},\n        classic_graph_writer::ClassicGraphWriter,\n    },\n", "");
        row.body = "use super::{classic_graph_quote::{GraphQuote, MAX_PREFIX, QUOTE_WORK}, classic_graph_writer::ClassicGraphWriter};\n" + row.body.replace(/^\/\/![^\n]*\n/, "");
    }
    if (name === "🔏️proof.rs") {
        row.body = row.body.replace("        writer::classic_graph_quote::GraphLoan,\n", "");
        row.body += "\nuse super::classic_graph_quote::GraphLoan;\n";
    }
    if (name === "🪡️graft.rs") {
        row.body = row.body.replace(", writer::classic_graph_quote::{GraphLoan, GraphOverlay}", "");
        row.body += "\nuse super::classic_graph_quote::{GraphLoan, GraphOverlay};\n";
    }
}

const staged = [...stage].map(([file, row]) => [file, row.digest, row.body]);
for (const [file, expected] of staged) {
    assert(typeof file === "string");
    assert(join(process.cwd(), file).length <= 256, file);
    const live = Bun.file(file);
    if (expected === null) assert(!await live.exists(), file + " was created by another owner");
    else assert.equal(hash(await live.text()), expected, file + " changed while preparing the overlay");
}
await Bun.write(join(output, "registered-save-overlay.json"), JSON.stringify(staged));
console.log("[DEBUG] Consolidated " + staged.length + " digest-guarded registered Save paths");
