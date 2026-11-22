"""Carry the authored natural purpose through the native Original creation owner."""
from difflib import unified_diff
from pathlib import Path
from runpy import run_path

ticket = Path(__file__).parent
wire = run_path(str(ticket / "natural-pdf-wire-stage.py"))
output = wire["output"]
plugin = wire["plugin"]
reactor = plugin / "⚛️reactor"
changes = dict(wire["receiver"]["changes"])
changes.update(wire["changes"])


def change(path, replacements):
    if path in changes:
        base, revised = changes[path]
    else:
        base = revised = path.read_text(encoding="utf-8")
    for old, new in replacements:
        assert revised.count(old) == 1, (path, old, revised.count(old))
        revised = revised.replace(old, new, 1)
    changes[path] = base, revised


def emit(name):
    patch = []
    for path, (base, staged) in changes.items():
        patch.extend(unified_diff(base.splitlines(keepends=True), staged.splitlines(keepends=True), f"a/{path}" if base else "/dev/null", f"b/{path}"))
    (output / name).write_bytes("".join(patch).encode("utf-8"))


lifetime = reactor / "🚪️lifetime/🦀️.rs"
base, staged = changes[lifetime]
start = staged.index("    /// 🏷️ Captures the exact natural launch marker")
end = staged.index("    pub fn prepare_natural_source", start)
staged = staged[:start] + '''    /// 🧩 Installs the already paid creation projection before this exact slot is published.
    pub fn install_natural_source_launch(&mut self, launch: NaturalSourceLaunchGrant) {
        assert!(!self.native_created && self.natural_source_launch.is_none());
        self.natural_source_launch = Some(launch);
    }

''' + staged[end:]
old = """struct NaturalSourceLaunchGrant {
    declaration: NaturalSourceDeclaration<'static>,
    maximum: u32,
    extent: u32,
}
"""
new = """pub struct NaturalSourceLaunchGrant {
    declaration: NaturalSourceDeclaration<'static>,
    maximum: u32,
    extent: u32,
}

impl NaturalSourceLaunchGrant {
    /// 📐 Projects the authenticated launch only when it exactly matches the mounted decoder capability.
    pub fn project(
        declaration: NaturalSourceDeclaration<'static>, maximum: u32,
        offered: &semio_framework::kernel::NaturalSourceWireDeclaration, extent: u32,
    ) -> Option<Self> {
        (extent != 0 && extent <= maximum && maximum <= NATURAL_SOURCE_TRANSPORT_MAXIMUM_BYTES
            && declaration.format_kind == offered.format_kind && declaration.extension == offered.extension
            && declaration.media_type == offered.media_type && declaration.binary == offered.binary)
            .then_some(Self { declaration, maximum, extent })
    }
}
"""
assert staged.count(old) == 1
changes[lifetime] = base, staged.replace(old, new, 1)

