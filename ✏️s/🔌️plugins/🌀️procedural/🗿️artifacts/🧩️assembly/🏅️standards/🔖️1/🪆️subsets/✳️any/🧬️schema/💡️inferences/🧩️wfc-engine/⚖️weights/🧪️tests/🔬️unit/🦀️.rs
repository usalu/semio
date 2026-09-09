use super::*;
use crate::wfc_engine::bitset::PatternSet;

#[test]
fn rejects_negative_and_nonfinite() {
    assert!(WeightTable::new(&[1.0, -1.0]).is_err());
    assert!(WeightTable::new(&[1.0, f64::NAN]).is_err());
    assert!(WeightTable::new(&[1.0, f64::INFINITY]).is_err());
}

#[test]
fn precomputes_terms() {
    let t = WeightTable::new(&[1.0, core::f64::consts::E]).unwrap();
    assert_eq!(t.w(PatternId(0)), 1.0);
    assert!((t.ln_w(PatternId(1)) - 1.0).abs() < 1e-12);
    assert!((t.w_ln_w(PatternId(1)) - core::f64::consts::E).abs() < 1e-9);
}

#[test]
fn zero_weight_terms_are_zero_not_nan() {
    let t = WeightTable::new(&[0.0, 2.0]).unwrap();
    assert_eq!(t.ln_w(PatternId(0)), 0.0);
    assert_eq!(t.w_ln_w(PatternId(0)), 0.0);
}

#[test]
fn integer_detection() {
    let t = WeightTable::new(&[1.0, 3.0, 5.0]).unwrap();
    assert!(t.has_integer_weights());
    assert_eq!(t.w_int(PatternId(1)), Some(3));

    let f = WeightTable::new(&[1.5, 2.0]).unwrap();
    assert!(!f.has_integer_weights());
}

#[test]
fn sum_over_matches_manual() {
    let t = WeightTable::new(&[1.0, 2.0, 4.0]).unwrap();
    let mut set = PatternSet::new_empty(3);
    set.set(PatternId(0), true);
    set.set(PatternId(2), true);
    let (sw, swlw) = t.sum_over(&set);
    assert_eq!(sw, 5.0);
    assert!((swlw - (0.0 + 4.0 * 4.0f64.ln())).abs() < 1e-12);
    assert_eq!(t.sum_int_over(&set), Some(5));
}
