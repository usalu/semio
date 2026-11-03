use super::*;

#[test]
fn the_norm_hooks_walk_the_framework_registry_of_their_own_domain() {
    let domain = "norm.test-focus-reexport";
    note_grips(domain, vec![GripRef { id: "a.length".to_owned(), entity: "a".to_owned() }, GripRef { id: "a.height".to_owned(), entity: "a".to_owned() }]);
    assert_eq!(walk_focus(domain, GripWalk::Forward), GripStep::To("a.length".to_owned()));
    assert_eq!(walk_focus(domain, GripWalk::ForwardInEntity), GripStep::To("a.height".to_owned()));
    assert_eq!(focused_grip(domain).as_deref(), Some("a.height"));
    set_focused_grip(domain, None);
    assert_eq!(noted_grips(domain).len(), 2);
    assert_eq!(walk_grips(&noted_grips(domain), None, GripWalk::Backward), GripStep::To("a.height".to_owned()));
}
