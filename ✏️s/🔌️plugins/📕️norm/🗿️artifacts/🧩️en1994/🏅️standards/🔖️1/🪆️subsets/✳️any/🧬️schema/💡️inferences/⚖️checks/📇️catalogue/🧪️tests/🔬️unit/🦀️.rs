use super::*;
use design_case::CaseCatalogue;

fn run(id: &str) {
    let entry = design_case::entry::<En1994Cases>(id).expect("catalogue entry");
    let case = design_case::seed::<En1994Cases>(&entry, "case-test");
    assert!(design_case::issues::<En1994Cases>(&entry, &case.values).is_empty(), "{id}: the example values are valid vocabulary rows");
    assert_eq!(design_case::declared::<En1994Cases>(&entry).len(), entry.inputs.len(), "{id}: every declared input has a vocabulary row");
    let results = evaluate_design_case(&case, AnnexChoice::En);
    assert!(!results.is_empty(), "{id}: no result");
    for result in results.iter().filter(|result| result.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable) {
        assert!(!result.clause.section.is_empty() && result.clause.family == entry.part, "{id}: clause link");
        assert!(result.utilization.is_finite() && result.utilization > 0.0, "{id}: utilisation {}", result.utilization);
        assert!(result.subject.path == format!("cases[id={}]", case.id), "{id}: addressed to the case row");
        assert!(!result.id.contains(".input."), "{id}: input issue {}", result.id);
    }
}

