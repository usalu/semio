import sys, re
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
P = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/📖️playbook/'
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'
POL = POLICY
# ---- main editor unit
import glob
main = [f for f in glob.glob(P + '🗿️artifacts/*/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs')]
assert len(main) == 1
p = main[0]
s, counts, roots = wrapper_port(p, 'PlaybookApp', 'MountedPlaybookApp')
s = read(p); s, n = ctor_fix(s); write(p, s)
print('playbook unit', counts, roots, n)
# other playbook editor user files (stale scan decides)
# ---- procedural extension
p = P + '🧩️extensions/🌀️procedural/🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:90], s.count(old))
    s = s.replace(old, new)
sub('''async fn new_app() -> VcsArtifactApp<ModuleApp> {
    let definition = create_module_app().await.expect("module definition").definition;
    let mut app = VcsArtifactApp::with_registry(ModuleApp, AppActionRegistry::from_definition(&definition), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    app.bind_instance_id(meta().instance_id).await;
    app
}''', f'''/// 🧹️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
struct ModuleTestApp(VcsArtifactApp<ModuleApp>, {SCOPE_T});

impl std::ops::Deref for ModuleTestApp {{
    type Target = VcsArtifactApp<ModuleApp>;
    fn deref(&self) -> &Self::Target {{
        &self.0
    }}
}}

impl std::ops::DerefMut for ModuleTestApp {{
    fn deref_mut(&mut self) -> &mut Self::Target {{
        &mut self.0
    }}
}}

async fn new_app() -> ModuleTestApp {{
    let definition = create_module_app().await.expect("module definition").definition;
    let scope = {SCOPE_T}::admit();
    let mut app = VcsArtifactApp::with_registry(ModuleApp, AppActionRegistry::from_definition(&definition), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}).await;
    semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut app, {POL});
    app.bind_instance_id(meta().instance_id).await;
    ModuleTestApp(app, scope)
}}''')
sub('''async fn act(app: &mut VcsArtifactApp<ModuleApp>, action: &str, args: Option<&DslValue>) {
    app.handle_action(action, args, &meta(), &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await.unwrap_or_else(|fault| panic!("{action}: {fault:?}"));''', f'''async fn act(app: &mut ModuleTestApp, action: &str, args: Option<&DslValue>) {{
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, action, args.cloned(), &meta(), &mut {IDENT}, app.1.cx(), {POL}.close).await.unwrap_or_else(|fault| panic!("{{action}}: {{fault:?}}"));''')
sub('''    let mut app: VcsArtifactApp<ModuleApp> = VcsArtifactApp::with_registry(ModuleApp, registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
    app.bind_instance_id(meta().instance_id).await;''', f'''    let scope = {SCOPE_T}::admit();
    let mut app = VcsArtifactApp::with_registry(ModuleApp, registry, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}).await;
    semio_framework_plugin::artifact_app_laws::settle_store_constructor(&mut app, {POL});
    app.bind_instance_id(meta().instance_id).await;
    let mut app = ModuleTestApp(app, scope);''')
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()', roots=('app',))
s, c = standard_calls(s, prof); print('procedural', c)
write(p, s)
