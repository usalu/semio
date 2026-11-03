//! 🌈️ Field overlays and bridge girder details of the EN 1994 scene (the end plate of the girder end): the ISO 834 temperature plate on the section of every encased column (one scalar field per fire minute, scrubbed by the `fire` track) and the crack-width band on the slab over the hogging zones of continuous girders (the crack width w_k/w_lim of EN 1994-1-1 7.4.3 along the zone, from the cracked-section bar stress scaled by the support-moment line of the elastic analysis).

use super::beams::local_frame;
use super::checks::catalogue::{self, column_fire, continuous, crack_width};
use super::checks::{part_1_1, part_1_2, part_column, part_en1990};
use super::facts::BeamFacts;
use super::kit::*;
use super::layout::Layout;
use super::scene::colour_maps::RampId;
use super::scene::*;
use super::SceneInput;
use std::collections::BTreeSet;

pub(crate) const FIRE_FIELD_PREFIX: &str = "fire-temp@";
pub(crate) const CRACK_FIELD: &str = "crack-width";
pub(crate) const TENSION_FIELD: &str = "rebar-stress";
pub(crate) const FATIGUE_FIELD: &str = "rebar-fatigue";
pub(crate) const FIRE_LEGEND: &str = "fire-temperature";
const FIRE_MINUTES: [f64; 4] = [30.0, 60.0, 90.0, 120.0];
pub(crate) const FIRE_MAX_C: f64 = 1100.0;
const CRACK_MAX: f64 = 1.2;
pub(crate) const PLATE_POINTS: usize = 25;
const BAND_POINTS: usize = 25;
const LIFT: f64 = 0.004;
const CRACK_PREFIXES: [&str; 1] = ["en1994.7.4.crack."];
const TENSION_PREFIXES: [&str; 1] = ["en1994.7.4.3.tension-stiffening."];
const TENSION_MAX: f64 = 1.2;
const FATIGUE_PREFIXES: [&str; 1] = ["en1994.2.6.8.2.rebar."];
const FATIGUE_MAX: f64 = 1.2;
const END_PLATE_PREFIXES: [&str; 1] = ["en1994.2.6.6.5.end-plate."];
const END_PLATE_THICKNESS: f64 = 0.02;

/// 🔥️ The field id of the temperature after `minutes` of the standard fire.
pub(crate) fn fire_field_id(minutes: f64) -> String {
    format!("{FIRE_FIELD_PREFIX}{minutes:.0}")
}

/// 🔥️ The fire minutes of the temperature plate: the rating's own duration and the earlier stops of 30, 60, 90 and 120 min.
pub(crate) fn fire_times(fire_rating: semio_s_artifact_norm_en1994_model::FireRating) -> Vec<f64> {
    fire_rating.rated().map(part_1_2::rating_minutes).map(|end| FIRE_MINUTES.into_iter().filter(|t| *t <= end + 1e-9).collect()).unwrap_or_default()
}

pub(crate) fn grid_values(rows: usize, columns: usize, value: impl Fn(usize, usize) -> f64) -> Vec<f64> {
    (0..rows).flat_map(|r| (0..columns).map(move |c| (r, c))).map(|(r, c)| value(r, c)).collect()
}

pub(crate) fn surface(id: String, collection: &'static str, points: Vec<Vec3>, rows: usize, columns: usize, role: MaterialRole, name: LocalizedCopy) -> Spec {
    let grid = PointGrid { rows: rows as u32, columns: columns as u32, points };
    Spec::new(EntityKind::Surface, id, collection, Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid } }), role, name, LAYER_FIELDS)
}

