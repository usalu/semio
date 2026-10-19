import sys, re
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
D = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/'
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'
p = D + '🧪️tests/🔬️unit/🦀️.rs'
s = read(p)
s, ty = alias_to_tuple(s, 'DagApp', vis='pub ')
old = '''        framework_new_app_with_registry::<EditorApp<DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(dag_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await
'''
assert s.count(old) == 1
s = s.replace(old, f'''        let scope = {SCOPE_T}::admit();
        let app = framework_new_app_with_registry::<EditorApp<DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>(dag_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POLICY}, &mut {IDENT}).await;
        DagApp(app, scope)
''')
write(p, s)
for rel in ['🧪️tests/🔬️unit/🦀️.rs', '🎭️modes/✏️edit/🛠️tools/🗂️reorganize/🧪️tests/🔬️unit/🦀️.rs', '🎮️commands/✏️node-graph-edit/🧪️tests/🔬️unit/🦀️.rs', '🎮️commands/➕️add-node/🧪️tests/🔬️unit/🦀️.rs']:
    s, counts, roots = user_port(D + rel, 'DagApp', 'DagApp', rename_raw=False, extra_roots=('app', 'fresh', 'instance_a', 'instance_b', 'sender', 'receiver'))
    print(rel[-30:], counts, roots)
