//! 🎞️ Combination track: scrubs the generated combination columns; every action glyph carries one factor field per column (γ·ψ of the action in that column, 0 outside it), so the scalar field of the viewport follows the selected combination.

use super::combinations::Column;
use super::kit::Out;
use super::scene::projection_kit::tracks;
use super::scene::*;

pub(crate) const TRACK_COMBINATION: &str = "combination";

/// 🔖️ The field id carrying the factors of one combination column.
pub(crate) fn field_id(column: &Column) -> String {
    format!("combination:{}", column.id)
}

fn caption(column: &Column) -> (String, String) {
    let (en, de) = column.family.title();
    let lead = column.leading();
    (format!("{en} · {} · lead {lead}", column.combination.equation.label()), format!("{de} · {} · führend {lead}", column.combination.equation.label()))
}

/// 🎞️ Adds the combination track and the factor fields of every action glyph; nothing is added below two columns, since there is nothing to scrub.
pub(crate) fn project(columns: &[Column], active: Option<&Column>, out: &mut Out) {
    let stops: Vec<_> = columns.iter().map(|c| { let (en, de) = caption(c); (en, de, Some(field_id(c)), None) }).collect();
    let Some(track) = tracks::indexed(TRACK_COMBINATION, TrackKind::Combination, ("Combination", "Kombination"), stops) else { return };
    out.global();
    let first = field_id(active.unwrap_or(&columns[0]));
    out.entities = std::mem::take(&mut out.entities).into_iter().map(|mut e| {
        if matches!(e.geometry, Geometry::Load(_)) {
            let action = e.entity.id.split(':').next().unwrap_or_default().to_string();
            let fields = columns.iter().map(|c| ScalarField { id: field_id(c), unit: QuantityKind::Dimensionless, samples: FieldSamples::Instance { values: vec![c.factor(&action).unwrap_or(0.0)] } }).collect();
            e.appearance.channel = ColourChannel::Fields { active: first.clone(), fields };
            e = e.sealed();
        }
        e
    }).collect();
    out.track(track);
}
