"""Stage the mounted app recipient and reserved natural-source publication receipt."""
from pathlib import Path
from runpy import run_path

ticket = Path(__file__).parent
sdk = run_path(str(ticket / "natural-pdf-sdk-stage.py"))
change, emit = sdk["change"], sdk["emit"]
changes, output, plugin, decl = (sdk[name] for name in ("changes", "output", "plugin", "decl"))
reactor = plugin / "⚛️reactor"
instance = plugin / "🧵️instance"
raw = reactor / "🏃️runtime/🧰️ops/🎞️raw-media"

factory = plugin / "👷️job/🏛️reserved/🦀️.rs"
change(factory, [
    ("use semio_plugin_decl_app::tool_contract::BOUNDED_FIRST_STEP_FACTORY;",
     "use semio_plugin_decl_app::tool_contract::{BOUNDED_FIRST_STEP_FACTORY, NATURAL_SOURCE_TOOL_CONTRACT};"),
    ('framework_reserved_job!(FrameworkSetActiveUtilityJob, FrameworkSetActiveUtilityJobFactory, "setActiveUtility", 21, 16_384, 64, 1_024, 16_384, lanes: [HostOnly]);',
     '''framework_reserved_job!(FrameworkSetActiveUtilityJob, FrameworkSetActiveUtilityJobFactory, "setActiveUtility", 21, 16_384, 64, 1_024, 16_384, lanes: [HostOnly]);

framework_reserved_job!(
    FrameworkImportNaturalSourceJob, FrameworkImportNaturalSourceJobFactory, "import-natural-source", 22,
    NATURAL_SOURCE_TOOL_CONTRACT.max_raw_wire_bytes, NATURAL_SOURCE_TOOL_CONTRACT.max_decoded_items,
    NATURAL_SOURCE_TOOL_CONTRACT.max_work_units_per_step, NATURAL_SOURCE_TOOL_CONTRACT.max_output_bytes,
    lanes: [Artifact, Child]
);'''),
    ("    registry.register(FrameworkImportMediaJobFactory::<A>::new(&controller_id))?;",
     """    registry.register(FrameworkImportMediaJobFactory::<A>::new(&controller_id))?;
    if A::natural_file_paged_maximum_bytes() != 0 {
        registry.register(FrameworkImportNaturalSourceJobFactory::<A>::new(&controller_id))?;
    }"""),
])
ingress = instance / "👷️jobs/📥️staging/🚪️ingress/🦀️.rs"
change(ingress, [
    ("    pub fn qualified_tool_proof(&self, stores: &InstanceStores<A>, verb: &str) -> Result<QualifiedToolView<'_>, Fault> {",
     """    pub fn qualified_tool_proof(&self, stores: &InstanceStores<A>, verb: &str) -> Result<QualifiedToolView<'_>, Fault> {
        use semio_plugin_decl_app::tool_contract::{NATURAL_SOURCE_TOOL_CONTRACT, NATURAL_SOURCE_TOOL_ID, NATURAL_SOURCE_TOOL_SCHEMA};
        if verb == NATURAL_SOURCE_TOOL_ID {
            return self.framework_registration(verb)
                .filter(|row| row.key.tool_id == NATURAL_SOURCE_TOOL_ID && row.schema_id == NATURAL_SOURCE_TOOL_SCHEMA
                    && row.contract == NATURAL_SOURCE_TOOL_CONTRACT
                    && row.factory_type_id == std::any::TypeId::of::<semio_plugin_job::reserved_jobs::FrameworkImportNaturalSourceJobFactory<A>>())
                .map(QualifiedToolView::FrameworkOwned)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-factory"),
                    "paged natural source has no registered original decoder factory"));
        }"""),
])

def source(name):
    return (ticket / name).read_text(encoding="utf-8")

