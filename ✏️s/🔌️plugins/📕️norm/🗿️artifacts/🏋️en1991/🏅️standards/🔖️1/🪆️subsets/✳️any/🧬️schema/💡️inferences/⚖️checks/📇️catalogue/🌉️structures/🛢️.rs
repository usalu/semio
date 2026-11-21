//! 🛢️ Circular tank wind and specified internal pressure actions from EN 1991-4:2006 Annex B.

use super::*;

pub(super) fn wind(c: &Case) -> R {
    let internal = match c.s("tank_wind_enclosure")? {
        "open_tank" | "open_basin" => -0.6,
        "small_vents" => -0.4,
        enclosure => return Err(format!("unknown structure {enclosure}")),
    };
    let external = c.n("tank_wind_cpe")?;
    let external_q = c.n("tank_wind_qe")?;
    let internal_q = c.n("tank_wind_qi")?;
    for (key, q) in [("tank_wind_qe", external_q), ("tank_wind_qi", internal_q)] {
        if !q.is_finite() || q < 0.0 {
            return Err(format!("input {key} must not be negative"));
        }
    }
    let external_wind = external_q * external;
    let internal_wind = internal_q * internal;
    let net_wind = external_wind - internal_wind;
    if !net_wind.is_finite() {
        return Err("w_net must be finite".into());
    }
    Ok(vec![
        Outcome::info("c_pi", "Internal wind pressure coefficient c_pi", "Innendruckbeiwert für Wind c_pi", QuantityKind::Dimensionless, internal),
        Outcome::info("external_wind", "External wind pressure w_e", "Äußerer Winddruck w_e", QuantityKind::Pressure, external_wind),
        Outcome::info("internal_wind", "Internal wind pressure w_i", "Innerer Winddruck w_i", QuantityKind::Pressure, internal_wind),
        Outcome::info("net_wind", "Net wind pressure w_e − w_i", "Netto-Winddruck w_e − w_i", QuantityKind::Pressure, net_wind)
            .steps("The external coefficient represents the selected surface and location. Use the applicable catch-basin adjustment when assessing a sheltered shell.")
            .steps_de("Der Außendruckbeiwert gilt für die betrachtete Fläche und Stelle. Für eine abgeschirmte Tankwand die Anpassung infolge des Auffangbeckens berücksichtigen."),
    ])
}

pub(super) fn specified_pressure(c: &Case) -> R {
    let (en, de) = match c.s("tank_pressure_situation")? {
        "operation" => ("Operating pressures, B.2.2(1).", "Betriebsdrücke, B.2.2(1)."),
        "test" => ("Test pressures, B.2.2(2).", "Prüfdrücke, B.2.2(2)."),
        situation => return Err(format!("unknown situation {situation}")),
    };
    let min = c.n("tank_pressure_min")?;
    let max = c.n("tank_pressure_max")?;
    let range = max - min;
    if !range.is_finite() {
        return Err("p_max − p_min must be finite".into());
    }
    if min > max {
        return Err("tank_pressure_min must not exceed tank_pressure_max".into());
    }
    Ok(vec![
        Outcome::info("min_pressure", "Minimum internal gauge pressure", "Minimaler innerer Relativdruck", QuantityKind::Pressure, min).steps(en).steps_de(de),
        Outcome::info("max_pressure", "Maximum internal gauge pressure", "Maximaler innerer Relativdruck", QuantityKind::Pressure, max).steps(en).steps_de(de),
        Outcome::info("pressure_range", "Span between the specified pressure limits", "Spanne zwischen den festgelegten Druckgrenzen", QuantityKind::Pressure, range),
    ])
}
