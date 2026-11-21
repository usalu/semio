"""🎟️ Stage registered picker work and its exact mounted mutation route."""
from hashlib import sha256
import json
from pathlib import Path

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
BASE = Path("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base")
DECL = PLUGIN / "📜️declaration"
INSTANCE = PLUGIN / "🧵️instance"
EDITOR = BASE / "✏️editor" / "🦀️.rs"
RENDER = INSTANCE / "🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs"
OWNER = INSTANCE / "👷️jobs/📥️staging/🏷️reserved/🦀️.rs"
ROUTE = INSTANCE / "📨️dispatch/📥️staging/🎟️producers/🦀️.rs"
gate = {row[0]: row[2] for row in json.loads((GENERATED / "pdf-open-wire-admission-overlay.json").read_text())["data"]}
for path, _, body in json.loads((GENERATED / "pdf-history-target-overlay.json").read_text()):
    if Path(path) == EDITOR:
        gate[path] = body


def digest(data):
    """🔑 Hash the exact staged UTF-8 source."""
    return sha256(data.encode()).hexdigest()


def once(source, anchor, result):
    """🎯 Require one exact source anchor."""
    assert source.count(anchor) == 1, anchor
    return source.replace(anchor, result)


class Candidate:
    """🧾 Keep a live-write preimage for each independently staged owner."""
    def __init__(self):
        self.baseline = {}
        self.sources = {}

    def read(self, path):
        """📚 Compose only explicit preceding stage postimages."""
        name = str(path)
        if name not in self.baseline:
            self.baseline[name] = gate.get(name) or path.read_text()
        return self.sources.get(name, self.baseline[name])

    def replace(self, path, anchor, result):
        """✍️ Replace a checked slice without touching shared files."""
        self.sources[str(path)] = once(self.read(path), anchor, result)

    def append(self, path, source):
        """➕ Stage a new source module beside the registered owner."""
        assert not path.exists(), path
        self.baseline[str(path)] = None
        self.sources[str(path)] = source

    def emit(self):
        """📋 Record the exact chained preimages and postimages for admission."""
        return [[name, digest(self.baseline[name]) if self.baseline[name] is not None else None, body]
                for name, body in self.sources.items()]


def registered_job(candidate):
    """🖼️ Delegate the runtime-issued picker to the format owner registered by the editor."""
    reserved = DECL / "👷️job/🧵️reserved-tool-job/🦀️.rs"
    row = "    Media { port: String, media: Media },"
    candidate.replace(reserved, row, row + """
    OriginalPicker { input: Box<semio_plugin_decl_media::original_media::RawMediaInput> },""")
    app = DECL / "🧩️app/🎛️artifact-app-trait/🦀️.rs"
    editor = DECL / "🪟️surface/✏️artifact-editor-trait/🦀️.rs"
    editor_app = DECL / "🪟️surface/🖊️editor-app/🦀️.rs"
    guarded_signature = """    fn build_original_picker_job(
        _request: &mut Option<ArtifactReservedToolJobRequest<APP>>,
        _body: &mut Option<Box<dyn TRAIT+Send>>,
        _construction: &mut semio_framework_job::OriginalConstructionAdmission,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure>"""
    default = """ {
        Err(semio_framework_value::ValueError::literal(
            semio_framework_value::ValueRefusalKind::UnsupportedOwner,
            "This editor has no registered Original picker format",
        ).into())
    }

"""
    for path, ty, trait in [(app, "Self", "crate::tool_contract::ArtifactReservedJob"),
                            (editor, "EditorApp<Self>", "semio_plugin_decl_app::tool_contract::ArtifactReservedJob")]:
        anchor = "    fn build_media_export_job("
        method = guarded_signature.replace("APP", ty).replace("TRAIT", trait)
        candidate.replace(path, anchor, method + default + anchor)
    signature = guarded_signature.replace("APP", "Self").replace("TRAIT", "semio_plugin_decl_app::tool_contract::ArtifactReservedJob")
    for name in ["request", "body", "construction", "cx"]:
        signature = signature.replace("_" + name + ":", name + ":")
    anchor = "    fn build_media_export_job("
    candidate.replace(editor_app, anchor, signature + """ {
        E::build_original_picker_job(request, body, construction, cx)
    }
""" + anchor)
    base_signature = signature.replace("ArtifactReservedToolJobRequest<Self>", "semio_plugin::app::ArtifactReservedToolJobRequest<semio_plugin::EditorApp<Self>>")
    base_signature = base_signature.replace("semio_plugin_decl_app::tool_contract::ArtifactReservedJob", "semio_plugin::app::ArtifactReservedJob")
    base_signature = base_signature.replace("semio_framework_job::", "semio_plugin::__job::")
    anchor = "    fn natural_file_codec()"
    candidate.replace(EDITOR, anchor, base_signature + """ {
        semio_plugin::app::admit_reserved_work::<semio_plugin::EditorApp<Self>, crate::standards::v1_7::subsets::base::editor::document::image_authoring::PickerPngWork<semio_plugin::EditorApp<Self>>>(
            request, body, construction, cx,
        )
    }

""" + anchor)
    candidate.append(EDITOR.with_name("📄️document") / "🖼️image-authoring/🧵️reserved-picker.rs", (TICKET / "📜️pdf-original-png-reserved-work.rs").read_text())


