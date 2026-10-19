import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
T = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=('app',))
p = T + '🧰️fixture/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)
sub('    pub struct BimApp(VcsArtifactApp<EditorApp<BimModelApp>>);\n', f'    /// Field `1` is the fixture\'s ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the\n    /// scope pays the caller\'s terminal retirement after the paid app close.\n    pub struct BimApp(pub VcsArtifactApp<EditorApp<BimModelApp>>, pub {SCOPE_T});\n')
sub('                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);', '                semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, self.1.cx()).expect("original caller paid app close");')
sub('    pub async fn bim_app() -> BimApp {\n        let mut app = new_app_with_registry::', f'    pub async fn bim_app() -> BimApp {{\n        let scope = {SCOPE_T}::admit();\n        let mut app = new_app_with_registry::')
sub('        BimApp(app)\n', '        BimApp(app, scope)\n')
sub('''        let tree = app.render(semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::BODY_KEY, None, view).await.expect("render world");
        let decoded = semio_framework_plugin::artifact_app_laws::built_surface_scene(&tree.root);
        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire world tree");
        decoded.expect("assembled 3D scene")''', '''        semio_framework_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::BODY_KEY, view, app.1.cx(), |root| semio_framework_plugin::artifact_app_laws::built_surface_scene(root)).await.expect("render world").expect("assembled 3D scene")''')
s, counts = standard_calls(s, prof)
print('fixture', counts)
write(p, s)
p = T + '⌨️completeness/🦀️.rs'
s = read(p)
sub('''    let tree = app.render(body, None, view).await.expect("render");
    let name = tree.root.accessibility.label.as_ref().map(|label| label.0.as_str().to_string());
    let description = tree.root.accessibility.description.as_ref().map(|label| label.0.as_str().to_string());
    semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("retire");
    (name, description)''', '''    semio_framework_plugin::artifact_app_laws::observe_registered_fixture_app(&mut app.0, body, view, app.1.cx(), |root| {
        let name = root.accessibility.label.as_ref().map(|label| label.0.as_str().to_string());
        let description = root.accessibility.description.as_ref().map(|label| label.0.as_str().to_string());
        (name, description)
    }).await.expect("render")''')
write(p, s)
