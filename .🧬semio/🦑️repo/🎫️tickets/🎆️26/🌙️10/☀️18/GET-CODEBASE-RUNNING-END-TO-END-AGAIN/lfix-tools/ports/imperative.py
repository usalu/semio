import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
p = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=('app', 'instance_a', 'instance_b', 'sender', 'receiver'))
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)
sub('    pub async fn imperative_app_with_registry() -> OwnedImperativeApp {\n        let mut app = new_app_with_registry::', f'    pub async fn imperative_app_with_registry() -> OwnedImperativeApp {{\n        let scope = {SCOPE_T}::admit();\n        let mut app = new_app_with_registry::')
sub('        OwnedImperativeApp(app)\n', '        OwnedImperativeApp(app, scope)\n')
sub('    pub struct OwnedImperativeApp(ImperativeApp);\n', f'    /// Field `1` is the fixture\'s ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the\n    /// scope pays the caller\'s terminal retirement after the paid app close.\n    pub struct OwnedImperativeApp(pub ImperativeApp, pub {SCOPE_T});\n')
sub('            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);\n        }\n    }\n\n    impl std::ops::Deref for OwnedImperativeApp',
    '            semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");\n        }\n    }\n\n    impl std::ops::Deref for OwnedImperativeApp')
for h in ['settle', 'close', 'dispatch', 'render']:
    sub(f'pub async fn {h}(app: &mut ImperativeApp', f'pub async fn {h}(app: &mut OwnedImperativeApp') if h != 'close' else sub('pub fn close(app: &mut ImperativeApp)', 'pub fn close(app: &mut OwnedImperativeApp)')
s, counts = standard_calls(s, prof)
print(counts)
write(p, s)