fn fire_plate(input: &SceneInput, layout: &Layout, column: &super::model::CompositeColumn, out: &mut Out) {
    let (Some(pose), Some(g)) = (layout.pose(&column.id), part_column::true_geometry(column)) else { return };
    let times = fire_times(input.document.fire_rating);
    if times.is_empty() || column.length_m <= 1e-6 {
        return;
    }
    let fields = column_fire::temperature_fields(&g, &times.iter().map(|t| t * 60.0).collect::<Vec<_>>());
    let Some(first) = fields.first() else { return };
    let (wx, wy) = (first.nx as f64 * first.dx, first.ny as f64 * first.dy);
    let (px, py) = (PLATE_POINTS, PLATE_POINTS);
    let at = |i: usize, n: usize, w: f64| -w + 2.0 * w * i as f64 / (n - 1) as f64;
    let z = column.length_m + LIFT;
    let points: Vec<Vec3> = (0..py).flat_map(|j| (0..px).map(move |i| (i, j))).map(|(i, j)| pose.point(at(i, px, wx), at(j, py, wy), z)).collect();
    let scalars: Vec<ScalarField> = fields.iter().zip(&times).map(|(field, t)| ScalarField { id: fire_field_id(*t), unit: QuantityKind::Temperature, samples: FieldSamples::Vertex { values: grid_values(py, px, |j, i| field.sample(at(i, px, wx), at(j, py, wy))) } }).collect();
    let last = fields.last().map_or(0.0, column_fire::Field::max);
    let name = loc(&format!("Fire temperature plate of {}", column.id), &format!("Brandtemperaturplatte von {}", column.id));
    let detail = loc(
        &format!("ISO 834 temperatures θ of the section at the column head after {} min, two-dimensional heat conduction; hottest point {last:.0} °C (values listed as unverified in the NDP registry).", times.iter().map(|t| format!("{t:.0}")).collect::<Vec<_>>().join(", ")),
        &format!("ISO-834-Temperaturen θ des Querschnitts am Stützenkopf nach {} min, zweidimensionale Wärmeleitung; heißester Punkt {last:.0} °C (im NDP-Register als ungeprüft geführt).", times.iter().map(|t| format!("{t:.0}")).collect::<Vec<_>>().join(", ")),
    );
    let spec = surface(format!("{}:fire-plate", column.id), "columns", points.clone(), py, px, MaterialRole::Concrete, name)
        .bound(binding(input.report, &column.id, &["en1994.1-2.4.3.5.column."]))
        .detailed(detail)
        .channel(ColourChannel::Fields { active: scalars[scalars.len() - 1].id.clone(), fields: scalars })
        .translucent(0.85);
    if !out.legends.iter().any(|l| l.id == FIRE_LEGEND) {
        out.legends.push(Legend { id: FIRE_LEGEND.into(), title: loc("Section temperature θ in the standard fire (°C)", "Querschnittstemperatur θ im Normbrand (°C)"), scale: LegendScale::Scalar { field_id: fire_field_id(times[times.len() - 1]), map: ColourMapRef { id: RampId::Warm.wire().into(), min: 20.0, max: FIRE_MAX_C }, unit: QuantityKind::Temperature } });
    }
    out.push(spec, &points);
}

/// 📏️ The crack width w_k [m] at a relative position of the hogging zone (1 = the support, the quasi-permanent support moment): the cracked-section bar stress scaled by the moment line, put through EN 1992-1-1, 7.3.4.
fn support_crack_width(f: &BeamFacts) -> impl Fn(f64) -> f64 + '_ {
    let beam = &f.beam;
    let sec = part_1_1::isection(&f.steel);
    let half = beam.spacing_m / 2.0;
    let beff = catalogue::effective_width_m(catalogue::le_hogging_m(beam.span_m, beam.neighbour_span_m()), 0.0, half, half, false);
    let m_hog = part_en1990::sls_quasi_permanent(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, f.annex, f.reliability_class).m_hog_nm.abs();
    let state = crack_width::hogging(&sec, beam.slab_thickness_m, f.hc, beff, beam.as_hogging_m2_per_m, beam.concrete_f_ck_pa);
    move |ratio: f64| crack_width::crack_width_m(state.sigma_per_nm * m_hog * ratio, beam.as_hogging_m2_per_m, beam.bar_spacing_m, f.hc, beam.concrete_f_ck_pa, beam.concrete_e_cm_pa)
}

