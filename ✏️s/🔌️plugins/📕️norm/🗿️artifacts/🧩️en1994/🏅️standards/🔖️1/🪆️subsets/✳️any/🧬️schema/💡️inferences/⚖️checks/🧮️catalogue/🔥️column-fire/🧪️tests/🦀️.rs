//! 🧪️ Column fire: the heat-conduction field against the independent numpy re-implementation (`🧰️scripts/en1994-column-fire-oracle.py`), cold-state resistance against hand sums, and the tabulated-data structure.

use super::*;

fn column() -> EncasedI {
    EncasedI { kind: Encasement::Full, b_c: 0.4, h_c: 0.4, h: 0.2, b: 0.2, tw: 0.0085, tf: 0.013, as_m2: 4.0 * 314.16e-6, cover: 0.05 }
}

fn strengths() -> FireStrengths {
    FireStrengths::new(355e6, 30e6, 500e6, RebarProduct::HotRolled)
}

fn rel(a: f64, b: f64) -> f64 {
    (a - b).abs() / b.abs().max(1e-12)
}

#[test]
fn field_after_sixty_minutes_matches_the_independent_reimplementation() {
    let f = temperature_field(&column(), 3600.0);
    assert_eq!((f.nx, f.ny), (20, 20));
    for ((i, j), expected) in [((0, 0), 24.4885), ((4, 4), 36.1839), ((0, 9), 74.7581), ((19, 0), 893.3178), ((19, 19), 941.3887), ((0, 19), 893.2765)] {
        assert!((f.at(i, j) - expected).abs() < 0.05, "cell ({i},{j}) {} vs {expected}", f.at(i, j));
    }
    assert!(f.steel[9 * 20] && !f.steel[0]);
}

#[test]
fn field_is_monotone_in_time_and_bounded_by_the_gas_temperature() {
    let fields = temperature_fields(&column(), &[1800.0, 3600.0, 7200.0]);
    assert!(fields[0].max() < fields[1].max() && fields[1].max() < fields[2].max());
    assert!(fields[2].max() < iso834_c(7200.0));
    assert!(fields[1].at(0, 0) < fields[1].at(10, 10) && fields[1].at(10, 10) < fields[1].at(19, 19));
    assert!(rel(fields[1].minutes, 60.0) < 1e-12);
}

#[test]
fn cold_section_reproduces_the_plastic_sum_and_the_stiffness_sum() {
    let g = column();
    let cold = temperature_field(&g, 0.0);
    let s = strengths();
    let r = resistance(&g, &cold, &s, 4.0);
    let expected = g.steel_area() * s.fy + g.concrete_area() * s.fck + g.as_m2 * s.fsk;
    assert!(rel(r.n_pl, expected) < 1e-9);
    let i_a = 2.0 * (0.2 * 0.013 * (0.1 - 0.0065_f64).powi(2) + 0.2 * 0.013_f64.powi(3) / 12.0) + 0.0085 * (0.2_f64 - 0.026).powi(3) / 12.0;
    let i_c = 0.4_f64.powi(4) / 12.0 - i_a;
    let ei = E_A * i_a + 30e6 / 2.5e-3 * i_c + E_S * g.as_m2 * 0.15 * 0.15;
    assert!(rel(r.ei, ei) < 0.03, "{} vs {ei}", r.ei);
    assert!(r.chi > 0.0 && r.chi <= 1.0 && rel(r.n_rd, r.chi * r.n_pl) < 1e-12);
    assert!(r.m_pl > 0.0);
}

#[test]
fn heated_section_loses_resistance_and_stiffness() {
    let g = column();
    let s = strengths();
    let cold = resistance(&g, &temperature_field(&g, 0.0), &s, 4.0);
    let hot = resistance(&g, &temperature_field(&g, 7200.0), &s, 4.0);
    assert!(hot.n_pl < cold.n_pl && hot.ei < cold.ei && hot.m_pl < cold.m_pl && hot.n_rd < cold.n_rd);
}

