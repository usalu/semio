//! 🌈️ Scalar fields of `ColourChannel::Fields`: active-field selection (toolbar choice or track keyframes with blending), per-vertex sampling for baked ramp colours, a stable value domain for the legend and field titles.

use super::config::LengthUnit;
use super::overlays::presentation;
use super::playback::TrackSample;
use super::scene::{ColourChannel, FieldSamples, LocalizedCopy, ParameterTrack, QuantityKind, SceneEntity, StationValue, StructuralScene};

/// 🎯️ Which fields are shown: `primary` (else each entity's own active field), optionally blended towards `secondary` by `weight`; `domain_ids` fixes the legend range across every field a track can show.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FieldSelection {
    pub primary: Option<String>,
    pub secondary: Option<String>,
    pub weight: f64,
    pub domain_ids: Vec<String>,
}

/// 🧷️ Where a field is sampled: mesh vertex `vertex` of `vertices` (node indices for grids), an optional member station and the pattern instance index.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Site {
    pub vertex: usize,
    pub vertices: usize,
    pub station: Option<f64>,
    pub instance: usize,
}

/// 🔎️ The field of one entity under a selection, with an optional blend partner.
#[derive(Clone, Copy, Debug)]
pub struct ResolvedField<'a> {
    pub id: &'a str,
    pub unit: QuantityKind,
    pub from: &'a FieldSamples,
    pub to: Option<&'a FieldSamples>,
    pub weight: f64,
}

/// 📋️ One selectable field of a scene for the toolbar.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldChoice {
    pub id: String,
    pub unit: QuantityKind,
    pub title: LocalizedCopy,
}

/// 🧭️ Selection from the toolbar choice and the current track frame; with a track all its keyframe fields share one legend domain.
pub fn selection(active: Option<&str>, track: Option<(&ParameterTrack, &TrackSample)>) -> FieldSelection {
    match track.and_then(|(t, s)| s.fields.as_ref().map(|f| (t, f))) {
        Some((t, (a, b, w))) => FieldSelection { primary: Some(a.clone()), secondary: b.clone(), weight: *w, domain_ids: t.keyframes.iter().filter_map(|k| k.field.clone()).collect() },
        None => FieldSelection { primary: active.map(str::to_string), secondary: None, weight: 0.0, domain_ids: active.map(str::to_string).into_iter().collect() },
    }
}

fn fields_of(entity: &SceneEntity) -> Option<(&str, &[super::scene::ScalarField])> {
    match &entity.appearance.channel {
        ColourChannel::Fields { active, fields } => Some((active.as_str(), fields.as_slice())),
        _ => None,
    }
}

/// 🔎️ The field an entity shows under a selection; entities lacking the selected field keep their own active one.
pub fn resolve<'a>(entity: &'a SceneEntity, selection: &FieldSelection) -> Option<ResolvedField<'a>> {
    let (active, fields) = fields_of(entity)?;
    let by = |id: &str| fields.iter().find(|f| f.id == id);
    let first = selection.primary.as_deref().and_then(by).or_else(|| by(active))?;
    let second = selection.secondary.as_deref().and_then(by).filter(|_| selection.primary.as_deref() == Some(first.id.as_str()));
    Some(ResolvedField { id: &first.id, unit: first.unit, from: &first.samples, to: second.map(|f| &f.samples), weight: if second.is_some() { selection.weight } else { 0.0 } })
}

fn station_value(stations: &[StationValue], t: f64) -> f64 {
    let Some(first) = stations.first() else { return 0.0 };
    if t <= first.at { return first.value; }
    stations.windows(2).find(|w| t <= w[1].at).map_or(stations[stations.len() - 1].value, |w| w[0].value + (w[1].value - w[0].value) * (t - w[0].at) / (w[1].at - w[0].at).max(1e-12))
}

fn resampled(values: &[f64], fraction: f64) -> f64 {
    let x = fraction.clamp(0.0, 1.0) * (values.len().max(1) - 1) as f64;
    let i = (x.floor() as usize).min(values.len().saturating_sub(1));
    let j = (i + 1).min(values.len().saturating_sub(1));
    values.get(i).copied().unwrap_or(0.0) * (1.0 - (x - i as f64)) + values.get(j).copied().unwrap_or(0.0) * (x - i as f64)
}

fn mean(samples: &FieldSamples) -> f64 {
    let values = raw(samples);
    values.iter().sum::<f64>() / values.len().max(1) as f64
}

fn raw(samples: &FieldSamples) -> Vec<f64> {
    match samples {
        FieldSamples::Vertex { values } | FieldSamples::Instance { values } => values.clone(),
        FieldSamples::Station { values } => values.iter().map(|v| v.value).collect(),
    }
}

