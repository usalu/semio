import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
N = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=('app',))
POL = prof.policy
p = N + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)
sub('    pub async fn note_app_with_registry_id(instance_id: u32) -> OwnedNoteApp {\n        let mut app = new_app_with_registry::', f'    pub async fn note_app_with_registry_id(instance_id: u32) -> OwnedNoteApp {{\n        let scope = {SCOPE_T}::admit();\n        let mut app = new_app_with_registry::')
sub('        OwnedNoteApp(app)\n', '        OwnedNoteApp(app, scope)\n')
sub('    pub struct OwnedNoteApp(NoteApp);\n', f'    /// Field `1` is the fixture\'s ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the\n    /// scope pays the caller\'s terminal retirement after the paid app close.\n    pub struct OwnedNoteApp(pub NoteApp, pub {SCOPE_T});\n')
sub('            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);\n        }\n    }\n\n    impl std::ops::Deref for OwnedNoteApp',
    '            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");\n        }\n    }\n\n    impl std::ops::Deref for OwnedNoteApp')
for h in ['settle', 'dispatch', 'dispatch_with_view', 'dispatch_with_view_for_instance', 'render', 'render_with_view', 'select_blocks', 'close_app']:
    sub(f'fn {h}(app: &mut NoteApp', f'fn {h}(app: &mut OwnedNoteApp')
# settle: maintenance + publication
i = s.index('            let demand = PluginApp::maintenance_retirement_demands(app,')
j = s.index('            if let Some(page) = app.take_typed_operation_result_page(receiver) {')
s = s[:i] + f'''            PluginApp::maintenance_step(&mut app.0, app.1.cx(), {POL}.maintenance).map_err(|failure| Fault::from(format!("{{failure:?}}")))?;
            app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.map_err(|failure| Fault::from(format!("{{failure:?}}")))?;
''' + s[j:]
# close_app: the framework's exact retained close
i = s.index('    pub async fn close_app(app: &mut OwnedNoteApp) {')
j = s.index('}\n}\n\nuse crate::editor::note::*;')
s = s[:i] + f'''    pub async fn close_app(app: &mut OwnedNoteApp) {{
        app.close();
    ''' + s[j:]
# select_blocks handle_action
sub('''        let result = app.handle_action("interactionSelect", args.as_ref(), &action_meta("test", 1, composite_view(NOTE_PLAY_WINDOW_COMPOSITE)), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;''',
    f'''        let result = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "interactionSelect", args, &action_meta("test", 1, composite_view(NOTE_PLAY_WINDOW_COMPOSITE)), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), {POL}.close).await.map_err(|failure| Fault::from(format!("{{failure:?}}")));''')
s, counts = standard_calls(s, prof)
print('unit', counts)
write(p, s)

for rel in ['📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs', '🎮️commands/🖊️ink-apply-events/🧪️tests/🔬️unit/🦀️.rs', '🎮️commands/🎥️set-camera/🧪️tests/🔬️unit/🦀️.rs', '🎮️commands/➕️add-block/🧪️tests/🔬️unit/🦀️.rs']:
    s = read(N + rel)
    s, counts = standard_calls(s, prof)
    print(rel[-30:], counts)
    write(N + rel, s)
