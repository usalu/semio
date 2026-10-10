use super::*;

#[test]
fn only_an_attached_top_names_a_target() {
    assert_eq!(top_target(&TopConstraint::Roof { roof: "r".into(), offset: 0.5 }), Some(("r", 0.5)));
    assert_eq!(top_target(&TopConstraint::Slab { slab: "s".into(), offset: -0.1 }), Some(("s", -0.1)));
    assert_eq!(top_target(&TopConstraint::Ceiling { ceiling: "c".into(), offset: 0.0 }), Some(("c", 0.0)));
    assert_eq!(top_target(&TopConstraint::StoreyTop { offset: 0.0 }), None);
}

#[test]
fn a_loop_of_references_is_found_from_every_node_that_reaches_it() {
    let edges = |node: &str| match node {
        "a" => vec!["b".to_string()],
        "b" => vec!["c".to_string()],
        "c" => vec!["a".to_string()],
        _ => Vec::new(),
    };
    assert_eq!(find_cycle("a", &edges), Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]));
    assert_eq!(find_cycle("x", &edges), None);
}
