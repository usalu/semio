//! 🏗️ Loading class guidance from informative Annex B of EN 1991-3:2006.

use super::*;

const CLASS_RANGES: &[(u8, u8, u8, u8)] = &[
    (1, 1, 0, 1), (1, 2, 0, 1), (1, 1, 1, 2), (2, 2, 4, 4),
    (3, 4, 6, 7), (2, 3, 3, 4), (3, 4, 6, 7), (2, 3, 6, 7),
    (3, 4, 7, 8), (4, 4, 8, 9), (4, 4, 6, 7), (2, 2, 4, 5),
    (3, 4, 6, 7), (1, 1, 3, 4), (2, 2, 3, 4), (2, 2, 4, 5),
    (3, 4, 6, 7), (1, 1, 1, 2), (2, 2, 3, 4), (3, 4, 4, 5),
    (1, 1, 2, 3), (1, 2, 1, 2), (2, 2, 3, 4), (3, 4, 4, 5),
    (2, 2, 4, 4), (2, 2, 3, 4), (3, 4, 4, 5), (1, 1, 1, 2),
];

pub(super) fn classify(c: &Case) -> R {
    let duty = c.s("crane_duty")?;
    let &(hc_min, hc_max, s_min, s_max) = duty.parse::<usize>().ok()
        .and_then(|item| item.checked_sub(1))
        .and_then(|index| CLASS_RANGES.get(index))
        .ok_or_else(|| "input crane_duty must lie within 1 to 28".to_string())?;
    let outputs = [
        ("hc_min", "Lowest recommended hoisting class HC", "Niedrigste empfohlene Hubklasse HC", hc_min),
        ("hc_max", "Highest recommended hoisting class HC", "Höchste empfohlene Hubklasse HC", hc_max),
        ("s_min", "Lowest recommended fatigue class S", "Niedrigste empfohlene Ermüdungsklasse S", s_min),
        ("s_max", "Highest recommended fatigue class S", "Höchste empfohlene Ermüdungsklasse S", s_max),
    ];
    Ok(outputs.into_iter().map(|(key, en, de, class)| {
        Outcome::info(key, en, de, QuantityKind::Dimensionless, f64::from(class))
            .steps(format!("Table B.1, item {duty}. Use this guidance where supplier classification is unavailable."))
            .steps_de(format!("Tabelle B.1, Zeile {duty}. Diese Empfehlung gilt bei fehlender Kranhersteller-Einstufung."))
    }).collect())
}
