//! 🔥️ The temperature field over the section of a composite beam at mid-span, one scalar field per fire minute (scrubbed by the `fire` track): the protected steel profile (lumped-capacity temperature of EN 1993-1-2 Eq. 4.27), the concrete slab (two-dimensional heat conduction from the soffit, `catalogue::beam_fire`) and the hogging reinforcement layer read from the slab field. The property tables are listed as unverified in the NDP registry (`ndp.en1994.1-2.*`).

use super::checks::catalogue::beam_fire::{Evolution, Point, SlabField};
use super::checks::catalogue::constants::HOGGING_REBAR_DEPTH_M;
use super::checks::{part_1_2, part_en1990};
use super::diagrams as dg;
use super::facts::BeamFacts;
use super::model::En1994Snapshot;
use super::norm::CheckReport;
use super::scene::diagram_kit::Xy;
use super::scene::projection_kit::diagram_parts::{column, stroke};
use super::fields::{fire_field_id, fire_times, grid_values, surface, FIRE_LEGEND, FIRE_MAX_C, PLATE_POINTS};
use super::kit::*;
use super::layout::Layout;
use super::scene::colour_maps::RampId;
use super::scene::*;
use super::SceneInput;

const STEEL_PREFIXES: [&str; 1] = ["en1994.1-2.4.3.theta."];
const RESISTANCE_PREFIXES: [&str; 1] = ["en1994.1-2.4.3.mfi."];
const SLAB_ROWS: usize = 13;
const REBAR_STRIP_M: f64 = 0.3;
const STEP_MIN: f64 = 5.0;

fn scalars(times: &[f64], value: impl Fn(usize, usize) -> f64, rows: usize, columns: usize) -> Vec<ScalarField> {
    times.iter().enumerate().map(|(k, t)| ScalarField { id: fire_field_id(*t), unit: QuantityKind::Temperature, samples: FieldSamples::Vertex { values: grid_values(rows, columns, |r, c| value(k, r * columns + c)) } }).collect()
}

fn stops(times: &[f64]) -> String {
    times.iter().map(|t| format!("{t:.0}")).collect::<Vec<_>>().join(", ")
}

fn slab_value(field: &SlabField, y: f64, depth: f64) -> f64 {
    field.sample(y.abs(), depth)
}

