//! 🧷 Diagrams of the information collections: the normative function of each row (footbridge load q_fk(L), rail Φ(L_Φ), hoist Φ₃(Δm/m), machine force F(f), bulk ratio K(φ_i), wheel footprint over the slab thickness, roof-surface μ₁(α)) with the row as a marker on the covering check, the LM3 axle line of the special vehicles against the assumed one, and the placements on the site plan.

use super::*;

struct Probe {
    id: String,
    at: Xy,
    check: Option<String>,
    en: String,
    de: String,
    binding: Option<FieldBinding>,
}

struct Curve {
    id: &'static str,
    en: &'static str,
    de: &'static str,
    points: Vec<Xy>,
    tone: Tone,
}

fn curve(id: &'static str, en: &'static str, de: &'static str, points: Vec<Xy>, tone: Tone) -> Curve {
    Curve { id, en, de, points, tone }
}

fn probe(id: &str, at: Xy, check: String, en: String, binding: Option<FieldBinding>) -> Probe {
    Probe { id: id.into(), at, check: Some(check), de: dec(en.clone()), en, binding }
}

/// 📈 One diagram: the curves of the normative function over the axis and the rows as markers bound to their checks; the table twin lists the first curve's abscissae with every curve.
fn row_curves(report: &CheckReport, id: &str, title: (&str, &str), description: (&str, &str), axes: [Axis; 2], names: [(&str, &str); 2], curves: Vec<Curve>, probes: Vec<Probe>, focus: EntityRef) -> Option<Diagram2d> {
    let first = curves.first()?;
    let mut columns = vec![column(names[0].0, names[0].1, &axes[0].unit, first.points.iter().map(|q| q[0]).collect())];
    columns.extend(curves.iter().map(|c| column(c.en, c.de, &axes[1].unit, c.points.iter().map(|q| q[1]).collect())));
    let markers = probes.into_iter().map(|r| marker(report, &r.id, r.at, r.check, loc(&r.en, &r.de), r.binding)).collect();
    Some(Diagram2d {
        id: id.into(),
        title: loc(title.0, title.1),
        description: loc(description.0, description.1),
        axes: axes.into(),
        series: curves.into_iter().map(|c| line(c.id, c.en, c.de, c.points, c.tone, 2.0, Dash::Solid)).collect(),
        markers,
        annotations: Vec::new(),
        focus: Some(focus),
        legend: true,
        table_twin: DataTable { columns },
    })
}

fn upper(rows: impl Iterator<Item = f64>, floor: f64, factor: f64) -> f64 {
    rows.fold(floor, f64::max) * factor
}

/// 🚶 Footbridge load q_fk(L) with the loaded length of every footbridge (EN 1991-2, 5.3.2.1).
pub(super) fn footbridge_load(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.footbridges.iter().filter(|r| r.loaded_length.is_finite() && r.loaded_length > 0.0).collect();
    let first = *rows.first()?;
    let l_max = upper(rows.iter().map(|r| r.loaded_length), 120.0, 1.25);
    let points = curves::sample(|l| part_2::footbridge_q_fk_pa(l) / 1000.0, 1.0, l_max, SAMPLES);
    let probes = rows.iter().map(|r| probe(&r.id, [r.loaded_length, part_2::footbridge_q_fk_pa(r.loaded_length) / 1000.0], format!("en1991.2.footbridge-load.{}", r.id), format!("{}: L = {:.1} m, q_fk = {:.2} kN/m²", r.id, r.loaded_length, part_2::footbridge_q_fk_pa(r.loaded_length) / 1000.0), Some(bound(format!("footbridges[id={}].loadedLength", r.id), "m", 1.0, 0.5)))).collect();
    row_curves(report, "footbridge:load", ("Footbridge load q_fk", "Last q_fk der Fußgängerbrücke"), ("Uniform load q_fk over the loaded length L with the loaded length of every footbridge as a marker; the marker drags the length (EN 1991-2, 5.3.2.1; registry values unverified).", "Gleichmäßig verteilte Last q_fk über der belasteten Länge L mit der belasteten Länge jeder Fußgängerbrücke als Marke; die Marke verschiebt die Länge (EN 1991-2, 5.3.2.1; Registerwerte ungeprüft)."), [axis("Loaded length L", "Belastete Länge L", QuantityKind::Length, "m", AxisScale::Linear, Some((1.0, l_max))), axis("Uniform load q_fk", "Gleichmäßige Last q_fk", QuantityKind::Pressure, "kN/m²", AxisScale::Linear, None)], [("Loaded length L", "Belastete Länge L"), ("q_fk", "q_fk")], vec![curve("q-fk", "q_fk(L)", "q_fk(L)", points, Tone::Info)], probes, EntityRef::new(EntityKind::Load, &first.id, format!("footbridges[id={}]", first.id)))
}

