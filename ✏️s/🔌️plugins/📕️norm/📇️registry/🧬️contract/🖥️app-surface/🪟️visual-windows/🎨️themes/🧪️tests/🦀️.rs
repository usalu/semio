//! 🧪️ Theme tests; scenarios in `🥒️.feature`. Independent oracle: the Python colour oracle of the colour maps checks the contrast of custom backgrounds; wire twins are compared textually.

use super::*;
use semio_framework_value::{FromValue, ToValue};

fn schema_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🪟️visual-windows/🎨️themes/🧬️schema")
}

#[test]
fn built_in_themes_are_valid_bilingual_and_accessible() {
    let themes = builtin_themes();
    assert_eq!(themes.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(), ["light", "dark", "high-contrast"]);
    for t in &themes {
        assert_eq!(validate(t), Vec::<String>::new(), "{}", t.id);
        assert_eq!(accessibility_findings(t), Vec::<String>::new(), "{}", t.id);
        assert!(!t.name.en.is_empty() && !t.name.de.is_empty());
    }
    assert_eq!(themes[2].variant.variant(), Variant::HighContrast);
    assert_eq!(themes[2].edge_style, EdgeStyle::Bold);
}

#[test]
fn bundles_round_trip_through_the_value_codec() {
    let mut t = builtin_themes().remove(1);
    t.background = Some(Rgb { r: 0, g: 0, b: 0 });
    t.role_colours.push(RoleColour { role: "steel".into(), rgb: Rgb { r: 10, g: 20, b: 30 } });
    assert_eq!(ThemeBundle::from_value(t.to_value()).expect("round trip"), t);
}

#[test]
fn invalid_bundles_report_each_violated_path() {
    let mut t = builtin_themes().remove(0);
    t.id = "Bad Id".into();
    t.utilisation_ramp = "diverging".into();
    t.result_ramps.stress = "warm".into();
    t.domain_max = 1.0;
    t.text_scale = 5.0;
    t.name.de.clear();
    t.role_colours = vec![RoleColour { role: "steel".into(), rgb: Rgb { r: 0, g: 0, b: 0 } }, RoleColour { role: "steel".into(), rgb: Rgb { r: 1, g: 1, b: 1 } }];
    let paths: Vec<String> = validate(&t).into_iter().map(|l| l.split(':').next().expect("path").to_owned()).collect();
    assert_eq!(paths, ["id", "name", "utilisationRamp", "resultRamps.stress", "domainMax", "textScale", "roleColours[1].role"]);
}

#[test]
fn custom_backgrounds_that_hide_ramps_are_reported() {
    let mut t = builtin_themes().remove(0);
    t.background = Some(Rgb { r: 255, g: 235, b: 120 });
    let findings = accessibility_findings(&t);
    assert!(findings.iter().any(|f| f.starts_with("utilisationRamp")), "{findings:?}");
    t.background = Some(Rgb { r: 255, g: 255, b: 255 });
    assert!(accessibility_findings(&t).is_empty());
}

#[test]
fn every_declared_type_is_in_every_twin() {
    let rust = std::fs::read_to_string(schema_dir().join("🦀️.rs")).expect("rust");
    let names: Vec<&str> = rust.lines().filter_map(|l| l.trim().strip_prefix("pub struct ").or_else(|| l.trim().strip_prefix("pub enum "))).map(|l| l.trim_end_matches(" {")).collect();
    assert_eq!(names, ["ThemeVariant", "EdgeStyle", "MotionPolicy", "ResultRamps", "RoleColour", "ThemeBundle"]);
    for twin in ["🛰️.proto", "🟦️.ts", "🔗️.graphql"] {
        let text = std::fs::read_to_string(schema_dir().join(twin)).expect("twin");
        for n in &names {
            assert!(text.contains(&format!("{n} ")) || text.contains(&format!("{n}{{")), "{n} missing in {twin}");
        }
    }
}

#[test]
fn result_fields_resolve_to_their_ramps() {
    let t = builtin_themes().remove(0);
    assert_eq!(ramp_for(&t, ResultField::Stress), Some(RampId::Diverging));
    assert_eq!(ramp_for(&t, ResultField::Temperature), Some(RampId::Warm));
    assert_eq!(ramp_for(&t, ResultField::Settlement), Some(RampId::Cool));
}

#[test]
fn system_preferences_pick_the_high_contrast_theme_and_motion_policy() {
    use crate::accessibility::A11yPrefs;
    let light = builtin_themes().remove(0);
    let calm = A11yPrefs::default();
    assert_eq!(effective_theme(calm, None).id, "light");
    assert_eq!(effective_theme(A11yPrefs { prefers_dark: true, ..calm }, None).id, "dark");
    assert_eq!(effective_theme(A11yPrefs { prefers_dark: true, ..calm }, Some(&light)).id, "light", "an explicit bundle beats the dark preference");
    for forced in [A11yPrefs { forced_colors: true, ..calm }, A11yPrefs { prefers_contrast_more: true, ..calm }] {
        let theme = effective_theme(forced, Some(&light));
        assert_eq!((theme.id.as_str(), theme.variant), ("high-contrast", ThemeVariant::HighContrast));
        assert!(accessibility_findings(&theme).is_empty() && theme.text_scale > 1.0 && theme.edge_style == EdgeStyle::Bold);
        let mut custom = builtin_themes().remove(2);
        custom.id = "mine".into();
        assert_eq!(effective_theme(forced, Some(&custom)).id, "mine");
    }
    let mut reduced = light.clone();
    assert!(motion_allowed(calm, &reduced) && !motion_allowed(A11yPrefs { prefers_reduced_motion: true, ..calm }, &reduced));
    reduced.motion = MotionPolicy::Reduced;
    assert!(!motion_allowed(calm, &reduced));
    reduced.motion = MotionPolicy::Full;
    assert!(motion_allowed(A11yPrefs { prefers_reduced_motion: true, ..calm }, &reduced));
}

#[test]
fn every_built_in_theme_uses_ramps_that_stay_distinguishable_for_colour_blind_users() {
    use semio_s_plugin_norm_structural_scene::colour_maps::metrics::{min_pairwise_delta_e2000, ALL_CVD};
    for theme in builtin_themes() {
        let variant = theme.variant.variant();
        let ramp = ramp(RampId::parse(&theme.utilisation_ramp).expect("ramp"), variant);
        let ends = [ramp.sample(0.0), ramp.sample(1.0)];
        for cvd in ALL_CVD {
            assert!(min_pairwise_delta_e2000(cvd, &ends) >= 15.0, "{} {cvd:?}", theme.id);
        }
    }
}