#[test]
fn partially_encased_chamber_heats_faster_than_the_fully_encased_core() {
    let mut g = column();
    let full = temperature_field(&g, 3600.0);
    g.kind = Encasement::BetweenFlanges;
    let chamber = temperature_field(&g, 3600.0);
    assert!(chamber.steel_mean() > full.steel_mean() + 100.0);
}

#[test]
fn material_factors_interpolate_between_the_nodes() {
    assert!(rel(kc_theta(250.0), 0.90) < 1e-12 && kc_theta(1500.0) == 0.0 && kc_theta(20.0) == 1.0);
    assert!(rel(ke_theta(150.0), 0.95) < 1e-12);
    assert!(rel(concrete_secant_modulus_pa(20.0, 30e6), 12e9) < 1e-12);
}

#[test]
fn the_tabulated_data_stay_empty_until_transcribed() {
    assert!(COLUMN_TABLE.is_empty());
    assert_eq!(check_tabulated(COLUMN_TABLE, 60, 0.5, 0.3, 0.05, 0.01), Tabulated::NoData);
}

#[test]
fn a_synthetic_row_is_selected_by_rating_and_the_next_load_level_and_keeps_its_verification_flag() {
    let cell = |v: f64, verified: bool| TableCell { value: Some(v), verified };
    let rows = [
        ColumnRow { rating_min: 60, load_level: 0.3, min_width_m: cell(0.15, true), min_axis_distance_m: cell(0.03, true), min_reinforcement_ratio: cell(0.01, true) },
        ColumnRow { rating_min: 60, load_level: 0.5, min_width_m: cell(0.25, true), min_axis_distance_m: cell(0.04, false), min_reinforcement_ratio: cell(0.01, true) },
    ];
    assert_eq!(check_tabulated(&rows, 60, 0.4, 0.3, 0.08, 0.02), Tabulated::Evaluated { utilization: (0.25_f64 / 0.3).max(0.04 / 0.08).max(0.01 / 0.02), verified: false });
    assert_eq!(check_tabulated(&rows, 60, 0.9, 0.3, 0.08, 0.02), Tabulated::NoData);
    assert_eq!(check_tabulated(&rows, 90, 0.3, 0.3, 0.08, 0.02), Tabulated::NoData);
    let missing = [ColumnRow { rating_min: 30, load_level: 0.5, min_width_m: TableCell::MISSING, min_axis_distance_m: cell(0.02, true), min_reinforcement_ratio: cell(0.0, true) }];
    assert_eq!(check_tabulated(&missing, 30, 0.5, 0.3, 0.05, 0.01), Tabulated::NoData);
}

#[test]
fn rebar_reduction_follows_the_product_table_and_interpolates_between_the_nodes() {
    let (hot, cold) = (RebarProduct::HotRolled.reduction(), RebarProduct::ColdWorked.reduction());
    assert_eq!((hot.at(600.0), cold.at(600.0)), (0.47, 0.40));
    assert_eq!((hot.at(20.0), cold.at(300.0), hot.at(1300.0)), (1.0, 1.0, 0.0));
    assert!((cold.at(450.0) - (0.94 + 0.67) / 2.0).abs() < 1e-12);
    assert!(cold.at(500.0) < hot.at(500.0) && cold.at(700.0) < hot.at(700.0));
}

#[test]
fn rebar_product_wire_values_round_trip_and_cite_thirteen_registry_rows() {
    for product in [RebarProduct::HotRolled, RebarProduct::ColdWorked] {
        assert_eq!(RebarProduct::from(match product { RebarProduct::HotRolled => semio_s_artifact_norm_en1994_model::ReinforcementProduct::HotRolled, RebarProduct::ColdWorked => semio_s_artifact_norm_en1994_model::ReinforcementProduct::ColdWorked }), product);
        assert_eq!(product.ndp_ids().len(), 13);
        assert!(product.ndp_ids().iter().all(|id| semio_s_plugin_norm_compliance::ndp_number_opt(id, semio_s_plugin_norm_compliance::AnnexChoice::En).is_some() && !semio_s_plugin_norm_compliance::ndp_is_verified_for(id, semio_s_plugin_norm_compliance::AnnexChoice::En)));
    }
}