change(decl / "🎞️media/📥️original-media/📥️natural-source.rs", [
    ("    pub fn format_runnable(&self) -> bool { matches!(self.stage, Stage::Sealed | Stage::Lent | Stage::Closing) }",
     """    pub fn input_ready(&self) -> bool { self.stage == Stage::Sealed }

    pub fn format_runnable(&self) -> bool {
        matches!(self.stage, Stage::Sealed | Stage::Lent | Stage::Closing) || self.completion_pending()
    }"""),
    ("        if self.stage != Stage::Lent || self.loan().operation != current { return Ok(false); }\n        self.change_stage(if published { Stage::Published } else { Stage::Failed }, true, cx)",
     """        if self.loan().operation != current { return Ok(false); }
        let settled = if published { Stage::Published } else { Stage::Failed };
        if self.stage == settled { return Ok(true); }
        if self.stage != Stage::Lent { return Ok(false); }
        self.change_stage(settled, true, cx)"""),
])
change(reactor / "🔌️app/🦀️.rs", [
    ("    fn natural_source_format_closed(&self) -> bool { true }",
     """    fn natural_source_format_closed(&self) -> bool { true }

    /// 💾 Advances the registered paged decoder, Store publication and Original source retirement.
    async fn advance_natural_source_format(
        &mut self, _source: &mut semio_plugin_decl_media::original_media::natural_source::NaturalFileSourceGuest,
        _cx: &mut semio_framework_job::StepContext<'_>,
    ) -> Result<(), semio_framework_job::OriginalTurnFailure> { Ok(()) }"""),
])
change(instance / "🖼️render/📥️staging/🏷️reserved-jobs/🦀️.rs", [
    ("    /// 🎟️ Donates the complete staged request", source("natural-pdf-reserved-render.rs") + "    /// 🎟️ Donates the complete staged request"),
    ("let media=matches!(owner.input.as_ref(),Some(ArtifactReservedToolInput::Media{..}));",
     "let media=matches!(owner.input.as_ref(),Some(ArtifactReservedToolInput::Media{..}|ArtifactReservedToolInput::NaturalPaged{..}));"),
])
change(instance / "📨️dispatch/📥️staging/🎟️producers/🦀️.rs", [
    ("    /// 🧾️ Prepares the original clipboard action", source("natural-pdf-reserved-mount.rs") + "    /// 🧾️ Prepares the original clipboard action"),
])
docs = instance / "📄️docs/📥️staging/🧱️components"
changes[docs / "🎟️natural-publication.rs"] = "", source("natural-pdf-publication-owner.rs")
change(docs / "🦀️.rs", [
    ("pub mod picker_publication;", 'pub mod picker_publication;\n#[path = "🎟️natural-publication.rs"]\npub mod natural_publication;'),
    ("    pub original_picker: Option<Box<picker_publication::PickerPublication>>,", """    pub original_picker: Option<Box<picker_publication::PickerPublication>>,
    pub natural_source: Option<Box<natural_publication::NaturalFilePublication>>,"""),
    ("            original_picker:None,", "            original_picker:None,\n            natural_source:None,"),
])
app = instance / "🔌️plugin-app-impl"
changes[app / "🎟️natural-publication/🦀️.rs"] = "", source("natural-pdf-appointed-publication.rs")
change(app / "🦀️.rs", [
    ("mod picker_publication;", 'mod picker_publication;\n#[path = "🎟️natural-publication/🦀️.rs"]\nmod natural_publication;'),
    ("    fn original_media_format_closed(&self) -> bool {", """    fn natural_source_capability(&self) -> Option<(semio_plugin_decl_media::original_media::natural_source_gate::NaturalSourceDeclaration<'static>, u32)> {
        let maximum = A::natural_file_paged_maximum_bytes();
        if maximum == 0 { return None; }
        let codec = semio_framework_async::poll::resolve_ready(A::natural_file_codec())?;
        if !codec.binary { return None; }
        Some((semio_plugin_decl_media::original_media::natural_source_gate::NaturalSourceDeclaration {
            format_kind: codec.format_kind, extension: codec.extension, media_type: codec.media_type, binary: codec.binary,
        }, maximum))
    }

    fn natural_source_format_closed(&self) -> bool { self.media.natural_source.is_none() }

    async fn advance_natural_source_format(
        &mut self, source: &mut semio_plugin_decl_media::original_media::natural_source::NaturalFileSourceGuest,
        cx: &mut StepContext<'_>,
    ) -> Result<(), OriginalTurnFailure> {
        self.advance_natural_publication(source, cx).await
    }

    fn original_media_format_closed(&self) -> bool {"""),
    ("        if !self.constructor.constructor_terminal_is_empty() { return self.stores.constructor_close_demands(&self.constructor, body); }",
     """        if !self.constructor.constructor_terminal_is_empty() { return self.stores.constructor_close_demands(&self.constructor, body); }
        if let Some(source) = self.media.natural_source.as_ref().filter(|_| self.natural_source_can_close()) {
            return source.close_demands(body);
        }"""),
    ("        if self.constructor.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput{reason:\"original constructor receipt awaits its phase recipient\"})}",
     """        if self.constructor.has_constructor_receipt(){return Ok(PluginLifecycleStep::AwaitingInput{reason:"original constructor receipt awaits its phase recipient"})}
        if self.media.natural_source.is_some() && self.natural_source_can_close()
            && self.original_controller_close.as_ref().is_none_or(|owner| (owner.terminal)(&self.app))
            && self.constructor.constructor_terminal_is_empty()
        {
            return semio_plugin_instance_docs::docs_components::natural_publication::NaturalFilePublication::close_mounted(
                &mut self.media.natural_source, cx, loan,
            );
        }"""),
])
change(app / "🔒️close-terminal/🦀️.rs", [
    ("            && self.media.original_picker.is_none()", "            && self.media.original_picker.is_none()\n            && self.media.natural_source.is_none()"),
    ("            self.media.reserved_mount.is_none(),", "            self.media.reserved_mount.is_none(),\n            self.media.natural_source.is_none(),"),
])
publication = instance / "🧾️typed-operations/📥️staging/📬️publication/📬️typed-operation-unit/🦀️.rs"
change(publication, [
    ("        let frontier=self.ingest.artifact_store_replacement_frontier(handle);",
     """        let natural = self.media.natural_source.as_ref().is_some_and(|owner|
            owner.mount_id == Some(mounted.operation.operation) && owner.source.matches(cx));
        let natural_receipt = if natural { size_of::<bool>() } else { 0 };
        let frontier=self.ingest.artifact_store_replacement_frontier(handle);"""),
    ("+ 3*std::mem::size_of::<bool>() + std::mem::size_of::<Option<ArtifactBoundedToolFault>>()",
     "+ 3*std::mem::size_of::<bool>() + natural_receipt + std::mem::size_of::<Option<ArtifactBoundedToolFault>>()"),
    ("            mounted.published_artifact=true;\n            mounted.ui_pending=true;",
     """            mounted.published_artifact=true;
            if natural { self.media.natural_source.as_mut().expect("matched natural publication").committed = true; }
            mounted.ui_pending=true;"""),
])
change(reactor / "🚪️lifetime/🦀️.rs", [
    ("    natural_source_cursor: usize,", "    natural_source_cursor: usize,\n    natural_format_cursor: usize,"),
    ("natural_source_cursor: 0 }", "natural_source_cursor: 0, natural_format_cursor: 0 }"),
    ("    pub fn pending_opening", """    /// 🧮 Selects one sealed natural source or format shutdown for this bounded guest turn.
    pub fn next_natural_format_poll(&mut self, closing: &[u32]) -> Option<u32> {
        let count = self.slots.values.len();
        for offset in 0..count {
            let index = (self.natural_format_cursor + offset) % count;
            let Some(slot) = self.slots.get(index) else { continue; };
            let instance = slot.cell.lifetime().instance_id;
            if slot.cell.is_live() && !closing.contains(&instance)
                && slot.natural_source.as_ref().is_some_and(NaturalSourceGuest::format_runnable)
            {
                self.natural_format_cursor = (index + 1) % count;
                return Some(instance);
            }
        }
        None
    }

    pub fn pending_opening"""),
])
change(raw / "🦀️.rs", [
    ("mod natural_source;", "mod natural_source;\npub use natural_source::drive_natural_source_format;"),
])
change(raw / "📥️natural-source.rs", [
    ("            if cancel || complete {", """            if cancel || complete {
                if complete && !app.natural_source_format_closed() { return Ok(None); }"""),
    ("fn drive_control<PA: PluginApp>(", """/// 🫴 Advances one sealed natural format owner under the same authenticated command Source.
pub async fn drive_natural_source_format<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, closing: &[u32], cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let instance = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("natural format lifetime is busy"))?
        .next_natural_format_poll(closing);
    let Some(instance) = instance else { return Ok(false); };
    let key = runtime.live_instance_key(instance)?;
    let instances = runtime.instances.try_borrow().map_err(|_| reactor_close_fault("natural format registry is busy"))?;
    let cell = instances.get(instance).ok_or_else(|| reactor_close_fault("natural format instance is absent"))?;
    let mut active = match cell.instance.try_lock() {
        Ok(value) => value,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(true),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err(reactor_close_fault("natural format instance is poisoned").into()),
    };
    let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| reactor_close_fault("natural format lifetime is busy"))?;
    let slot = lifetimes.get_mut(instance).ok_or_else(|| reactor_close_fault("natural format lifetime is absent"))?;
    if !slot.admit_command_origin(key, cx)? { return Ok(true); }
    let source = slot.natural_source.as_mut().expect("selected natural format owner");
    active.app.advance_natural_source_format(source, cx).await?;
    Ok(source.format_runnable())
}

fn drive_control<PA: PluginApp>("""),
])
turn = reactor / "🔄️turn/🌀️poll-kernel-turn/🦀️.rs"
change(turn, [
    ("    let media_format_runnable = command_page.is_none() && !pending_media_control",
     """    let natural_format_runnable = command_page.is_none() && !pending_media_control
        && semio_plugin_runtime_ops::ops_raw_media::drive_natural_source_format(runtime, &close_instances, cx).await?;
    let media_format_runnable = command_page.is_none() && !pending_media_control"""),
    ("    let command_ingress = match media_ingress {", "    let media_format_runnable = media_format_runnable || natural_format_runnable;\n    let command_ingress = match media_ingress {"),
])
emit("natural-source-sdk-publication.patch")

