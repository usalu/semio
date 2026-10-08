use super::*;

#[test]
fn sections_become_profiles_and_report_their_top() {
    let rectangle = Section::Rectangle { centre: [0.0, 0.1], width: 0.3, depth: 0.4 };
    assert_eq!(profile_of(&rectangle), Some(Profile::Rectangle { width: 0.3, depth: 0.4 }));
    assert!((profile_top(&rectangle) - 0.3).abs() < 1e-12);
    assert_eq!(profile_of(&Section::Circle { diameter: 0.5 }), Some(Profile::Circle { diameter: 0.5 }));
    assert_eq!(profile_top(&Section::Circle { diameter: 0.5 }), 0.25);
    let outline = Section::Outline { outer: vec![([0.0, 0.0], 0.0), ([1.0, 0.0], 0.0), ([1.0, 2.0], 0.0)], holes: vec![] };
    assert!(matches!(profile_of(&outline), Some(Profile::Custom { outline }) if outline.len() == 3));
    assert_eq!(profile_top(&outline), 2.0);
}

#[test]
fn unused_ids_count_up_from_the_base() {
    assert_eq!(Import::unused("ct", |_| false), "ct");
    assert_eq!(Import::unused("ct", |candidate| candidate == "ct" || candidate == "ct-2"), "ct-3");
}
