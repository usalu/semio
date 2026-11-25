import sys,io
p=sys.argv[1]
s=io.open(p,encoding='utf-8').read()
old_parts='''impl ContextMenuWireRequest {
    pub fn into_parts(self) -> Result<(ContextMenuRequest, ViewModel), Fault> {
        let view = match self.window_instance_id.as_deref() {
            Some(window) => self.view_state.for_window_instance(window).ok_or_else(|| plugin_internal_fault(format!("unknown context-menu window instance {window}")))?,
            None => self.view_state.for_panel(),
        };
        let request = ContextMenuRequest { menu: self.menu, surface: self.surface, window_instance_id: self.window_instance_id, point: self.point };
        Ok((request, view))
    }
}'''
new_parts='''impl ContextMenuWireRequest {
    /// 📦️ Splits the request from the decoded host view, whose ordered roots its caller must hand to retirement.
    pub fn into_parts(self) -> (ContextMenuRequest, ViewModel) {
        (ContextMenuRequest { menu: self.menu, surface: self.surface, window_instance_id: self.window_instance_id, point: self.point }, self.view_state)
    }
}
/// 🧭️ Reads the window or panel projection a context menu targets while the decoded host view keeps both ordered roots.
pub fn with_context_menu_view<R>(request: &ContextMenuRequest, view: &ViewModel, read: impl FnOnce(&ViewModel) -> R) -> Result<R, Fault> {
    match request.window_instance_id.as_deref() {
        Some(window) => view.scope_window_instance(window).map(|scoped| read(&scoped)).ok_or_else(|| plugin_internal_fault(format!("unknown context-menu window instance {window}"))),
        None => Ok(read(&view.scope_panel())),
    }
}
/// 🧾️ Answers a context menu on its scoped projection, then displaces the host view into the instance's funded retirement on every path.
pub fn answer_context_menu<PA: PluginApp>(instance: &mut AppInstance<PA>, request: &ContextMenuRequest, view: ViewModel) -> Result<Vec<ContextMenuItemSpec>, Fault> {
    let items = with_context_menu_view(request, &view, |scoped| ::semio_framework_async::poll::resolve_ready(instance.app.context_menu(request, scoped)));
    instance.surface_contexts.displace_view(view);
    items
}'''
assert s.count(old_parts)==1
s=s.replace(old_parts,new_parts)
old_call='''    let (request, view_state) = wire.into_parts().map_err(|fault| fault.message)?;
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let items = ::semio_framework_async::poll::resolve_ready(instance.app.context_menu(&request, &view_state));'''
new_call='''    let (request, view_state) = wire.into_parts();
    with_instances_mut(runtime, |list| {
        let mut instance = find_instance(list, instance_id)?;
        let items = answer_context_menu(&mut instance, &request, view_state)?;'''
assert s.count(old_call)==1
s=s.replace(old_call,new_call)
s=s.replace("use semio_plugin_plugin_program::{PluginProgram, decode_wire_serialized};","use semio_plugin_plugin_program::{AppInstance, PluginProgram, decode_wire_serialized};",1)
s=s.replace("use semio_ui_layout::layout::{ContextMenuPoint, ContextMenuRequest,","use semio_ui_layout::layout::{ContextMenuItemSpec, ContextMenuPoint, ContextMenuRequest,",1)
io.open(p,'w',encoding='utf-8',newline='').write(s)
