from devlib import *
P = EXT_ROOT + '🧩️composition/🧪️tests/🧊️generation3d/🧪️tests/🔬️brep-extension/🦀️.rs'
s = read(P)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:100], s.count(old))
    s = s.replace(old, new)
POL = 'semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT'
CX = "cx: &mut semio_framework_job::StepContext<'_>"
sub('''    use semio_framework_plugin::{ExtensionResourceOwner, PluginCloseStep};
    let mut geometry''', f'''    use semio_framework_plugin::{{ExtensionResourceOwner, PluginLifecycleStep}};
    let mut scope = {SCOPE_T}::admit();
    let mut geometry''')
sub('''            if owner.close_step(8, 4096).expect("geometry fixture owner closure") == PluginCloseStep::Complete && owner.terminal_is_empty() { break; }''',
    '''            if matches!(owner.close_step(scope.cx()).expect("geometry fixture owner closure"), PluginLifecycleStep::Complete(_)) && owner.terminal_is_empty() { break; }''')
sub('''            if owner.close_step(8, 4096).expect("math fixture owner closure") == PluginCloseStep::Complete && owner.terminal_is_empty() { break; }''',
    '''            if matches!(owner.close_step(scope.cx()).expect("math fixture owner closure"), PluginLifecycleStep::Complete(_)) && owner.terminal_is_empty() { break; }''')
sub('''pub async fn settle<P: PluginApp>(app: &mut P, receiver: u32, action_meta: &ActionMeta) -> SettledExtensionInvocations {
    settle_extension_invocations(app, receiver, action_meta, &mut |pending| {
        let outcome = serve(pending);
        outcome
    })
    .await''', f'''pub async fn settle<P: PluginApp>(app: &mut P, receiver: u32, action_meta: &ActionMeta, {CX}) -> SettledExtensionInvocations {{
    settle_extension_invocations(app, receiver, action_meta, &mut |pending| {{
        let outcome = serve(pending);
        outcome
    }}, {POL}, &mut {IDENT}, cx)
    .await''')
sub('''pub async fn dispatch_effect_command<P: PluginApp>(app: &mut P, command_id: &str, args: Option<&semio_framework_value::DslValue>, action_meta: &ActionMeta) -> Result<(), Fault> {''',
    f'''pub async fn dispatch_effect_command<P: PluginApp>(app: &mut P, command_id: &str, args: Option<&semio_framework_value::DslValue>, action_meta: &ActionMeta, {CX}) -> Result<(), Fault> {{''')
sub('''    use semio_framework::manifest::{CommandAddress, CommandInvocation, CommandOwnerAddress};
    let arguments = match args {
        Some(semio_framework_value::DslValue::Object(entries)) => entries.iter().cloned().collect(),
        _ => std::collections::BTreeMap::new(),
    };
    let app_id = app.app_id().await.to_string();
    let invocation = CommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::App { plugin_id: String::new(), app_id }, command_id: command_id.to_string() }, arguments };
    app.handle_command(&invocation, None, action_meta).await.map(|_| ())''', f'''    use semio_framework::manifest::{{CommandAddress, CommandOwnerAddress}};
    let arguments = match args {{
        Some(arguments @ semio_framework_value::DslValue::Object(_)) => Some(arguments.clone()),
        _ => None,
    }};
    let app_id = app.app_id().await.to_string();
    let address = CommandAddress {{ owner: CommandOwnerAddress::App {{ plugin_id: String::new(), app_id }}, command_id: command_id.to_string() }};
    {LAWS_T}::dispatch_fixture_command(app, &address, arguments, None, action_meta, &mut {IDENT}, cx, {POL}.close).await.map(|_| ()).map_err(|failure| Fault::from(format!("{{failure:?}}")))''')
sub('''initial: &[semio_framework_plugin::Effect], serve: &mut dyn FnMut(&PendingExtensionInvocation) -> Result<Vec<u8>, Fault>) -> PreviewRunReceipt {''',
    f'''initial: &[semio_framework_plugin::Effect], serve: &mut dyn FnMut(&PendingExtensionInvocation) -> Result<Vec<u8>, Fault>, {CX}) -> PreviewRunReceipt {{''')
sub('''            effects.extend(settle_registered_typed_operation(app, action_meta.instance_id).await.expect("the run's driver turns settle").effects);''',
    f'''            effects.extend(settle_registered_typed_operation(app, action_meta.instance_id, {POL}, &mut {IDENT}, cx).await.expect("the run's driver turns settle").effects);''')
sub('''settle_extension_invocations(app, action_meta.instance_id, &action_meta, serve).await.expect("in-process extension round trip");''',
    f'''settle_extension_invocations(app, action_meta.instance_id, &action_meta, serve, {POL}, &mut {IDENT}, cx).await.expect("in-process extension round trip");''')
sub('''                app.handle_action(&action, args.as_ref(), &action_meta).await.unwrap_or_else(|fault| panic!("{action} dispatch: {fault:?}"));''',
    f'''                {DFA_T}(app, &action, args.clone(), &action_meta, &mut {IDENT}, cx, {POL}.close).await.unwrap_or_else(|fault| panic!("{{action}} dispatch: {{fault:?}}"));''')
sub('''            dispatch_effect_command(app, &action, args.as_ref(), &action_meta).await.unwrap_or_else(''', '''            dispatch_effect_command(app, &action, args.as_ref(), &action_meta, cx).await.unwrap_or_else(''')
sub('''            let hop = settle_registered_typed_operation(app, action_meta.instance_id).await.expect("retained publication");''',
    f'''            let hop = settle_registered_typed_operation(app, action_meta.instance_id, {POL}, &mut {IDENT}, cx).await.expect("retained publication");''')
write(P, s)
print('brep done')
