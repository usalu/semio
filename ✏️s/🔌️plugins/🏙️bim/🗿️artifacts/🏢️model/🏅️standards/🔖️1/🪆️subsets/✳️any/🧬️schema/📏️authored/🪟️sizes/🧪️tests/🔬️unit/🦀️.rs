use super::*;
use crate::WindowType;

fn window() -> WindowType {
    WindowType { name: "w".into(), width: 1.2, height: 1.4, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 2, material: "m".into(), u_value: None, g_value: None }
}

fn opening(kind: OpeningKind) -> Opening {
    Opening { host: "w".into(), kind, offset: 1.0, sill_override: None, width: None, height: None, flip_hand: false, flip_facing: false, name: String::new(), reveal_depth: None, reveal_material: None }
}

#[test]
fn an_opening_takes_the_size_of_its_type_and_an_override_wins() {
    let mut snapshot = ModelSnapshot::default();
    snapshot.window_types.insert("t".into(), window());
    let mut row = opening(OpeningKind::Window { window_type: "t".into() });
    assert_eq!(resolve_size(&snapshot, &row), Resolved { width: 1.2, height: 1.4, sill: 0.9, type_found: true });
    row.width = Some(2.0);
    row.sill_override = Some(0.0);
    assert_eq!(resolve_size(&snapshot, &row), Resolved { width: 2.0, height: 1.4, sill: 0.0, type_found: true });
}

#[test]
fn a_missing_type_resolves_to_nothing_and_a_void_to_its_own_size() {
    let snapshot = ModelSnapshot::default();
    assert!(!resolve_size(&snapshot, &opening(OpeningKind::Door { door_type: "gone".into() })).type_found);
    assert_eq!(resolve_size(&snapshot, &opening(OpeningKind::Void { width: 0.5, height: 0.7 })), Resolved { width: 0.5, height: 0.7, sill: 0.0, type_found: true });
}