macro_rules! entry_tests {
    ($($name:ident => $id:literal),* $(,)?) => { $(#[test] fn $name() { run($id); })* };
}

entry_tests! {
    stud_with_profiled_sheeting => "C94-01",
    patch_loading_with_bending => "C94-02",
    column_load_introduction => "C94-03",
    deflection_simple_span => "C94-04",
    deflection_two_span => "C94-05",
    steel_strength_in_fire => "C94-06",
    sagging_plastic_moment => "C94-07",
    hogging_plastic_moment => "C94-08",
    floor_vibration => "C94-09",
    continuous_beam_ltb => "C94-10",
    creep_and_shrinkage_deflection => "C94-11",
    protected_steel_in_fire => "C94-12",
    partially_encased_columns_by_table_4_6 => "C94-13",
    concrete_filled_columns_by_table_4_7 => "C94-14",
    fully_encased_columns_by_table_4_4 => "C94-15",
}

#[test]
fn table_4_6_distinguishes_dimension_failure_from_unavailable_classification() {
    use semio_s_plugin_norm_compliance::CheckStatus;

    let entry = design_case::entry::<En1994Cases>("C94-13").unwrap();
    let demo = design_case::seed::<En1994Cases>(&entry, "table-case");
    let assess = |case: &DesignCase| evaluate_design_case(case, AnnexChoice::De);
    assert!(assess(&demo).iter().all(|check| check.status == CheckStatus::Pass));

    let mut narrow = demo.clone();
    narrow.values = narrow.values.with_number("fire_col_width", 0.25);
    assert_eq!(assess(&narrow)[0].status, CheckStatus::Fail);

    let mut insufficient_cover = demo.clone();
    insufficient_cover.values = insufficient_cover.values.with_number("fire_col_ambient_axis", 0.07);
    assert_eq!(assess(&insufficient_cover)[1].status, CheckStatus::Fail);

    let mut unsupported = demo;
    unsupported.values = unsupported.values.with_text("fire_col_rating", "R120");
    assert_eq!(assess(&unsupported)[0].status, CheckStatus::NotApplicable);
}

#[test]
fn table_4_7_handles_unreinforced_tubes_and_unsupported_loads() {
    use semio_s_plugin_norm_compliance::CheckStatus;

    let entry = design_case::entry::<En1994Cases>("C94-14").unwrap();
    let demo = design_case::seed::<En1994Cases>(&entry, "filled-tube");
    let assess = |case: &DesignCase| evaluate_design_case(case, AnnexChoice::De);
    assert_eq!(assess(&demo).len(), 2);
    assert!(assess(&demo).iter().all(|check| check.status == CheckStatus::Pass));

    let mut unreinforced = demo.clone();
    unreinforced.values = unreinforced.values.with_text("fire_col_ext_rating", "R30")
        .with_text("fire_fill_shape", "circular").with_text("fire_fill_rebar", "none")
        .with_number("fire_col_depth", 0.35).with_number("fire_fill_ratio", 0.0)
        .with_number("fire_fill_axis", 0.0).with_number("fire_col_ambient_axis", 0.0)
        .with_number("fire_fill_wall", 0.014);
    assert_eq!(assess(&unreinforced).len(), 1);
    assert_eq!(assess(&unreinforced)[0].status, CheckStatus::Pass);

    let mut unsupported = demo;
    unsupported.values = unsupported.values.with_text("fire_col_ext_rating", "R180").with_number("fire_col_load_level", 0.60);
    assert_eq!(assess(&unsupported)[0].status, CheckStatus::NotApplicable);
}

#[test]
fn table_4_4_keeps_each_set_of_minimum_dimensions_together() {
    use semio_s_plugin_norm_compliance::CheckStatus;

    let entry = design_case::entry::<En1994Cases>("C94-15").unwrap();
    let mut case = design_case::seed::<En1994Cases>(&entry, "encased-column");
    let assess = |case: &DesignCase| evaluate_design_case(case, AnnexChoice::De);
    assert_eq!(assess(&case).len(), 3);
    assert!(assess(&case).iter().all(|check| check.status == CheckStatus::Pass));

    case.values = case.values.with_text("fire_col_ext_rating", "R120").with_number("fire_enc_steel_cover", 0.05);
    assert_eq!(assess(&case)[0].status, CheckStatus::Pass);
    case.values = case.values.with_number("fire_col_width", 0.30);
    assert_eq!(assess(&case)[0].status, CheckStatus::Fail);
    case.values = case.values.with_number("fire_enc_bar_diameter", 0.010);
    assert_eq!(assess(&case)[2].status, CheckStatus::Fail);
}

#[test]
fn a_missing_input_is_a_single_not_applicable_result() {
    let entry = design_case::entry::<En1994Cases>("C94-01").expect("entry");
    let mut case = design_case::seed::<En1994Cases>(&entry, "case-gap");
    case.values.numbers.remove("p_ed");
    let results = evaluate_design_case(&case, AnnexChoice::En);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
}

#[test]
fn the_demo_cases_cover_every_entry_once() {
    let rows = catalogue_demo_cases();
    let entries = En1994Cases::entries();
    assert_eq!(rows.len(), entries.len());
    assert_eq!(entries.len(), ENTRIES.len());
    for (row, entry) in rows.iter().zip(&entries) {
        assert_eq!(row.entry_id, entry.id);
        assert!(!evaluate_design_case(&DesignCase::from(row), AnnexChoice::De).is_empty());
    }
}

#[test]
fn every_geometry_key_is_a_declared_input_of_its_entry() {
    for entry in En1994Cases::entries() {
        let geometry = entry.geometry.unwrap_or_else(|| panic!("{} declares no geometry", entry.id));
        for key in geometry.keys() {
            assert!(entry.inputs.contains(&key), "{}: geometry key {key} is no input", entry.id);
        }
    }
}

fn utilisation_with(id: &str, key: &str, value: f64) -> f64 {
    let entry = design_case::entry::<En1994Cases>(id).expect("catalogue entry");
    let mut case = design_case::seed::<En1994Cases>(&entry, "case-sweep");
    case.values.numbers.insert(key.into(), value);
    evaluate_design_case(&case, AnnexChoice::En)[0].utilization
}

#[test]
fn a_stiffer_floor_beam_lowers_the_vibration_utilisation() {
    assert!(utilisation_with("C94-09", "i_short", 1.2e-3) < utilisation_with("C94-09", "i_short", 6e-4));
}

#[test]
fn a_stiffer_slab_restraint_raises_the_buckling_resistance() {
    assert!(utilisation_with("C94-10", "i2_slab", 2e-4) < utilisation_with("C94-10", "i2_slab", 1e-5));
}

#[test]
fn creep_and_shrinkage_raise_the_deflection() {
    assert!(utilisation_with("C94-11", "phi_t", 3.5) > utilisation_with("C94-11", "phi_t", 0.0));
    assert!(utilisation_with("C94-11", "eps_cs", 5e-4) > utilisation_with("C94-11", "eps_cs", 0.0));
}

#[test]
fn a_thicker_protection_lowers_the_steel_temperature_ratio() {
    assert!(utilisation_with("C94-12", "d_p", 0.04) < utilisation_with("C94-12", "d_p", 0.01));
}

#[test]
fn the_floor_acceleration_is_reported_as_not_modelled_next_to_the_frequency() {
    let entry = design_case::entry::<En1994Cases>("C94-09").expect("entry");
    let results = evaluate_design_case(&design_case::seed::<En1994Cases>(&entry, "case-accel"), AnnexChoice::En);
    let gap = results.iter().find(|result| result.id.contains(".acceleration.")).expect("the acceleration gap is listed");
    assert_eq!(gap.status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
    assert_ne!(results[0].status, semio_s_plugin_norm_compliance::CheckStatus::NotApplicable);
    assert_eq!(results.len(), 2);
}

#[test]
fn insulating_encasement_matches_published_cover_and_mesh_boundaries() {
    use semio_s_plugin_norm_compliance::CheckStatus;
    for line in include_str!("insulating-encasement.tsv").lines().skip(1) {
        let [id, rating, basis, cover, mesh, spacing, diameter, min_cover, max_cover, expected] =
            line.split('\t').collect::<Vec<_>>().try_into().unwrap();
        let entry = design_case::entry::<En1994Cases>(id).unwrap();
        let mut demo = design_case::seed::<En1994Cases>(&entry, "insulation");
        demo.values = demo.values.with_text("fire_col_ext_rating", rating)
            .with_text(if id == "C94-16" { "fire_ins_beam_basis" } else { "fire_ins_column_basis" }, basis)
            .with_text("fire_ins_mesh", mesh);
        for (key, value) in [
            ("fire_ins_cover", cover), ("fire_ins_spacing", spacing), ("fire_ins_diameter", diameter),
            ("fire_ins_mesh_cover_min", min_cover), ("fire_ins_mesh_cover_max", max_cover),
        ] {
            demo.values = demo.values.with_number(key, value.parse().unwrap());
        }
        let results = evaluate_design_case(&demo, AnnexChoice::De);
        assert!(results.iter().all(|result| !result.id.contains(".input.") || result.status == CheckStatus::NotApplicable), "{line}");
        let actual = if results.iter().any(|result| result.status == CheckStatus::NotApplicable) {
            "unavailable"
        } else if results.iter().any(|result| result.status == CheckStatus::Fail) {
            "fail"
        } else if !results.is_empty() && results.iter().all(|result| result.status == CheckStatus::Pass) {
            "pass"
        } else {
            panic!("Unexpected assessment for {line}");
        };
        assert_eq!(actual, expected, "{line}");
    }
}

#[test]
fn beam_fire_tables_require_valid_geometry_deck_and_bar_cover() {
    use semio_s_plugin_norm_compliance::CheckStatus;
    let entry = design_case::entry::<En1994Cases>("C94-18").unwrap();
    let mut demo = design_case::seed::<En1994Cases>(&entry, "beam-fire");
    demo.values = demo.values.with_text("fire_beam_basis", "verified").with_text("fire_col_ext_rating", "R90");
    let assess = |case: &DesignCase| evaluate_design_case(case, AnnexChoice::De);
    let checks = assess(&demo);
    assert_eq!(checks.len(), 2);
    assert!(checks.iter().all(|row| row.status == CheckStatus::Pass));

    let mut sample = demo.clone();
    sample.values = sample.values.with_number("fire_beam_ambient_u1", 0.07);
    assert_eq!(assess(&sample)[1].status, CheckStatus::Fail);
    sample = demo.clone();
    sample.values = sample.values.with_number("fire_beam_fy", 460e6);
    assert_eq!(assess(&sample)[0].status, CheckStatus::Fail);
    sample = demo.clone();
    sample.values = sample.values.with_number("fire_beam_as", 0.0);
    assert_eq!(assess(&sample)[0].status, CheckStatus::Fail);
    sample = demo.clone();
    sample.values = sample.values.with_text("fire_beam_deck", "profiled").with_number("fire_beam_deck_coverage", 0.849);
    assert_eq!(assess(&sample)[0].status, CheckStatus::NotApplicable);
    sample.values = sample.values.with_text("fire_beam_deck", "closed_voids");
    assert!(assess(&sample).iter().all(|row| row.status == CheckStatus::Pass));
    sample = demo.clone();
    sample.values = sample.values.with_number("fire_beam_slab", 0.119);
    assert_eq!(assess(&sample)[0].status, CheckStatus::NotApplicable);
    sample = demo;
    sample.values = sample.values.with_number("fire_beam_width", 0.16).with_number("fire_beam_web", 0.01);
    assert_eq!(assess(&sample)[0].status, CheckStatus::NotApplicable);
}
