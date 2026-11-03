//! 📊️ Spectrum grouping and the Miner check of the fatigue details (the S–N math is tested in `fatigue_math`).

use super::*;

fn row(id: &str, member: &str, category: &str, ds: f64, n: f64) -> FatigueDetail {
    FatigueDetail { id: id.into(), member_id: member.into(), detail_category: category.into(), delta_sigma_c: 0.0, delta_sigma_ed: ds, n_cycles: n, m1: 3.4, m2: 5.4 }
}

#[test]
fn rows_of_one_member_and_category_form_one_spectrum() {
    let rows = vec![row("f1", "m1", "71", 60.0e6, 2.0e5), row("f2", "m2", "71", 40.0e6, 1.0e6), row("f3", "m1", "71", 40.0e6, 1.0e6), row("f4", "m1", "56", 40.0e6, 1.0e6), row("f5", "", "71", 1.0, 1.0), row("f6", "", "71", 1.0, 1.0)];
    assert_eq!(part_1_3::spectrum_groups(&rows), vec![vec![0, 2], vec![1], vec![3], vec![4], vec![5]]);
}

fn spectrum(rows: &[FatigueDetail]) -> Vec<CheckResult> { check_fatigue_spectrum(&rows.iter().collect::<Vec<_>>(), AnnexChoice::En) }

#[test]
fn the_spectrum_check_names_the_governing_block_and_a_failing_one_offers_remedies() {
    let c = spectrum(&[row("f1", "m1", "71", 60.0e6, 2.0e5), row("f2", "m1", "71", 40.0e6, 1.0e6)]);
    assert_eq!((c.len(), c[0].id.as_str()), (1, "en1999.1-3.fat.miner.f1"));
    assert!(c[0].explanation.en.contains("governing block f1"), "{}", c[0].explanation.en);
    assert!(spectrum(&[row("f1", "m1", "71", 60.0e6, 2.0e5)]).is_empty());
    let f = &spectrum(&[row("f1", "m1", "28", 90.0e6, 2.0e6), row("f2", "m1", "28", 80.0e6, 2.0e6)])[0];
    assert!(f.utilization > 1.0 && f.remedies.len() >= 2, "{} {}", f.utilization, f.remedies.len());
}