fn crack_bands(input: &SceneInput, layout: &Layout, f: &BeamFacts, out: &mut Out) {
    let (Some(pose), true) = (layout.pose(&f.beam.id), f.hogging) else { return };
    let Some(gov) = binding(input.report, &f.beam.id, &CRACK_PREFIXES) else { return };
    let half = f.slab_width() / 2.0;
    let w_k = support_crack_width(f);
    let w_lim = f.beam.wk_limit_m.max(0.0002);
    for (k, zone) in continuous::hogging_band_profile(&f.layout, f.stiffness, BAND_POINTS).into_iter().enumerate() {
        let (x_a, x_b) = (zone[0].0, zone[zone.len() - 1].0);
        let points: Vec<Vec3> = [-half, half].iter().flat_map(|y| zone.iter().map(move |p| (p.0, *y))).map(|(x, y)| pose.point(x, y, f.depth + LIFT + LIFT)).collect();
        let values = grid_values(2, BAND_POINTS, |_, i| w_k(zone[i].1) / w_lim);
        let tag = if k == 0 { "crack-band".to_string() } else { format!("crack-band-{}", k + 1) };
        let name = loc(&format!("Crack-width band of {}", f.beam.id), &format!("Rissbreitenband von {}", f.beam.id));
        let detail = loc(
            &format!("Crack width w_k = {:.3} mm of {:.3} mm at the support (w_k/w_lim = {:.2}), falling with the cracked-section bar stress along the support moment line over {x_a:.2} to {x_b:.2} m (EN 1994-1-1, 7.4.3; EN 1992-1-1, 7.3.4).", w_k(1.0) * 1e3, w_lim * 1e3, w_k(1.0) / w_lim),
            &format!("Rissbreite w_k = {:.3} mm von {:.3} mm an der Stütze (w_k/w_lim = {:.2}), abfallend mit der Stahlspannung im gerissenen Querschnitt entlang der Stützmomentenlinie über {x_a:.2} bis {x_b:.2} m (EN 1994-1-1, 7.4.3; EN 1992-1-1, 7.3.4).", w_k(1.0) * 1e3, w_lim * 1e3, w_k(1.0) / w_lim),
        );
        let spec = surface(format!("{}:{tag}", f.beam.id), "beams", points.clone(), 2, BAND_POINTS, MaterialRole::Concrete, name)
            .bound(Some(gov.clone()))
            .detailed(detail)
            .channel(ColourChannel::Fields { active: CRACK_FIELD.into(), fields: vec![ScalarField { id: CRACK_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } }] })
            .translucent(0.9);
        if !out.legends.iter().any(|l| l.id == CRACK_FIELD) {
            out.legends.push(Legend { id: CRACK_FIELD.into(), title: loc("Crack-width utilisation w_k/w_lim", "Rissbreitenausnutzung w_k/w_lim"), scale: LegendScale::Scalar { field_id: CRACK_FIELD.into(), map: ColourMapRef { id: RampId::Failure.wire().into(), min: 0.0, max: CRACK_MAX }, unit: QuantityKind::Dimensionless } });
        }
        out.push(spec, &points);
    }
}

/// 🧵 The bar stress utilisation σ_s/(0.8·f_sk) [-] at a relative position of the hogging zone (1 = the support, the characteristic hogging moment of the span envelope): σ_s = σ_s,0·ratio + Δσ_s of the cracked section with tension stiffening (EN 1994-1-1, 7.4.3).
fn support_bar_stress(f: &BeamFacts) -> impl Fn(f64) -> f64 + '_ {
    let beam = &f.beam;
    let sec = part_1_1::isection(&f.steel);
    let half = beam.spacing_m / 2.0;
    let beff = catalogue::effective_width_m(catalogue::le_hogging_m(beam.span_m, beam.neighbour_span_m()), 0.0, half, half, false);
    let m_hog = part_en1990::sls_characteristic(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, f.annex, f.reliability_class).m_hog_nm.abs();
    let state = crack_width::hogging(&sec, beam.slab_thickness_m, f.hc, beff, beam.as_hogging_m2_per_m, beam.concrete_f_ck_pa);
    move |ratio: f64| (state.sigma_per_nm * m_hog * ratio + state.delta_sigma_pa) / (0.8 * catalogue::constants::REBAR_FSK)
}

