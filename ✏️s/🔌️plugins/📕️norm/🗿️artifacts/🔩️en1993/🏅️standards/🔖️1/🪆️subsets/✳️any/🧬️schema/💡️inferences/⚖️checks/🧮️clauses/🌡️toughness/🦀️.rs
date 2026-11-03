//! 🌡️ EN 1993-1-10 Table 2.1 maximum thicknesses for brittle fracture avoidance (mm, °C) by grade, sub-grade, stress level σ_Ed/f_y(t) and reference temperature T_Ed. See [EN 1993-1-10 Table 2.1](https://eurocodes.jrc.ec.europa.eu/).

use crate::misc::{permissible_thickness, reference_temperature, ToughnessRow};
use semio_s_plugin_norm_compliance::{ndp_number_opt, AnnexChoice};

pub const TEMPERATURES: [f64; 7] = [10.0, 0.0, -10.0, -20.0, -30.0, -40.0, -50.0];

/// 🌡️ Stress levels σ_Ed/f_y(t) of Table 2.1 with their registry key, ascending.
pub const LEVELS: [(f64, &str); 3] = [(0.25, "r25"), (0.5, "r50"), (0.75, "r75")];

/// 🌡️ Sub-grades listed per grade in Table 2.1 (the pairing is unverified, see the registry notes).
pub const GRADES: [(&str, &[&str]); 5] = [
    ("S235", &["JR", "J0", "J2"]),
    ("S275", &["JR", "J0", "J2", "N", "NL", "M", "ML"]),
    ("S355", &["JR", "J0", "J2", "K2", "N", "NL", "M", "ML"]),
    ("S420", &["N", "NL", "M", "ML"]),
    ("S460", &["N", "NL", "M", "ML", "Q", "QL", "QL1"]),
];

/// 🌡️ Why no thickness limit could be read from Table 2.1.
#[derive(Clone, Debug, PartialEq)]
pub enum Gap {
    UnknownColumn,
    StressAboveTable,
    TemperatureBelowTable,
    CellNotTranscribed(String),
}

/// 🌡️ A thickness limit with the stress columns it was read from and whether it is the conservative next-higher-stress column.
#[derive(Clone, Debug, PartialEq)]
pub struct Limit {
    pub t_max: f64,
    pub levels: Vec<f64>,
    pub conservative: bool,
}

/// 🌡️ Whether the grade and sub-grade are listed in Table 2.1.
pub fn is_encoded(grade: &str, sub_grade: &str) -> bool {
    GRADES.iter().any(|(g, subs)| *g == grade && subs.contains(&sub_grade))
}

/// 🌡️ Sub-grades Table 2.1 lists for the grade.
pub fn sub_grades(grade: &str) -> &'static [&'static str] {
    GRADES.iter().find(|(g, _)| *g == grade).map_or(&[], |(_, subs)| subs)
}

/// 🌡️ Registry id of one cell.
pub fn cell_id(grade: &str, sub_grade: &str, level_key: &str, temperature: f64) -> String {
    format!("ndp.en1993.1-10.t21.{}-{}.{level_key}.t{temperature}", grade.to_ascii_lowercase(), sub_grade.to_ascii_lowercase())
}

/// 🌡️ One registered cell when EN and NA-DE agree and it is transcribed.
pub fn cell(id: &str) -> Option<f64> {
    match (ndp_number_opt(id, AnnexChoice::En), ndp_number_opt(id, AnnexChoice::De)) {
        (Some(en), Some(de)) if en == de => Some(en),
        _ => None,
    }
}

