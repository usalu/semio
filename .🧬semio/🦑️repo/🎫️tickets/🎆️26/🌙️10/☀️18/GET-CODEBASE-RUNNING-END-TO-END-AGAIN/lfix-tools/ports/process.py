import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
R = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()')
POL = prof.policy
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'

def edit(p, subs):
    s = read(p)
    for old, new, *cnt in subs:
        c = cnt[0] if cnt else 1
        assert s.count(old) == c, (p[-40:], old[:90], s.count(old))
        s = s.replace(old, new)
    return s

# ---------------------------------------------------------------- support
p = R + '🧰️support/🦀️.rs'
s = edit(p, [
 ('use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members};',
  'use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, dispatch_fixture_action, meta, new_app_with_registry_and_members, render_registered_fixture_app, OriginalFixtureScope};'),
 ('pub struct Process3dApp(Process3dRawApp);\n',
  '''/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the paid app close.
pub struct Process3dApp(pub Process3dRawApp, pub OriginalFixtureScope);

impl Process3dApp {
    /// 🧹️ Closes the registered app through its exact retained close state machine under the scope's original caller; closing a closed app is a no-op.
    pub fn close_fixture(&mut self) {
        close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
    }
}
'''),
])
i = s.index('impl Drop for Process3dApp {')
j = s.index('/// ✏️ Adapts `create_process3d_app`')
s = s[:i] + '''impl Drop for Process3dApp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close_fixture();
        }
    }
}

''' + s[j:]
for old, new in [
 ('fn seed_domain_catalog_contributions(app: &mut Process3dRawApp) {', 'fn seed_domain_catalog_contributions(app: &mut Process3dApp) {'),
 ('''    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry_and_members::<EditorApp<Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(process3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())));
    ::semio_framework_async::poll::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    Process3dApp(app)''', f'''    let scope = OriginalFixtureScope::admit();
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry_and_members::<EditorApp<Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(process3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}));
    ::semio_framework_async::poll::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    Process3dApp(app, scope)'''),
 ('''    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry_and_members::<EditorApp<Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(process3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())));
    ::semio_framework_async::poll::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    seed_domain_catalog_contributions(&mut app);
    Process3dApp(app)''', f'''    let scope = OriginalFixtureScope::admit();
    let mut app = ::semio_framework_async::poll::resolve_ready(new_app_with_registry_and_members::<EditorApp<Process3dPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(process3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}));
    ::semio_framework_async::poll::resolve_ready(PluginApp::bind_instance_id(&mut app, meta("local").instance_id));
    let mut app = Process3dApp(app, scope);
    seed_domain_catalog_contributions(&mut app);
    app'''),
 ('pub fn dispatch(app: &mut Process3dRawApp,', 'pub fn dispatch(app: &mut Process3dApp,'),
 ('pub fn dispatch_with_utility(app: &mut Process3dRawApp,', 'pub fn dispatch_with_utility(app: &mut Process3dApp,'),
 ('pub fn settled_dispatch(app: &mut Process3dRawApp,', 'pub fn settled_dispatch(app: &mut Process3dApp,'),
 ('pub fn settled_dispatch_with_utility(app: &mut Process3dRawApp,', 'pub fn settled_dispatch_with_utility(app: &mut Process3dApp,'),
 ('pub fn action(app: &mut Process3dRawApp,', 'pub fn action(app: &mut Process3dApp,'),
 ('pub fn settled_action(app: &mut Process3dRawApp,', 'pub fn settled_action(app: &mut Process3dApp,'),
 ('pub fn settle(app: &mut Process3dRawApp)', 'pub fn settle(app: &mut Process3dApp)'),
 ('pub fn render(app: &mut Process3dRawApp,', 'pub fn render(app: &mut Process3dApp,'),
 ('pub fn render_with_view(app: &mut Process3dRawApp,', 'pub fn render_with_view(app: &mut Process3dApp,'),
 ('pub fn main_window_measures(app: &mut Process3dRawApp)', 'pub fn main_window_measures(app: &mut Process3dApp)'),
 ('''::semio_framework_async::poll::resolve_ready(app.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).expect("dispatch");''',
  '''::semio_framework_async::poll::resolve_ready(app.0.dispatch_typed(command, &action_meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("dispatch");'''),
 ('''    let admitted = ::semio_framework_async::poll::resolve_ready(app.handle_action(action, args, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).expect("action dispatch");''',
  f'''    let admitted = ::semio_framework_async::poll::resolve_ready(dispatch_fixture_action(&mut app.0, action, args.cloned(), &meta("local"), &mut {IDENT}, app.1.cx(), {POL}.close)).expect("action dispatch");'''),
 ('''::semio_framework_async::poll::resolve_ready(semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted)).expect("settle the framework-reserved admission");''',
  '''::semio_framework_async::poll::resolve_ready(semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted)).expect("settle the framework-reserved admission");'''),
 ('''::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())).expect("settle the typed operation")''',
  '''::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("settle the typed operation")'''),
 ('''    let tree = ::semio_framework_async::poll::resolve_ready(app.render(body_key, None, view_state)).expect("render");
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("render projection")''',
  '''    ::semio_framework_async::poll::resolve_ready(render_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx())).expect("render projection")'''),
]:
    assert s.count(old) == 1, old[:80]
    s = s.replace(old, new)
write(p, s)
print('support ok')
