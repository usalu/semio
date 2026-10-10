use super::*;
use crate::Storey;

fn storey(level: i32, height: f64) -> Storey {
    Storey { building: "b".into(), name: format!("level {level}"), level, height, cut_height: None }
}

fn tower() -> ModelSnapshot {
    let mut snapshot = ModelSnapshot::default();
    for (id, level, height) in [("below", -1, 2.5), ("ground", 0, 3.0), ("first", 1, 2.8), ("second", 2, 2.7), ("deep", -2, 2.0)] {
        snapshot.storeys.insert(id.into(), storey(level, height));
    }
    snapshot.storeys.insert("other".into(), Storey { building: "c".into(), ..storey(0, 9.0) });
    snapshot
}

#[test]
fn the_elevation_is_the_sum_of_the_heights_between_the_datum_and_the_storey() {
    let snapshot = tower();
    let expect = [("ground", 0.0), ("first", 3.0), ("second", 5.8), ("below", -2.5), ("deep", -4.5), ("other", 0.0)];
    for (id, want) in expect {
        assert!((elevation(&snapshot, id).expect("the storey exists") - want).abs() < 1e-12, "{id}");
    }
    assert_eq!(elevation(&snapshot, "nowhere"), None);
}

#[test]
fn the_stacking_names_the_storey_below_as_the_parent_and_the_datum_storeys_have_none() {
    let rows = stacking(&tower(), "b");
    let parent = |id: &str| rows.iter().find(|(row, _)| row == id).map(|(_, parent)| parent.clone());
    assert_eq!(parent("ground"), Some(None));
    assert_eq!(parent("second"), Some(Some("first".to_string())));
    assert_eq!(parent("below"), Some(None));
    assert_eq!(parent("deep"), Some(Some("below".to_string())));
}
