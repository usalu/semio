//! 🏷️ Typed entities in the scene: the tapers, cutouts, fastener patterns, panels, floors, diaphragms, trusses and bracings of the document, each drawn as the case row it stands for by [`super::rows::draw_row`] and owned by the entity row, so the live scene re-projects exactly the entity an edit touches. Holes add a void cylinder, floors add the creep keyframes and a creep time diagram; grips and diagram markers are re-pointed from the case inputs to the entity fields. See [`super`].

use super::kit::*;
use super::model::CaseRow;
use super::norm::CheckReport;
use super::rows::{self, special};
use super::scene::diagram_kit::{curves, Xy};
use super::scene::projection_kit::diagram_parts::{column, stroke};
use super::scene::*;
use super::schema::checks::timber as t;

const HOLE_RGB: (u8, u8, u8) = (250, 250, 250);
const CREEP_AMPLITUDE: f64 = 0.04;
const CREEP_SAMPLES: usize = 60;
const YEAR_S: f64 = 365.25 * 86_400.0;

fn input(row: &CaseRow, key: &str) -> Option<f64> {
    row.numbers.iter().find(|n| n.id == key).map(|n| n.value)
}

/// 🎬️ Every typed entity of the document as scene entities: `origin_of` resolves where an entity stands and `tag_of` the ids of its grips whose tag is among the given ones.
pub fn project(input: &Input, origin_of: &dyn Fn(&str) -> Option<Vec3>, tag_of: &dyn Fn(&str, &[&str]) -> Vec<String>, out: &mut dyn Sink) {
    let d = input.document;
    for table in t::ENTITY_TABLES {
        for index in 0..t::entity_ids(d, table).len() {
            let Some((row, _)) = t::drawing_row(d, table, index) else { continue };
            if !out.wants(&row.id) {
                continue;
            }
            let Some(origin) = origin_of(&row.id) else { continue };
            let tag = |keys: &[&str]| tag_of(&row.id, keys);
            out.set_home(table);
            rows::draw_row(input, &row, &origin, &tag, out);
            match table {
                "cutouts" => hole(input, &row, &origin, &tag, out),
                "floors" => creep(input, &row, &origin, out),
                _ => {}
            }
            out.set_home("");
        }
    }
}

/// 🕳️ The void cylinder of a hole through the member web.
fn hole(input: &Input, row: &CaseRow, origin: &Vec3, tag: &dyn Fn(&[&str]) -> Vec<String>, out: &mut dyn Sink) {
    let metres = |key: &str| self::input(row, key).map(|v| v / 1e3);
    let (Some(diameter), Some(x), Some(z), Some(b)) = (metres("hole_d"), metres("hole_x"), metres("hole_z"), metres("b")) else { return };
    let at = |x: f64, y: f64, z: f64| v3(origin.x + x, origin.y + y, origin.z + z);
    let binding = binding_of(input.report, &row.id, "en1995.t-18");
    let (from, to) = (at(x, -0.01, z), at(x, b + 0.01, z));
    let text = format!("hole ⌀{:.0} mm at x = {:.0} mm, {:.0} mm above the bottom", diameter * 1e3, x * 1e3, z * 1e3);
    let text_de = format!("Loch ⌀{:.0} mm bei x = {:.0} mm, {:.0} mm über der Unterkante", diameter * 1e3, x * 1e3, z * 1e3);
    out.push(
        Spec {
            kind: EntityKind::Solid,
            id: format!("{}:hole", row.id),
            collection: "cutouts",
            geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: Frame { origin: from.clone(), x: Vec3::Z, y: Vec3::X, z: Vec3::Y }, radius: diameter / 2.0, height: b + 0.02 } }),
            appearance: Some(tinted(HOLE_RGB, 0.9, StyleState::Normal)),
            role: MaterialRole::Neutral,
            binding,
            name: loc(&format!("Hole of {}", row.label_en), &format!("Loch von {}", row.label_de)),
            detail: LocalizedCopy::new(text, text_de),
            layer: LAYER_MEMBERS,
            handles: tag(&["hole_d", "hole_x", "hole_z"]),
            deformation: None,
        },
        &[from, to],
    );
}

