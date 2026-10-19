import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
V = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=('app', 'instance'))
p = V + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:80], s.count(old))
    s = s.replace(old, new)
sub('    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};', '    use semio_framework_plugin::artifact_app_laws::{close_registered_fixture_app, meta, new_app_with_registry, render_registered_fixture_app, OriginalFixtureScope};')
sub('''    pub struct VcsApp(pub(crate) VcsArtifactApp<EditorApp<VcsPlayApp>>);
''', '''    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `instance.1.cx()`,
    /// and the scope pays the caller's terminal retirement after the paid app close.
    pub struct VcsApp(pub(crate) VcsArtifactApp<EditorApp<VcsPlayApp>>, pub(crate) OriginalFixtureScope);

    impl VcsApp {
        /// 🧹️ Closes the registered app through its exact retained close state machine under the scope's original caller; closing a closed app is a no-op.
        pub fn close_fixture(&mut self) {
            close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");
        }
    }
''')
sub('''            if !std::thread::panicking() {
                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);
            }''', '''            if !std::thread::panicking() {
                self.close_fixture();
            }''')
sub('''        let mut instance = new_app_with_registry::<EditorApp<VcsPlayApp>>(''', '''        let scope = OriginalFixtureScope::admit();
        let mut instance = new_app_with_registry::<EditorApp<VcsPlayApp>>(''')
sub('        let mut instance = VcsApp(instance);', '        let mut instance = VcsApp(instance, scope);')
sub('''        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(instance.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render")).expect("render json")''',
    '''        render_registered_fixture_app(&mut instance.0, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), instance.1.cx()).await.expect("render json")''')
s, n = method_rewrite(s, prof, 'dispatch_typed', 3, lambda a, r: a + [prof.cx(r)]); print('dispatch', n)
s, n = func_rewrite(s, 'settle_registered_typed_operation', 4, lambda a: a + ['instance.1.cx()'] if root_name(a[0]) in prof.roots else None); print('settle', n)
s, n = handle_action_rewrite(s, prof); print('handle_action', n)
write(p, s)