/// 🧵 The bar-stress bands over the hogging zones: σ_s/(0.8·f_sk) with the tension-stiffening increment Δσ_s, bound to the tension-stiffening check.
fn tension_bands(input: &SceneInput, layout: &Layout, f: &BeamFacts, out: &mut Out) {
    let (Some(pose), true) = (layout.pose(&f.beam.id), f.hogging) else { return };
    let Some(gov) = binding(input.report, &f.beam.id, &TENSION_PREFIXES) else { return };
    let half = f.slab_width() / 2.0;
    let u = support_bar_stress(f);
    for (k, zone) in continuous::hogging_band_profile(&f.layout, f.stiffness, BAND_POINTS).into_iter().enumerate() {
        let (x_a, x_b) = (zone[0].0, zone[zone.len() - 1].0);
        let points: Vec<Vec3> = [-half, half].iter().flat_map(|y| zone.iter().map(move |p| (p.0, *y))).map(|(x, y)| pose.point(x, y, f.depth + 3.0 * LIFT)).collect();
        let values = grid_values(2, BAND_POINTS, |_, i| u(zone[i].1));
        let tag = if k == 0 { "tension-band".to_string() } else { format!("tension-band-{}", k + 1) };
        let name = loc(&format!("Bar-stress band of {}", f.beam.id), &format!("Bewehrungsspannungsband von {}", f.beam.id));
        let detail = loc(
            &format!("Bar stress σ_s/(0.8·f_sk) = {:.2} at the support incl. the tension-stiffening increment Δσ_s, over {x_a:.2} to {x_b:.2} m (EN 1994-1-1, 7.4.3; EN 1994-2, 6.8.5.4).", u(1.0)),
            &format!("Bewehrungsspannung σ_s/(0,8·f_sk) = {:.2} an der Stütze inkl. Zugversteifung Δσ_s, über {x_a:.2} bis {x_b:.2} m (EN 1994-1-1, 7.4.3; EN 1994-2, 6.8.5.4).", u(1.0)),
        );
        let spec = surface(format!("{}:{tag}", f.beam.id), "beams", points.clone(), 2, BAND_POINTS, MaterialRole::Concrete, name)
            .bound(Some(gov.clone()))
            .detailed(detail)
            .channel(ColourChannel::Fields { active: TENSION_FIELD.into(), fields: vec![ScalarField { id: TENSION_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } }] })
            .translucent(0.9);
        if !out.legends.iter().any(|l| l.id == TENSION_FIELD) {
            out.legends.push(Legend { id: TENSION_FIELD.into(), title: loc("Bar stress utilisation σ_s/(0.8·f_sk)", "Bewehrungsspannungsausnutzung σ_s/(0,8·f_sk)"), scale: LegendScale::Scalar { field_id: TENSION_FIELD.into(), map: ColourMapRef { id: RampId::Failure.wire().into(), min: 0.0, max: TENSION_MAX }, unit: QuantityKind::Dimensionless } });
        }
        out.push(spec, &points);
    }
}