/// 📍️ Value of one set of samples at a site: vertex values are taken per node when the counts match, else resampled along the station or the vertex index; station fields need a station, else their mean applies.
pub fn value_at(samples: &FieldSamples, site: Site) -> f64 {
    match samples {
        FieldSamples::Vertex { values } if values.len() == site.vertices && site.vertices > 0 => values[site.vertex.min(site.vertices - 1)],
        FieldSamples::Vertex { values } => resampled(values, site.station.unwrap_or(site.vertex as f64 / (site.vertices.max(2) - 1) as f64)),
        FieldSamples::Station { values } => site.station.map_or_else(|| mean(samples), |t| station_value(values, t)),
        FieldSamples::Instance { values } => values.get(site.instance).or(values.last()).copied().unwrap_or(0.0),
    }
}

impl ResolvedField<'_> {
    pub fn value(&self, site: Site) -> f64 {
        let a = value_at(self.from, site);
        self.to.map_or(a, |to| a + (value_at(to, site) - a) * self.weight)
    }
}

/// 🌡️ Value range of the shown fields over every entity, stable across a track's keyframes; a flat range is padded so the ramp stays defined.
pub fn domain(scene: &StructuralScene, selection: &FieldSelection) -> Option<(f64, f64)> {
    let mut range: Option<(f64, f64)> = None;
    for entity in &scene.entities {
        let Some((active, fields)) = fields_of(entity) else { continue };
        let wanted: Vec<&super::scene::ScalarField> = fields.iter().filter(|f| selection.domain_ids.contains(&f.id)).collect();
        let shown = if wanted.is_empty() { fields.iter().filter(|f| f.id == active).collect() } else { wanted };
        for value in shown.iter().flat_map(|f| raw(&f.samples)) {
            range = Some(range.map_or((value, value), |(lo, hi)| (lo.min(value), hi.max(value))));
        }
    }
    range.map(|(lo, hi)| if hi - lo > 1e-12 { (lo, hi) } else { let pad = 0.5 * lo.abs().max(1.0); (lo - pad, hi + pad) })
}

/// 🔤️ Display title of a field id in English and German; unknown ids are humanised.
pub fn title(id: &str) -> LocalizedCopy {
    let key = id.split('@').next().unwrap_or(id).to_lowercase();
    let known = match key.as_str() {
        "utilisation" | "utilization" => Some(("Utilisation", "Ausnutzung")),
        "settlement" => Some(("Settlement", "Setzung")),
        "temperature" | "section-temperature" => Some(("Temperature", "Temperatur")),
        "stress" | "sigma" => Some(("Stress", "Spannung")),
        "strain" => Some(("Strain", "Dehnung")),
        "force" | "tendon-force" => Some(("Force", "Kraft")),
        "moment" => Some(("Moment", "Moment")),
        "displacement" => Some(("Displacement", "Verschiebung")),
        "soil-pressure" => Some(("Soil pressure", "Sohldruck")),
        "drift" | "storey-drift" => Some(("Storey drift", "Stockwerksverschiebung")),
        "char-depth" | "charring" => Some(("Char depth", "Abbrandtiefe")),
        "creep" => Some(("Creep", "Kriechen")),
        "shrinkage" => Some(("Shrinkage", "Schwinden")),
        "moisture" => Some(("Moisture", "Feuchte")),
        _ => None,
    };
    known.map_or_else(|| {
        let spaced = id.replace(['-', '_', '@'], " ");
        let mut chars = spaced.chars();
        let text = chars.next().map(|c| c.to_uppercase().collect::<String>() + chars.as_str()).unwrap_or_default();
        LocalizedCopy::new(text.clone(), text)
    }, |(en, de)| LocalizedCopy::new(en, de))
}

/// 📋️ Distinct fields of a scene in first-seen order, the toolbar's field menu.
pub fn available(scene: &StructuralScene) -> Vec<FieldChoice> {
    let mut out: Vec<FieldChoice> = Vec::new();
    for field in scene.entities.iter().filter_map(fields_of).flat_map(|(_, f)| f.iter()) {
        if !out.iter().any(|c| c.id == field.id) {
            out.push(FieldChoice { id: field.id.clone(), unit: field.unit, title: title(&field.id) });
        }
    }
    out
}

/// 📏️ Presentation scale and unit symbol of a field kind.
pub fn scale_of(unit: QuantityKind, length: LengthUnit) -> (f64, &'static str) {
    let (scale, symbol, _) = presentation(unit, length);
    (scale, symbol)
}

/// 🕸️ Source node of every vertex of a flat-shaded row-major grid mesh (four triangles per cell, as `mesh::grid` emits).
pub fn grid_node(vertex: usize, rows: usize, columns: usize) -> usize {
    const CORNERS: [usize; 12] = [0, 1, 3, 0, 3, 2, 0, 3, 1, 0, 2, 3];
    let cells = columns.saturating_sub(1).max(1);
    let (cell, corner) = (vertex / 12, CORNERS[vertex % 12]);
    let (r, c) = ((cell / cells).min(rows.saturating_sub(2)), cell % cells);
    (r + corner / 2) * columns + c + corner % 2
}
