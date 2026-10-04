"""🛬️ S3-W2A W2A-6: `artifact_app_laws::load_document` / `load_document_text` drive the stepped archive load (stamp, admit,
poll, acknowledge); the plugin crate's own laws (T1) and puzzle 2d's reload move onto them."""
import pathlib
import re

ROOT = pathlib.Path("/Users/ueli/Documents/semio")
P = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"
LIB = P / "🦀️.rs"

helper = '''        /// 🛬️ The archive operation a law's whole-document load runs under: bit 61 alone, below the restore (62) and cold-pair
        /// (63) namespaces.
        pub const LAW_DOCUMENT_LOAD_OPERATION: u64 = 1 << 61;

        /// 🛬️ Loads `files` into `app` the only way a whole document loads (`📓️api-stepped-document-load.md` §4): the pair is
        /// stamped with the live store's identity — exactly what the runtime does to an `Effect::LoadDocument` — admitted as a
        /// whole-document archive load (`members: []`), polled to its terminal state over real polls and acknowledged. A load
        /// that does not land answers its own fault.
        pub async fn load_document<A, M>(app: &mut VcsArtifactApp<A, M>, files: &store::ArtifactPackFiles) -> Result<(), super::Fault>
        where
            A: ArtifactApp,
            M: super::SpaceMember + super::MemberFactory + Send + 'static,
        {
            let semio_framework::kernel::Effect::LoadDocument { pack, spr } = app.stamp_load_document_identity(semio_framework::kernel::Effect::LoadDocument { pack: files.pack.clone(), spr: files.spr.clone() })? else {
                return Err(super::Fault::from("the identity stamp answered something other than a document load"));
            };
            PluginApp::begin_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION, protocol::DocumentArchivePack { parent_pack: pack, parent_spr: spr, members: Vec::new() })?;
            let mut polls = 0usize;
            let status = loop {
                let status = PluginApp::poll_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION).await?;
                if !matches!(status.state, protocol::DocumentArchiveLoadState::Pending | protocol::DocumentArchiveLoadState::Running) {
                    break status;
                }
                polls += 1;
                if polls >= 1_000_000 {
                    return Err(super::Fault::from("the document archive load never reached a terminal state"));
                }
            };
            PluginApp::acknowledge_document_archive_load(app, LAW_DOCUMENT_LOAD_OPERATION)?;
            match status.state {
                protocol::DocumentArchiveLoadState::Ready => Ok(()),
                protocol::DocumentArchiveLoadState::Cancelled => Err(super::Fault::from("the document archive load was cancelled")),
                _ => Err(semio_framework_diagnostic::decode_fault_bytes(&status.fault)),
            }
        }

        /// 📜️ [`load_document`] for a text pair: the DSL and op log parse into the envelope whose pack and SPR the archive load
        /// takes, through the same printers `document_pack` uses.
        pub async fn load_document_text<A, M>(app: &mut VcsArtifactApp<A, M>, files: &store::ArtifactTextFiles) -> Result<(), super::Fault>
        where
            A: ArtifactApp,
            M: super::SpaceMember + super::MemberFactory + Send + 'static,
        {
            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;
            let pack = store::print_document_pack(&parsed.into_envelope().into_owners()).await.map_err(|error| error.into_fault())?;
            load_document(app, &pack).await
        }

        /// ⌨️ One long typing run'''

plan = {
    LIB: [
        ("        /// ⌨️ One long typing run", helper),
        ("                        app.load_document_pack(&store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap_or_else(|fault| panic!(\"the app's own boot example must load: {fault:?}\"));\n",
         "                        load_document(&mut app, &store::ArtifactPackFiles { pack, spr, ops: String::new() }).await.unwrap_or_else(|fault| panic!(\"the app's own boot example must load: {fault:?}\"));\n"),
    ],
    P / "🧪️tests/🧪️history-label-reload/🦀️.rs": [
        ("    reloaded.load_document_text(&live.document_text().await.expect(\"document text\")).await.expect(\"text reload\");\n",
         "    artifact_app_laws::load_document_text(&mut reloaded, &live.document_text().await.expect(\"document text\")).await.expect(\"text reload\");\n"),
        ("    repacked.load_document_pack(&live.document_pack().await.expect(\"document pack\")).await.expect(\"pack reload\");\n",
         "    artifact_app_laws::load_document(&mut repacked, &live.document_pack().await.expect(\"document pack\")).await.expect(\"pack reload\");\n"),
    ],
    P / "🧪️tests/🧪️history-edit-acceptance/🦀️.rs": [
        ("        app.load_document_text(&files).await.map_err(|fault| AcceptanceSeedFault::Base(format!(\"the base document does not load: {fault:?}\")))?;\n",
         "        artifact_app_laws::load_document_text(&mut app, &files).await.map_err(|fault| AcceptanceSeedFault::Base(format!(\"the base document does not load: {fault:?}\")))?;\n"),
        ("                saved.load_document_text(&files).await.map_err(|fault| format!(\"the document does not load: {fault:?}\"))?;\n",
         "                artifact_app_laws::load_document_text(&mut saved, &files).await.map_err(|fault| format!(\"the document does not load: {fault:?}\"))?;\n"),
    ],
    P / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs": [
        ("        restored.load_document_pack(&files).await.expect(\"load document pack\");\n",
         "        artifact_app_laws::load_document(&mut restored, &files).await.expect(\"load document pack\");\n"),
    ],
    ROOT / "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🧪️history-edit-runtime/🦀️.rs": [
        ("        \"text\" => block_on(reloaded.load_document_text(&block_on(app.document_text()).expect(\"document text\"))).unwrap_or_else(|fault| panic!(\"{what}: text reload: {fault:?}\")),\n        _ => block_on(reloaded.load_document_pack(&block_on(app.document_pack()).expect(\"document pack\"))).unwrap_or_else(|fault| panic!(\"{what}: pack reload: {fault:?}\")),\n",
         "        \"text\" => block_on(semio_framework_plugin::app::artifact_app_laws::load_document_text(&mut reloaded, &block_on(app.document_text()).expect(\"document text\"))).unwrap_or_else(|fault| panic!(\"{what}: text reload: {fault:?}\")),\n        _ => block_on(semio_framework_plugin::app::artifact_app_laws::load_document(&mut reloaded, &block_on(app.document_pack()).expect(\"document pack\"))).unwrap_or_else(|fault| panic!(\"{what}: pack reload: {fault:?}\")),\n"),
    ],
}

texts = {}
for path, edits in plan.items():
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        count = text.count(old)
        if count != 1:
            raise SystemExit(f"{path.name}: anchor count {count}: {old[:100]!r}")
        text = text.replace(old, new)
    texts[path] = text
TT = P / "🧪️tests/🧪️time-travel/🦀️.rs"
tt = TT.read_text(encoding="utf-8")
tt, text_sites = re.subn(r"\b(reloaded)\.load_document_text\(&", r"artifact_app_laws::load_document_text(&mut \1, &", tt)
tt, pack_sites = re.subn(r"\b(repacked)\.load_document_pack\(&", r"artifact_app_laws::load_document(&mut \1, &", tt)
if (text_sites, pack_sites) != (3, 3):
    raise SystemExit(f"time-travel sites {text_sites}/{pack_sites}")
texts[TT] = tt
for path, text in texts.items():
    path.write_text(text, encoding="utf-8")
print(f"law document load: helper + {sum(len(e) for e in plan.values()) - 1} migrations over {len(plan)} files")