/// 🔁️ The reinforcement-fatigue bands over the hogging zones of a bridge girder: Δσ_s,equ/(Δσ_Rsk/γ_s,fat) along the support moment line, the cracked range falling with the moment and the tension-stiffening increment Δσ_s (`ndp.en1994.2.fatigue-tension-stiffening`) constant, bound to the reinforcement fatigue check.
fn fatigue_bands(input: &SceneInput, layout: &Layout, f: &BeamFacts, out: &mut Out) {
    let (Some(pose), true) = (layout.pose(&f.beam.id), f.hogging && f.bridge) else { return };
    let Some(gov) = binding(input.report, &f.beam.id, &FATIGUE_PREFIXES) else { return };
    let Some(check) = check_with(input.report, &format!("en1994.2.6.8.2.rebar.{}", f.beam.id)) else { return };
    let (beam, sec) = (&f.beam, part_1_1::isection(&f.steel));
    let half = beam.spacing_m / 2.0;
    let beff = catalogue::effective_width_m(catalogue::le_hogging_m(beam.span_m, beam.neighbour_span_m()), 0.0, half, half, false);
    let as_total = beam.as_hogging_m2_per_m * beff;
    let cr = catalogue::cracked(&sec, beam.slab_thickness_m, as_total, catalogue::constants::HOGGING_REBAR_DEPTH_M);
    let ts = catalogue::delta_sigma_ts_fatigue(catalogue::fctm_pa(beam.concrete_f_ck_pa), cr.area, cr.i, sec.area(), sec.i_y, as_total / (beff * f.hc).max(1e-9));
    let r_lim = catalogue::fatigue::rebar_curve().resistance_pa(beam.n_cycles).max(1.0);
    let (u_ts, u_support) = (ts / r_lim, check.utilization);
    let u = move |ratio: f64| u_ts + ratio * (u_support - u_ts).max(0.0);
    let lifted = f.depth + 5.0 * LIFT;
    let half_w = f.slab_width() / 2.0;
    for (k, zone) in continuous::hogging_band_profile(&f.layout, f.stiffness, BAND_POINTS).into_iter().enumerate() {
        let (x_a, x_b) = (zone[0].0, zone[zone.len() - 1].0);
        let points: Vec<Vec3> = [-half_w, half_w].iter().flat_map(|y| zone.iter().map(move |p| (p.0, *y))).map(|(x, y)| pose.point(x, y, lifted)).collect();
        let values = grid_values(2, BAND_POINTS, |_, i| u(zone[i].1));
        let tag = if k == 0 { "fatigue-band".to_string() } else { format!("fatigue-band-{}", k + 1) };
        let name = loc(&format!("Reinforcement fatigue band of {}", f.beam.id), &format!("Bewehrungsermüdungsband von {}", f.beam.id));
        let detail = loc(
            &format!("Δσ_s,equ/Δσ_R = {u_support:.2} at the support, of which {u_ts:.2} from the tension-stiffening increment Δσ_s = {:.1} MPa, over {x_a:.2} to {x_b:.2} m (EN 1994-2, 6.8.5.4).", ts / 1e6),
            &format!("Δσ_s,equ/Δσ_R = {u_support:.2} an der Stütze, davon {u_ts:.2} aus dem Zugversteifungsanteil Δσ_s = {:.1} MPa, über {x_a:.2} bis {x_b:.2} m (EN 1994-2, 6.8.5.4).", ts / 1e6),
        );
        let spec = surface(format!("{}:{tag}", f.beam.id), "beams", points.clone(), 2, BAND_POINTS, MaterialRole::Concrete, name)
            .bound(Some(gov.clone()))
            .detailed(detail)
            .channel(ColourChannel::Fields { active: FATIGUE_FIELD.into(), fields: vec![ScalarField { id: FATIGUE_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } }] })
            .translucent(0.9);
        if !out.legends.iter().any(|l| l.id == FATIGUE_FIELD) {
            out.legends.push(Legend { id: FATIGUE_FIELD.into(), title: loc("Reinforcement fatigue utilisation Δσ_s,equ/Δσ_R", "Bewehrungsermüdungsausnutzung Δσ_s,equ/Δσ_R"), scale: LegendScale::Scalar { field_id: FATIGUE_FIELD.into(), map: ColourMapRef { id: RampId::Failure.wire().into(), min: 0.0, max: FATIGUE_MAX }, unit: QuantityKind::Dimensionless } });
        }
        out.push(spec, &points);
    }
}

