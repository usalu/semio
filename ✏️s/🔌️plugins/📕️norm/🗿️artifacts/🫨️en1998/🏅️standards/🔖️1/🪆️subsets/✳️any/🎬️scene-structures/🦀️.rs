//! 🏗️ Structure views: the spectrum analysis of a bridge, the wall pressure of a tank and the modal response of a tower, computed with the schema kernels the checks call, shared by the scene entities, the handles and the diagrams so picture and report cannot disagree. Diagrams: deck mode shapes and pier moments, wall pressure profile, tower mode shapes and modal forces.

extern crate semio_s_artifact_norm_en1998_model as model;
extern crate semio_s_artifact_norm_en1998_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

#[path = "🏷️variables/🦀️.rs"]
pub mod variables;

#[path = "🌉️bridge-action-views/🦀️.rs"]
mod bridge_action_views;

#[path = "🛞️isolation-views/🦀️.rs"]
mod isolation_views;

use model::{En1998Bridge, En1998Silo, En1998Snapshot, En1998Tank, En1998Tower, PierStiffnessBasis};
use scene::projection_kit::diagram_parts::{self, column, stroke};
use scene::projection_kit::loc;
use scene::*;
use norm::CheckReport;
use schema::{bridge, part_1, part_2, part_3, part_4, part_6, tank, tower};

/// 🌍️ Site spectrum constants of one document: design ground acceleration inputs and corner periods.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Site {
    pub a_gr: f64,
    pub s: f64,
    pub tb: f64,
    pub tc: f64,
    pub td: f64,
    pub gamma_i: f64,
    pub section: part_3::AnnexA,
}

const TONES: [Tone; 3] = [Tone::Info, Tone::Warning, Tone::Error];

/// 🌉️ The two spectrum analyses of one bridge with pier or bearing rows.
pub struct BridgeView {
    pub supports: Vec<bridge::Support>,
    pub longitudinal: Option<bridge::Response>,
    pub transverse: Option<bridge::Response>,
}

pub fn bridge_view(document: &En1998Snapshot, b: &En1998Bridge, site: &Site) -> Option<BridgeView> {
    let supports = bridge::supports(&b.id, &document.piers, &document.bearings, b.pier_cracked_inertia_ratio, &site.section);
    if supports.is_empty() {
        return None;
    }
    let q = part_2::design_behaviour_factor(b.period_ratio);
    let gamma_i = part_2::bridge_gamma_i(b.importance_class);
    let mass = part_1::entity_seismic_mass_kg(b.permanent_gk_n, &b.variables, false, b.correlated_occupancy);
    let design = |t: f64| part_1::design_spectrum_at(site.a_gr, gamma_i, site.s, site.tb, site.tc, site.td, q, t);
    let elastic = |t: f64| gamma_i * part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, t);
    let xs: Vec<f64> = supports.iter().map(|s| s.x_m).collect();
    let run = |direction: bridge::Direction, ei: f64| {
        let stiffness: Vec<f64> = supports.iter().map(|s| s.stiffness_n_per_m(direction)).collect();
        bridge::analyse(direction, mass, b.deck_length_m, ei, &xs, &stiffness, &design, &elastic)
    };
    Some(BridgeView { longitudinal: run(bridge::Direction::Longitudinal, 0.0), transverse: run(bridge::Direction::Transverse, b.deck_ei_nm2), supports })
}

/// 🛢️ The wall pressure of one tank at the settled accelerations of the base shear check.
pub struct TankView {
    pub fluid_height_m: f64,
    pub profile: Vec<tank::WallPressure>,
    pub convective_period_s: f64,
}

pub fn tank_view(t: &En1998Tank, site: &Site) -> TankView {
    let fluid_height = tank::fluid_height_m(t.height_m, t.filling_ratio);
    let period = part_4::convective_period_s(fluid_height, t.radius_m);
    let a_i = site.gamma_i * part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, site.tc);
    let a_c = site.gamma_i * part_1::elastic_response_spectrum_eta(site.a_gr, site.s, site.tb, site.tc, site.td, period, part_4::convective_damping_eta());
    let rho = tank::density_kg_m3(t.filling_ratio.clamp(0.0, 1.0) * t.content_qk_n / tank::GRAVITY, t.radius_m, fluid_height);
    TankView { fluid_height_m: fluid_height, profile: tank::wall_profile(fluid_height, t.radius_m, rho, a_i, a_c, 20), convective_period_s: period }
}

/// 🗼️ The modes and the CQC response of one tower with a given stiffness.
pub struct TowerView {
    pub modes: tower::Modes,
    pub response: tower::Response,
    pub q: f64,
}

pub fn tower_view(t: &En1998Tower, site: &Site) -> Option<TowerView> {
    let mass = part_1::entity_seismic_mass_kg(t.permanent_gk_n, &t.variables, false, t.correlated_occupancy);
    let q = part_6::tower_behaviour_factor(t.q_nominal, t.is_chimney);
    let modes = tower::modes(mass, t.height_m, t.ei_nm2, t.base_rotational_stiffness_nm)?;
    let response = tower::respond(&modes, &|period| part_1::design_spectrum_at(site.a_gr, site.gamma_i, site.s, site.tb, site.tc, site.td, q, period));
    Some(TowerView { modes, response, q })
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str) -> Axis {
    diagram_parts::axis(en, de, quantity, unit, AxisScale::Linear, None)
}

