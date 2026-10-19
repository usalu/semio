import sys
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/lfix-tools/ports')
from wrapper import *
R = '/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/'
prof = Profile(lambda r: f'{r}.0', lambda r: f'{r}.1.cx()')
POL = prof.policy
IDENT = 'semio_framework_plugin::artifact_app_laws::fixture_identity()'
def sub_in(s, old, new, count=1):
    assert s.count(old) == count, (old[:90], s.count(old))
    return s.replace(old, new)

# interaction-behavior
p = R + '🔬️unit/🎮️interaction-behavior/🦀️.rs'
s = read(p)
def f(a):
    return ['&mut app.0'] + a[1:] + ['app.1.cx()'] if a[0].strip() == '&mut app' else None
s, n = func_rewrite(s, 'assert_undo_redo_round_trip', 7, f); print('undo_redo', n)
write(p, s)

# contributions-and-examples
p = R + '🔬️unit/🧩️contributions-and-examples/🦀️.rs'
s = read(p)
s = sub_in(s, '''    ::semio_framework_async::poll::resolve_ready(PluginApp::handle_command(&mut *app, &invocation, None, &semio_framework_plugin::artifact_app_laws::meta("local"))).expect("the host's setContributions command passes the host-configuration output gate");''',
f'''    let mut arguments = Some(DslValue::object(invocation.arguments.iter().map(|(name, value)| (name.clone(), value.clone()))));
    let mut fault = None;
    for _ in 0..semio_framework_plugin::artifact_app_laws::OriginalFixtureCaller::maximum_turns() {{
        let handled = ::semio_framework_async::poll::resolve_ready(PluginApp::handle_command(&mut app.0, &invocation.address, &mut arguments, &mut fault, None, &semio_framework_plugin::artifact_app_laws::meta("local"), &mut {IDENT}, app.1.cx())).expect("the host's setContributions command passes the host-configuration output gate");
        if handled.is_some() {{
            return;
        }}
        PluginApp::maintenance_step(&mut app.0, app.1.cx(), {POL}.maintenance).expect("setContributions maintenance step");
    }}
    panic!("the host's setContributions command stayed pending under its original caller");''')
s = sub_in(s, 'fn settle_member_less_archive(app: &mut context::Process3dRawApp,', 'fn settle_member_less_archive(app: &mut context::Process3dApp,')
s = sub_in(s, 'fn published_example_load(app: &mut context::Process3dRawApp,', 'fn published_example_load(app: &mut context::Process3dApp,')
s = sub_in(s, 'PluginApp::begin_document_archive_load(app, operation,', 'PluginApp::begin_document_archive_load(&mut app.0, operation,')
s = sub_in(s, 'PluginApp::poll_document_archive_load(app, operation, &mut semio_framework_plugin::artifact_app_laws::fixture_identity())', 'PluginApp::poll_document_archive_load(&mut app.0, operation, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())')
s = sub_in(s, 'PluginApp::acknowledge_document_archive_load(app, operation)', 'PluginApp::acknowledge_document_archive_load(&mut app.0, operation)')
s = sub_in(s, 'let _ = PluginApp::maintenance_step(app, 1, 4_096).expect("archive maintenance step");', f'let _ = PluginApp::maintenance_step(&mut app.0, app.1.cx(), {POL}.maintenance).expect("archive maintenance step");')
s = sub_in(s, 'PluginApp::maintenance_step(app, 1, 4_096).expect("example maintenance step");', f'PluginApp::maintenance_step(&mut app.0, app.1.cx(), {POL}.maintenance).expect("example maintenance step");')
s = sub_in(s, '::semio_framework_async::poll::resolve_ready(app.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation)).expect("example publication");', '::semio_framework_async::poll::resolve_ready(app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx())).expect("example publication");')
write(p, s)

# production-envelope
p = R + '🔬️unit/🏭️production-envelope/🦀️.rs'
s = read(p)
s = sub_in(s, 'fn admit_production_envelope(app: &mut crate::context::Process3dRawApp,', 'fn admit_production_envelope(app: &mut crate::context::Process3dApp,')
s = sub_in(s, 'fn drive_production_envelope(app: &mut crate::context::Process3dRawApp,', 'fn drive_production_envelope(app: &mut crate::context::Process3dApp,')
s = sub_in(s, 'assert!(app.seal_artifact_envelope_ingress(handle).expect(', 'assert!(app.0.seal_artifact_envelope_ingress(handle, app.1.cx()).expect(')
s = sub_in(s, 'PluginApp::maintenance_step(app, 1, store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("one Process3d production maintenance turn");', f'PluginApp::maintenance_step(&mut app.0, app.1.cx(), {POL}.maintenance).expect("one Process3d production maintenance turn");')
s = sub_in(s, 'let poll = app.advance_artifact_envelope_load(handle).expect(', 'let poll = app.0.advance_artifact_envelope_load(handle, app.1.cx()).expect(')
write(p, s)
