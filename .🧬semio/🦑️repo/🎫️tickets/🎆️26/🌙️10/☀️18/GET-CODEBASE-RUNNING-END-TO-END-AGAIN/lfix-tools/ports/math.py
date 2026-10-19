import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
M = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()')
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'
POL = 'semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT'

def edit(p, subs, fn=None):
    s = read(p)
    for old, new, *cnt in subs:
        c = cnt[0] if cnt else 1
        assert s.count(old) == c, (p[-30:], old[:80], s.count(old))
        s = s.replace(old, new)
    if fn:
        s = fn(s)
    write(p, s)

def unit(s):
    s, n = method_rewrite(s, prof, 'dispatch_typed', 3, lambda a, r: a + [prof.cx(r)]); print('unit dispatch', n)
    return s
edit(M + '🧪️tests/🔬️unit/🦀️.rs', [
 ('    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members};',
  '    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry_and_members, render_registered_fixture_app, OriginalFixtureScope};'),
 ('''        let mut app = new_app_with_registry_and_members::<EditorApp<EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(equation_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;''',
  f'''        let scope = OriginalFixtureScope::admit();
        let mut app = new_app_with_registry_and_members::<EditorApp<EquationPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(equation_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}).await;'''),
 ('        OwnedMathApp(app)\n', '        OwnedMathApp(app, scope)\n'),
 ('    pub async fn settle(app: &mut MathApp) {\n        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(app, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("settle the typed operation");',
  '    pub async fn settle(app: &mut OwnedMathApp) {\n        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");'),
 ('    pub struct OwnedMathApp(MathApp);\n', '    /// Field `1` is the fixture\'s ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the\n    /// scope pays the caller\'s terminal retirement after the paid app close.\n    pub struct OwnedMathApp(pub MathApp, pub OriginalFixtureScope);\n'),
 ('            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);',
  '            close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");'),
 ('    pub async fn dispatch(app: &mut MathApp, command: EquationCommand) -> InvocationResult {\n        let result = app.dispatch_typed(command, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("dispatch");',
  '    pub async fn dispatch(app: &mut OwnedMathApp, command: EquationCommand) -> InvocationResult {\n        let result = app.0.dispatch_typed(command, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");'),
 ('''    pub async fn render(app: &mut MathApp, body_key: &str) -> String {
        // 🌱️ `UiNode` (`semio-framework-plugin`, framework-owned) has not itself gained `ToValue` —
        // `Debug` gives every test caller here the same "does the render mention X" substring check.
        format!("{:?}", app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render"))''',
  '''    pub async fn render(app: &mut OwnedMathApp, body_key: &str) -> String {
        render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("render")'''),
], unit)
print('unit ok')

# commands test
p = M + '🎮️commands/🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
s, n = method_rewrite(s, prof, 'dispatch_typed', 3, lambda a, r: a + [prof.cx(r)]); print('cmd dispatch', n)
old = '''&mut app,
        EquationCommand::AddNode(AddNode { x: 1.0, y: 2.0 }),'''
assert s.count(old) == 1
s = s.replace(old, '''&mut app.0,
        EquationCommand::AddNode(AddNode { x: 1.0, y: 2.0 }),''')
old = '''before + 1,semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()
    )'''
assert s.count(old) == 1
s = s.replace(old, '''before + 1,semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()
    )''')
s = s.replace('async fn edit_rows(app: &mut MathApp)', 'async fn edit_rows(app: &mut OwnedMathApp)')
s = s.replace('use crate::editor::equation::unit_tests::context::{dispatch, math_app, MathApp};', 'use crate::editor::equation::unit_tests::context::{dispatch, math_app, OwnedMathApp};')
write(p, s)
