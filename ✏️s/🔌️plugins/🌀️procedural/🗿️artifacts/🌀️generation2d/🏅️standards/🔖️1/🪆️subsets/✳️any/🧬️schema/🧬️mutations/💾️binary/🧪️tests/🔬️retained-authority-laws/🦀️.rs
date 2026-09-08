
use super::*;

fn close_session(session: &mut Generation2dMutationSession) {
    for _ in 0..GENERATION2D_MAXIMUM_DOMAIN_ITEMS {
        if session.close_step(1) {
            assert!(session.terminal_is_empty());
            return;
        }
    }
    panic!("P2 retained mutation session did not close");
}

#[test]
fn every_fourteen_variant_decodes_through_retained_structural_grants() {
    let mutations = generation2d_all_retained_mutation_fixtures_for_test();
    assert_eq!(mutations.len(), GENERATION2D_MUTATION_VARIANT_COUNT);
    for mutation in mutations {
        let bytes = encode_op(&mutation).expect("P2 retained mutation fixture encode");
        let mut session = Generation2dMutationSession::new(bytes.len(), GENERATION2D_MAXIMUM_DOMAIN_ITEMS).expect("P2 retained mutation preflight");
        for byte in bytes {
            assert!(session.ingress_ready());
            session.admit_byte(byte).expect("one retained mutation byte");
            for _ in 0..GENERATION2D_OWNER_BYTES {
                session.grant().expect("one retained mutation ingress grant");
                if session.ingress_ready() {
                    break;
                }
            }
            assert!(session.ingress_ready(), "symbol expansion must hand input ownership back before the next byte");
        }
        session.seal().expect("exact retained mutation seal");
        let mut ready = false;
        for _ in 0..100_000 {
            if session.grant().expect("one retained semantic grant") {
                ready = true;
                break;
            }
        }
        assert!(ready, "retained P2 mutation owner must converge");
        assert_eq!(session.take().expect("typed P2 mutation handoff"), mutation);
        close_session(&mut session);
    }
}

#[test]
fn deterministic_all_field_ledger_includes_the_2d_only_variant() {
    let mutations = generation2d_all_retained_mutation_fixtures_for_test();
    let mut left = store::ArtifactStoreInitializationDigest::new(b"generation2d.all14");
    let mut right = store::ArtifactStoreInitializationDigest::new(b"generation2d.all14");
    for mutation in &mutations {
        generation2d_observe_mutation(&mut left, mutation);
        generation2d_observe_mutation(&mut right, mutation);
    }
    assert_eq!(left.finish(), right.finish());
    assert!(mutations.iter().any(|mutation| matches!(mutation, Generation2dMutation::ClearWidgetLayout(_))));
}
