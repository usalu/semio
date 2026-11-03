use super::*;
use crate::python_oracle;
use serde_json::{json, Value};

fn oracle(request: &Value) -> Value {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../📐️quantities-clauses-and-text/🔢️units/🔮️oracles/🐍️.py");
    let output = python_oracle::run_ok(&script, &[], Some(request.to_string().as_bytes()));
    serde_json::from_slice(&output.stdout).expect("oracle json")
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1e-300) + 1e-300
}

#[test]
fn every_unit_factor_offset_and_dimension_matches_the_independent_oracle() {
    let symbols: Vec<_> = UnitId::ALL.iter().map(|u| u.symbol()).collect();
    let answer = oracle(&json!({ "units": symbols }));
    let rows = answer["units"].as_array().expect("units");
    assert_eq!(rows.len(), UnitId::ALL.len());
    let mut dims = std::collections::HashMap::new();
    for (unit, row) in UnitId::ALL.iter().zip(rows) {
        assert!(close(unit.factor(), row["factor"].as_f64().unwrap()), "{} factor {} vs {}", unit.symbol(), unit.factor(), row["factor"]);
        assert!(close(unit.offset(), row["offset"].as_f64().unwrap()), "{} offset", unit.symbol());
        let previous = dims.insert(unit.family(), row["dim"].clone());
        assert!(previous.is_none_or(|p| p == row["dim"]), "{:?} mixes dimensions at {}", unit.family(), unit.symbol());
    }
}

#[test]
fn conversions_between_all_compatible_units_match_the_oracle() {
    let cases: Vec<_> = UnitId::ALL.iter().flat_map(|from| from.compatible().map(move |to| (*from, to))).collect();
    let request: Vec<_> = cases.iter().map(|(from, to)| json!({ "value": 123.456, "from": from.symbol(), "to": to.symbol() })).collect();
    let answer = oracle(&json!({ "conversions": request }));
    for ((from, to), expected) in cases.iter().zip(answer["conversions"].as_array().unwrap()) {
        let got = from.convert(123.456, *to).expect("compatible");
        assert!(close(got, expected.as_f64().unwrap()), "{} -> {}: {got} vs {expected}", from.symbol(), to.symbol());
    }
}

#[test]
fn number_formatting_matches_the_oracle_for_both_locales() {
    let values = [0.0, -0.001, 0.5, 4.25, 1234.5, 12345.678, -987654.321, 1e9, 3.0e-7];
    let mut cases = Vec::new();
    for value in values {
        for precision in [0u8, 2, 3] {
            for locale in [Locale::En, Locale::De] {
                cases.push((value, precision, locale));
            }
        }
    }
    let request: Vec<_> = cases.iter().map(|(v, p, l)| json!({ "value": v, "precision": p, "locale": if *l == Locale::De { "de" } else { "en" } })).collect();
    let answer = oracle(&json!({ "numbers": request }));
    for ((value, precision, locale), expected) in cases.iter().zip(answer["numbers"].as_array().unwrap()) {
        assert_eq!(format_number(*value, *precision, *locale), expected.as_str().unwrap(), "{value} p{precision} {locale:?}");
    }
}

#[test]
fn parsing_matches_the_oracle_value_and_unit() {
    let texts = ["12,5 kN", "1.250,5mm", "3", "1 250 mm", "-0.0 m", "2e3 N", "1e-3", "1\u{2009}250,75\u{a0}kN·m", "12.5kN", "0,25 MPa", "7 °C", "100 %"];
    let answer = oracle(&json!({ "parses": texts }));
    for (text, expected) in texts.iter().zip(answer["parses"].as_array().unwrap()) {
        let parsed = Measure::parse(text, UnitId::Metre).expect(text);
        assert!(close(parsed.value, expected["value"].as_f64().unwrap()), "{text}");
        match expected["factor"].as_f64() {
            Some(factor) => assert!(close(parsed.unit.factor(), factor) && close(parsed.unit.offset(), expected["offset"].as_f64().unwrap()), "{text}"),
            None => assert_eq!(parsed.unit, UnitId::Metre, "{text}"),
        }
    }
}

