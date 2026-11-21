//! 🚆️ EN 1991-2:2003 + AC:2010 railway fatigue under §6.9 and Annex D.

use super::*;

pub(super) fn real_train(c: &Case) -> R {
    let length = c.pos("l_phi")?;
    let speed = c.pos("rail_fatigue_speed")?;
    if speed > 200.0 {
        return Err("input rail_fatigue_speed must lie within 0 to 200".into());
    }

    let v = speed / 3.6;
    let k = if length <= 20.0 { v / 160.0 } else { v / (47.16 * length.powf(0.408)) };
    let vehicle_effect = k / (1.0 - k + k.powi(4));
    let track_effect = 0.56 * (-length.powi(2) / 100.0).exp();
    let factor = 1.0 + 0.5 * (vehicle_effect + 0.5 * track_effect);

    Ok(vec![Outcome::info(
        "amplification", "Average real-train amplification", "Mittlere dynamische Erhöhung für reale Züge",
        QuantityKind::Dimensionless, factor,
    )
    .steps(format!("D.1–D.5: 1 + ½(φ′ + ½φ″) = {factor:.5}. The simplified method assumes a 100-year life. Apply §6.4.6.6 where dynamic analysis is required."))
    .steps_de(format!("D.1–D.5: 1 + ½(φ′ + ½φ″) = {factor:.5}. Das vereinfachte Verfahren setzt 100 Jahre Nutzungsdauer voraus. Ist eine dynamische Berechnung erforderlich, gilt Abschnitt 6.4.6.6."))])
}

pub(super) fn steel_bridge(c: &Case) -> R {
    let (phi_2, _) = part_2::rail_dynamic_factors(c.pos("l_phi")?);
    let lambda = c.pos("rail_fatigue_lambda")?;
    let gamma_f = c.pos("rail_fatigue_gamma_f")?;
    let gamma_m = c.pos("rail_fatigue_gamma_m")?;
    let strength = c.pos("rail_fatigue_strength")?;
    let stress = c.n("rail_fatigue_stress")?;
    if !stress.is_finite() || stress < 0.0 {
        return Err("input rail_fatigue_stress must not be negative".into());
    }
    let demand = gamma_f * lambda * phi_2 * stress;
    let capacity = strength / gamma_m;
    if !capacity.is_finite() || capacity <= 0.0 {
        return Err("Δσ_C / γ_Mf must be a positive finite number".into());
    }
    if !(demand / capacity).is_finite() {
        return Err("γ_Ff·λ·Φ₂·Δσ_71 / (Δσ_C/γ_Mf) must be finite".into());
    }

    let equation = format!("D.6: γ_Ff·λ·{phi_2:.5}·Δσ_71 ≤ Δσ_C/γ_Mf.");
    Ok(vec![Outcome::limit(
        "stress_range", "Equivalent steel fatigue stress range", "Schädigungsäquivalente Schwingbreite für Stahl",
        QuantityKind::Stress, demand, capacity,
    )
    .steps(format!("{equation} Use the stress range with LM71 and, where required, SW/0 in the governing position on up to two tracks. Determine λ, Δσ_C and γ_Mf from the applicable material rules."))
    .steps_de(format!("{equation} Maßgebend ist die Schwingbreite aus LM71 und, falls erforderlich, SW/0 in ungünstigster Stellung auf bis zu zwei Gleisen. λ, Δσ_C und γ_Mf sind nach den geltenden Werkstoffnormen zu bestimmen."))])
}

struct Train {
    number: u8,
    passages: u16,
    tonnes: u16,
}

const fn train(number: u8, passages: u16, tonnes: u16) -> Train {
    Train { number, passages, tonnes }
}

const STANDARD: &[Train] = &[
    train(1, 12, 663),
    train(2, 12, 530),
    train(3, 5, 940),
    train(4, 5, 510),
    train(5, 7, 2160),
    train(6, 12, 1431),
    train(7, 8, 1035),
    train(8, 6, 1035),
];
const HEAVY: &[Train] = &[train(5, 6, 2160), train(6, 13, 1431), train(11, 16, 1135), train(12, 16, 1135)];
const LIGHT: &[Train] = &[train(1, 10, 663), train(2, 5, 530), train(5, 2, 2160), train(9, 190, 296)];

pub(super) fn traffic_mix(c: &Case) -> R {
    let (table, trains) = match c.s("rail_fatigue_mix")? {
        "standard" => ("D.1", STANDARD),
        "heavy" => ("D.2", HEAVY),
        "light" => ("D.3", LIGHT),
        _ => return Err("input rail_fatigue_mix must be standard, heavy or light".into()),
    };
    let passages: f64 = trains.iter().map(|train| f64::from(train.passages)).sum();
    let mass: f64 = trains.iter().map(|train| f64::from(train.passages) * f64::from(train.tonnes) * 1000.0).sum();
    let mut results = vec![
        Outcome::info("trains_per_day", "Trains per day and track", "Züge pro Tag und Gleis", QuantityKind::Dimensionless, passages)
            .steps(format!("Table {table}. Select a project-specific mix where the published mix does not represent the actual traffic."))
            .steps_de(format!("Tabelle {table}. Wenn die veröffentlichte Verkehrsmischung den tatsächlichen Verkehr nicht abbildet, ist eine projektspezifische Mischung zu wählen.")),
        Outcome::info("daily_mass", "Daily traffic mass per track", "Tägliche Verkehrsmasse je Gleis", QuantityKind::Mass, mass)
            .steps("Σ (train mass × trains per day). Nominal basis per track: 25 million tonnes per year.")
            .steps_de("Σ (Zugmasse × Züge pro Tag). Nennwert je Gleis: 25 Millionen Tonnen pro Jahr."),
    ];
    for train in trains {
        results.push(Outcome::info(
            &format!("trains_{}", train.number), &format!("Type {} trains per day", train.number),
            &format!("Züge des Typs {} pro Tag", train.number), QuantityKind::Dimensionless, f64::from(train.passages),
        )
        .steps(format!("{} t per train.", train.tonnes))
        .steps_de(format!("{} t je Zug.", train.tonnes)));
    }
    Ok(results)
}