/// 🔥️ Adds the steel, slab and reinforcement temperature plates of one beam at mid-span.
pub(crate) fn project(input: &SceneInput, layout: &Layout, f: &BeamFacts, out: &mut Out) {
    let Some(pose) = layout.pose(&f.beam.id) else { return };
    let times = fire_times(input.document.fire_rating);
    if times.is_empty() {
        return;
    }
    let fire = part_1_2::beam_fire(&f.beam, input.document.steel_f_y_pa, input.document.insulation_thickness_m);
    let Evolution { points, fields } = fire.evolve(&times);
    let (x, half_steel) = (f.l_total / 2.0, f.steel.width_m / 2.0);
    let top = f.depth - f.beam.slab_thickness_m;
    let bottom = top - f.steel.height_m;
    let end = &points[points.len() - 1];
    let list = stops(&times);
    let active = |fields: &[ScalarField]| fields[fields.len() - 1].id.clone();
    let legend = |out: &mut Out| {
        if !out.legends.iter().any(|l| l.id == FIRE_LEGEND) {
            out.legends.push(Legend { id: FIRE_LEGEND.into(), title: loc("Section temperature θ in the standard fire (°C)", "Querschnittstemperatur θ im Normbrand (°C)"), scale: LegendScale::Scalar { field_id: fire_field_id(times[times.len() - 1]), map: ColourMapRef { id: RampId::Warm.wire().into(), min: 20.0, max: FIRE_MAX_C }, unit: QuantityKind::Temperature } });
        }
    };

    let (px, py) = (2, PLATE_POINTS);
    let steel_points: Vec<Vec3> = (0..py).flat_map(|j| (0..px).map(move |i| (i, j))).map(|(i, j)| pose.point(x, -half_steel + 2.0 * half_steel * i as f64, bottom + (top - bottom) * j as f64 / (py - 1) as f64)).collect();
    let steel_fields = scalars(&times, |k, _| points[k].steel_c, py, px);
    let name = loc(&format!("Fire temperature plate of {}", f.beam.id), &format!("Brandtemperaturplatte von {}", f.beam.id));
    let detail = loc(
        &format!("Uniform temperature of the protected steel section at mid-span after {list} min of ISO 834 (A_p/V = {:.0} 1/m, board {:.0} mm); θ_a = {:.0} °C at the end (lumped capacity, no gradient over the profile).", fire.am_v, fire.board_m * 1e3, end.steel_c),
        &format!("Gleichmäßige Temperatur des geschützten Stahlquerschnitts in Feldmitte nach {list} min ISO 834 (A_p/V = {:.0} 1/m, Bekleidung {:.0} mm); θ_a = {:.0} °C am Ende (konzentrierte Kapazität, ohne Gradient über das Profil).", fire.am_v, fire.board_m * 1e3, end.steel_c),
    );
    let spec = surface(format!("{}:fire-plate", f.beam.id), "beams", steel_points.clone(), py, px, MaterialRole::Steel, name)
        .bound(binding(input.report, &f.beam.id, &STEEL_PREFIXES))
        .detailed(detail)
        .channel(ColourChannel::Fields { active: active(&steel_fields), fields: steel_fields })
        .translucent(0.85);
    legend(out);
    out.push(spec, &steel_points);

    let half = fire.slab().half_width_m;
    let ys = f.beam.slab_thickness_m;
    let (cols, rows) = (PLATE_POINTS, SLAB_ROWS);
    let across = |c: usize| -half + 2.0 * half * c as f64 / (cols - 1) as f64;
    let depth = |r: usize| ys * r as f64 / (rows - 1) as f64;
    let slab_points: Vec<Vec3> = (0..rows).flat_map(|r| (0..cols).map(move |c| (r, c))).map(|(r, c)| pose.point(x, across(c), top + depth(r))).collect();
    let slab_fields = scalars(&times, |k, i| slab_value(&fields[k], across(i % cols), depth(i / cols) * fire.thermal_m / ys), rows, cols);
    let name = loc(&format!("Fire temperature plate of the slab of {}", f.beam.id), &format!("Brandtemperaturplatte der Platte von {}", f.beam.id));
    let detail = loc(
        &format!("Temperatures θ of the slab section at mid-span after {list} min of ISO 834 from two-dimensional heat conduction (fire on the soffit, the steel flange shields {:.0} mm of it); soffit {:.0} °C, reinforcement layer {:.0} °C, slab top {:.0} °C at the end (equivalent solid slab of h_eff = {:.0} mm for the {:.0} mm deck, h_c + h_p·b_0/pitch; unverified: ndp.en1994.1-2.{{concrete-conduction,alpha-unexposed,rho-concrete,heff}}).", f.steel.width_m * 1e3, end.soffit_c, end.rebar_c, end.slab_top_c, fire.thermal_m * 1e3, ys * 1e3),
        &format!("Temperaturen θ des Plattenquerschnitts in Feldmitte nach {list} min ISO 834 aus zweidimensionaler Wärmeleitung (Brand an der Unterseite, der Stahlflansch schirmt {:.0} mm davon ab); Unterseite {:.0} °C, Bewehrungslage {:.0} °C, Plattenoberseite {:.0} °C am Ende (äquivalente Vollplatte mit h_eff = {:.0} mm für die {:.0} mm dicke Decke, h_c + h_p·b_0/Teilung; ungeprüft: ndp.en1994.1-2.{{concrete-conduction,alpha-unexposed,rho-concrete,heff}}).", f.steel.width_m * 1e3, end.soffit_c, end.rebar_c, end.slab_top_c, fire.thermal_m * 1e3, ys * 1e3),
    );
    let spec = surface(format!("{}:fire-slab", f.beam.id), "beams", slab_points.clone(), rows, cols, MaterialRole::Concrete, name)
        .bound(binding(input.report, &f.beam.id, &RESISTANCE_PREFIXES))
        .detailed(detail)
        .channel(ColourChannel::Fields { active: active(&slab_fields), fields: slab_fields })
        .translucent(0.85);
    legend(out);
    out.push(spec, &slab_points);

    let z = f.depth - HOGGING_REBAR_DEPTH_M;
    let strip: Vec<Vec3> = [x - REBAR_STRIP_M / 2.0, x + REBAR_STRIP_M / 2.0].iter().flat_map(|xs| [-half, half].map(|y| pose.point(*xs, y, z))).collect();
    let rebar_fields = scalars(&times, |k, _| points[k].rebar_c, 2, 2);
    let name = loc(&format!("Reinforcement temperature of {}", f.beam.id), &format!("Bewehrungstemperatur von {}", f.beam.id));
    let detail = loc(
        &format!("Temperature of the reinforcement layer {:.0} mm below the slab top after {list} min of ISO 834: {:.0} °C at the end, k_s,θ = {:.2} of the strength ({} bars, EN 1992-1-2 Table 3.2a; read from the slab field; unverified: ndp.en1994.1-2.ks.*).", HOGGING_REBAR_DEPTH_M * 1e3, end.rebar_c, end.rebar_ks, fire.rebar.wire()),
        &format!("Temperatur der Bewehrungslage {:.0} mm unter der Plattenoberseite nach {list} min ISO 834: {:.0} °C am Ende, k_s,θ = {:.2} der Festigkeit ({}-Stäbe, EN 1992-1-2 Tabelle 3.2a; aus dem Plattenfeld gelesen; ungeprüft: ndp.en1994.1-2.ks.*).", HOGGING_REBAR_DEPTH_M * 1e3, end.rebar_c, end.rebar_ks, fire.rebar.wire()),
    );
    let spec = surface(format!("{}:fire-rebar", f.beam.id), "beams", strip.clone(), 2, 2, MaterialRole::Rebar, name)
        .bound(binding(input.report, &f.beam.id, &RESISTANCE_PREFIXES))
        .detailed(detail)
        .channel(ColourChannel::Fields { active: active(&rebar_fields), fields: rebar_fields })
        .translucent(0.95);
    legend(out);
    out.push(spec, &strip);
}

