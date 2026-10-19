import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
L = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()')
P = prof.policy
p = L + '🧪️tests/🧰️support/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:80], s.count(old))
    s = s.replace(old, new)
sub('use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry, project_and_retire_fixture_tree};',
    'use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, dispatch_fixture_action, meta, new_app_with_registry, render_registered_fixture_app, OriginalFixtureScope};')
sub('''/// 🔁️ The settle loop's maintenance grant. `settle_registered_typed_operation` pages maintenance at
/// exactly one 4 KiB page; a lowpoly document carrying mesh content is larger than that, so its
/// close cursor never receives a grant it can act on and the operation never retires (measured
/// 2026-09-17: a 2 299-byte snapshot settles, a 4 240-byte one hangs).
const SETTLE_GRANT_BYTES: usize = 1 << 20;

''', '')
sub('''pub struct LowpolyApp(pub VcsArtifactApp<EditorApp<LowpolyPlayApp>>);
''', '''/// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
/// scope pays the caller's terminal retirement after the paid app close.
pub struct LowpolyApp(pub VcsArtifactApp<EditorApp<LowpolyPlayApp>>, pub OriginalFixtureScope);

impl LowpolyApp {
    /// 🧹️ Closes the registered app through its exact retained close state machine under the scope's original caller; closing a closed app is a no-op.
    pub fn close_fixture(&mut self) {
        close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
    }
}
''')
sub('''        if !std::thread::panicking() {
            close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
        }''', '''        if !std::thread::panicking() {
            self.close_fixture();
        }''')
sub('''    let mut app = new_app_with_registry::<EditorApp<LowpolyPlayApp>>(''', '''    let scope = OriginalFixtureScope::admit();
    let mut app = new_app_with_registry::<EditorApp<LowpolyPlayApp>>(''')
sub('''    LowpolyApp(app)
}''', '''    LowpolyApp(app, scope)
}''')
sub('''    let result = app.0.dispatch_typed(command, &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{verb} refused: {fault:?}"));
    settle(&mut app.0, &verb).await;''', '''    let result = app.0.dispatch_typed(command, &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("{verb} refused: {fault:?}"));
    settle(app, &verb).await;''')
sub('''    let args = semio_framework_value::DslValue::from(&args);
    let result = app.0.handle_action(action, Some(&args), &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{action} refused: {fault:?}"));''', '''    let args = semio_framework_value::DslValue::from(&args);
    let result = dispatch_fixture_action(&mut app.0, action, Some(args), &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("{action} refused: {fault:?}"));''')
sub('''    settle(&mut app.0, action).await;
}''', '''    settle(app, action).await;
}''')
sub('''    if let Err(fault) = app.0.dispatch_typed(command, &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await {
        return fault.message;
    }
    settle_faults(&mut app.0, &verb).await''', '''    if let Err(fault) = app.0.dispatch_typed(command, &action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await {
        return fault.message;
    }
    settle_faults(app, &verb).await''')
sub('async fn settle_faults(app: &mut VcsArtifactApp<EditorApp<LowpolyPlayApp>>, action: &str) -> Vec<String> {', 'async fn settle_faults(app: &mut LowpolyApp, action: &str) -> Vec<String> {')
sub('pub async fn settle(app: &mut VcsArtifactApp<EditorApp<LowpolyPlayApp>>, action: &str) -> usize {', 'pub async fn settle(app: &mut LowpolyApp, action: &str) -> usize {')
sub('''        let _ = app.maintenance_step(1, SETTLE_GRANT_BYTES).unwrap_or_else(|fault| panic!("{action} maintenance: {fault:?}"));
        app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));''',
    '''        let _ = app.0.maintenance_step(app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).unwrap_or_else(|fault| panic!("{action} maintenance: {fault:?}"));
        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.unwrap_or_else(|fault| panic!("{action} publication: {fault:?}"));''', 2)
sub('''    let tree = app.0.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");
    project_and_retire_fixture_tree(tree).expect("rendered fixture observation and retirement")''', '''    render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("rendered fixture observation and retirement")''')
write(p, s)

p = L + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
s, n = method_rewrite(s, prof, 'ingest_operations', 2, lambda a, r: a + [prof.cx(r)], retarget=True)
print('ingest', n)
write(p, s)

p = L + '🎮️commands/🧲️transform/🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
sub('''    let result = app.0.handle_action(verb, Some(&args), &crate::context::action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));''',
    '''    let result = semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, verb, Some(args), &crate::context::action_meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.unwrap_or_else(|fault| panic!("{verb}: {fault:?}"));''')
sub('''        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.expect("a driver turn");''',
    '''        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("a driver turn");''')
write(p, s)