/// 🚆 Dynamic factors Φ₂ (careful maintenance) and Φ₃ (standard) over the determinant length with every rail span (EN 1991-2, 6.4.5.2).
pub(super) fn rail_phi(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.rail_dynamics.iter().filter(|r| r.determinant_length.is_finite() && r.determinant_length > 0.0 && part_2::rail_phi_for(&r.maintenance, r.determinant_length).is_some()).collect();
    let first = *rows.first()?;
    let l_max = upper(rows.iter().map(|r| r.determinant_length), 40.0, 1.5);
    let sample = |maintenance: &'static str| curves::sample(|l| part_2::rail_phi_for(maintenance, l).unwrap_or(f64::NAN), 1.0, l_max, SAMPLES);
    let probes = rows.iter().filter_map(|r| part_2::rail_phi_for(&r.maintenance, r.determinant_length).map(|phi| probe(&r.id, [r.determinant_length, phi], format!("en1991.2.rail-dynamic.{}", r.id), format!("{}: L_Φ = {:.1} m, Φ = {phi:.3} ({})", r.id, r.determinant_length, r.maintenance), Some(bound(format!("railDynamics[id={}].determinantLength", r.id), "m", 1.0, 0.5))))).collect();
    row_curves(report, "rail:dynamic-factor", ("Rail dynamic factor Φ", "Dynamischer Beiwert Φ der Schiene"), ("Φ₂ (carefully maintained track) and Φ₃ (standard maintenance) over the determinant length L_Φ with every span as a marker; the marker drags the length (EN 1991-2, 6.4.5.2).", "Φ₂ (sorgfältig unterhaltenes Gleis) und Φ₃ (Regelunterhaltung) über der maßgebenden Länge L_Φ mit jedem Feld als Marke; die Marke verschiebt die Länge (EN 1991-2, 6.4.5.2)."), [axis("Determinant length L_Φ", "Maßgebende Länge L_Φ", QuantityKind::Length, "m", AxisScale::Linear, Some((1.0, l_max))), axis("Dynamic factor Φ", "Dynamischer Beiwert Φ", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)], [("Determinant length L_Φ", "Maßgebende Länge L_Φ"), ("Φ", "Φ")], vec![curve("phi-careful", "Φ₂ careful", "Φ₂ sorgfältig", sample("careful"), Tone::Info), curve("phi-standard", "Φ₃ standard", "Φ₃ Regel", sample("standard"), Tone::Warning)], probes, EntityRef::new(EntityKind::Load, &first.id, format!("railDynamics[id={}]", first.id)))
}

