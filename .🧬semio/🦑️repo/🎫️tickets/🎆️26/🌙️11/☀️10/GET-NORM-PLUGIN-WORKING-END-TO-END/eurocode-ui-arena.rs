use semio_framework_value::RetainedCloneGrant;
use semio_ui_contract_values::{close_ui_value_page_one, ui_value_arena_registry, UiArenaOpening, UiArenaRegistration};

const GRANT: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 1, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 256 << 20,
    maximum_release_bytes: 256 << 20, maximum_depth: 64,
};

pub fn register() -> UiArenaRegistration {
    let mut opening = UiArenaOpening::new();
    while !opening.ready() {
        assert_ne!(opening.advance(GRANT).result.unwrap().progress(), Default::default());
    }
    let mut backing = Some(opening.take(GRANT).result.unwrap().unwrap().0);
    let mut owner = None;
    UiArenaRegistration::admit(ui_value_arena_registry(), &mut backing, &mut owner, GRANT).result.unwrap();
    owner.expect("registered UI arena")
}

pub fn drain() {
    for _ in 0..100_000 {
        if semio_ui_contract::close_built_node_page_one() && close_ui_value_page_one() {
            return;
        }
    }
    panic!("UI collection handbacks did not drain");
}

pub fn close(mut owner: UiArenaRegistration) {
    drain();
    for _ in 0..100_000 {
        if owner.terminal_is_empty() {
            return;
        }
        owner.close_step(GRANT).result.unwrap();
    }
    panic!("UI arena did not close");
}
