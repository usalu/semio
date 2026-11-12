use super::*;

#[test]
fn cad_presence_empty_terminal_is_its_own_empty_witness() {
    assert!(terminal_is_empty(&empty_terminal()));
    assert!(!terminal_is_empty(&CadPresence::default()));
    drop(store_disposer());
}