/// 🏗️ Release factor Φ₃ = 1 − Δm/m (1 + β₃) of a grab and a magnet over the released fraction with every hoist (EN 1991-3, 2.5).
pub(super) fn hoist_release(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.crane_hoists.iter().filter_map(|r| part_3::beta_3(&r.release_device).filter(|_| (0.0..=1.0).contains(&r.release_fraction)).map(|b| (r, b))).collect();
    let (first, _) = *rows.first()?;
    let sample = |device: &str| part_3::beta_3(device).map(|b| curves::sample(|f| part_3::phi_3(f, b), 0.0, 1.0, SAMPLES));
    let probes = rows.iter().map(|(r, b)| probe(&r.id, [r.release_fraction, part_3::phi_3(r.release_fraction, *b)], format!("en1991.3.hoist-release.{}", r.id), format!("{}: Δm/m = {:.2}, Φ₃ = {:.3} ({})", r.id, r.release_fraction, part_3::phi_3(r.release_fraction, *b), r.release_device), Some(bound(format!("craneHoists[id={}].releaseFraction", r.id), "1", 1.0, 0.01)))).collect();
    row_curves(report, "hoist:release-factor", ("Hoist release factor Φ₃", "Abwurffaktor Φ₃ des Hubwerks"), ("Φ₃ = 1 − Δm/m (1 + β₃) over the released payload fraction for a grab and a magnet with every hoist as a marker; the marker drags the fraction (EN 1991-3, 2.5).", "Φ₃ = 1 − Δm/m (1 + β₃) über dem abgeworfenen Lastanteil für Greifer und Magnet mit jedem Hubwerk als Marke; die Marke verschiebt den Anteil (EN 1991-3, 2.5)."), [axis("Released fraction Δm/m", "Abgeworfener Anteil Δm/m", QuantityKind::Dimensionless, "1", AxisScale::Linear, Some((0.0, 1.0))), axis("Release factor Φ₃", "Abwurffaktor Φ₃", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)], [("Released fraction Δm/m", "Abgeworfener Anteil Δm/m"), ("Φ₃", "Φ₃")], vec![curve("phi3-grab", "Φ₃ grab", "Φ₃ Greifer", sample("grab")?, Tone::Info), curve("phi3-magnet", "Φ₃ magnet", "Φ₃ Magnet", sample("magnet")?, Tone::Warning)], probes, EntityRef::new(EntityKind::Load, &first.id, format!("craneHoists[id={}]", first.id)))
}

/// 🌀 Equivalent static force F = m e (2π f)² of every machine over the operating frequency (EN 1991-3, 5).
pub(super) fn machine_force(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.rotating_machines.iter().filter(|r| r.mass.is_finite() && r.mass > 0.0 && r.eccentricity.is_finite() && r.eccentricity >= 0.0 && r.frequency.is_finite() && r.frequency > 0.0).collect();
    let first = *rows.first()?;
    let f_max = upper(rows.iter().map(|r| r.frequency), 10.0, 1.5);
    let points = curves::sample(|f| part_3::machine_dynamic_force_n(first.mass, first.eccentricity, f) / 1000.0, 0.1, f_max, SAMPLES);
    let probes = rows.iter().map(|r| { let kn = part_3::machine_dynamic_force_n(r.mass, r.eccentricity, r.frequency) / 1000.0; probe(&r.id, [r.frequency, kn], format!("en1991.3.machine-force.{}", r.id), format!("{}: f = {:.2} Hz, F = {kn:.2} kN", r.id, r.frequency), Some(bound(format!("rotatingMachines[id={}].frequency", r.id), "Hz", 1.0, 0.05))) }).collect();
    row_curves(report, "machine:force", ("Unbalance force of the machines", "Unwuchtkraft der Maschinen"), ("F = m e ω² of the first machine over the operating frequency with the force of every machine at its frequency as a marker; the marker drags the frequency (EN 1991-3, 5; formula unverified).", "F = m e ω² der ersten Maschine über der Betriebsfrequenz mit der Kraft jeder Maschine bei ihrer Frequenz als Marke; die Marke verschiebt die Frequenz (EN 1991-3, 5; Formel ungeprüft)."), [axis("Frequency f", "Frequenz f", QuantityKind::Dimensionless, "Hz", AxisScale::Linear, Some((0.1, f_max))), axis("Force F", "Kraft F", QuantityKind::Force, "kN", AxisScale::Linear, None)], [("Frequency f", "Frequenz f"), ("Force F", "Kraft F")], vec![curve("force", "F(f) of the first machine", "F(f) der ersten Maschine", points, Tone::Info)], probes, EntityRef::new(EntityKind::Load, &first.id, format!("rotatingMachines[id={}]", first.id)))
}

