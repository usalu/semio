//! 💨️ EN 1991-2:2003 rail aerodynamic load factors using the pressure values from Figures 6.22–6.24.

use super::*;

fn non_negative(c: &Case, key: &str) -> Result<f64, String> {
    let number = c.n(key)?;
    if !number.is_finite() || number < 0.0 {
        return Err(format!("input {key} must not be negative"));
    }
    Ok(number)
}

fn end_factor(c: &Case) -> Result<f64, String> {
    Ok(if non_negative(c, "rail_aero_end")? <= 5.0 { 2.0 } else { 1.0 })
}

fn train_factor(c: &Case, key: &str) -> Result<f64, String> {
    match c.s(key)? {
        "unfavourable" => Ok(1.0),
        "smooth" => Ok(0.85),
        "streamlined" => Ok(0.6),
        train => Err(format!("unknown model {train}")),
    }
}

fn vertical_magnitude(c: &Case) -> Result<f64, String> {
    let panel = match c.s("rail_aero_small_panel")? {
        "full" => 1.0,
        "small" => 1.3,
        panel => return Err(format!("unknown model {panel}")),
    };
    Ok(non_negative(c, "rail_aero_q")? * train_factor(c, "rail_aero_train")? * panel)
}

fn pressures(pressure: f64) -> R {
    if !pressure.is_finite() {
        return Err("q_k must be finite".into());
    }
    Ok(vec![
        Outcome::info("pressure", "Characteristic pressure", "Charakteristischer Druck", QuantityKind::Pressure, pressure)
            .steps("Section 6.6.1(5) doubles the action within 5 m of either structural end. A dynamically sensitive structure may require a dynamic study.")
            .steps_de("Abschnitt 6.6.1(5) verdoppelt die Einwirkung bis 5 m von jedem Bauwerksende. Bei schwingungsanfälligen Tragwerken kann eine dynamische Untersuchung erforderlich sein."),
        Outcome::info("suction", "Characteristic suction", "Charakteristischer Sog", QuantityKind::Pressure, -pressure),
    ])
}

pub(super) fn vertical(c: &Case) -> R {
    pressures(end_factor(c)? * vertical_magnitude(c)?)
}

pub(super) fn crossing_surface(c: &Case) -> R {
    let first = non_negative(c, "rail_aero_q")? * train_factor(c, "rail_aero_train")?;
    let second = if c.opt("rail_aero_q_other").is_some() {
        non_negative(c, "rail_aero_q_other")? * train_factor(c, "rail_aero_other_train")?
    } else {
        0.0
    };
    let edge_factor = match c.opt("rail_aero_edge") {
        Some(_) if non_negative(c, "rail_aero_edge")? <= 1.5 => 0.75,
        _ => 1.0,
    };
    let mut outcomes = pressures(end_factor(c)? * edge_factor * (first + second))?;
    outcomes.push(Outcome::info("loaded_half_width", "Loaded width on each side of the track", "Belastete Breite je Seite der Gleisachse", QuantityKind::Length, 10.0)
        .steps("Section 6.6.3 adds the actions of up to two opposing trains.")
        .steps_de("Abschnitt 6.6.3 addiert die Einwirkungen von bis zu zwei entgegenkommenden Zügen."));
    Ok(outcomes)
}

pub(super) fn adjacent_surface(c: &Case) -> R {
    let height_factor = |height: f64| ((7.5 - height) / 3.7).clamp(0.0, 1.0);
    let first = non_negative(c, "rail_aero_q")? * height_factor(c.pos("rail_aero_height")?);
    let second = if c.opt("rail_aero_q_other").is_some() {
        non_negative(c, "rail_aero_q_other")? * height_factor(c.pos("rail_aero_other_height")?)
    } else {
        0.0
    };
    pressures(end_factor(c)? * (first + second))
}

pub(super) fn multiple_surfaces(c: &Case) -> R {
    let min = c.pos("rail_aero_clearance_min")?;
    let max = c.pos("rail_aero_clearance_max")?;
    if min > max {
        return Err("rail_aero_clearance_min must not exceed rail_aero_clearance_max".into());
    }
    let effective_distance = (0.6 * min + 0.4 * max.min(6.0)).min(6.0);
    let mut outcomes = vertical(c)?;
    outcomes.push(Outcome::info("effective_distance", "Effective distance a′ for Figure 6.22", "Maßgebender Abstand a′ für Bild 6.22", QuantityKind::Length, effective_distance));
    Ok(outcomes)
}

pub(super) fn enclosure(c: &Case) -> R {
    let length = c.pos("rail_aero_enclosed_length")?;
    if length > 20.0 {
        return Err("input rail_aero_enclosed_length must lie within 0 to 20".into());
    }
    let overhead_factor = match c.s("rail_aero_enclosed_tracks")? {
        "one" => 2.5,
        "two" => 3.5,
        count => return Err(format!("unknown model {count}")),
    };
    let end = end_factor(c)?;
    let wall_pressure = 2.0 * end * non_negative(c, "rail_aero_q")?;
    let overhead_pressure = overhead_factor * end * non_negative(c, "rail_aero_enclosed_overhead")?;
    if !overhead_pressure.is_finite() {
        return Err("q_2k must be finite".into());
    }
    let mut outcomes = pressures(wall_pressure)?;
    outcomes.extend([
        Outcome::info("overhead_pressure", "Characteristic overhead pressure", "Charakteristischer Druck oben", QuantityKind::Pressure, overhead_pressure),
        Outcome::info("overhead_suction", "Characteristic overhead suction", "Charakteristischer Sog oben", QuantityKind::Pressure, -overhead_pressure),
    ]);
    Ok(outcomes)
}
