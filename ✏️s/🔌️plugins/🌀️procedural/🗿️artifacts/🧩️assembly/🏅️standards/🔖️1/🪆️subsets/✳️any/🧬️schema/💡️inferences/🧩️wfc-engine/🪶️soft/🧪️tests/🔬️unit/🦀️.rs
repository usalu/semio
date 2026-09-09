use super::*;

#[test]
fn best_of_n_keeps_the_highest_scoring_attempt() {
    let scorer = ScoreFn { f: |a: &[PatternId]| a.iter().map(|p| p.get() as f64).sum() };
    let (best, solved) = best_of_n(0, 5, BestOfNKeep::Highest, &scorer, |seed| Some(vec![PatternId(seed as u32 % 10)]));
    assert_eq!(solved, 5);
    let best = best.unwrap();
    assert_eq!(best.assignment, vec![PatternId(4)]); // seeds 0..5 -> patterns 0..5, max is 4
}

#[test]
fn best_of_n_keeps_the_lowest_scoring_attempt() {
    let scorer = ScoreFn { f: |a: &[PatternId]| a.iter().map(|p| p.get() as f64).sum() };
    let (best, _) = best_of_n(0, 5, BestOfNKeep::Lowest, &scorer, |seed| Some(vec![PatternId(seed as u32 % 10)]));
    assert_eq!(best.unwrap().assignment, vec![PatternId(0)]);
}

#[test]
fn best_of_n_skips_failed_attempts() {
    let scorer = ScoreFn { f: |_: &[PatternId]| 0.0 };
    let (best, solved) = best_of_n(0, 5, BestOfNKeep::Highest, &scorer, |seed| if seed == 2 { Some(vec![PatternId(0)]) } else { None });
    assert_eq!(solved, 1);
    assert!(best.is_some());
}

#[test]
fn best_of_n_returns_none_when_every_attempt_fails() {
    let scorer = ScoreFn { f: |_: &[PatternId]| 0.0 };
    let (best, solved) = best_of_n(0, 3, BestOfNKeep::Highest, &scorer, |_| None);
    assert_eq!(solved, 0);
    assert!(best.is_none());
}

#[test]
fn weight_field_identity_is_all_ones() {
    let field = WeightField::identity(2, 3);
    assert_eq!(field.node_count(), 2);
    for n in 0..field.node_count() {
        for p in 0..3 {
            assert_eq!(field.get(NodeId::from_index(n), PatternId::from_index(p)), 1.0);
        }
    }
}

#[test]
fn weight_field_set_and_get_roundtrip() {
    let mut field = WeightField::identity(2, 2);
    field.set(NodeId(0), PatternId(1), 2.5);
    assert_eq!(field.get(NodeId(0), PatternId(1)), 2.5);
    assert_eq!(field.get(NodeId(0), PatternId(0)), 1.0);
}