/// 🌾 Lateral pressure ratios K and K_m over the angle of internal friction with every bulk solid (EN 1991-4, Annex E).
pub(super) fn bulk_ratio(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.bulk_solids.iter().filter(|r| r.internal_friction.is_finite() && r.internal_friction > 0.0 && r.internal_friction < std::f64::consts::FRAC_PI_2).collect();
    let first = *rows.first()?;
    let k = curves::sample(|deg| 1.0 - deg.to_radians().sin(), 1.0, 89.0, SAMPLES);
    let k_mean = curves::sample(|deg| part_4::k_mean(deg.to_radians()), 1.0, 89.0, SAMPLES);
    let probes = rows.iter().map(|r| { let deg = r.internal_friction.to_degrees(); let km = part_4::k_mean(r.internal_friction); probe(&r.id, [deg, km], format!("en1991.4.bulk-k-mean.{}", r.id), format!("{}: φ_i = {deg:.1}°, K_m = {km:.3}", r.id), Some(bound(format!("bulkSolids[id={}].internalFriction", r.id), "rad", 0.017453292519943295, 0.0017453292519943296))) }).collect();
    row_curves(report, "bulk:pressure-ratio", ("Bulk solid pressure ratio", "Horizontallastverhältnis des Schüttguts"), ("K = 1 − sin φ_i and K_m over the angle of internal friction with every bulk solid as a marker; the marker drags the angle (EN 1991-4, Annex E; factor unverified).", "K = 1 − sin φ_i und K_m über dem inneren Reibungswinkel mit jedem Schüttgut als Marke; die Marke verschiebt den Winkel (EN 1991-4, Anhang E; Faktor ungeprüft)."), [axis("Angle of internal friction φ_i", "Innerer Reibungswinkel φ_i", QuantityKind::Angle, "°", AxisScale::Linear, Some((1.0, 89.0))), axis("Pressure ratio K", "Horizontallastverhältnis K", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)], [("Angle φ_i", "Winkel φ_i"), ("K", "K")], vec![curve("k", "K", "K", k, Tone::Info), curve("k-mean", "K_m", "K_m", k_mean, Tone::Warning)], probes, EntityRef::new(EntityKind::Solid, &first.id, format!("bulkSolids[id={}]", first.id)))
}

