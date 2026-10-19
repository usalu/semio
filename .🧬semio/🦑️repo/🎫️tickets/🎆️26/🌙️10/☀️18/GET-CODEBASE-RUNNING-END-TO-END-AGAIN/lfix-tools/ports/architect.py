import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
E = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=('app',))
POL = prof.policy
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'

# ---- unit
p = E + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)
sub('    pub struct ArchitectApp(MountedArchitectApp);\n', f'    /// Field `1` is the fixture\'s ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the\n    /// scope pays the caller\'s terminal retirement after the paid app close.\n    pub struct ArchitectApp(pub MountedArchitectApp, pub {SCOPE_T});\n')
sub('                close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);', '                close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");')
import re
m = re.search(r'(    pub async fn app_with_registry\(\) -> ArchitectApp \{\n)(        let mut app = new_app_with_registry_and_members::<EditorApp<ArchitectPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>\(architect_app_manifest_for_tests, semio_framework_os_kernel::ActorId\(semio_framework_os_kernel::LOCAL_ACTOR_ID.into\(\)\))(\)\.await;)', s)
assert m, 'ctor'
s = s.replace(m.group(0), m.group(1) + f'        let scope = {SCOPE_T}::admit();\n' + m.group(2) + f', {POL}, &mut {IDENT}' + m.group(3))
sub('        ArchitectApp(app)\n', '        ArchitectApp(app, scope)\n')
s, counts = standard_calls(s, prof)
print('unit', counts)
write(p, s)

# ---- window ownership
p = E + '🧪️tests/🔬️window-ownership/🦀️.rs'
s = read(p)
sub('''                async fn dispatch(app: &mut ArchitectApp, command: ArchitectCommand, view: Option<&ViewModel>) -> Result<TypedOperationFixtureReceipt, String> {''', '''                async fn dispatch(app: &mut ArchitectApp, command: ArchitectCommand, view: Option<&ViewModel>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<TypedOperationFixtureReceipt, String> {''')
sub('''app.dispatch_typed(command, &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.map_err(|error| format!("{error:?}"))?;''', '''app.dispatch_typed(command, &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), cx).await.map_err(|error| format!("{error:?}"))?;''')
s, n = func_rewrite(s, 'settle_registered_typed_operation', 4, lambda a: a + ['cx']); print('settle', n)
sub('''                async fn render(app: &mut ArchitectApp, body_key: &str, view: &ViewModel) -> Result<String, String> {
                    let tree = app.render(body_key, None, view).await.map_err(|error| format!("{error:?}"))?;
                    artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)''', '''                async fn render(app: &mut ArchitectApp, body_key: &str, view: &ViewModel, cx: &mut semio_framework_job::StepContext<'_>) -> Result<String, String> {
                    artifact_app_laws::render_registered_fixture_app(app, body_key, view, cx).await.map_err(|error| format!("{error:?}"))''')
sub('''                let mut app = Box::new(artifact_app_laws::new_app_with_registry_and_members::<''', f'''                let mut scope = artifact_app_laws::OriginalFixtureScope::admit();
                let mut app = Box::new(artifact_app_laws::new_app_with_registry_and_members::<''')
# dispatch / render call sites (multi-line aware)
def add_scope_cx(src, name, arity):
    calls = [c for c in find_calls(src, name, 'func', arity) if c.args[0].strip() in ('&mut app', '&mut reopened')]
    for c in reversed(calls):
        src = src[:c.open + 1] + c.raw.rstrip().rstrip(',') + ', scope.cx()' + src[c.close:]
    return src, len(calls)
s, n1 = add_scope_cx(s, 'dispatch', 3); s, n2 = add_scope_cx(s, 'render', 3); print('ws dispatch/render', n1, n2)
sub('''                    artifact_app_laws::close_registered_fixture_app(&mut *reopened, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);''', f'''                    artifact_app_laws::close_registered_fixture_app(&mut *reopened, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).map_err(|error| format!("{{error:?}}"))?;''')
sub('''                artifact_app_laws::close_registered_fixture_app(&mut *app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);''', f'''                artifact_app_laws::close_registered_fixture_app(&mut *app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, scope.cx()).expect("original caller paid app close");''')
write(p, s)
