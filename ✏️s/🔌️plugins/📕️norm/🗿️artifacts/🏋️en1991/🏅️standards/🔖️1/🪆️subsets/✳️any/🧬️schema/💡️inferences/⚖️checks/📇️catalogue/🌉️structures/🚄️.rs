//! 🚄️ First-generation EN 1991-2 HSLM rail loads and model selection under §6.4.6.1.1.

use super::*;

struct Train {
    coaches: u32,
    coach_mm: u32,
    bogie_mm: u32,
    force_kn: u32,
}

const fn train(coaches: u32, coach_mm: u32, bogie_mm: u32, force_kn: u32) -> Train {
    Train { coaches, coach_mm, bogie_mm, force_kn }
}

const UNIVERSAL_TRAINS: &[(&str, Train)] = &[
    ("A1", train(18, 18_000, 2_000, 170)),
    ("A2", train(17, 19_000, 3_500, 200)),
    ("A3", train(16, 20_000, 2_000, 180)),
    ("A4", train(15, 21_000, 3_000, 190)),
    ("A5", train(14, 22_000, 2_000, 170)),
    ("A6", train(13, 23_000, 2_000, 180)),
    ("A7", train(13, 24_000, 2_000, 190)),
    ("A8", train(12, 25_000, 2_500, 190)),
    ("A9", train(11, 26_000, 2_000, 210)),
    ("A10", train(11, 27_000, 2_000, 210)),
];

impl Train {
    /// 🛤️ Longitudinal axle positions from the leading axle, in half-millimetres.
    fn axle_positions(&self) -> Vec<u32> {
        const LAST_POWER_AXLE_MM: u32 = 17_000;
        const POWER_TO_COACH_AXLES_MM: u32 = 3_525;
        let first_coach_axle_mm = LAST_POWER_AXLE_MM + POWER_TO_COACH_AXLES_MM;
        let first_end = [0, 3_000, 14_000, LAST_POWER_AXLE_MM, first_coach_axle_mm, first_coach_axle_mm + self.bogie_mm];
        let power_joint = LAST_POWER_AXLE_MM * 2 + POWER_TO_COACH_AXLES_MM;
        let rear = 2 * (power_joint + (self.coaches + 2) * self.coach_mm);
        let shared_bogies = (1..=self.coaches + 1).flat_map(|coach| {
            let centre = power_joint + 2 * coach * self.coach_mm;
            [centre - self.bogie_mm, centre + self.bogie_mm]
        });
        first_end.iter().map(|axle| 2 * axle)
            .chain(shared_bogies)
            .chain(first_end.iter().rev().map(|axle| rear - 2 * axle))
            .collect()
    }
}

pub(super) fn universal_train(c: &Case) -> R {
    let id = c.s("rail_hslm_a_train")?;
    let (_, train) = UNIVERSAL_TRAINS.iter().find(|(name, _)| *name == id)
        .ok_or_else(|| format!("unknown model {id}"))?;
    let force = f64::from(train.force_kn) * 1000.0;
    let distance = |mm: u32| f64::from(mm) / 1000.0;
    let mut results = vec![
        Outcome::info("intermediate_coaches", "Intermediate coaches", "Mittelwagen", QuantityKind::Dimensionless, f64::from(train.coaches))
            .steps(format!("Table 6.3, train {id}. Apply Table 6.4 to select the train set required for the bridge."))
            .steps_de(format!("Tabelle 6.3, Zug {id}. Die für die Brücke erforderlichen Züge nach Tabelle 6.4 auswählen.")),
        Outcome::info("coach_length", "Coach length D", "Wagenlänge D", QuantityKind::Length, distance(train.coach_mm)),
        Outcome::info("bogie_spacing", "Bogie axle spacing d", "Achsabstand im Drehgestell d", QuantityKind::Length, distance(train.bogie_mm)),
        Outcome::info("axle_force", "Vertical point force per axle P", "Vertikale Einzellast je Achse P", QuantityKind::Force, force),
    ];
    results.extend(axle_outcomes(train.axle_positions().into_iter().map(|position| f64::from(position) / 2000.0)));
    Ok(results)
}

const HSLM_B_POINTS: &[(f64, u32, f64)] = &[
    (1.0, 10, 3.5), (1.6, 10, 3.0), (2.5, 10, 3.0), (2.8, 15, 3.0), (3.2, 15, 3.0),
    (3.5, 20, 3.0), (3.8, 20, 3.25), (4.2, 20, 3.25), (4.5, 15, 3.5), (4.8, 20, 2.5),
    (5.5, 10, 2.5), (5.8, 15, 3.5), (6.5, 10, 3.5),
];