/// ⏳️ The deformed floor at the creep keyframes: a surface per age, coloured by the deflection, every one on its own layer behind the age track.
fn creep(input: &Input, row: &CaseRow, origin: &Vec3, out: &mut dyn Sink) {
    let d = input.document;
    let Some(floor) = d.floors.iter().find(|f| f.id == row.id) else { return };
    let (span, width) = (floor.span_m, floor.width_m);
    let w_fin = t::floor_creep_at(d, floor, 50.0);
    if span <= 0.0 || width <= 0.0 || w_fin <= 0.0 {
        return;
    }
    let scale = (CREEP_AMPLITUDE * span / w_fin).clamp(1.0, 400.0);
    let binding = binding_of(input.report, &floor.id, "en1995.7.2.floor-creep");
    let (columns, rows) = (special::FLOOR_COLUMNS, special::FLOOR_ROWS);
    let z0 = 0.206;
    for (years, layer) in CREEP_YEARS.iter().zip(CREEP_LAYERS) {
        let w = t::floor_creep_at(d, floor, f64::from(*years));
        let (mut points, mut samples, mut values) = (Vec::new(), Vec::new(), Vec::new());
        for j in 0..rows {
            for i in 0..columns {
                let (x, y) = (span * f64::from(i) / f64::from(columns - 1), width * f64::from(j) / f64::from(rows - 1));
                let shape = (std::f64::consts::PI * x / span).sin();
                points.push(v3(origin.x + x, origin.y + y, origin.z + z0));
                samples.push(SurfaceSample { at: Vec2::new(x, y), delta: Vec3::new(0.0, 0.0, -scale * w * shape) });
                values.push(w * shape * 1e3);
            }
        }
        let overlay = DeformationOverlay { kind: DeformationKind::Deformed, index: 0, frequency: None, participation: 1.0, scale: 1.0, samples: Vec::new(), surface: samples, motion: Motion::Static, load_factor: None };
        let field = ScalarField { id: "deflection".into(), unit: QuantityKind::Length, samples: FieldSamples::Vertex { values } };
        let appearance = Appearance { role: MaterialRole::Timber, channel: ColourChannel::Fields { active: "deflection".into(), fields: vec![field] }, style: StyleState::Normal, opacity: 0.95, hatch: None, deformation: Some(overlay) };
        let detail = LocalizedCopy::new(format!("w = {:.2} mm after {years} a (k_def = {:.2}), drawn ×{scale:.0}", w * 1e3, t::floor_k_def(d, floor)), format!("w = {:.2} mm nach {years} a (k_def = {:.2}), {scale:.0}-fach dargestellt", w * 1e3, t::floor_k_def(d, floor)));
        out.push(
            Spec {
                kind: EntityKind::Surface,
                id: format!("{}:creep@{years}", floor.id),
                collection: "floors",
                geometry: Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows, columns, points: points.clone() } } }),
                appearance: Some(appearance),
                role: MaterialRole::Timber,
                binding: binding.clone(),
                name: loc(&format!("Floor deflection of {} after {years} a", row.label_en), &format!("Deckendurchbiegung von {} nach {years} a", row.label_de)),
                detail,
                layer,
                handles: Vec::new(),
                deformation: None,
            },
            &points,
        );
    }
    out.legend(Legend { id: format!("deflection:{}", floor.id), title: loc("Floor deflection (mm)", "Deckendurchbiegung (mm)"), scale: LegendScale::Scalar { field_id: "deflection".into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: w_fin * 1e3 }, unit: QuantityKind::Length } });
}

/// 🎞️ The creep age track: scrubbing it reveals the deformed floor of every age in turn.
pub fn finish(scene: &mut StructuralScene) {
    if !scene.entities.iter().any(|e| e.layer == CREEP_LAYERS[0]) {
        return;
    }
    let keyframes = CREEP_YEARS.iter().zip(CREEP_LAYERS).map(|(years, layer)| super::scene::projection_kit::tracks::key(f64::from(*years) * YEAR_S, (&format!("{years} a"), &format!("{years} a")), None, Some(layer))).collect();
    scene.tracks.push(super::scene::projection_kit::tracks::track("creep", TrackKind::Age, ("Age under load", "Alter unter Last"), QuantityKind::Time, (0.0, 50.0 * YEAR_S, YEAR_S), keyframes));
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str) -> Axis {
    Axis { label: loc(en, de), quantity, unit: unit.to_string(), scale: AxisScale::Linear, domain: None, ticks: TickRule::Auto }
}

