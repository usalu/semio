"""🎞️ Connect Original media transport to the registered mounted publisher."""
import hashlib
import json
from pathlib import Path
from runpy import run_path

TICKET = Path(__file__).resolve().parent
GENERATED = TICKET / "🗑️generated"
PLUGIN = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
RUNTIME = PLUGIN / "⚛️reactor"
INSTANCE = PLUGIN / "🧵️instance"


def digest(source):
    """🔑 Hash the exact UTF-8 source."""
    return hashlib.sha256(source.encode()).hexdigest()


def one(source, before, after):
    """✏️ Restrict a change to its one observed seam."""
    assert source.count(before) == 1, before
    return source.replace(before, after)


def stage():
    """🧩 Compose the reactor, app and mounted Store ownership candidates."""
    run_path(str(TICKET / "raw-pdf-picker-stage.py"))["stage"]()
    run_path(str(TICKET / "📜️raw-pdf-reserved-stage.py"))["main"]()
    run_path(str(TICKET / "📜️pdf-png-format-stage.py"))["stage"]()
    product = json.loads((GENERATED / "raw-pdf-picker-overlay.json").read_text())
    sources, preimages = product["candidate"], product["preimages"]

    def edit(path, before, after):
        name = str(path)
        if name not in sources:
            sources[name] = path.read_text()
            preimages[name] = digest(sources[name])
        sources[name] = one(sources[name], before, after)

    def add(path, origin):
        assert not path.exists() and str(path) not in sources, path
        preimages[str(path)] = None
        sources[str(path)] = (TICKET / origin).read_text()

    for name, before, body in json.loads((GENERATED / "raw-pdf-reserved-overlay.json").read_text()):
        assert name not in sources, name
        preimages[name] = before
        sources[name] = body

    format_stage = json.loads((GENERATED / "raw-png-format-overlay.json").read_text())
    for name, body in format_stage["candidate"].items():
        before = format_stage["preimages"][name]
        if name in sources:
            assert preimages[name] == before and name.endswith("/🖼️image-authoring/🦀️.rs"), name
            body = one(body, "mod admission;", 'mod admission;\n#[path = "📂️picker-work.rs"]\nmod picker_work;\npub use picker_work::PngPickerWork;')
        preimages[name] = before
        sources[name] = body
    image_authoring = next(Path(name) for name in sources if name.endswith("/🖼️image-authoring/🦀️.rs"))
    edit(image_authoring, "pub use format_work::PngFormatWork;",
         'pub use format_work::PngFormatWork;\n#[path = "🧵️reserved-picker.rs"]\nmod reserved_picker;\npub use reserved_picker::PickerPngWork;')

    components = INSTANCE / "📄️docs/📥️staging/🧱️components/🦀️.rs"
    edit(components, "use semio_ui_contract::*;", """use semio_ui_contract::*;
#[path = "🎟️picker-publication.rs"]
pub mod picker_publication;""")
    edit(components, "pub struct MediaTransfers<A: ArtifactApp> {",
         "pub struct MediaTransfers<A: ArtifactApp> {\n    pub original_picker: Option<Box<picker_publication::PickerPublication>>,")
    edit(components, "            reserved_mount:ManuallyDrop::new(None),",
         "            original_picker:None,\n            reserved_mount:ManuallyDrop::new(None),")
    add(components.with_name("🎟️picker-publication.rs"), "📜️raw-picker-mounted-publication.rs")

    app = RUNTIME / "🔌️app/🦀️.rs"
    edit(app, """    fn advance_original_media_format(
        &mut self, _picker: &mut semio_plugin_decl_media::original_media::RawMediaPicker,""",
         """    async fn advance_original_media_format(
        &mut self, _picker: &mut semio_plugin_decl_media::original_media::RawMediaPicker,""")
    edit(app, "    fn original_media_format_closed(&self) -> bool { true }",
         """    fn original_media_format_closed(&self) -> bool { true }

    /// 🧾 Observes irreversible picker publication before accepting a late cancellation.
    fn observe_original_media_commit(
        &mut self, _picker: &mut semio_plugin_decl_media::original_media::RawMediaPicker,
        _cx: &semio_framework_job::StepContext<'_>,
    ) -> Result<(), semio_framework_job::OriginalTurnFailure> { Ok(()) }""")
    mounted = INSTANCE / "🔌️plugin-app-impl/🦀️.rs"
    edit(mounted, "mod typed_operation_results;",
         """mod typed_operation_results;
#[path = "🎟️picker-publication/🦀️.rs"]
mod picker_publication;""")
    add(mounted.with_name("🎟️picker-publication") / "🦀️.rs", "📜️raw-picker-mounted-app.rs")
    edit(mounted, "    async fn app_id(&self) -> &str {", """    fn original_media_format_closed(&self) -> bool {
        self.media.original_picker.is_none()
    }

    fn observe_original_media_commit(
        &mut self, picker: &mut semio_plugin_decl_media::original_media::RawMediaPicker, cx: &StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        self.observe_picker_commit(picker, cx)
    }

    async fn advance_original_media_format(
        &mut self, picker: &mut semio_plugin_decl_media::original_media::RawMediaPicker,
        _identity: &mut EntityIdentityAuthority<'_>, cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        self.advance_picker_publication(picker, cx).await
    }

    async fn app_id(&self) -> &str {""")
    edit(mounted, "    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {",
         """    fn close_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.tools.tool_operations.is_empty() {
            if let Some(owner) = self.media.original_picker.as_ref() { return owner.close_demand(body); }
        }""")
    edit(mounted, "    fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{",
         """    fn close_step(&mut self,cx:&mut StepContext<'_>,loan:RetainedCloneGrant)->Result<PluginLifecycleStep,semio_plugin_ownership::bounded_ownership::PluginLifecycleFailure>{
        if self.tools.tool_operations.is_empty() {
            if let Some(owner) = self.media.original_picker.as_ref() {
                if !owner.source.matches(cx) {
                    return Ok(PluginLifecycleStep::AwaitingInput { reason: "Original picker awaits its retained receiving source" });
                }
                let demand = owner.close_demand(loan.maximum_copy_bytes)?;
                return semio_plugin_ownership::bounded_ownership::original_lifecycle_step(cx, loan, demand, |same, _| {
                    let before = same.retained_progress();
                    let slot = &mut self.media.original_picker;
                    if slot.as_mut().unwrap().close_input(same)? {
                        same.with_original_retirement(|same| semio_plugin_instance_docs::docs_components::picker_publication::PickerPublication::retire(slot, same))?;
                    }
                    let progress = semio_plugin_ownership::bounded_ownership::original_lifecycle_delta(before, same.retained_progress())?;
                    Ok((PluginLifecycleStep::Progress(progress), false))
                });
            }
        }""")
    terminal = INSTANCE / "🔌️plugin-app-impl/🔒️close-terminal/🦀️.rs"
    edit(terminal, "            && self.media.reserved_mount.is_none()",
         "            && self.media.reserved_mount.is_none()\n            && self.media.original_picker.is_none()")
    publication = INSTANCE / "🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs"
    edit(publication, "                            mounted.published_artifact = true;", """                            mounted.published_artifact = true;
                            if let Some(owner) = self.media.original_picker.as_mut().filter(|owner|
                                owner.mount_id == Some(mounted.operation.operation) && owner.source.matches(cx)) {
                                owner.committed = Some(mounted.operation.operation);
                            }""")

    prologue = RUNTIME / "🏃️runtime/🧰️ops/🎞️raw-media/🦀️.rs"
    edit(prologue, "closing: &[u32], identity: &mut EntityIdentityAuthority<'_>, cx:",
         "closing: &[u32], _identity: &mut EntityIdentityAuthority<'_>, cx:")
    edit(prologue, "        app.advance_original_media_format(&mut slot.raw_media.picker, identity, cx)?;\n", "")
    edit(prologue, "        Ok(slot.raw_media.control(event, live.as_ref(), cx)?.map(CommandIngressStatus::OriginalMediaAck))",
         """        Ok(slot.raw_media.control(event, live.as_ref(), cx)?.map(CommandIngressStatus::OriginalMediaAck))""")
    edit(prologue, "    Ok(with_slot(runtime, instance, cx, |slot, app, cx| {\n        let live = app.original_media_live_document();\n        Ok(slot.raw_media.control",
         """    Ok(with_slot(runtime, instance, cx, |slot, app, cx| {
        app.observe_original_media_commit(&mut slot.raw_media.picker, cx)?;
        let live = app.original_media_live_document();
        Ok(slot.raw_media.control""")
    edit(prologue, "fn raw_control_close(", """/// 🎛️ Drives one paid picker mount or return while the actor retains its real command Source.
pub async fn drive_raw_media_format<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, closing: &[u32], priority: Option<u32>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut()
        .map_err(|_| reactor_close_fault("Original picker lifetime is busy"))?;
    let preferred = priority.filter(|id| lifetimes.get_mut(*id).is_some_and(|slot| slot.raw_media.picker.receipt_due()));
    let Some(instance) = preferred.or_else(|| lifetimes.next_raw_media_poll(&[])) else { return Ok(false); };
    let key = runtime.live_instance_key(instance)?;
    let instances = runtime.instances.try_borrow()
        .map_err(|_| reactor_close_fault("Original picker registry is busy"))?;
    let cell = instances.get(instance).ok_or_else(|| reactor_close_fault("Original picker instance is absent"))?;
    let mut active = match cell.instance.try_lock() {
        Ok(value) => value,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(true),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err(reactor_close_fault("Original picker instance is poisoned").into()),
    };
    let slot = lifetimes.get_mut(instance).ok_or_else(|| reactor_close_fault("Original picker lifetime is absent"))?;
    if !slot.admit_command_origin(key, cx)? { return Ok(true); }
    if closing.contains(&instance) && !slot.raw_media.picker.request_supersession(cx)? { return Ok(true); }
    active.app.advance_original_media_format(&mut slot.raw_media.picker, identity, cx).await?;
    Ok(slot.raw_media.picker.receipt_due())
}

fn raw_control_close(""")
    poll_path = RUNTIME / "🔄️turn/🌀️poll-kernel-turn/🦀️.rs"
    if str(poll_path) not in sources:
        paths = [Path(name) for name, body in sources.items() if "let picker_pending = " in body]
        assert len(paths) == 1, paths
        poll_path = paths[0]
    edit(poll_path, "    let command_ingress = match raw {", """    let picker_priority = match raw.as_ref() {
        Some(semio_framework::kernel::CommandIngressStatus::OriginalMediaAck(ack))
            if ack.phase == semio_framework::kernel::OriginalMediaWirePhase::Cancel => Some(ack.loan.instance_id),
        _ => close_instances.first().copied(),
    };
    let picker_work = command_page.is_none() &&
        semio_plugin_runtime_ops::ops_raw_media::drive_raw_media_format(runtime, &close_instances, picker_priority, identity, cx).await?;
    let command_ingress = match raw {""")
    edit(poll_path, "    let picker_pending = semio_plugin_runtime_ops::ops_raw_media::continue_original_picker_effects(runtime, cx)?;",
         "    let picker_pending = semio_plugin_runtime_ops::ops_raw_media::continue_original_picker_effects(runtime, cx)? || picker_work;")

    for name, expected in preimages.items():
        path = Path(name)
        actual = digest(path.read_text()) if path.is_file() else None
        assert actual == expected, (name, expected, actual)
    path = GENERATED / "raw-picker-mounted-overlay.json"
    path.write_text(json.dumps(product, ensure_ascii=False, indent=2) + "\n")
    print("[DEBUG] Staged mounted Original picker admission and custody", len(sources))


if __name__ == "__main__":
    stage()
