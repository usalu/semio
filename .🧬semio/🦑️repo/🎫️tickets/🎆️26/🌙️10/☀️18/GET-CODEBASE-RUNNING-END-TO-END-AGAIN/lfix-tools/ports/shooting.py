import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
E = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=('app',))
POL = prof.policy
p = E + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)
sub('    pub struct ShootingApp(VcsArtifactApp<EditorApp<ShootingPlayApp>>);\n', f'    /// Field `1` is the fixture\'s ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the\n    /// scope pays the caller\'s terminal retirement after the paid app close.\n    pub struct ShootingApp(pub VcsArtifactApp<EditorApp<ShootingPlayApp>>, pub {SCOPE_T});\n')
sub('                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);',
    '                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");')
sub('    pub async fn shooting_app() -> ShootingApp {\n        let mut app = new_app_with_registry::', f'    pub async fn shooting_app() -> ShootingApp {{\n        let scope = {SCOPE_T}::admit();\n        let mut app = new_app_with_registry::')
sub('        ShootingApp(app)\n', '        ShootingApp(app, scope)\n')
sub('''        app.maintenance_step(semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: policy.maximum_items,''', '''        app.0.maintenance_step(app.1.cx(), semio_framework_value::retained_clone::RetainedCloneGrant { maximum_items: policy.maximum_items,''')
# trees
sub('''        let tree = app.render(SHOOTING_PLAY_BODY_SCENE, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render scene");
        let decoded = semio_framework_plugin::artifact_app_laws::built_surface_scene(&tree.root);
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire scene tree");
        decoded.expect("assembled 3D scene")''', '''        semio_framework_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, SHOOTING_PLAY_BODY_SCENE, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx(), |root| semio_framework_plugin::artifact_app_laws::built_surface_scene(root)).await.expect("render scene").expect("assembled 3D scene")''')
sub('''        let tree = app.render(SHOOTING_PLAY_BODY_ICON, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render icon");
        let decoded = match &tree.root.component {
            semio_framework_plugin::Component::Surface(props) => semio_framework_ui_scene::decode(props),
            _ => panic!("icon surface"),
        };
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire icon tree");
        decoded.expect("packed icon scene")''', '''        semio_framework_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, SHOOTING_PLAY_BODY_ICON, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx(), |root| match &root.component {
            semio_framework_plugin::Component::Surface(props) => semio_framework_ui_scene::decode(props),
            _ => panic!("icon surface"),
        }).await.expect("render icon").expect("packed icon scene")''')
s, counts = standard_calls(s, prof)
print('unit', counts)
write(p, s)
p = E + '🎮️commands/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
s, counts = standard_calls(s, prof)
print('gumball', counts)
write(p, s)
