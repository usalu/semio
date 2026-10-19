import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
CUR = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()')
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'

# ---- context module + unit file
p = CUR + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:70], s.count(old))
    s = s.replace(old, new)

sub('''    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry_and_members as new_app_with_registry_impl};''',
    '''    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry_and_members as new_app_with_registry_impl, observe_registered_fixture_app, render_registered_fixture_app, OriginalFixtureScope};''')
sub('''        let mut app = new_app_with_registry_impl::<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>(sourcing_manifest_for_tests, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into())).await;''',
    f'''        let scope = OriginalFixtureScope::admit();
        let mut app = new_app_with_registry_impl::<EditorApp<SourcingCurationApp>, semio_s_artifact_stdio_semio::SemioMembers>(sourcing_manifest_for_tests, protocol::ActorId(protocol::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut {IDENT}).await;''')
sub('''        SourcingTestApp(app)
    }''', '''        SourcingTestApp(app, scope)
    }''')
sub('''    /// the teardown impossible to forget, and it is idempotent: a test that closes explicitly (see
    /// `retained_example_load_publishes_authored_stock_and_closes_exact_owners`) leaves nothing to do.
    pub struct SourcingTestApp(SourcingApp);
''', '''    /// the teardown impossible to forget, and it is idempotent: a test that closes explicitly (see
    /// `retained_example_load_publishes_authored_stock_and_closes_exact_owners`) leaves nothing to do.
    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads
    /// `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct SourcingTestApp(pub SourcingApp, pub OriginalFixtureScope);

    impl SourcingTestApp {
        /// 🧹️ Closes the registered app through its exact retained close state machine under the scope's original caller; closing a closed app is a no-op.
        pub fn close_fixture(&mut self) {
            close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
        }
    }
''')
i = s.index('    impl Drop for SourcingTestApp {')
j = s.index('    /// 🎬️ Dispatches one typed command and drives its retained operation')
s = s[:i] + '''    impl Drop for SourcingTestApp {
        fn drop(&mut self) {
            if !std::thread::panicking() {
                self.close_fixture();
            }
        }
    }
    
''' + s[j:]
sub('pub async fn dispatch(app: &mut SourcingApp, command: SourcingCurationCommand) -> InvocationResult {', 'pub async fn dispatch(app: &mut SourcingTestApp, command: SourcingCurationCommand) -> InvocationResult {')
sub('pub async fn settle(app: &mut SourcingApp) -> Vec<semio_framework::kernel::Effect> {', 'pub async fn settle(app: &mut SourcingTestApp) -> Vec<semio_framework::kernel::Effect> {')
sub('pub async fn render(app: &mut SourcingApp, body_key: &str) -> String {\n        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render")).expect("bounded retained fixture projection")\n    }',
    'pub async fn render(app: &mut SourcingTestApp, body_key: &str) -> String {\n        render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("bounded retained fixture projection")\n    }')
sub('pub async fn table_of(app: &mut SourcingApp, body_key: &str) -> (Vec<String>, Vec<serde_json::Value>) {\n        let rendered = app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render");\n        let scene = table_scene_of(&rendered.root).expect("a table scene in the rendered body");\n        let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");\n        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");\n        (columns.iter().map(|column| column["id"].as_str().unwrap_or_default().to_string()).collect(), rows)\n    }',
    'pub async fn table_of(app: &mut SourcingTestApp, body_key: &str) -> (Vec<String>, Vec<serde_json::Value>) {\n        observe_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx(), |root| {\n            let scene = table_scene_of(root).expect("a table scene in the rendered body");\n            let columns: Vec<serde_json::Value> = serde_json::from_str(&scene.columns_json).expect("columns json");\n            let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows json");\n            (columns.iter().map(|column| column["id"].as_str().unwrap_or_default().to_string()).collect(), rows)\n        }).await.expect("render")\n    }')
sub('pub async fn row_ids(app: &mut SourcingApp, body_key: &str)', 'pub async fn row_ids(app: &mut SourcingTestApp, body_key: &str)')
# dispatch / settle internals
sub('''        let result = app.dispatch_typed(command, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.expect("dispatch");''',
    '''        let result = app.0.dispatch_typed(command, &meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");''')
sub('''            app.maintenance_step(1, 4_096).expect("maintenance step");
            app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation).await.expect("typed operation publication");''',
    '''            app.0.maintenance_step(app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("maintenance step");
            app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("typed operation publication");''')
write(p, s)
print('context ok')