/// 📈️ The diagrams of the bridges, tanks and towers of the document.
pub fn diagrams(d: &En1998Snapshot, report: &CheckReport, site: &Site) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for b in &d.bridges {
        let view = bridge_view(d, b, site);
        let derived = view.as_ref().and_then(|v| v.longitudinal.as_ref()).and_then(|r| r.periods_s.first().copied());
        out.extend(bridge_action_views::bearing_displacement(d, report, b, site, derived));
        if let Some(view) = view {
            out.extend(deck_modes(b, &view));
            out.push(pier_moments(d, report, b, &view));
            out.extend(pier_capacity(d, report, b, &view));
            out.push(bridge_action_views::bridge_spectrum(b, site));
            out.extend(bridge_action_views::pier_behaviour(d, report, b));
            out.extend(bridge_action_views::pier_moment_curvature(d, b, site));
        }
    }
    out.extend(isolation_views::isolation_loops(d, report));
    out.extend(isolation_views::pier_base_spectra(d, report, site));
    for t in &d.tanks {
        out.push(wall_pressure(t, &tank_view(t, site)));
    }
    out.extend(d.silos.iter().filter_map(|silo| silo_demand(silo, site)));
    out.extend(d.silos.iter().filter_map(|silo| silo_fill(silo, site)));
    for t in &d.towers {
        if let Some(view) = tower_view(t, site) {
            out.push(tower_modes(t, &view));
            out.push(tower_modal_forces(t, &view));
            out.extend(tower_levels(t, &view));
        }
    }
    out
}