pub(super) fn uniform_train(c: &Case) -> R {
    let span = c.pos("rail_hslm_b_span")?;
    if span > 7.0 {
        return Err("input rail_hslm_b_span must lie within 0 to 7".into());
    }
    let published = HSLM_B_POINTS.iter().find(|(point, _, _)| (span - point).abs() < 1e-9);
    let manual = if c.opt("rail_hslm_b_count").is_some() || c.opt("rail_hslm_b_spacing").is_some() {
        let count = c.n("rail_hslm_b_count")?;
        let spacing = c.n("rail_hslm_b_spacing")?;
        if !(10.0..=20.0).contains(&count) {
            return Err("input rail_hslm_b_count must lie within 10 to 20".into());
        }
        if count.fract() != 0.0 {
            return Err("input rail_hslm_b_count must be a whole number".into());
        }
        if !(2.5..=3.5).contains(&spacing) {
            return Err("input rail_hslm_b_spacing must lie within 2.5 to 3.5".into());
        }
        Some((count as u32, spacing))
    } else {
        None
    };
    let (count, spacing, en, de) = match (published, manual) {
        (Some(&(_, n, d)), Some(inputs)) if inputs != (n, d) => {
            return Err("inputs rail_hslm_b_count and rail_hslm_b_spacing must match Figure 6.14".into());
        }
        (Some(&(_, n, d)), _) => (n, d, "Read from the labelled span in Figure 6.14.", "Aus der beschrifteten Stützweite in Bild 6.14 abgelesen."),
        (None, Some((n, d))) => (n, d, "Uses the supplied Figure 6.14 inputs for this span.", "Verwendet die eingegebenen Werte aus Bild 6.14 für diese Stützweite."),
        (None, None) => return Err("missing inputs: rail_hslm_b_count, rail_hslm_b_spacing (Figure 6.14)".into()),
    };
    let mut results = vec![
        Outcome::info("axle_count", "Number of axle loads N", "Anzahl der Achslasten N", QuantityKind::Dimensionless, f64::from(count)).steps(en).steps_de(de),
        Outcome::info("axle_spacing", "Uniform axle spacing d", "Gleichmäßiger Achsabstand d", QuantityKind::Length, spacing),
        Outcome::info("axle_force", "Vertical point force per axle P", "Vertikale Einzellast je Achse P", QuantityKind::Force, 170_000.0),
    ];
    results.extend(axle_outcomes((0..count).map(|index| f64::from(index) * spacing)));
    Ok(results)
}

fn axle_outcomes(positions: impl Iterator<Item = f64>) -> impl Iterator<Item = Outcome> {
    positions.enumerate().map(|(index, position)| {
        let axle = index + 1;
        Outcome::info(
            &format!("axle_at_{axle}"), &format!("Axle {axle} position from lead axle"), &format!("Lage der Achse {axle} ab erster Achse"),
            QuantityKind::Length, position,
        )
    })
}

pub(super) fn load_model_selection(c: &Case) -> R {
    let span = c.pos("span")?;
    let (a_all, a_one_eligible, b) = match c.s("rail_hslm_structure")? {
        "simple" if span < 7.0 => (0.0, 0.0, 1.0),
        "simple" => (1.0, 1.0, 0.0),
        "continuous" | "complex" => (1.0, 0.0, 0.0),
        "complex_floor" => (1.0, 0.0, 1.0),
        shape => return Err(format!("unknown structure {shape}")),
    };
    Ok(vec![
        Outcome::info("a_all", "HSLM-A applies", "HSLM-A ist anzuwenden", QuantityKind::Dimensionless, a_all)
            .steps("Where applicable, consider A1–A10. Table 6.4 permits Annex E critical-train selection for qualifying simply supported spans of 7 m or more.")
            .steps_de("Falls anzuwenden, A1–A10 berücksichtigen. Tabelle 6.4 erlaubt die Auswahl des maßgebenden Zugs nach Anhang E für geeignete Einfeldtragwerke ab 7 m."),
        Outcome::info("a_one_eligible", "Annex E critical-train selection permitted", "Auswahl des maßgebenden Zugs nach Anhang E zulässig", QuantityKind::Dimensionless, a_one_eligible),
        Outcome::info("b", "HSLM-B applies", "HSLM-B ist anzuwenden", QuantityKind::Dimensionless, b)
            .steps("Apply HSLM-B to qualifying simple spans under 7 m and additionally to complex structures with significant floor vibration modes. Check project and national requirements for continuous and complex structures.")
            .steps_de("HSLM-B bei geeigneten Einfeldtragwerken unter 7 m und zusätzlich bei komplexen Tragwerken mit maßgebenden Fahrbahnplattenschwingungen ansetzen. Für Durchlauf- und komplexe Tragwerke die Projektvorgaben und nationalen Festlegungen prüfen."),
    ])
}
