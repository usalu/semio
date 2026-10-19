use super::*;

#[test]
fn fem3d_scalar_publication_refuses_reused_counters_from_another_document() {
    let identity = recovery_identity(97, 29);
    let shell = MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let (shell, recovery) = reserve_recovery_state(&mut registry, identity);
        let mut state = recoverable_state(identity, recovery);
        let solver = state.solver.as_mut().unwrap();
        assert!(solver.admit_page(0, false, &mut state.backing));
        assert!(solver.admit_page(0, true, &mut state.backing));
        *registry.shells[shell as usize].borrow_mut() = Some(state);
        shell
    });
    let exact = AppRenderOperationContext {
        app_instance_id: identity.app_instance_id,
        base_revision: identity.base_revision,
        generation: identity.generation,
        canonical_base_revision: identity.canonical_base_revision,
        mounted_policy: semio_framework_plugin::MountedOwnerPolicyV1 { preparation: PAGE_GRANT, maintenance: PAGE_GRANT, close: PAGE_GRANT },
    };
    let mut stale = exact;
    stale.canonical_base_revision[31] ^= 1;
    let scalar = Fem3dSolverScalar { displacement: [1.0, 2.0, 3.0], ..Fem3dSolverScalar::default() };
    assert_eq!(publish_solver_scalar(stale, 0, scalar), Err(scalar));
    assert!(!publish_solver_progress(stale, Fem3dVisualState::ValidatedFinal, 0.0, 1e-8, 1, 1));
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let state = registry.shells[shell as usize].borrow();
        let solver = state.as_ref().unwrap().solver.as_ref().unwrap();
        assert!(solver.scalar(0).is_none());
        assert!(!solver.ready());
    });
    assert_eq!(publish_solver_scalar(exact, 0, scalar), Ok(()));
    assert!(publish_solver_progress(exact, Fem3dVisualState::ValidatedFinal, 0.0, 1e-8, 1, 1));
    MOUNTED.with(|registry| {
        let registry = registry.borrow();
        let state = registry.shells[shell as usize].borrow();
        assert_eq!(state.as_ref().unwrap().solver.as_ref().unwrap().scalar(0), Some(scalar));
    });
    MOUNTED.with(|registry| {
        let mut registry = registry.borrow_mut();
        let state = registry.shells[shell as usize].borrow_mut().take().unwrap();
        drop(state);
        drain_recovery_state(&mut registry, identity, shell);
    });
    eprintln!("[DEBUG] FEM3D refused foreign canonical scalar/progress without changing fields, accepted exact authority, and retired original pages");
}
