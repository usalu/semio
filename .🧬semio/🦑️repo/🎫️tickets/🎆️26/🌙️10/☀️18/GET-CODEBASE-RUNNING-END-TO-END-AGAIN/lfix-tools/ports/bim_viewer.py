import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
p = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs'
raw = Profile(lambda r: r, lambda r: 'scope.cx()', roots=('app', 'reopened'))
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)
# scope per test fn that builds apps
s = s.replace('    let mut app = Box::new(artifact_app_laws::new_app_with_registry::<ViewerApp<BimModelViewer>>(', f'    let mut scope = {SCOPE_T}::admit();\n    let mut app = Box::new(artifact_app_laws::new_app_with_registry::<ViewerApp<BimModelViewer>>(')
assert s.count('OriginalFixtureScope::admit()') == 2, s.count('OriginalFixtureScope::admit()')
s, n = method_rewrite(s, raw, 'dispatch_typed', 3, lambda a, r: a + ['scope.cx()'], retarget=False); print('dispatch', n)
rep = {}
s, _ = func_calls_with_cx(s, raw, [('settle_registered_typed_operation', 4, None), ('close_registered_fixture_app', 2, 'original caller paid app close')], rep); print(rep)
# the raw first-arg normalisation turned `&mut *app` into `&mut app`; both fine for Box<..> deref? keep explicit deref
s = s.replace('(&mut app, 91,', '(&mut *app, 91,').replace('close_registered_fixture_app(&mut app,', 'close_registered_fixture_app(&mut *app,').replace('close_registered_fixture_app(&mut reopened,', 'close_registered_fixture_app(&mut *reopened,')
sub('''async fn render_world(app: &mut VcsArtifactApp<ViewerApp<BimModelViewer>>, view: &ViewModel) -> Result<semio_framework_plugin::World3dScene, String> {
    let tree = app.render(semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::world::BODY_KEY, None, view).await.map_err(|error| format!("{error:?}"))?;
    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;''', '''async fn render_world(app: &mut VcsArtifactApp<ViewerApp<BimModelViewer>>, view: &ViewModel, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::World3dScene, String> {
    let json = artifact_app_laws::render_registered_fixture_app(app, semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::world::BODY_KEY, view, cx).await.map_err(|error| format!("{error:?}"))?;''')
sub('''async fn render_plan(app: &mut VcsArtifactApp<ViewerApp<BimModelViewer>>, view: &ViewModel) -> Result<semio_framework_plugin::Canvas2dScene, String> {
    let tree = app.render(semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::BODY_KEY, None, view).await.map_err(|error| format!("{error:?}"))?;
    let json = artifact_app_laws::project_and_retire_fixture_tree(tree).map_err(str::to_string)?;''', '''async fn render_plan(app: &mut VcsArtifactApp<ViewerApp<BimModelViewer>>, view: &ViewModel, cx: &mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin::Canvas2dScene, String> {
    let json = artifact_app_laws::render_registered_fixture_app(app, semio_s_artifact_bim_model_family_tests::viewer::bim::modes::view::windows::plan::BODY_KEY, view, cx).await.map_err(|error| format!("{error:?}"))?;''')
sub('render_world(target, &addressed(&view, "left")).await?', 'render_world(target, &addressed(&view, "left"), scope.cx()).await?')
sub('render_world(target, &addressed(&view, "right")).await?', 'render_world(target, &addressed(&view, "right"), scope.cx()).await?')
sub('render_plan(target, &addressed(&view, "plan")).await?', 'render_plan(target, &addressed(&view, "plan"), scope.cx()).await?')
sub('render_plan(&mut app, &addressed(&view, "plan")).await.expect("render");', 'render_plan(&mut app, &addressed(&view, "plan"), scope.cx()).await.expect("render");')
write(p, s)
