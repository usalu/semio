import sys, re, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wrapper import *
R = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/'
G3 = R + '🧊️generation3d/'
G2 = R + '🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/'
f3 = G3 + '🧪️tests/🔬️app-fixture/🦀️.rs'
s, c, r = wrapper_port(f3, 'Generation3dAppFixture', 'Generation3dApp', extra_roots=('app',))
print('g3 fixture', c, r)
f2 = G2 + '✏️editor/🧪️tests/🔬️app-fixture/🦀️.rs'
s = read(f2)
s, ty = alias_to_tuple(s, 'Generation2dApp', vis='pub ')
old = '    let mut app = new_app_with_registry::<EditorApp<Generation2dPlayApp>>('
assert s.count(old) == 1
s = s.replace(old, f'    let scope = {SCOPE_T}::admit();\n' + old)
old = '    app.bind_instance_id(1).await;\n    app\n}'
assert s.count(old) == 1
s = s.replace(old, '    app.bind_instance_id(1).await;\n    Generation2dApp(app, scope)\n}')
old = 'close_registered_fixture_app(&mut app, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT);'
assert s.count(old) == 1
s = s.replace(old, 'close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");')
old = '''            Box::pin(app.handle_command(&invocation, None, action_meta)).await.unwrap_or_else(|fault| panic!("the shell redispatches the run's {action} hop: {fault:?}"));'''
assert s.count(old) == 1
s = s.replace(old, f'''            Box::pin(semio_framework_plugin::artifact_app_laws::dispatch_fixture_command(&mut app.0, &invocation.address, Some(semio_framework_value::DslValue::object(invocation.arguments.iter().map(|(name, value)| (name.clone(), value.clone())))), None, action_meta, &mut {IDENT}, app.1.cx(), {POLICY}.close)).await.unwrap_or_else(|fault| panic!("the shell redispatches the run's {{action}} hop: {{fault:?}}"));''')
write(f2, s)
prof = Profile(lambda x: f'{x}.0', lambda x: f'{x}.1.cx()', roots=('app',))
s = read(f2); s, counts = standard_calls(s, prof); write(f2, s); print('g2 fixture', counts)