/// 🌡️ One stress column at T_Ed by linear interpolation between the listed reference temperatures; the first untranscribed cell otherwise.
pub fn column_at(grade: &str, sub_grade: &str, level_key: &str, t_ed: f64) -> Result<f64, Gap> {
    let rows = TEMPERATURES
        .iter()
        .map(|&temperature| {
            let id = cell_id(grade, sub_grade, level_key, temperature);
            cell(&id).map(|t_max| ToughnessRow { temperature, t_max }).ok_or(Gap::CellNotTranscribed(id))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(permissible_thickness(&rows, t_ed))
}

/// 🌡️ Permissible thickness for the stress level σ_Ed/f_y(t) = `stress_ratio` at T_Ed: the bracketing stress columns are interpolated linearly in the ratio; a missing lower column or a ratio below 0.25 falls back on the next-higher-stress column (conservative), a ratio above 0.75 or T_Ed below −50 °C is outside the table.
pub fn limit(grade: &str, sub_grade: &str, stress_ratio: f64, t_ed: f64) -> Result<Limit, Gap> {
    if !is_encoded(grade, sub_grade) {
        return Err(Gap::UnknownColumn);
    }
    if t_ed < TEMPERATURES[6] {
        return Err(Gap::TemperatureBelowTable);
    }
    select(stress_ratio, &|level| column_at(grade, sub_grade, LEVELS[level].1, t_ed))
}

/// 🌡️ Stress-level selection and interpolation over the three columns of Table 2.1, given the value of each column (index into [`LEVELS`]) at T_Ed.
pub fn select(stress_ratio: f64, column: &dyn Fn(usize) -> Result<f64, Gap>) -> Result<Limit, Gap> {
    if stress_ratio > LEVELS[2].0 + 1e-9 {
        return Err(Gap::StressAboveTable);
    }
    let upper = LEVELS.iter().position(|(level, _)| *level + 1e-9 >= stress_ratio).unwrap_or(2);
    let Some((used, hi)) = (upper..LEVELS.len()).find_map(|k| column(k).ok().map(|v| (k, v))) else {
        return Err(column(upper).err().unwrap_or(Gap::UnknownColumn));
    };
    let exact = (LEVELS[upper].0 - stress_ratio).abs() < 1e-9;
    if used != upper || exact || upper == 0 {
        return Ok(Limit { t_max: hi, levels: vec![LEVELS[used].0], conservative: used != upper || !exact });
    }
    Ok(match column(upper - 1) {
        Ok(lo) => {
            let f = (stress_ratio - LEVELS[upper - 1].0) / (LEVELS[upper].0 - LEVELS[upper - 1].0);
            Limit { t_max: lo + f * (hi - lo), levels: vec![LEVELS[upper - 1].0, LEVELS[upper].0], conservative: false }
        }
        Err(_) => Limit { t_max: hi, levels: vec![LEVELS[upper].0], conservative: true },
    })
}

/// 🌡️ Permissible thickness at T_Ed for a stress level σ_Ed/f_y(t); `None` outside the table or when a needed cell is not transcribed.
pub fn max_thickness(grade: &str, sub_grade: &str, stress_ratio: f64, t_ed: f64) -> Option<f64> {
    limit(grade, sub_grade, stress_ratio, t_ed).ok().map(|l| l.t_max)
}

/// 🌡️ T_Ed = T_md + ΔT_r + ΔT_σ + ΔT_R + ΔT_ε̇ + ΔT_εcf and utilisation t/t_max of the actual thickness.
pub fn brittle_fracture_utilisation(grade: &str, sub_grade: &str, stress_ratio: f64, t: f64, t_md: f64, dt_r: f64, dt_sigma: f64, dt_margin: f64, dt_rate: f64, dt_cf: f64) -> Option<(f64, f64)> {
    let t_ed = reference_temperature(t_md, dt_r, dt_sigma, dt_margin, dt_rate, dt_cf);
    max_thickness(grade, sub_grade, stress_ratio, t_ed).map(|m| (t_ed, t / m))
}

/// 🌡️ Curve of the permissible thickness over T_Ed (ascending temperature) for the stress level of the member, for the toughness diagram; empty when the limit is unavailable.
pub fn curve(grade: &str, sub_grade: &str, stress_ratio: f64) -> Vec<(f64, f64)> {
    TEMPERATURES.iter().rev().filter_map(|&t_ed| max_thickness(grade, sub_grade, stress_ratio, t_ed).map(|m| (t_ed, m))).collect()
}
