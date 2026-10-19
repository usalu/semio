from devlib import *
R = EXT_ROOT + '🧩️composition/🧪️tests/🎬️sequence-runtime/'
P = R + '🦀️.rs'
s = read(P)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:100], s.count(old))
    s = s.replace(old, new)
POL = 'semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT'
sub('    pub struct SequenceApp(pub(crate) VcsArtifactApp<EditorApp<SequencePlayApp>, SemioMembers>);\n', f'''    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct SequenceApp(pub(crate) VcsArtifactApp<EditorApp<SequencePlayApp>, SemioMembers>, pub(crate) {SCOPE_T});
''')
sub('            if !std::thread::panicking() { close_registered_fixture_app(&mut self.0); }', f'            if !std::thread::panicking() {{ close_registered_fixture_app(&mut self.0, {POL}, self.1.cx()).expect("original caller paid app close"); }}')
sub('''        let mut app = VcsArtifactApp::<EditorApp<SequencePlayApp>, SemioMembers>::with_registry(EditorApp::default(), AppActionRegistry::from_definition(&create_sequence_app()), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
        app.bind_instance_id(1).await;
        SequenceApp(app)''', f'''        let scope = {SCOPE_T}::admit();
        let mut app = VcsArtifactApp::<EditorApp<SequencePlayApp>, SemioMembers>::with_registry(EditorApp::default(), AppActionRegistry::from_definition(&create_sequence_app()), semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}).await;
        app.bind_instance_id(1).await;
        {LAWS_T}::settle_store_constructor(&mut app, {POL});
        SequenceApp(app, scope)''')
sub('''        app.0.dispatch_typed(command, &script_window_meta()).await.expect("dispatch");
        settle_registered_typed_operation(&mut app.0, 1).await.expect("settle typed operation");''', f'''        app.0.dispatch_typed(command, &script_window_meta(), &mut {IDENT}, app.1.cx()).await.expect("dispatch");
        settle_registered_typed_operation(&mut app.0, 1, {POL}, &mut {IDENT}, app.1.cx()).await.expect("settle typed operation");''')
sub('''        project_and_retire_fixture_tree(app.0.render(body_key, None, view).await.expect("render")).expect("retire rendered tree")''', f'''        {LAWS_T}::render_registered_fixture_app(&mut app.0, body_key, view, app.1.cx()).await.expect("render")''')
write(P, s)
print('seq context done')
