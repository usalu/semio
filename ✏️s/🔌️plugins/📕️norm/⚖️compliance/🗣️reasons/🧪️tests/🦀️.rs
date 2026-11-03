use super::*;

#[test]
fn rule_codes_are_unique_and_templates_place_every_hole() {
    let mut codes: Vec<&str> = RULES.iter().map(|rule| rule.code).collect();
    codes.sort_unstable();
    codes.dedup();
    assert_eq!(codes.len(), RULES.len());
    for rule in RULES {
        let holes = rule.en.matches("{}").count();
        for index in 0..holes {
            assert!(rule.de.contains(&format!("{{{index}}}")), "{} drops hole {index}", rule.code);
        }
        assert!(!rule.de.contains(&format!("{{{holes}}}")), "{} places a hole the pattern lacks", rule.code);
        assert_ne!(rule.en, rule.de, "{} is untranslated", rule.code);
    }
}

#[test]
fn reasons_capture_parameters_and_translate_both_ways() {
    let missing = explain_reason("missing input b and h (or area, i_y, w_y)");
    assert_eq!(missing.code, "input.missing");
    assert_eq!(missing.de, "Eingabe b und h (oder area, i_y, w_y) fehlt");
    let zone = explain_reason("unknown wind zone 7; admissible: 1, 2, 3, 4");
    assert_eq!((zone.code, zone.params.as_slice()), ("value.unknown-wind-zone", ["7".to_string(), "1, 2, 3, 4".to_string()].as_slice()));
    assert_eq!(zone.de, "unbekannte Windzone 7; zulässig: 1, 2, 3, 4");
    let range = explain_reason("input lane must be 1, 2 or 3");
    assert_eq!(range.de, "Eingabe lane muss 1, 2 oder 3 sein");
    assert_eq!(explain_reason("input x must be positive.").de, "Eingabe x muss positiv sein");
}

#[test]
fn unmapped_reason_keeps_its_text_under_the_unmapped_code() {
    let reason = explain_reason("something nobody wrote a rule for");
    assert_eq!((reason.code, reason.en.as_str(), reason.de.as_str()), (UNMAPPED_REASON, "something nobody wrote a rule for", "something nobody wrote a rule for"));
}

#[test]
fn english_heuristic_flags_sentences_and_allows_symbols_units_and_german() {
    assert!(looks_english("missing input capacity"));
    assert!(looks_english("The value must be finite"));
    assert!(!looks_english("Eingabe b und h fehlt"));
    assert!(!looks_english("N_Rd = 12.5 kN/m²"));
    assert!(!looks_english("γ_M0, f_yk, EN 1992-1-1 §6.2"));
    assert!(!looks_english("Dicke erhöhen"));
}

#[test]
fn every_rule_renders_german_without_english_sentences() {
    for rule in RULES {
        let params: Vec<String> = (0..rule.en.matches("{}").count()).map(|index| format!("p{index}")).collect();
        let de = fill(rule.de, &params);
        assert!(!looks_english(&de), "{} leaks English into German: {de}", rule.code);
    }
}

#[test]
fn the_en1997_gate_texts_have_a_rule_and_a_german_text() {
    for (text, code) in [
        ("flow net needs ΔH > 0 and a layer thicker than the embedment", "geometry.flow-net"),
        ("base width must exceed toe length plus stem thickness", "geometry.base-width"),
        ("no depth of zero net pressure found below the dredge level", "geometry.zero-net-pressure"),
        ("prop depth must lie above the dredge level", "geometry.prop-depth"),
        ("γ_sat must exceed γ_w", "value.gamma-sat"),
        ("t2 must exceed t1", "geometry.t2-exceeds-t1"),
        ("the exit gradient must be positive", "value.exit-gradient"),
    ] {
        let reason = explain_reason(text);
        assert_eq!(reason.code, code, "{text}");
        assert!(!looks_english(&reason.de), "{text}: {}", reason.de);
    }
}