fn list(points: &[Point], value: &dyn Fn(&Point) -> f64) -> Vec<Xy> {
    points.iter().map(|p| [p.minutes, value(p)]).collect()
}

fn minutes_axis(end: f64) -> Axis {
    dg::axis("Fire duration t", "Branddauer t", QuantityKind::Time, "min", Some((0.0, end)))
}

fn temperatures(f: &BeamFacts, points: &[Point], end: f64, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let last = &points[points.len() - 1];
    let check = worst(report, id, &["en1994.1-2.4.3.theta."]);
    let (gas, steel, soffit, rebar, top) = (list(points, &|p| p.gas_c), list(points, &|p| p.steel_c), list(points, &|p| p.soffit_c), list(points, &|p| p.rebar_c), list(points, &|p| p.slab_top_c));
    Diagram2d {
        id: format!("fire-temperature:{id}"),
        title: loc(&format!("Section temperatures in the fire – {id}"), &format!("Querschnittstemperaturen im Brand – {id}")),
        description: loc(
            &format!("Temperatures over the ISO 834 duration: gas, protected steel (lumped capacity, EN 1993-1-2 Eq. 4.27), and the mean of the slab soffit, the reinforcement layer and the slab top from the two-dimensional slab field; at {end:.0} min the steel is {:.0} °C, the reinforcement {:.0} °C (values listed as unverified in the NDP registry: ndp.en1994.1-2.*). The 3D plates show the same fields.", last.steel_c, last.rebar_c),
            &format!("Temperaturen über die ISO-834-Branddauer: Gas, geschützter Stahl (konzentrierte Kapazität, EN 1993-1-2 Gl. 4.27) sowie die Mittelwerte von Plattenunterseite, Bewehrungslage und Plattenoberseite aus dem zweidimensionalen Plattenfeld; nach {end:.0} min hat der Stahl {:.0} °C, die Bewehrung {:.0} °C (Werte im NDP-Register als ungeprüft geführt: ndp.en1994.1-2.*). Die 3D-Platten zeigen dieselben Felder.", last.steel_c, last.rebar_c),
        ),
        axes: vec![minutes_axis(end), dg::axis("Temperature θ", "Temperatur θ", QuantityKind::Temperature, "°C", None)],
        series: vec![
            dg::polyline("gas", "Gas θ_g", "Gas θ_g", &gas, stroke(Tone::Neutral, 1.5, Dash::Dashed)),
            dg::polyline("steel", "Protected steel θ_a", "Geschützter Stahl θ_a", &steel, stroke(Tone::Warning, 2.0, Dash::Solid)),
            dg::polyline("soffit", "Slab soffit", "Plattenunterseite", &soffit, stroke(Tone::Error, 1.5, Dash::Solid)),
            dg::polyline("rebar", "Reinforcement layer", "Bewehrungslage", &rebar, stroke(Tone::Info, 2.0, Dash::Dotted)),
            dg::polyline("top", "Slab top", "Plattenoberseite", &top, stroke(Tone::Info, 1.0, Dash::Solid)),
        ],
        markers: vec![Marker {
            id: "steel-end".into(),
            at: Vec2::new(end, last.steel_c),
            shape: MarkerShape::Diamond,
            tone: check.map_or(Tone::Neutral, |c| dg::tone_of_ratio(c.utilization)),
            label: loc(&format!("θ_a = {:.0} °C after {end:.0} min", last.steel_c), &format!("θ_a = {:.0} °C nach {end:.0} min", last.steel_c)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: dg::beam_entity(id),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Fire duration t", "Branddauer t", "min", gas.iter().map(|q| q[0]).collect()),
                column("Gas θ_g", "Gas θ_g", "°C", gas.iter().map(|q| q[1]).collect()),
                column("Steel θ_a", "Stahl θ_a", "°C", steel.iter().map(|q| q[1]).collect()),
                column("Slab soffit", "Plattenunterseite", "°C", soffit.iter().map(|q| q[1]).collect()),
                column("Reinforcement", "Bewehrung", "°C", rebar.iter().map(|q| q[1]).collect()),
                column("Slab top", "Plattenoberseite", "°C", top.iter().map(|q| q[1]).collect()),
            ],
        },
    }
}

fn resistance(f: &BeamFacts, points: &[Point], end: f64, report: &CheckReport) -> Diagram2d {
    let id = f.beam.id.as_str();
    let last = &points[points.len() - 1];
    let check = worst(report, id, &["en1994.1-2.4.3.mfi."]);
    let m_ed = part_en1990::fire_combination(&f.beam.actions, f.beam.span_m, f.beam.adjacent_span_m, f.beam.support, f.beam.spacing_m, f.annex, f.reliability_class).m_nm / 1e3;
    let (steel_only, with_kc) = (list(points, &|p| p.reduced.steel_only_nm / 1e3), list(points, &|p| p.reduced.m_nm / 1e3));
    let cold = points[0].reduced.steel_only_nm / 1e3;
    let upper = (cold.max(m_ed) * 1.1).max(1.0);
    Diagram2d {
        id: format!("fire-resistance:{id}"),
        title: loc(&format!("Reduced plastic moment resistance over the fire duration – {id}"), &format!("Abgeminderte plastische Momententragfähigkeit über die Branddauer – {id}")),
        description: loc(
            &format!("M_fi,Rd(t) of the full-connection sagging section with k_y,θ(θ_a) on the steel (the model of the fire bending check, EN 1994-1-2 4.3.4.2) and additionally with the mean k_c,θ of the slab compression zone from the slab field, against the fire design moment M_fi,Ed = {m_ed:.1} kNm; at {end:.0} min M_fi,Rd = {:.1} kNm (cold {cold:.1} kNm, k_y,θ = {:.2}, k_c,θ = {:.2}; unverified: ndp.en1994.1-2.tables, ndp.en1994.1-2.kc-theta).", last.reduced.steel_only_nm / 1e3, last.reduced.ky, last.reduced.kc),
            &format!("M_fi,Rd(t) des Querschnitts bei voller Verdübelung im Feld mit k_y,θ(θ_a) am Stahl (Modell des Brandbiegenachweises, EN 1994-1-2 4.3.4.2) und zusätzlich mit dem mittleren k_c,θ der Druckzone der Platte aus dem Plattenfeld, gegen das Bemessungsmoment im Brandfall M_fi,Ed = {m_ed:.1} kNm; nach {end:.0} min ist M_fi,Rd = {:.1} kNm (kalt {cold:.1} kNm, k_y,θ = {:.2}, k_c,θ = {:.2}; ungeprüft: ndp.en1994.1-2.tables, ndp.en1994.1-2.kc-theta).", last.reduced.steel_only_nm / 1e3, last.reduced.ky, last.reduced.kc),
        ),
        axes: vec![minutes_axis(end), dg::axis("Moment M", "Moment M", QuantityKind::Moment, "kN·m", Some((0.0, upper)))],
        series: vec![
            dg::polyline("m-fi-rd", "M_fi,Rd(t) with k_y,θ", "M_fi,Rd(t) mit k_y,θ", &steel_only, stroke(Tone::Info, 2.0, Dash::Solid)),
            dg::polyline("m-fi-rd-kc", "M_fi,Rd(t) with k_y,θ and k_c,θ", "M_fi,Rd(t) mit k_y,θ und k_c,θ", &with_kc, stroke(Tone::Warning, 1.5, Dash::Dashed)),
            dg::limit("m-fi-ed", "Fire design moment M_fi,Ed", "Bemessungsmoment im Brandfall M_fi,Ed", AxisRef::Y, m_ed, stroke(Tone::Error, 1.5, Dash::Dotted)),
        ],
        markers: vec![Marker {
            id: "rating".into(),
            at: Vec2::new(end, last.reduced.steel_only_nm / 1e3),
            shape: MarkerShape::Circle,
            tone: check.map_or(Tone::Neutral, |c| dg::tone_of_ratio(c.utilization)),
            label: loc(&format!("M_fi,Rd = {:.1} kNm after {end:.0} min, {:.0} % of M_pl,Rd", last.reduced.steel_only_nm / 1e3, last.reduced.steel_only_nm / 1e3 / cold.max(1e-9) * 100.0), &format!("M_fi,Rd = {:.1} kNm nach {end:.0} min, {:.0} % von M_pl,Rd", last.reduced.steel_only_nm / 1e3, last.reduced.steel_only_nm / 1e3 / cold.max(1e-9) * 100.0)),
            utilisation: check.map(|c| c.utilization),
            bound_check: check.map(|c| c.id.clone()),
            binding: None,
        }],
        annotations: Vec::new(),
        focus: dg::beam_entity(id),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Fire duration t", "Branddauer t", "min", steel_only.iter().map(|q| q[0]).collect()),
                column("Steel θ_a", "Stahl θ_a", "°C", points.iter().map(|p| p.steel_c).collect()),
                column("k_y,θ", "k_y,θ", "1", points.iter().map(|p| p.reduced.ky).collect()),
                column("k_c,θ", "k_c,θ", "1", points.iter().map(|p| p.reduced.kc).collect()),
                column("M_fi,Rd with k_y,θ", "M_fi,Rd mit k_y,θ", "kN·m", steel_only.iter().map(|q| q[1]).collect()),
                column("M_fi,Rd with k_y,θ and k_c,θ", "M_fi,Rd mit k_y,θ und k_c,θ", "kN·m", with_kc.iter().map(|q| q[1]).collect()),
            ],
        },
    }
}

/// 🔥️ The temperature and the reduced-resistance diagrams of one beam over the duration of the fire rating.
pub(crate) fn diagrams(f: &BeamFacts, document: &En1994Snapshot, report: &CheckReport) -> Vec<Diagram2d> {
    let Some(rating) = document.fire_rating.rated() else { return Vec::new() };
    let end = part_1_2::rating_minutes(rating);
    let minutes: Vec<f64> = (0..=(end / STEP_MIN).round() as usize).map(|k| k as f64 * STEP_MIN).collect();
    let fire = part_1_2::beam_fire(&f.beam, document.steel_f_y_pa, document.insulation_thickness_m);
    let points = fire.evolve(&minutes).points;
    vec![temperatures(f, &points, end, report), resistance(f, &points, end, report)]
}
