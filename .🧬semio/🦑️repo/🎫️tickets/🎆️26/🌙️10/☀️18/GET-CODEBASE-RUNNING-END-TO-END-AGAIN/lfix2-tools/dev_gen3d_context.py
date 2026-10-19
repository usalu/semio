from devlib import *
P = EXT_ROOT + '🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
s = read(P)
def sub(old, new, count=1):
    global s
    assert s.count(old) == count, (old[:100], s.count(old))
    s = s.replace(old, new)

POL = 'semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT'
# 1. fixture struct: tuple with the scope, hand-rolled close ladder replaced by the framework's paid close
sub('    pub struct Generation3dAppFixture(Generation3dApp);\n', f'''    /// Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the scope pays the caller's terminal retirement after the paid app close.
    pub struct Generation3dAppFixture(pub Generation3dApp, pub {SCOPE_T});
''')
a = s.index('    /// 🔎️ Names the authority a stalled close ladder was last waiting on')
b = s.index('    impl Drop for Generation3dAppFixture {')
s = s[:a] + s[b:]
sub('''    impl Drop for Generation3dAppFixture {
        fn drop(&mut self) {
            let witness = close_ladder_witness(&mut self.0);
            assert!(std::thread::panicking() || self.0.close_terminal_is_empty(), "Generation3d app fixture did not reach its terminal-empty close witness{witness}");
        }
    }
''', f'''    impl Drop for Generation3dAppFixture {{
        fn drop(&mut self) {{
            if !std::thread::panicking() {{
                {LAWS}::close_registered_fixture_app(&mut self.0, {POL}, self.1.cx()).expect("original caller paid app close");
            }}
        }}
    }}
''')
# 2. constructor: scope first, then the registry-built app with its policy + identity
sub('''        let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
        app.bind_instance_id(1).await;
        Generation3dAppFixture(app)''', f'''        let scope = {SCOPE_T}::admit();
        let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), {POL}, &mut {IDENT}).await;
        app.bind_instance_id(1).await;
        Generation3dAppFixture(app, scope)''')
# 3. helper signatures over the fixture (the ones that need the lent context)
s = re.sub(r'(pub (?:async )?fn \w+\(app: &mut )Generation3dApp\b', r'\1Generation3dAppFixture', s)
# 4. bodies
sub('''        let admitted = app.handle_action(semio_framework::INTERACTION_SELECT_ACTION_ID, Some(&args), &meta("local")).await.expect("interaction selection admitted");
        semio_framework_plugin::app::settle_framework_reserved_admission(app, admitted).await.expect("interaction selection settles its reserved tool job")''', f'''        let admitted = {DFA_T}(&mut app.0, semio_framework::INTERACTION_SELECT_ACTION_ID, Some(args.clone()), &meta("local"), &mut {IDENT}, app.1.cx(), {POL}.close).await.expect("interaction selection admitted");
        semio_framework_plugin::app::settle_framework_reserved_admission(&mut app.0, admitted).await.expect("interaction selection settles its reserved tool job")''')
sub('''        app.dispatch_typed(command, &action_meta).await?;
        settle_registered_typed_operation(app, action_meta.instance_id).await
    }''', f'''        app.0.dispatch_typed(command, &action_meta, &mut {IDENT}, app.1.cx()).await.map_err(|failure| semio_framework_plugin::Fault::from(format!("{{failure:?}}")))?;
        settle_registered_typed_operation(&mut app.0, action_meta.instance_id, {POL}, &mut {IDENT}, app.1.cx()).await.map_err(|failure| semio_framework_plugin::Fault::from(format!("{{failure:?}}")))
    }}''')
sub('''        app.handle_action(action, Some(&args), &action_meta).await?;
        settle_registered_typed_operation(app, action_meta.instance_id).await
    }''', f'''        {DFA_T}(&mut app.0, action, Some(args), &action_meta, &mut {IDENT}, app.1.cx(), {POL}.close).await.map_err(|failure| semio_framework_plugin::Fault::from(format!("{{failure:?}}")))?;
        settle_registered_typed_operation(&mut app.0, action_meta.instance_id, {POL}, &mut {IDENT}, app.1.cx()).await.map_err(|failure| semio_framework_plugin::Fault::from(format!("{{failure:?}}")))
    }}''')
sub('''        settle_registered_typed_operation(app, meta("local").instance_id).await.expect("retained publication")''', f'''        settle_registered_typed_operation(&mut app.0, meta("local").instance_id, {POL}, &mut {IDENT}, app.1.cx()).await.expect("retained publication")''')
sub('''        semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(app.render(body_key, None, view_state).await.expect("render")).expect("render json")''', f'''        {LAWS}::render_registered_fixture_app(&mut app.0, body_key, view_state, app.1.cx()).await.expect("render json")''')
# act_with_view has `let args: DslValue = args.into();` before: ok (Some(args))
open(P, 'w', encoding='utf-8').write(s)
print('context done')