def bounded_route(candidate):
    """📥 Use the normal reserved import preparation for the exact picked input."""
    candidate.replace(OWNER, " pub wire_phase:ReservedWirePhase,", " pub wire_phase:ReservedWirePhase,\n pub original_picker:bool,")
    candidate.replace(OWNER, "wire_phase:ReservedWirePhase::Cursor,admission_cursor:",
                      "wire_phase:ReservedWirePhase::Cursor,original_picker:false,admission_cursor:")
    candidate.replace(OWNER, "A::build_reserved_tool_job(&mut self.request,&mut self.body,&mut self.wire_construction,cx)?;return Ok(false)",
                      """if self.original_picker{A::build_original_picker_job(&mut self.request,&mut self.body,&mut self.wire_construction,cx)?;}else{A::build_reserved_tool_job(&mut self.request,&mut self.body,&mut self.wire_construction,cx)?;}return Ok(false)""")
    render_marker = "    /// 🎟️ Donates the complete staged request"
    candidate.replace(RENDER, render_marker, """    /// 🖼️ Gives the verified picker input to the registered import producer without recopying its pages.
    pub fn advance_reserved_picker(&mut self, input: &mut Option<Box<semio_plugin_decl_media::original_media::RawMediaInput>>,
        meta: &ActionMeta, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if let Some(owner) = self.pending.reserved_preparation.as_mut() {
            owner.validate(self.stores.store.content_revision(), cx)?;
            if owner.job.is_some() { return Ok(true); }
            if owner.request.is_some() || owner.body.is_some() { return owner.advance_builder(cx); }
        }
        if !self.prepare_reserved_header("import-media", meta, cx)? { return Ok(false); }
        let owner = self.pending.reserved_preparation.as_mut()
            .ok_or_else(|| semio_framework_value::ValueError::invariant("picker requires reserved preparation"))?;
        if !owner.original_picker {
            let Some(picked) = input.as_ref() else { return Ok(false); };
            let origin = picked.loan();
            if origin.instance_id != meta.instance_id || origin.document_id != self.stores.store.envelope().id
                || origin.base_revision != &self.stores.store.content_revision() || owner.input.is_some() || owner.raw_wire.is_some()
            {
                return Err(semio_framework_value::ValueError::invariant("picker import changed its original destination").into());
            }
            let copied = size_of::<Option<ArtifactReservedToolInput>>() + size_of::<Option<Vec<u8>>>() + size_of::<bool>();
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() };
            if cx.should_yield() || cx.is_cancelled() || cx.retained_grant().maximum_depth == 0 || !progress.fits(cx.retained_grant()) {
                return Ok(false);
            }
            cx.consume_retained(progress)?;
            cx.consume_fuel(1);
            *owner.input = Some(ArtifactReservedToolInput::OriginalPicker { input: input.take().unwrap() });
            *owner.raw_wire = Some(Vec::new());
            owner.original_picker = true;
            return Ok(false);
        }
        if !self.prepare_reserved_snapshot_read(cx)? || !self.prepare_reserved_wire_admission(cx)? { return Ok(false); }
        self.assemble_reserved_request("import-media", meta, &mut None, 1, cx)?;
        Ok(false)
    }

""" + render_marker)
    route_marker = "    /// 🧾️ Prepares the original clipboard action"
    candidate.replace(ROUTE, route_marker, """    /// 🪝️ Mounts the Original picker through the checked import route and retains its exact operation.
    pub async fn advance_original_reserved_picker(&mut self,
        input: &mut Option<Box<semio_plugin_decl_media::original_media::RawMediaInput>>, meta: &ActionMeta,
        finished: &mut Option<semio_framework_job::Operation>, cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<bool, semio_framework_job::OriginalTurnFailure> {
        if finished.is_some() { return Ok(true); }
        if self.original_reserved_prepares() && !self.as_render().advance_reserved_picker(input, meta, cx)? {
            return Ok(false);
        }
        self.advance_original_reserved_mount("import-media", meta, finished, cx).await
    }

""" + route_marker)


def mounted_publication(candidate):
    """🔒 Mark only the admitted picker mount for the Store's atomic source transaction."""
    publication = INSTANCE / "🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs"
    old = """                    if let Some((mut publication,ownership))=self.stores.store.admit_apply_batch_with_pages(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,&mut emit.artifact_mutation_pages,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),None,outbound,retained).map_err(|error|plugin_sdk_fault(error.to_string()))? {"""
    new = """                    let admitted = if self.media.original_picker.as_ref().is_some_and(|owner|
                        owner.mount_id == Some(mounted.operation.operation) && owner.source.matches(cx)) {
                        if !emit.artifact_mutations.is_empty() || emit.transaction.is_some() || streams {
                            return Err(plugin_sdk_fault("Original picker requires one internally qualified paged edit").into());
                        }
                        self.stores.store.admit_original_media_batch_with_pages(
                            mounted.operation.operation, mounted.artifact_generation, mounted.canonical_revision,
                            &mut mounted.actor_capture, &mut emit.artifact_mutation_pages, self.stores.artifact_one_item_factory.as_ref(),
                            self.tools.tool_controller_id(), Some(&mounted.verb), None, retained,
                        )
                    } else {
                        self.stores.store.admit_apply_batch_with_pages(mounted.operation.operation,mounted.artifact_generation,mounted.canonical_revision,&mut mounted.actor_capture,&mut emit.artifact_mutations,&mut emit.artifact_mutation_pages,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut emit.transaction,Some(&mounted.verb),None,outbound,retained)
                    };
                    if let Some((mut publication,ownership)) = admitted.map_err(|error|plugin_sdk_fault(error.to_string()))? {"""
    candidate.replace(publication, old, new)


def main():
    """💾 Save the complete guard manifest for validation while builds retain source custody."""
    candidate = Candidate()
    registered_job(candidate)
    bounded_route(candidate)
    mounted_publication(candidate)
    path = GENERATED / "raw-pdf-reserved-overlay.json"
    path.write_text(json.dumps(candidate.emit(), ensure_ascii=False, indent=2) + "\n")
    print("[DEBUG] Staged picker reserved producer paths", len(candidate.sources))


if __name__ == "__main__":
    main()