/// 🛞 Dispersed wheel footprint over the slab thickness of the first dispersal row with the footprint of every row (EN 1991-2, 4.3.6).
pub(super) fn dispersal_footprint(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let rows: Vec<_> = d.load_dispersals.iter().filter(|r| [r.wheel_length, r.wheel_width, r.pavement_thickness, r.slab_thickness].iter().all(|v| v.is_finite() && *v >= 0.0) && r.wheel_length > 0.0 && r.wheel_width > 0.0).collect();
    let first = *rows.first()?;
    let t_max = upper(rows.iter().map(|r| r.slab_thickness), 0.5, 2.0);
    let area = |r: &super::super::model::LoadDispersal, slab: f64| part_2::dispersed_side_m(r.wheel_length, r.pavement_thickness, slab) * part_2::dispersed_side_m(r.wheel_width, r.pavement_thickness, slab);
    let points = curves::sample(|slab| area(first, slab), 0.0, t_max, SAMPLES);
    let probes = rows.iter().map(|r| probe(&r.id, [r.slab_thickness, area(r, r.slab_thickness)], format!("en1991.2.dispersal-footprint.{}", r.id), format!("{}: t = {:.2} m, A = {:.3} m²", r.id, r.slab_thickness, area(r, r.slab_thickness)), Some(bound(format!("loadDispersals[id={}].slabThickness", r.id), "m", 1.0, 0.01)))).collect();
    row_curves(report, "dispersal:footprint", ("Dispersed wheel footprint", "Ausgebreitete Radaufstandsfläche"), ("Footprint of the first wheel at the slab mid-plane over the slab thickness (45° dispersal through pavement and half the slab) with the footprint of every row as a marker; the marker drags the thickness (EN 1991-2, 4.3.6).", "Aufstandsfläche des ersten Rades in der Plattenmittelebene über der Plattendicke (Ausbreitung durch Belag und halbe Platte) mit der Fläche jeder Zeile als Marke; die Marke verschiebt die Dicke (EN 1991-2, 4.3.6)."), [axis("Slab thickness", "Plattendicke", QuantityKind::Length, "m", AxisScale::Linear, Some((0.0, t_max))), axis("Footprint A", "Aufstandsfläche A", QuantityKind::Area, "m²", AxisScale::Linear, None)], [("Slab thickness", "Plattendicke"), ("Footprint A", "Aufstandsfläche A")], vec![curve("footprint", "A(t) of the first wheel", "A(t) des ersten Rades", points, Tone::Info)], probes, EntityRef::new(EntityKind::Load, &first.id, format!("loadDispersals[id={}]", first.id)))
}

/// ❄️ Snow shape coefficient μ₁ over the pitch for every roof shape in use with every roof surface (EN 1991-1-3, 5.3 / Table 5.2).
pub(super) fn roof_surface_mu(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let shapes = ["flat", "monopitch", "duopitch", "hipped"];
    let rows: Vec<_> = d.roof_surfaces.iter().filter(|r| shapes.contains(&r.shape.trim().to_ascii_lowercase().as_str()) && r.pitch_deg.is_finite()).collect();
    let first = *rows.first()?;
    let mu = |shape: &str, pitch: f64| part_1_3::shape_mu(shape, pitch, false, 0.0, 0.0, false, 0.0, d.annex);
    let sample = |shape: &'static str| curves::sample(|pitch| mu(shape, pitch), 0.0, 60.0, SAMPLES);
    let used = |shape: &str| rows.iter().any(|r| r.shape.trim().eq_ignore_ascii_case(shape));
    let mut lines = Vec::new();
    for (shape, en, de, tone) in [("monopitch", "μ₁ monopitch", "μ₁ Pultdach", Tone::Info), ("duopitch", "μ₁ duopitch", "μ₁ Satteldach", Tone::Warning), ("flat", "μ₁ flat", "μ₁ Flachdach", Tone::Neutral), ("hipped", "μ₁ hipped", "μ₁ Walmdach", Tone::Info)] {
        if used(shape) { lines.push(curve(shape, en, de, sample(shape), tone)); }
    }
    let probes = rows.iter().map(|r| { let m = mu(&r.shape.trim().to_ascii_lowercase(), r.pitch_deg); probe(&r.id, [r.pitch_deg, m], format!("en1991.1-3.roof-surface-mu.{}", r.id), format!("{}: α = {:.1}°, μ₁ = {m:.3} ({})", r.id, r.pitch_deg, r.shape), Some(bound(format!("roofSurfaces[id={}].pitchDeg", r.id), "deg", 1.0, 0.5))) }).collect();
    row_curves(report, "roof:surface-mu", ("Snow shape coefficient of the roof surfaces", "Schneelast-Formbeiwert der Dachflächen"), ("μ₁ over the roof pitch for the shapes in use with every roof surface as a marker; the marker drags the pitch (EN 1991-1-3, 5.3 / Table 5.2).", "μ₁ über der Dachneigung für die verwendeten Dachformen mit jeder Dachfläche als Marke; die Marke verschiebt die Neigung (EN 1991-1-3, 5.3 / Tabelle 5.2)."), [axis("Pitch α", "Dachneigung α", QuantityKind::Angle, "°", AxisScale::Linear, Some((0.0, 60.0))), axis("Shape coefficient μ₁", "Formbeiwert μ₁", QuantityKind::Dimensionless, "1", AxisScale::Linear, None)], [("Pitch α", "Dachneigung α"), ("μ₁", "μ₁")], lines, probes, EntityRef::new(EntityKind::Surface, &first.id, format!("roofSurfaces[id={}]", first.id)))
}