change(reactor / "🔌️app/🦀️.rs", [
    ("    fn original_media_format_closed(&self) -> bool { true }",
     """    fn original_media_format_closed(&self) -> bool { true }

    /// 🌿 Advertises a mounted decoder that accepts the paid paged natural source.
    fn natural_source_capability(&self) -> Option<(semio_plugin_decl_media::original_media::natural_source_gate::NaturalSourceDeclaration<'static>, u32)> {
        None
    }

    /// 🕊️ Reports whether the mounted natural format owner has reached terminal retirement.
    fn natural_source_format_closed(&self) -> bool { true }"""),
])
cleanup = reactor / "🏃️runtime/📥️staging/♻️reactor-lifecycle/🧯️cleanup-pumps/🦀️.rs"
change(cleanup, [
    ("pub source:std::mem::ManuallyDrop<Option<(String,String)>>,", "pub source:std::mem::ManuallyDrop<Option<(String,String,semio_framework::kernel::ActorInstanceOpenPurpose)>>,pub natural_launch:Option<crate::instance_lifetime::NaturalSourceLaunchGrant>,"),
    ("ControlledRetirement<(String,String)>", "ControlledRetirement<(String,String,semio_framework::kernel::ActorInstanceOpenPurpose)>"),
])
opening = reactor / "🏃️runtime/📦️instances/🚪️opening/🦀️.rs"
change(opening, [
    ("app_id:String,actor:String,identity:", "app_id:String,actor:String,purpose:semio_framework::kernel::ActorInstanceOpenPurpose,identity:"),
    ("source:std::mem::ManuallyDrop::new(Some((app_id,actor))),", "source:std::mem::ManuallyDrop::new(Some((app_id,actor,purpose))),natural_launch:None,"),
    (" if let Some(source)=opening.source_closing.as_mut(){",
     """ if !opening.closing && opening.natural_launch.is_none() {
  if let Some((_, _, semio_framework::kernel::ActorInstanceOpenPurpose::NaturalSource { declaration, extent })) = opening.source.as_ref() {
   let (mounted, maximum) = opening.app.as_ref().and_then(PluginApp::natural_source_capability)
       .ok_or_else(||plugin_internal_fault("mounted app has no paged natural source decoder"))?;
   let launch = semio_plugin_reactor_lifecycle::instance_lifetime::NaturalSourceLaunchGrant::project(mounted, maximum, declaration, *extent)
       .ok_or_else(||plugin_internal_fault("natural source launch does not match the mounted decoder declaration and extent"))?;
   let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of_val(&opening.natural_launch), ..Default::default() };
   if cx.retained_grant().maximum_depth == 0 || cx.fuel_remaining() == 0 || !progress.fits(cx.retained_grant()) { return Ok(false); }
   cx.consume_retained(progress)?;
   cx.consume_fuel(1);
   opening.natural_launch = Some(launch);
   return Ok(false);
  }
 }
 if let Some(source)=opening.source_closing.as_mut(){"""),
    (" let actor=opening.authority.take().ok_or_else", """ let launch_progress = RetainedCloneProgress {
  copied_items: usize::from(opening.natural_launch.is_some()),
  copied_bytes: opening.natural_launch.as_ref().map_or(0, size_of_val),
  ..Default::default()
 };
 if opening.natural_launch.is_some() {
  if cx.fuel_remaining() < 2 || cx.retained_grant().maximum_items < 2 || !launch_progress.fits(cx.retained_grant()) { return Ok(false); }
  cx.consume_retained(launch_progress)?;
  cx.consume_fuel(1);
 }
 let launch = opening.natural_launch.take();
 let actor=opening.authority.take().ok_or_else"""),
    ("slot.native_created=true;slot.pending_creation.take();", "if let Some(launch)=launch{slot.install_natural_source_launch(launch);}slot.native_created=true;slot.pending_creation.take();"),
])
dispatch = reactor / "🔄️turn/📨️event-dispatch/🦀️.rs"
change(dispatch, [
    ("Event::StagedFormRequested { .. }|Event::OriginalMediaControl(_)))",
     "Event::StagedFormRequested { .. }|Event::OriginalMediaControl(_)|Event::NaturalSourcePrepare(_)|Event::NaturalSourceControl(_)))"),
    ("cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,..Default::default()})?;",
     """let copy_bytes = if matches!(events.as_ref(), Some(Event::InstanceOpen { purpose: semio_framework::kernel::ActorInstanceOpenPurpose::NaturalSource { .. }, .. })) {
                size_of::<(String, String, semio_framework::kernel::ActorInstanceOpenPurpose)>()
            } else { 0 };
            let progress = semio_framework_value::RetainedCloneProgress { copied_items: 1, copied_bytes: copy_bytes, ..Default::default() };
            if !progress.fits(grant) { return Ok(None); }
            cx.consume_retained(progress)?;"""),
    ("Event::InstanceOpen { request, app_id, actor, assets, quotas, .. }", "Event::InstanceOpen { request, app_id, actor, assets, quotas, purpose, .. }"),
    ("instance(runtime,request,app_id.0,actor,identity,cx)", "instance(runtime,request,app_id.0,actor,purpose,identity,cx)"),
    ("Event::OriginalMediaControl(_) => unreachable!", "Event::OriginalMediaControl(_) | Event::NaturalSourcePrepare(_) | Event::NaturalSourceControl(_) => unreachable!"),
])
emit("paid-open.patch")