scope = output / "receiver-check"
natural = changes[decl / "🎞️media/📥️original-media/📥️natural-source.rs"][1]
(scope / "natural.rs").write_text(natural, encoding="utf-8")
route = changes[raw / "📥️natural-source.rs"][1]
(scope / "route.rs").write_text(route.split("\n", 1)[1], encoding="utf-8")
(scope / "publication-owner.rs").write_text(source("natural-pdf-publication-owner.rs").split("\n", 1)[1], encoding="utf-8")
crate_path = scope / "lib.rs"
crate = crate_path.read_text(encoding="utf-8").replace(
    "pub trait PluginApp {",
    "pub trait PluginApp {\n    async fn advance_natural_source_format(&mut self, _: &mut natural_source::NaturalFileSourceGuest, _: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> { Ok(()) }",
).replace(
    "    pub struct StubRegistry;",
    """    pub struct StubRegistry<PA>(std::marker::PhantomData<PA>);
    impl<PA> StubRegistry<PA> {
        pub fn next_natural_format_poll(&mut self, _: &[u32]) -> Option<u32> { unreachable!() }
        pub fn get_mut(&mut self, _: u32) -> Option<&mut crate::instance_lifetime::NativeLifetimeSlot<PA>> { unreachable!() }
    }
    pub struct Active<PA> { pub app: PA }
    pub struct Cell<PA> { pub instance: std::sync::Mutex<Active<PA>> }
    pub struct InstanceRegistry<PA>(std::marker::PhantomData<PA>);
    impl<PA> InstanceRegistry<PA> { pub fn get(&self, _: u32) -> Option<&Cell<PA>> { unreachable!() } }
""",
).replace(
    "        pub guest_lifetimes: std::cell::RefCell<StubRegistry>,",
    "        pub guest_lifetimes: std::cell::RefCell<StubRegistry<PA>>,\n        pub instances: std::cell::RefCell<InstanceRegistry<PA>>,",
).replace(
    "    impl StubRegistry {",
    "    impl<PA> StubRegistry<PA> {",
)
crate += '\nmod actual_publication_owner { include!("publication-owner.rs"); }\n'
crate_path.write_text(crate, encoding="utf-8")