/// 🔩️ The end plate of a bridge girder: a vertical plate across the girder axis at its start, hanging from the slab top, bound to the bearing check.
fn end_plate(input: &SceneInput, layout: &Layout, f: &BeamFacts, out: &mut Out) {
    let (w, h) = (f.beam.end_plate_width_m, f.beam.end_plate_height_m);
    let (Some(pose), true) = (layout.pose(&f.beam.id), f.bridge && w > 1e-6 && h > 1e-6) else { return };
    let top = f.depth;
    let frame = local_frame(pose, (-END_PLATE_THICKNESS / 2.0, 0.0, top - h / 2.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0));
    let geometry = Geometry::Plate(PlateGeometry { frame: frame.clone(), outline: rect(-w / 2.0, w / 2.0, -h / 2.0, h / 2.0), thickness: END_PLATE_THICKNESS, offset: 0.0, stiffeners: Vec::new() });
    let name = loc(&format!("End plate of {}", f.beam.id), &format!("Endplatte von {}", f.beam.id));
    let detail = loc(
        &format!("{:.0} × {:.0} mm plate at the girder end; the studs of the {:.2} m end zone press it against the concrete (EN 1992-1-1, 6.7).", w * 1e3, h * 1e3, f.beam.end_zone_length_m),
        &format!("Platte {:.0} × {:.0} mm am Trägerende; die Bolzen des {:.2} m langen Endbereichs drücken sie gegen den Beton (EN 1992-1-1, 6.7).", w * 1e3, h * 1e3, f.beam.end_zone_length_m),
    );
    let spec = Spec::new(EntityKind::Plate, format!("{}:end-plate", f.beam.id), "beams", geometry, MaterialRole::Steel, name, LAYER_STEEL).bound(binding(input.report, &f.beam.id, &END_PLATE_PREFIXES)).detailed(detail);
    out.push(spec, &[frame.origin.clone()]);
}

/// 🌈️ Adds the temperature plates of the wanted columns and the crack-width bands of the wanted girders.
pub(crate) fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    for column in &input.document.columns {
        if out.wants(&column.id) {
            fire_plate(input, layout, column, out);
        }
    }
    for beam in &input.document.beams {
        if out.wants(&beam.id) {
            let f = BeamFacts::of(input.document, beam);
            if f.l_total > 1e-6 {
                crack_bands(input, layout, &f, out);
                tension_bands(input, layout, &f, out);
                fatigue_bands(input, layout, &f, out);
                super::fire_section::project(input, layout, &f, out);
                end_plate(input, layout, &f, out);
            }
        }
    }
}

/// 🔥️ The `fire` track over the temperature fields present in the merged scene, one keyframe per fire minute.
pub(crate) fn fire_track(entities: &[SceneEntity]) -> Option<ParameterTrack> {
    let minutes: BTreeSet<u32> = entities
        .iter()
        .filter_map(|e| match &e.appearance.channel {
            ColourChannel::Fields { fields, .. } => Some(fields),
            _ => None,
        })
        .flatten()
        .filter_map(|f| f.id.strip_prefix(FIRE_FIELD_PREFIX)?.parse().ok())
        .collect();
    let stops: Vec<(f64, String, String)> = minutes.into_iter().map(|m| (f64::from(m) * 60.0, fire_field_id(f64::from(m)), format!("{m} min"))).collect();
    let rows: Vec<(f64, &str, &str, &str)> = stops.iter().map(|(at, id, name)| (*at, id.as_str(), name.as_str(), name.as_str())).collect();
    projection_kit::tracks::timed("fire", TrackKind::Time, ("Fire exposure time", "Branddauer"), &rows)
}