fn deck_modes(b: &En1998Bridge, view: &BridgeView) -> Option<Diagram2d> {
    let response = view.transverse.as_ref()?;
    let mut series = Vec::new();
    let mut columns = vec![column("Position x", "Lage x", "m", response.node_x_m.clone())];
    for (n, (shape, period)) in response.shapes.iter().zip(&response.periods_s).enumerate().take(3) {
        let label = format!("{}: T = {period:.3} s, M_eff = {:.0} %", n + 1, response.effective_mass_ratio[n] * 100.0);
        series.push(Series::Polyline { id: format!("mode-{}", n + 1), label: loc(&label, &label), points: response.node_x_m.iter().zip(shape).map(|(x, v)| Vec2::new(*x, *v)).collect(), style: stroke(TONES[n % 3], 2.0, Dash::Solid) });
        columns.push(column(&format!("\u{3c6} {}", n + 1), &format!("\u{3c6} {}", n + 1), "1", shape.clone()));
    }
    let markers = view.supports.iter().map(|s| Marker { id: format!("reference:support-{}", if s.is_pier() { &s.pier_id } else { &s.bearing_ids[0] }), at: Vec2::new(s.x_m, 0.0), shape: MarkerShape::Triangle, tone: Tone::Neutral, label: loc(&format!("{} at {:.1} m", if s.is_pier() { format!("pier {}", s.pier_id) } else { "abutment".into() }, s.x_m), &format!("{} bei {:.1} m", if s.is_pier() { format!("Pfeiler {}", s.pier_id) } else { "Widerlager".into() }, s.x_m)), utilisation: None, bound_check: None, binding: None }).collect();
    Some(Diagram2d {
        id: format!("deck-modes:{}", b.id),
        title: loc(&format!("Transverse deck modes \u{2013} {}", b.id), &format!("Querschwingformen des \u{dc}berbaus \u{2013} {}", b.id)),
        description: loc("First transverse modes of the deck on the pier and bearing springs from the mechanics kernel (EN 1998-2, 4.2), normalised to the peak, with period and effective modal mass share.", "Erste Querschwingformen des \u{dc}berbaus auf den Pfeiler- und Lagerfedern aus dem Mechanikkern (EN 1998-2, 4.2), auf den Gr\u{f6}\u{df}twert normiert, mit Periode und effektivem Modalmassenanteil."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m"), axis("Mode shape \u{3c6}", "Eigenform \u{3c6}", QuantityKind::Dimensionless, "1")],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, &b.id, format!("bridges[id={}]", b.id))),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn pier_moments(d: &En1998Snapshot, report: &CheckReport, b: &En1998Bridge, view: &BridgeView) -> Diagram2d {
    let mut demand = Vec::new();
    let mut resistance = Vec::new();
    let mut markers = Vec::new();
    let mut positions = Vec::new();
    let (mut demand_values, mut resistance_values) = (Vec::new(), Vec::new());
    for (index, support) in view.supports.iter().enumerate() {
        let Some(pier) = d.piers.iter().find(|pier| pier.id == support.pier_id && pier.height_m > 0.0) else { continue };
        let force = [&view.longitudinal, &view.transverse].iter().filter_map(|r| r.as_ref()).map(|r| r.support_force_n[index].abs()).fold(0.0_f64, f64::max);
        let m_ed = bridge::pier_base_moment_nm(force, pier.height_m);
        demand.push(Vec2::new(pier.x_m, m_ed));
        resistance.push(Vec2::new(pier.x_m, pier.moment_rd_nm));
        positions.push(pier.x_m);
        demand_values.push(m_ed);
        resistance_values.push(pier.moment_rd_nm);
        let utilisation = report.checks.iter().find(|c| c.id == format!("en1998.2.{}.pier.{}.flexure", b.id, pier.id)).map(|c| c.utilization).filter(|u| u.is_finite());
        let mut binding = FieldBinding::new(format!("piers[id={}].momentRdNm", pier.id), Affine::IDENTITY, "N*m");
        binding.quantise = Some(Quantise::Round { step: 1000.0 });
        markers.push(Marker { id: ::semio_s_plugin_norm_structural_scene::projection_kit::diagram_parts::checked_marker_id(&format!("pier-{}", pier.id), &utilisation.map(|_| format!("en1998.2.{}.pier.{}.flexure", b.id, pier.id))), at: Vec2::new(pier.x_m, pier.moment_rd_nm), shape: MarkerShape::Diamond, tone: utilisation.map_or(Tone::Neutral, |u| scene::utilisation_tone(u)), label: loc(&format!("{}: M_Ed = {:.0} kN\u{b7}m, M_Rd = {:.0} kN\u{b7}m", pier.id, m_ed / 1000.0, pier.moment_rd_nm / 1000.0), &format!("{}: M_Ed = {:.0} kN\u{b7}m, M_Rd = {:.0} kN\u{b7}m", pier.id, m_ed / 1000.0, pier.moment_rd_nm / 1000.0)), utilisation, bound_check: utilisation.map(|_| format!("en1998.2.{}.pier.{}.flexure", b.id, pier.id)), binding: Some(binding) });
    }
    Diagram2d {
        id: format!("pier-moments:{}", b.id),
        title: loc(&format!("Pier base moments \u{2013} {}", b.id), &format!("Pfeilerfu\u{df}momente \u{2013} {}", b.id)),
        description: loc("Base moment M_Ed = F\u{b7}H of the design spectrum analysis against the plastic hinge moment M_Rd of every pier along the deck; drag a marker to change M_Rd.", "Fu\u{df}moment M_Ed = F\u{b7}H der Antwortspektrenanalyse gegen das plastische Gelenkmoment M_Rd jedes Pfeilers l\u{e4}ngs des \u{dc}berbaus; Marker ziehen \u{e4}ndert M_Rd."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m"), axis("Moment", "Moment", QuantityKind::Moment, "N·m")],
        series: vec![
            Series::Scatter { id: "demand".into(), label: loc("Demand M_Ed", "Einwirkung M_Ed"), points: demand.iter().map(|v| ScatterPoint { at: v.clone(), tone: Tone::Warning, label: None }).collect() },
            Series::Scatter { id: "resistance".into(), label: loc("Resistance M_Rd", "Widerstand M_Rd"), points: resistance.iter().map(|v| ScatterPoint { at: v.clone(), tone: Tone::Info, label: None }).collect() },
        ],
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, &b.id, format!("bridges[id={}]", b.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", positions), column("M_Ed", "M_Ed", "N·m", demand_values), column("M_Rd", "M_Rd", "N·m", resistance_values)] },
    }
}

fn wall_pressure(t: &En1998Tank, view: &TankView) -> Diagram2d {
    let z: Vec<f64> = view.profile.iter().map(|r| r.z_m).collect();
    let curve = |pick: fn(&tank::WallPressure) -> f64| -> Vec<Vec2> { view.profile.iter().map(|r| Vec2::new(pick(r), r.z_m)).collect() };
    let columns = vec![
        column("Height z", "H\u{f6}he z", "m", z),
        column("Hydrostatic", "Hydrostatisch", "Pa", view.profile.iter().map(|r| r.hydrostatic_pa).collect()),
        column("Impulsive", "Impulsiv", "Pa", view.profile.iter().map(|r| r.impulsive_pa).collect()),
        column("Convective", "Konvektiv", "Pa", view.profile.iter().map(|r| r.convective_pa).collect()),
        column("Total", "Gesamt", "Pa", view.profile.iter().map(|r| r.total_pa).collect()),
    ];
    Diagram2d {
        id: format!("wall-pressure:{}", t.id),
        title: loc(&format!("Wall pressure \u{2013} {}", t.id), &format!("Wanddruck \u{2013} {}", t.id)),
        description: loc(&format!("Hydrostatic, impulsive and convective wall pressure of the tank over the fluid height (EN 1998-4 Annex A), T_c = {:.2} s; the total adds the hydrostatic pressure to the SRSS of the dynamic parts.", view.convective_period_s), &format!("Hydrostatischer, impulsiver und konvektiver Wanddruck des Tanks \u{fc}ber die F\u{fc}llh\u{f6}he (EN 1998-4 Anhang A), T_c = {:.2} s; die Summe addiert den hydrostatischen Druck zur SRSS der dynamischen Anteile.", view.convective_period_s)),
        axes: vec![axis("Pressure", "Druck", QuantityKind::Pressure, "Pa"), axis("Height z", "H\u{f6}he z", QuantityKind::Length, "m")],
        series: vec![
            Series::Polyline { id: "hydrostatic".into(), label: loc("Hydrostatic", "Hydrostatisch"), points: curve(|r| r.hydrostatic_pa), style: stroke(Tone::Neutral, 1.5, Dash::Dashed) },
            Series::Polyline { id: "impulsive".into(), label: loc("Impulsive", "Impulsiv"), points: curve(|r| r.impulsive_pa), style: stroke(Tone::Info, 2.0, Dash::Solid) },
            Series::Polyline { id: "convective".into(), label: loc("Convective", "Konvektiv"), points: curve(|r| r.convective_pa), style: stroke(Tone::Warning, 2.0, Dash::Solid) },
            Series::Polyline { id: "total".into(), label: loc("Total", "Gesamt"), points: curve(|r| r.total_pa), style: stroke(Tone::Error, 2.5, Dash::Solid) },
            Series::Limit { id: "shell-height".into(), label: loc("Shell height", "Schalenh\u{f6}he"), axis: AxisRef::Y, value: t.height_m, style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
        ],
        markers: fill_level_marker(t, view).into_iter().collect(),
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Shell, &t.id, format!("tanks[id={}]", t.id))),
        legend: true,
        table_twin: DataTable { columns },
    }
}

/// 🏭️ Diagram `silo-demand:<id>`: the axial shell demand N_Ed = m (g + 0.9 a_g) and the base shear V_Ed = S_d m over the ground acceleration a_g, against the shell resistance N_Rd and the anchorage resistance V_Rd, with the design acceleration marked (EN 1998-4, 3.4 and 3.5).
fn silo_demand(silo: &En1998Silo, site: &Site) -> Option<Diagram2d> {
    let mass = part_1::filled_content_seismic_mass_kg(silo.permanent_gk_n, silo.content_qk_n, silo.content_category, silo.filling_ratio);
    if site.a_gr <= 0.0 || !(mass > 0.0) {
        return None;
    }
    let q = part_4::silo_behaviour_factor(silo.q_nominal);
    let axial = |a: f64| part_4::silo_axial_demand_n(mass, a);
    let shear = |a: f64| part_1::base_shear_n(part_1::design_spectrum_at(a, site.gamma_i, site.s, site.tb, site.tc, site.td, q, site.tc), mass, 1.0);
    let a: Vec<f64> = (0..=20).map(|i| 2.0 * site.a_gr * f64::from(i) / 20.0).collect();
    let (n, v): (Vec<f64>, Vec<f64>) = a.iter().map(|x| (axial(*x), shear(*x))).unzip();
    let curve = |values: &[f64]| a.iter().zip(values).map(|(x, y)| Vec2::new(*x, *y)).collect::<Vec<_>>();
    let mut series = vec![
        Series::Polyline { id: "n-ed".into(), label: loc("Axial demand N_Ed", "Normalkraft N_Ed"), points: curve(&n), style: stroke(Tone::Info, 2.0, Dash::Solid) },
        Series::Polyline { id: "v-ed".into(), label: loc("Base shear V_Ed", "Basisquerkraft V_Ed"), points: curve(&v), style: stroke(Tone::Warning, 2.0, Dash::Solid) },
    ];
    for (id, en, de, value, dash) in [("n-rd", "Shell resistance N_Rd", "Wandwiderstand N_Rd", silo.n_rd_n, Dash::Dashed), ("v-rd", "Anchorage resistance V_Rd", "Verankerungswiderstand V_Rd", silo.v_rd_n, Dash::Dotted)] {
        if value > 0.0 {
            series.push(Series::Limit { id: id.into(), label: loc(en, de), axis: AxisRef::Y, value, style: stroke(Tone::Error, 1.0, dash) });
        }
    }
    let marker = |id: &str, value: f64, limit: f64, en: &str, de: &str| Marker { id: format!("reference:{id}"), at: Vec2::new(site.a_gr, value), shape: MarkerShape::Circle, tone: if limit > 0.0 { scene::utilisation_tone(value / limit) } else { Tone::Info }, label: loc(&format!("{en} = {:.0} kN", value / 1e3), &format!("{de} = {:.0} kN", value / 1e3)), utilisation: (limit > 0.0).then(|| value / limit), bound_check: None, binding: None };
    Some(Diagram2d {
        id: format!("silo-demand:{}", silo.id),
        title: loc(&format!("Silo seismic demand \u{2013} {}", silo.id), &format!("Silo-Erdbebeneinwirkung \u{2013} {}", silo.id)),
        description: loc("Axial shell demand and base shear of the filled silo over the ground acceleration against the shell and anchorage resistances; the markers show the design acceleration (EN 1998-4, 3.4 and 3.5).", "Normalkraft der Silowand und Basisquerkraft des gef\u{fc}llten Silos \u{fc}ber der Bodenbeschleunigung gegen die Wand- und Verankerungswiderst\u{e4}nde; die Marken zeigen die Bemessungsbeschleunigung (EN 1998-4, 3.4 und 3.5)."),
        axes: vec![axis("Ground acceleration a_g", "Bodenbeschleunigung a_g", QuantityKind::Acceleration, "m/s²"), axis("Force", "Kraft", QuantityKind::Force, "N")],
        series,
        markers: vec![marker("design-n", axial(site.a_gr), silo.n_rd_n, "N_Ed", "N_Ed"), marker("design-v", shear(site.a_gr), silo.v_rd_n, "V_Ed", "V_Ed")],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Shell, &silo.id, format!("silos[id={}]", silo.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Ground acceleration a_g", "Bodenbeschleunigung a_g", "m/s²", a.clone()), column("Axial demand N_Ed", "Normalkraft N_Ed", "N", n), column("Base shear V_Ed", "Basisquerkraft V_Ed", "N", v)] },
    })
}

fn tower_modes(t: &En1998Tower, view: &TowerView) -> Diagram2d {
    let elevations = &view.modes.elevations_m;
    let mut series = Vec::new();
    let mut columns = vec![column("Elevation z", "H\u{f6}he z", "m", elevations.clone())];
    for (n, mode) in view.modes.modes.iter().enumerate().take(3) {
        let label = format!("{}: T = {:.3} s, M_eff = {:.0} %", n + 1, mode.period_s, mode.mass_ratio * 100.0);
        series.push(Series::Polyline { id: format!("mode-{}", n + 1), label: loc(&label, &label), points: mode.shape.iter().zip(elevations).map(|(phi, z)| Vec2::new(*phi, *z)).collect(), style: stroke(TONES[n % 3], 2.0, Dash::Solid) });
        columns.push(column(&format!("\u{3c6} {}", n + 1), &format!("\u{3c6} {}", n + 1), "1", mode.shape.clone()));
    }
    Diagram2d {
        id: format!("tower-modes:{}", t.id),
        title: loc(&format!("Tower modes \u{2013} {}", t.id), &format!("Turmeigenformen \u{2013} {}", t.id)),
        description: loc(&format!("First modes of the cantilever on the foundation spring k\u{3b8} = {:.3e} N\u{b7}m/rad (0 = fixed base) from the mechanics kernel (EN 1998-6, 4.3), normalised to the tip.", t.base_rotational_stiffness_nm), &format!("Erste Eigenformen des Kragtr\u{e4}gers auf der Gr\u{fc}ndungsfeder k\u{3b8} = {:.3e} N\u{b7}m/rad (0 = Einspannung) aus dem Mechanikkern (EN 1998-6, 4.3), auf die Spitze normiert.", t.base_rotational_stiffness_nm)),
        axes: vec![axis("Mode shape \u{3c6}", "Eigenform \u{3c6}", QuantityKind::Dimensionless, "1"), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m")],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &t.id, format!("towers[id={}]", t.id))),
        legend: true,
        table_twin: DataTable { columns },
    }
}

fn tower_modal_forces(t: &En1998Tower, view: &TowerView) -> Diagram2d {
    let elevations = &view.modes.elevations_m;
    let mut series = Vec::new();
    let mut columns = vec![column("Elevation z", "H\u{f6}he z", "m", elevations.clone())];
    for (n, forces) in view.response.modal_forces_n.iter().enumerate().take(3) {
        let label = format!("{}: V = {:.0} kN, M = {:.0} kN\u{b7}m", n + 1, view.response.modal_shear_n[n] / 1000.0, view.response.modal_moment_nm[n] / 1000.0);
        series.push(Series::Polyline { id: format!("force-{}", n + 1), label: loc(&label, &label), points: forces.iter().zip(elevations).map(|(f, z)| Vec2::new(*f, *z)).collect(), style: stroke(TONES[n % 3], 2.0, Dash::Solid) });
        columns.push(column(&format!("F {}", n + 1), &format!("F {}", n + 1), "N", forces.clone()));
    }
    Diagram2d {
        id: format!("tower-forces:{}", t.id),
        title: loc(&format!("Modal inertia forces \u{2013} {}", t.id), &format!("Modale Tr\u{e4}gheitskr\u{e4}fte \u{2013} {}", t.id)),
        description: loc(&format!("Inertia forces F_i = m_i\u{b7}\u{393}\u{b7}\u{3c6}_i\u{b7}S_d(T) of the first modes at the nodes; CQC combination: V = {:.0} kN, M = {:.0} kN\u{b7}m.", view.response.base_shear_n / 1000.0, view.response.overturning_nm / 1000.0), &format!("Tr\u{e4}gheitskr\u{e4}fte F_i = m_i\u{b7}\u{393}\u{b7}\u{3c6}_i\u{b7}S_d(T) der ersten Eigenformen an den Knoten; CQC-\u{dc}berlagerung: V = {:.0} kN, M = {:.0} kN\u{b7}m.", view.response.base_shear_n / 1000.0, view.response.overturning_nm / 1000.0)),
        axes: vec![axis("Force F", "Kraft F", QuantityKind::Force, "N"), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m")],
        series,
        markers: Vec::new(),
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &t.id, format!("towers[id={}]", t.id))),
        legend: true,
        table_twin: DataTable { columns },
    }
}

fn governing_direction(view: &BridgeView, index: usize) -> (f64, bridge::Direction) {
    let force = |r: &Option<bridge::Response>| r.as_ref().map_or(0.0, |r| r.support_force_n[index].abs());
    let (longitudinal, transverse) = (force(&view.longitudinal), force(&view.transverse));
    if transverse > longitudinal { (transverse, bridge::Direction::Transverse) } else { (longitudinal, bridge::Direction::Longitudinal) }
}

/// 🌉️ Diagram `pier-capacity:<bridge>`: the force-displacement capacity of every pier (elastic cantilever stiffness up to the plastic hinge force `M_Rd / H`, then the hinge plateau) with the shear resistance `V_Rd` as a limit and the design demand point of the governing direction; the yield marker is draggable onto `momentRdNm` (EN 1998-2, 5.3 and 5.6).
fn pier_capacity(d: &En1998Snapshot, report: &CheckReport, b: &En1998Bridge, view: &BridgeView) -> Option<Diagram2d> {
    let (mut series, mut markers) = (Vec::new(), Vec::new());
    let (mut yield_u, mut yield_f, mut demand_u, mut demand_f, mut shear_rd) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for (index, support) in view.supports.iter().enumerate() {
        let Some(pier) = d.piers.iter().find(|pier| pier.id == support.pier_id && pier.height_m > 0.0 && pier.moment_rd_nm > 0.0) else { continue };
        let (force, direction) = governing_direction(view, index);
        let k = if direction == bridge::Direction::Longitudinal { support.pier_longitudinal_n_per_m } else { support.pier_transverse_n_per_m };
        if !(k > 0.0) {
            continue;
        }
        let hinge_force = pier.moment_rd_nm / pier.height_m;
        let (u_y, u_d) = (hinge_force / k, force / k);
        let u_max = (2.0 * u_y).max(1.25 * u_d);
        series.push(Series::Polyline { id: format!("capacity-{}", pier.id), label: loc(&format!("Capacity {}", pier.id), &format!("Kapazit\u{e4}t {}", pier.id)), points: vec![Vec2::new(0.0, 0.0), Vec2::new(u_y, hinge_force), Vec2::new(u_max, hinge_force)], style: stroke(TONES[index % 3], 2.0, Dash::Solid) });
        if pier.shear_rd_n > 0.0 {
            series.push(Series::Limit { id: format!("shear-{}", pier.id), label: loc(&format!("Shear resistance V_Rd {}", pier.id), &format!("Querkraftwiderstand V_Rd {}", pier.id)), axis: AxisRef::Y, value: pier.shear_rd_n, style: stroke(Tone::Error, 1.0, Dash::Dashed) });
        }
        let check = format!("en1998.2.{}.pier.{}.flexure", b.id, pier.id);
        let utilisation = report.checks.iter().find(|c| c.id == check).map(|c| c.utilization).filter(|u| u.is_finite());
        let mut binding = FieldBinding::new(format!("piers[id={}].momentRdNm", pier.id), Affine { base: 0.0, k: pier.height_m }, "N*m");
        binding.quantise = Some(Quantise::Round { step: 1000.0 });
        markers.push(Marker { id: format!("reference:yield-{}", pier.id), at: Vec2::new(u_y, hinge_force), shape: MarkerShape::Diamond, tone: Tone::Info, label: loc(&format!("{}: hinge force M_Rd/H = {:.0} kN at {:.1} mm", pier.id, hinge_force / 1e3, u_y * 1e3), &format!("{}: Gelenkkraft M_Rd/H = {:.0} kN bei {:.1} mm", pier.id, hinge_force / 1e3, u_y * 1e3)), utilisation: None, bound_check: None, binding: Some(binding) });
        markers.push(Marker { id: format!("demand-{}", pier.id), at: Vec2::new(u_d, force), shape: MarkerShape::Circle, tone: utilisation.map_or(Tone::Neutral, scene::utilisation_tone), label: loc(&format!("{}: demand {:.0} kN at {:.1} mm", pier.id, force / 1e3, u_d * 1e3), &format!("{}: Einwirkung {:.0} kN bei {:.1} mm", pier.id, force / 1e3, u_d * 1e3)), utilisation, bound_check: utilisation.map(|_| check), binding: None });
        yield_u.push(u_y);
        yield_f.push(hinge_force);
        demand_u.push(u_d);
        demand_f.push(force);
        shear_rd.push(pier.shear_rd_n);
    }
    if series.is_empty() {
        return None;
    }
    Some(Diagram2d {
        id: format!("pier-capacity:{}", b.id),
        title: loc(&format!("Pier capacity curves \u{2013} {}", b.id), &format!("Pfeilerkapazit\u{e4}tskurven \u{2013} {}", b.id)),
        description: loc("Force-displacement capacity of every pier at the deck: the pier stiffness of the bridge model of the modal analysis (cantilever 3EI/H\u{b3}, in series with the bearing: the support force passes through the pier, so the pier displacement is F/k_pier) up to the plastic hinge force M_Rd/H, then the hinge plateau, against the demand of the governing direction and the shear resistance; drag a yield marker to change M_Rd (EN 1998-2, 5.3 and 5.6).", "Kraft-Verschiebungs-Kapazit\u{e4}t jedes Pfeilers am \u{dc}berbau: die Pfeilersteifigkeit des Br\u{fc}ckenmodells der Modalanalyse (Kragarm 3EI/H\u{b3} in Reihe mit dem Lager: die Auflagerkraft l\u{e4}uft durch den Pfeiler, die Pfeilerverschiebung ist also F/k_Pfeiler) bis zur plastischen Gelenkkraft M_Rd/H, danach das Gelenkplateau, gegen die Einwirkung der ma\u{df}gebenden Richtung und den Querkraftwiderstand; eine Fl\u{e4}chenmarke ziehen \u{e4}ndert M_Rd (EN 1998-2, 5.3 und 5.6)."),
        axes: vec![axis("Pier top displacement", "Pfeilerkopfverschiebung", QuantityKind::Length, "m"), axis("Horizontal force", "Horizontalkraft", QuantityKind::Force, "N")],
        series,
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Plate, &b.id, format!("bridges[id={}]", b.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Hinge displacement", "Gelenkverschiebung", "m", yield_u), column("Hinge force M_Rd/H", "Gelenkkraft M_Rd/H", "N", yield_f), column("Demand displacement", "Einwirkungsverschiebung", "m", demand_u), column("Demand force", "Einwirkungskraft", "N", demand_f), column("Shear resistance V_Rd", "Querkraftwiderstand V_Rd", "N", shear_rd)] },
    })
}

fn fill_level_marker(t: &En1998Tank, view: &TankView) -> Option<Marker> {
    let top = view.profile.iter().max_by(|a, c| a.z_m.total_cmp(&c.z_m))?;
    let binding = (t.height_m > 0.0).then(|| {
        let mut binding = FieldBinding::new(format!("tanks[id={}].fillingRatio", t.id), Affine { base: 0.0, k: 1.0 / t.height_m }, "1");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        binding
    });
    let text = format!("{:.2} m ({:.0} %)", view.fluid_height_m, t.filling_ratio.clamp(0.0, 1.0) * 100.0);
    Some(Marker { id: "reference:fill-level".into(), at: Vec2::new(top.total_pa, view.fluid_height_m), shape: MarkerShape::Diamond, tone: Tone::Info, label: loc(&format!("Fill level {text}"), &format!("F\u{fc}llstand {text}")), utilisation: None, bound_check: None, binding })
}

/// 🏭️ Diagram `silo-fill:<id>`: the axial shell demand and the base shear at the design acceleration over the filling ratio (the seismic mass carries psi_E of the content, EN 1998-4, 3.4) against the shell and anchorage resistances; the markers are draggable onto `fillingRatio`.
fn silo_fill(silo: &En1998Silo, site: &Site) -> Option<Diagram2d> {
    let q = part_4::silo_behaviour_factor(silo.q_nominal);
    let at = |ratio: f64| {
        let mass = part_1::filled_content_seismic_mass_kg(silo.permanent_gk_n, silo.content_qk_n, silo.content_category, ratio);
        let sd = part_1::design_spectrum_at(site.a_gr, site.gamma_i, site.s, site.tb, site.tc, site.td, q, site.tc);
        (part_4::silo_axial_demand_n(mass, site.a_gr), part_1::base_shear_n(sd, mass, 1.0))
    };
    if site.a_gr <= 0.0 || !(at(1.0).0 > 0.0) {
        return None;
    }
    let ratios: Vec<f64> = (0..=20).map(|i| f64::from(i) / 20.0).collect();
    let (n, v): (Vec<f64>, Vec<f64>) = ratios.iter().map(|r| at(*r)).unzip();
    let curve = |values: &[f64]| ratios.iter().zip(values).map(|(x, y)| Vec2::new(*x, *y)).collect::<Vec<_>>();
    let mut series = vec![
        Series::Polyline { id: "n-ed".into(), label: loc("Axial demand N_Ed", "Normalkraft N_Ed"), points: curve(&n), style: stroke(Tone::Info, 2.0, Dash::Solid) },
        Series::Polyline { id: "v-ed".into(), label: loc("Base shear V_Ed", "Basisquerkraft V_Ed"), points: curve(&v), style: stroke(Tone::Warning, 2.0, Dash::Solid) },
    ];
    for (id, en, de, value, dash) in [("n-rd", "Shell resistance N_Rd", "Wandwiderstand N_Rd", silo.n_rd_n, Dash::Dashed), ("v-rd", "Anchorage resistance V_Rd", "Verankerungswiderstand V_Rd", silo.v_rd_n, Dash::Dotted)] {
        if value > 0.0 {
            series.push(Series::Limit { id: id.into(), label: loc(en, de), axis: AxisRef::Y, value, style: stroke(Tone::Error, 1.0, dash) });
        }
    }
    let current = silo.filling_ratio.clamp(0.0, 1.0);
    let (n_now, v_now) = at(current);
    let marker = |id: &str, value: f64, limit: f64, en: &str, de: &str| {
        let mut binding = FieldBinding::new(format!("silos[id={}].fillingRatio", silo.id), Affine::IDENTITY, "1");
        binding.quantise = Some(Quantise::Round { step: 0.01 });
        Marker { id: format!("reference:{id}"), at: Vec2::new(current, value), shape: MarkerShape::Circle, tone: if limit > 0.0 { scene::utilisation_tone(value / limit) } else { Tone::Info }, label: loc(&format!("{en} = {:.0} kN at {:.0} % filling", value / 1e3, current * 100.0), &format!("{de} = {:.0} kN bei {:.0} % F\u{fc}llung", value / 1e3, current * 100.0)), utilisation: None, bound_check: None, binding: Some(binding) }
    };
    Some(Diagram2d {
        id: format!("silo-fill:{}", silo.id),
        title: loc(&format!("Silo filling \u{2013} {}", silo.id), &format!("Silof\u{fc}llung \u{2013} {}", silo.id)),
        description: loc("Axial shell demand and base shear of the silo at the design acceleration over the filling ratio against the shell and anchorage resistances; drag a marker to change the filling ratio (EN 1998-4, 3.4 and 3.5). The wall pressure of the granular content is not modelled.", "Normalkraft der Silowand und Basisquerkraft des Silos bei Bemessungsbeschleunigung \u{fc}ber dem F\u{fc}llgrad gegen die Wand- und Verankerungswiderst\u{e4}nde; eine Marke ziehen \u{e4}ndert den F\u{fc}llgrad (EN 1998-4, 3.4 und 3.5). Der Wanddruck des Sch\u{fc}ttguts ist nicht modelliert."),
        axes: vec![diagram_parts::axis("Filling ratio", "F\u{fc}llgrad", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.0))), axis("Force", "Kraft", QuantityKind::Force, "N")],
        series,
        markers: vec![marker("fill-n", n_now, silo.n_rd_n, "N_Ed", "N_Ed"), marker("fill-v", v_now, silo.v_rd_n, "V_Ed", "V_Ed")],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Shell, &silo.id, format!("silos[id={}]", silo.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Filling ratio", "F\u{fc}llgrad", "1", ratios.clone()), column("Axial demand N_Ed", "Normalkraft N_Ed", "N", n), column("Base shear V_Ed", "Basisquerkraft V_Ed", "N", v)] },
    })
}

/// 🗼️ Diagrams `tower-shear:<id>` and `tower-drift:<id>`: the CQC storey shear per level and the design displacement `q d_e` with the inter-level drift (EN 1998-1, 4.3.3.3 and 4.3.4); the tip marker is draggable onto `heightM`.
fn tower_levels(t: &En1998Tower, view: &TowerView) -> Vec<Diagram2d> {
    let profile = tower::level_profile(&view.modes, &view.response, view.q);
    let z = &view.modes.elevations_m;
    let focus = Some(EntityRef::new(EntityKind::Member, &t.id, format!("towers[id={}]", t.id)));
    let line = |id: &str, en: &str, de: &str, values: &[f64], tone: Tone, from: usize| Series::Polyline { id: id.into(), label: loc(en, de), points: values.iter().zip(z).skip(from).map(|(v, e)| Vec2::new(*v, *e)).collect(), style: stroke(tone, 2.0, Dash::Solid) };
    let shear = Diagram2d {
        id: format!("tower-shear:{}", t.id),
        title: loc(&format!("Tower storey shear \u{2013} {}", t.id), &format!("Turm-Stockwerksquerkraft \u{2013} {}", t.id)),
        description: loc(&format!("CQC-combined shear force over the elevation from the modal inertia forces; base shear V = {:.0} kN.", view.response.base_shear_n / 1000.0), &format!("CQC-kombinierte Querkraft \u{fc}ber der H\u{f6}he aus den modalen Tr\u{e4}gheitskr\u{e4}ften; Fu\u{df}querkraft V = {:.0} kN.", view.response.base_shear_n / 1000.0)),
        axes: vec![axis("Shear force V", "Querkraft V", QuantityKind::Force, "N"), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m")],
        series: vec![line("shear", "Storey shear V(z)", "Querkraft V(z)", &profile.shear_n, Tone::Info, 0)],
        markers: vec![Marker { id: "reference:base-shear".into(), at: Vec2::new(profile.shear_n[0], z[0]), shape: MarkerShape::Diamond, tone: Tone::Info, label: loc(&format!("Base shear {:.0} kN", profile.shear_n[0] / 1000.0), &format!("Fu\u{df}querkraft {:.0} kN", profile.shear_n[0] / 1000.0)), utilisation: None, bound_check: None, binding: None }],
        annotations: Vec::new(),
        focus: focus.clone(),
        legend: true,
        table_twin: DataTable { columns: vec![column("Elevation z", "H\u{f6}he z", "m", z.clone()), column("Shear force V", "Querkraft V", "N", profile.shear_n.clone())] },
    };
    let mut binding = FieldBinding::new(format!("towers[id={}].heightM", t.id), Affine::IDENTITY, "m");
    binding.quantise = Some(Quantise::Round { step: 0.1 });
    let (top_d, top_z) = (*profile.displacement_m.last().unwrap_or(&0.0), *z.last().unwrap_or(&0.0));
    let drift = Diagram2d {
        id: format!("tower-drift:{}", t.id),
        title: loc(&format!("Tower displacement and drift \u{2013} {}", t.id), &format!("Turmverschiebung und -verformung \u{2013} {}", t.id)),
        description: loc(&format!("Design displacement d_s = q\u{b7}d_e (q = {:.2}) and inter-level drift per node over the elevation, CQC combined; drag the tip marker to change the tower height.", view.q), &format!("Bemessungsverschiebung d_s = q\u{b7}d_e (q = {:.2}) und Verformung zwischen den Knoten \u{fc}ber der H\u{f6}he, CQC-kombiniert; die Spitzenmarke ziehen \u{e4}ndert die Turmh\u{f6}he.", view.q)),
        axes: vec![axis("Displacement", "Verschiebung", QuantityKind::Length, "m"), axis("Elevation z", "H\u{f6}he z", QuantityKind::Length, "m")],
        series: vec![line("displacement", "Design displacement d_s", "Bemessungsverschiebung d_s", &profile.displacement_m, Tone::Info, 0), line("drift", "Inter-level drift", "Verformung zwischen Knoten", &profile.drift_m, Tone::Warning, 1)],
        markers: vec![Marker { id: "reference:tip".into(), at: Vec2::new(top_d, top_z), shape: MarkerShape::Diamond, tone: Tone::Info, label: loc(&format!("Tip: d_s = {:.1} mm at {:.1} m", top_d * 1e3, top_z), &format!("Spitze: d_s = {:.1} mm bei {:.1} m", top_d * 1e3, top_z)), utilisation: None, bound_check: None, binding: Some(binding) }],
        annotations: Vec::new(),
        focus,
        legend: true,
        table_twin: DataTable { columns: vec![column("Elevation z", "H\u{f6}he z", "m", z.clone()), column("Design displacement d_s", "Bemessungsverschiebung d_s", "m", profile.displacement_m), column("Inter-level drift", "Verformung zwischen Knoten", "m", profile.drift_m)] },
    };
    vec![shear, drift]
}