fn tone_of_ratio(ratio: f64) -> Tone {
    super::scene::utilisation_tone(ratio)
}

/// 📈️ Deflection over the age of a floor: the creep curve of 7.2 with the limit line and the final value as a marker bound to the creep check.
pub fn creep_diagram(document: &super::model::En1995Snapshot, report: &CheckReport, floor: &super::model::Floor) -> Option<Diagram2d> {
    let w_fin = t::floor_creep_at(document, floor, 50.0);
    if floor.span_m <= 0.0 || w_fin <= 0.0 {
        return None;
    }
    let check = worst(report, &floor.id, "en1995.7.2.floor-creep");
    let limit = floor.span_m / if document.annex == super::norm::AnnexChoice::De { 200.0 } else { 250.0 } * 1e3;
    let curve: Vec<Xy> = curves::sample(|years| t::floor_creep_at(document, floor, years) * 1e3, 0.0, 50.0, CREEP_SAMPLES);
    let points = |ys: &[Xy]| ys.iter().map(|p| Vec2::new(p[0], p[1])).collect::<Vec<_>>();
    let line = |id: &str, en: &str, de: &str, ys: Vec<Xy>, tone: Tone, dash: Dash| Series::Polyline { id: id.into(), label: loc(en, de), points: points(&ys), style: stroke(tone, 2.0, dash) };
    Some(Diagram2d {
        id: format!("creep-time:{}", floor.id),
        title: loc(&format!("Creep deflection over time – {}", floor.id), &format!("Kriechverformung über die Zeit – {}", floor.id)),
        description: loc(
            "w(t) = w_G (1 + k_def φ(t)) + w_Q (1 + ψ₂ k_def φ(t)) of the floor strip with the end value of EN 1995-1-1 7.2 and the limit L/250 (EN) or L/200 (NA-DE); the shape of φ(t) uses an unverified time constant.",
            "w(t) = w_G (1 + k_def φ(t)) + w_Q (1 + ψ₂ k_def φ(t)) des Deckenstreifens mit dem Endwert nach EN 1995-1-1 7.2 und der Grenze L/250 (EN) bzw. L/200 (NA-DE); der Verlauf φ(t) nutzt eine ungeprüfte Zeitkonstante.",
        ),
        axes: vec![axis("Age under load", "Alter unter Last", QuantityKind::Time, "a"), axis("Deflection", "Durchbiegung", QuantityKind::Length, "mm")],
        series: vec![line("creep", "Deflection w(t)", "Durchbiegung w(t)", curve.clone(), Tone::Info, Dash::Solid), line("limit", "Limit", "Grenzwert", vec![[0.0, limit], [50.0, limit]], Tone::Error, Dash::Dotted)],
        markers: vec![Marker {
            id: "marker-w-fin".into(),
            at: Vec2::new(50.0, w_fin * 1e3),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |c| tone_of_ratio(c.utilization)),
            label: loc(&format!("w_fin = {:.2} mm", w_fin * 1e3), &format!("w_fin = {:.2} mm", w_fin * 1e3)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &floor.id, format!("floors[id={}]", floor.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Age under load", "Alter unter Last", "a", curve.iter().map(|p| p[0]).collect()), column("Deflection", "Durchbiegung", "mm", curve.iter().map(|p| p[1]).collect())] },
    })
}

/// 🔁️ Re-points the focus and the draggable markers of a diagram drawn for a case row to the entity fields the inputs stand for; markers on inputs without a field of their own keep their value but lose the grip.
pub fn rehome(mut diagram: Diagram2d, table: &str, row: &CaseRow, homes: &[t::Home]) -> Diagram2d {
    let prefix = format!("cases[id={}].numbers[id=", row.id);
    diagram.focus = Some(EntityRef::new(EntityKind::Group, &row.id, format!("{table}[id={}]", row.id)));
    for marker in &mut diagram.markers {
        marker.binding = marker.binding.take().and_then(|mut binding| {
            let key = binding.path.strip_prefix(&prefix)?.split(']').next()?.to_string();
            let home = homes.iter().find(|home| home.key == key && home.scale != 0.0)?;
            let (base, k) = home.stored(binding.map.base, binding.map.k);
            binding.path = home.path.clone();
            binding.map = Affine { base, k };
            if let Some(unit) = home.unit() {
                binding.unit = unit.into();
            }
            Some(binding)
        });
    }
    diagram
}
