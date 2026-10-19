from devlib import *
P = EXT_ROOT + '🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs'
s = read(P)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:100], s.count(old))
    s = s.replace(old, new)
POL = 'semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT'
sub('    pub struct Generation3dViewerFixture(Generation3dViewerHarness);\n', f'''    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct Generation3dViewerFixture(pub Generation3dViewerHarness, pub {SCOPE_T});
''')
sub('''            let witness = crate::editor_domain::editor_laws::context::close_ladder_witness(&mut self.0);
            assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "Generation3d viewer fixture did not reach its terminal-empty close witness{witness}");''', f'''            if !std::thread::panicking() {{
                {LAWS_T}::close_registered_fixture_app(&mut self.0, {POL}, self.1.cx()).expect("original caller paid app close");
            }}''')
sub('''        let mut app = new_app_with_registry::<ViewerApp<Generation3dViewer>>(generation3d_viewer_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
        app.bind_instance_id(1).await;
        Generation3dViewerFixture(app)''', f'''        let scope = {SCOPE_T}::admit();
        let mut app = new_app_with_registry::<ViewerApp<Generation3dViewer>>(generation3d_viewer_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}).await;
        app.bind_instance_id(1).await;
        Generation3dViewerFixture(app, scope)''')
s = re.sub(r'(pub (?:async )?fn \w+\(app: &mut )Generation3dViewerHarness\b', r'\1Generation3dViewerFixture', s)
sub('''        app.dispatch_typed(command, &meta("local")).await.expect("dispatch")''', f'''        app.0.dispatch_typed(command, &meta("local"), &mut {IDENT}, app.1.cx()).await.expect("dispatch")''')
sub('''        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)).await.expect("render")).expect("render json")''', f'''        {LAWS_T}::render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("render json")''')
sub('''        crate::brep_extension::dispatch_effect_command(app, command_id, args, action_meta).await''', '''        crate::brep_extension::dispatch_effect_command(&mut app.0, command_id, args, action_meta, app.1.cx()).await''')
sub('''        app.dispatch_typed(command, &action_meta).await?;
        settle_registered_typed_operation(app, action_meta.instance_id).await''', f'''        app.0.dispatch_typed(command, &action_meta, &mut {IDENT}, app.1.cx()).await.map_err(|failure| semio_framework_plugin::Fault::from(format!("{{failure:?}}")))?;
        settle_registered_typed_operation(&mut app.0, action_meta.instance_id, {POL}, &mut {IDENT}, app.1.cx()).await.map_err(|failure| semio_framework_plugin::Fault::from(format!("{{failure:?}}")))''')
sub('''        crate::brep_extension::owed_run_actions(app, shell_view).await''', '''        crate::brep_extension::owed_run_actions(&mut app.0, shell_view).await''')
sub('''        crate::brep_extension::drive_preview_run(app, shell_view, initial, &mut crate::brep_extension::serve).await''', '''        crate::brep_extension::drive_preview_run(&mut app.0, shell_view, initial, &mut crate::brep_extension::serve, app.1.cx()).await''')
sub('''        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")''', f'''        {LAWS_T}::render_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx()).await.expect("render json")''')
write(P, s)
print('viewer context done')
