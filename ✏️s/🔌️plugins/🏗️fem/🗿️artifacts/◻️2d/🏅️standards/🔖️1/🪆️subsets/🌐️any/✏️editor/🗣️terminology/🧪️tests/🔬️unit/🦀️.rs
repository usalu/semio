use super::*;
use semio_framework_plugin::{Locale, Terminology, ViewModel};

fn view(locale: Locale, terminology: Terminology) -> ViewModel {
    ViewModel { locale, terminology, ..Default::default() }
}

#[test]
fn labels_resolve_every_locale_and_terminology_axis_from_the_shared_view_state() {
    assert_eq!(fem2d_labels(&ViewModel::default()).nodes.as_str(), "Nodes");
    assert_eq!(fem2d_labels(&view(Locale::De, Terminology::Native)).nodes.as_str(), "Knoten");
    assert_eq!(fem2d_labels(&view(Locale::En, Terminology::Reuse)).load_cases.as_str(), "Load Cases");
    assert_eq!(fem2d_labels(&view(Locale::De, Terminology::Reuse)).load_cases.as_str(), "Lastfälle");
    assert!(fem2d_is_de_locale(&view(Locale::De, Terminology::Native)));
    assert!(!fem2d_is_de_locale(&ViewModel::default()));
}

/// 🗣️ Every declared label carries a non-empty spelling on all four axes, and `reuse` repeats
/// `native`: fem2d has no second vocabulary, so a diverging reuse cell would be a typo nobody reads.
#[test]
fn every_label_is_non_empty_and_reuse_repeats_native() {
    let mut native_en = Vec::new();
    let mut native_de = Vec::new();
    Fem2dLabels::NATIVE_EN.for_each_label(|field, text| native_en.push((field, text.as_str().to_string())));
    Fem2dLabels::NATIVE_DE.for_each_label(|field, text| native_de.push((field, text.as_str().to_string())));
    let mut index = 0;
    Fem2dLabels::REUSE_EN.for_each_label(|field, text| {
        assert!(!text.as_str().is_empty(), "{field}: reuse_en is never empty");
        assert_eq!(native_en[index].0, field, "both walks visit the same fields in the same order");
        assert_eq!(native_en[index].1, text.as_str(), "{field}: reuse_en repeats native_en");
        index += 1;
    });
    assert_eq!(index, Fem2dLabels::FIELD_NAMES.len());
    index = 0;
    Fem2dLabels::REUSE_DE.for_each_label(|field, text| {
        assert!(!text.as_str().is_empty(), "{field}: reuse_de is never empty");
        assert_eq!(native_de[index].1, text.as_str(), "{field}: reuse_de repeats native_de");
        index += 1;
    });
    assert_eq!(index, Fem2dLabels::FIELD_NAMES.len());
}

/// 🇩🇪️ No label ships English in its German slot. `app_labels!` already makes a MISSING locale a
/// compile error; this closes the locale that is PRESENT but wrong.
#[test]
fn german_is_a_translation_rather_than_a_copy_of_the_english() {
    let identical: std::collections::BTreeSet<&str> = FEM2D_LABELS_IDENTICAL_BY_DESIGN.iter().copied().collect();
    let declared: std::collections::BTreeSet<&str> = Fem2dLabels::FIELD_NAMES.iter().copied().collect();
    assert!(identical.is_subset(&declared), "every identical-by-design field is a declared label");
    let mut english = Vec::new();
    Fem2dLabels::NATIVE_EN.for_each_label(|field, text| english.push((field, text.as_str().to_string())));
    let mut checked = 0;
    let mut index = 0;
    Fem2dLabels::NATIVE_DE.for_each_label(|field, text| {
        let (english_field, english_text) = &english[index];
        index += 1;
        assert_eq!(*english_field, field, "both walks visit the same fields in the same order");
        if identical.contains(field) {
            assert_eq!(english_text.as_str(), text.as_str(), "{field}: declared identical by design, so it must actually be identical");
        } else {
            assert_ne!(english_text.as_str(), text.as_str(), "{field}: the German slot still holds the English text");
            checked += 1;
        }
    });
    assert_eq!(checked, Fem2dLabels::FIELD_NAMES.len() - identical.len(), "every non-identical field was checked");
}

/// 🗣️ The label roster the artifact tree, the inspector and the results panel all read — a field
/// removed here breaks a panel that has no other vocabulary to fall back on.
#[test]
fn the_roster_covers_every_noun_field_and_verb_the_panels_bind() {
    let declared: std::collections::BTreeSet<&str> = Fem2dLabels::FIELD_NAMES.iter().copied().collect();
    let required = [
        "node", "element", "region", "support", "load", "material", "section", "load_case", "combination", "term", "nodes", "elements", "regions", "supports", "loads", "materials", "sections", "load_cases", "combinations", "terms", "bar", "beam",
        "id", "name", "x", "y", "kind", "start", "end", "youngs_modulus", "poisson_ratio", "density", "area", "second_moment_of_area", "fixed", "tx", "ty", "rz", "dof", "value", "wx", "wy", "pressure", "thickness", "mesh_size", "outline", "holes",
        "self_weight", "factor", "points", "modal_count", "buckling_count", "deformation_scale", "source", "mode", "static_mode", "modal", "buckling", "mode_index", "phase", "playing", "play", "pause", "speed", "loop_mode", "ping_pong", "once",
        "waveform", "ramp", "sine", "step", "frequency", "load_factor", "focus", "delete", "none", "summary", "schema", "analysis", "results", "artifact", "inspection",
    ];
    for field in required {
        assert!(declared.contains(field), "{field}: the panels bind this label");
    }
}