#[test]
fn symbols_aliases_and_localized_names_resolve_back_to_their_unit() {
    for unit in UnitId::ALL {
        assert_eq!(UnitId::lookup(unit.symbol()), Some(*unit), "symbol {}", unit.symbol());
        for locale in [Locale::En, Locale::De] {
            let name = unit.name(locale);
            assert!(!name.is_empty());
            assert_eq!(UnitId::lookup(name), Some(*unit), "{locale:?} name {name}");
        }
    }
    assert_eq!(UnitId::lookup("m2"), Some(UnitId::SquareMetre));
    assert_eq!(UnitId::lookup("kNm"), Some(UnitId::KilonewtonMetre));
    assert_eq!(UnitId::lookup("kN*m"), Some(UnitId::KilonewtonMetre));
    assert_eq!(UnitId::lookup("Kilonewton"), Some(UnitId::Kilonewton));
    assert_eq!(UnitId::lookup("furlong"), None);
}

#[test]
fn offsets_and_family_guards() {
    assert!(close(UnitId::Celsius.convert(25.0, UnitId::Kelvin).unwrap(), 298.15));
    assert!(close(UnitId::Kelvin.convert(273.15, UnitId::Celsius).unwrap().abs() + 1.0, 1.0));
    assert!(close(UnitId::Degree.convert(180.0, UnitId::Radian).unwrap(), std::f64::consts::PI));
    assert_eq!(UnitId::Metre.convert(1.0, UnitId::Newton), Err(UnitError::Incompatible { from: UnitId::Metre, to: UnitId::Newton }));
    assert_eq!(UnitId::Kilonewton.quantity_kind(), Some(QuantityKind::Force));
    assert_eq!(UnitId::Hertz.quantity_kind(), None);
    assert_eq!(UnitId::MetrePerSecond.quantity_kind(), Some(QuantityKind::Velocity));
    assert_eq!(UnitId::KilometrePerHour.quantity_kind(), Some(QuantityKind::Velocity));
    assert!(close(UnitId::KilometrePerHour.convert(36.0, UnitId::MetrePerSecond).unwrap(), 10.0));
    assert_eq!(crate::Quantity::velocity_m_s(27.0), crate::Quantity::new(QuantityKind::Velocity, 27.0));
    assert_eq!(UnitId::Kilonewton.base(), Some(UnitId::Newton));
}

#[test]
fn parse_errors_are_specific() {
    assert_eq!(Measure::parse("  ", UnitId::Metre), Err(UnitError::Empty));
    assert_eq!(Measure::parse("12 furlong", UnitId::Metre), Err(UnitError::UnknownUnit("furlong".into())));
    assert!(matches!(Measure::parse("kN", UnitId::Metre), Err(UnitError::InvalidNumber(_))));
}

#[test]
fn measures_round_trip_through_si_and_format() {
    let measure = Measure::parse("12,5 kN", UnitId::Newton).unwrap();
    assert!(close(measure.si(), 12_500.0) && close(measure.to(UnitId::Meganewton).unwrap(), 0.0125));
    assert_eq!(measure.format(1, Locale::De), "12,5\u{a0}kN");
    assert_eq!(measure.format(1, Locale::En), "12.5\u{a0}kN");
    assert_eq!(Measure { value: 1234567.891, unit: UnitId::Pascal }.format(2, Locale::De), "1\u{2009}234\u{2009}567,89\u{a0}Pa");
}

#[test]
fn the_engineering_preset_picks_site_units() {
    assert_eq!(UnitId::Metre.in_system(UnitSystem::Engineering), UnitId::Millimetre);
    assert_eq!(UnitId::Pascal.in_system(UnitSystem::Engineering), UnitId::Megapascal);
    assert_eq!(UnitId::Newton.in_system(UnitSystem::Engineering), UnitId::Kilonewton);
    assert_eq!(UnitId::Newton.in_system(UnitSystem::Si), UnitId::Newton);
    assert_eq!(UnitId::Kelvin.in_system(UnitSystem::Engineering), UnitId::Kelvin);
}