/// 🚛 LM3 special vehicles: the axle line of every designation against the assumed LM3 axle line (EN 1991-2, 4.3.4 and Annex A).
pub(super) fn special_vehicle_axles(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let pairs: Vec<Pair> = d
        .special_vehicles
        .iter()
        .filter_map(|v| part_2::special_vehicle(&v.designation).map(|(_, axle, _)| Pair { id: v.id.clone(), name: format!("{} ({})", v.id, v.designation), required: axle / 1000.0, assumed: d.assumed_bridge_lm3 / 1000.0, check: format!("en1991.2.lm3-vehicle.{}", v.id), path: "assumedBridgeLm3".into(), unit: "N", k: 1000.0, step: 1000.0 }))
        .collect();
    let first = d.special_vehicles.first()?;
    compare(report, "special:vehicle-axles", loc("LM3 special vehicle axle lines", "LM3-Sonderfahrzeug-Achslinien"), loc("Axle line of every special vehicle designation against the assumed LM3 axle line of the document; the marker drags the assumed line (EN 1991-2, 4.3.4 and Annex A).", "Achslinie jeder Sonderfahrzeug-Bezeichnung gegen die angenommene LM3-Achslinie des Dokuments; die Marke verschiebt die angenommene Linie (EN 1991-2, 4.3.4 und Anhang A)."), [row_axis("Special vehicle", "Sonderfahrzeug", pairs.len()), axis("Axle line", "Achslinie", QuantityKind::Force, "kN", AxisScale::Linear, None)], &pairs, EntityRef::new(EntityKind::Load, &first.id, format!("specialVehicles[id={}]", first.id)))
}

/// 📍 Placements on the site plan: the position of every placed entity in the plan frame (x east, y north).
pub(super) fn placement_plan(d: &En1991Snapshot, report: &CheckReport) -> Option<Diagram2d> {
    let first = d.placements.first()?;
    let (xs, ys): (Vec<f64>, Vec<f64>) = d.placements.iter().map(|p| (p.x, p.y)).unzip();
    let markers = d.placements.iter().map(|p| marker(report, &format!("reference:placement-{}", p.id), [p.x, p.y], None, loc(&format!("{}: x = {:.2} m, y = {:.2} m, yaw {:.0}°", p.id, p.x, p.y, p.yaw), &dec(format!("{}: x = {:.2} m, y = {:.2} m, Gieren {:.0}°", p.id, p.x, p.y, p.yaw))), Some(bound(format!("placements[id={}].x", p.id), "m", 1.0, 0.1)))).collect();
    Some(Diagram2d {
        id: "placement:plan".into(),
        title: loc("Placements on the site plan", "Platzierungen im Lageplan"),
        description: loc("Position of every placed entity in the plan frame; the marker drags the x position.", "Lage jeder platzierten Einheit im Lageplan; die Marke verschiebt die x-Lage."),
        axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear, None), axis("Position y", "Lage y", QuantityKind::Length, "m", AxisScale::Linear, None)],
        series: Vec::new(),
        markers,
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Group, &first.id, format!("placements[id={}]", first.id))),
        legend: false,
        table_twin: DataTable { columns: vec![column("Position x", "Lage x", "m", xs), column("Position y", "Lage y", "m", ys)] },
    })
}
