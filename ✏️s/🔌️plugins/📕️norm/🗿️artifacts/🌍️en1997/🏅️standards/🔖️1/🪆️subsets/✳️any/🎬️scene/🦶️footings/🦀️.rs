//! 🦶️ Footings: footing body with pedestal, effective area on the base, load resultant and action glyphs, bearing failure mechanism, sliding plane and dimensions.

use super::kit::*;
use super::mechanics as mech;
use super::scene::*;
use super::Ctx;

const PEDESTAL: f64 = 0.4;
const PEDESTAL_ABOVE_GROUND: f64 = 0.3;
const PLATE_THICKNESS: f64 = 0.02;
const MECHANISM_FROM: f64 = 0.7;

fn rect(w: f64, l: f64) -> Polygon2 {
    polygon2(&[[-w / 2.0, -l / 2.0], [w / 2.0, -l / 2.0], [w / 2.0, l / 2.0], [-w / 2.0, l / 2.0]])
}

fn load(out: &mut Out, id: String, name: LocalizedCopy, geometry: LoadGlyph, handles_at: &Vec3) {
    out.push(Spec::new(EntityKind::Load, id, "footings", Geometry::Load(geometry), MaterialRole::Load { kind: LoadKind::Permanent }, name, LAYER_LOADS), std::slice::from_ref(handles_at));
}

pub fn project(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for f in &d.footings {
        if !out.own("footings", &f.id) {
            continue;
        }
        if f.width <= 1e-6 || f.length <= 1e-6 {
            continue;
        }
        let pose = layout.pose(&f.id);
        let base = -f.embedment;
        let t = mech::footing_thickness(f);
        let overall = ctx.binding(&f.id, &[]);
        let case = mech::governing_case(f);
        let actions = case.map(mech::actions);
        let e_b = actions.as_ref().map_or(0.0, mech::ecc);
        let (cx, b_eff) = mech::effective_strip(f.width, e_b);
        let origin = pose.at(0.0, 0.0, base);
        out.push(
            Spec::new(
                EntityKind::Solid,
                f.id.clone(),
                "footings",
                Geometry::Solid(SolidGeometry {
                    shape: SolidShape::FootingBody { frame: pose.upright([0.0, 0.0, base]), footprint: rect(f.width, f.length), thickness: t, pedestal: Some(Vec2::new(PEDESTAL, PEDESTAL)), pedestal_height: (f.embedment - t).max(0.0) + PEDESTAL_ABOVE_GROUND },
                }),
                MaterialRole::Concrete,
                loc(&format!("Footing {}", f.id), &format!("Fundament {}", f.id)),
                LAYER_FOOTINGS,
            )
            .bound(overall.clone())
            .detail(format!("{:.2} x {:.2} m, embedment {:.2} m", f.width, f.length, f.embedment), format!("{:.2} x {:.2} m, Einbindetiefe {:.2} m", f.width, f.length, f.embedment))
            .handles(ctx.handles_of(&f.id)),
            &[origin.clone(), pose.at(f.width / 2.0, f.length / 2.0, base), pose.at(-f.width / 2.0, -f.length / 2.0, base + t)],
        );
        let entity = entity_ref(EntityKind::Solid, &f.id, "footings");
        let top = pose.at(0.0, 0.0, PEDESTAL_ABOVE_GROUND);
        if let Some(clause) = overall.as_ref().and_then(|b| clause_of(report, &b.check_id)) {
            out.annotate(Annotation::Clause { id: format!("{}:clause", f.id), anchor: entity.clone(), clause }, &top);
        }
        let ecc_binding = ctx.binding(&f.id, &["en1997.6.5.4"]).or_else(|| overall.clone());
        let plate_origin = pose.at(cx, 0.0, base - PLATE_THICKNESS);
        out.push(
            Spec::new(
                EntityKind::Plate,
                format!("{}:effective-area", f.id),
                "footings",
                Geometry::Plate(PlateGeometry { frame: pose.upright([cx, 0.0, base - PLATE_THICKNESS]), outline: rect(b_eff, f.length), thickness: PLATE_THICKNESS, offset: 0.0, stiffeners: Vec::new() }),
                MaterialRole::Neutral,
                loc(&format!("Effective area of {}", f.id), &format!("Wirksame Fläche von {}", f.id)),
                LAYER_FOOTINGS,
            )
            .bound(ecc_binding)
            .detail(format!("B' = {:.2} m, e_B = {:.3} m", b_eff, e_b), format!("B' = {:.2} m, e_B = {:.3} m", b_eff, e_b))
            .opacity(0.8),
            &[plate_origin],
        );
        contact_pressure(ctx, f, out);
        pressure_bulb(ctx, f, actions.as_ref().map_or(0.0, |a| mech::base_pressure(f, a)), out);
        investigation_ghost(ctx, f, out);
        if let Some(a) = actions.filter(|a| a.v > 1.0) {
            let at = pose.at(cx, 0.0, base);
            load(out, format!("{}:resultant", f.id), loc(&format!("Load resultant of {}", f.id), &format!("Lastresultierende von {}", f.id)), LoadGlyph::Point { at: at.clone(), direction: Vec3::new(0.0, 0.0, -1.0), magnitude: force(a.v), case: load_case(case.map_or("", |c| c.id.as_str()), LoadKind::Permanent) }, &at);
            let lc = case.map_or("", |c| c.id.as_str());
            let dir_x = pose.dir(&Vec3::X);
            let dir_y = pose.dir(&Vec3::Y);
            if a.h.abs() > 1.0 {
                load(out, format!("{}:load-h", f.id), loc(&format!("Horizontal load on {}", f.id), &format!("Horizontallast auf {}", f.id)), LoadGlyph::Point { at: top.clone(), direction: mul(&dir_x, a.h.signum()), magnitude: force(a.h.abs()), case: load_case(lc, LoadKind::Variable) }, &top);
            }
            if a.m.abs() > 1.0 {
                load(out, format!("{}:load-m", f.id), loc(&format!("Moment on {}", f.id), &format!("Moment auf {}", f.id)), LoadGlyph::Moment { at: top.clone(), axis: dir_y, magnitude: Quantity::new(QuantityKind::Moment, a.m.abs()), case: load_case(lc, LoadKind::Variable) }, &top);
            }
        }
        if let Some(lc) = case {
            for (tag, value, kind, en, de, shift) in [("g", lc.vertical_permanent, LoadKind::Permanent, "permanent", "ständig", -0.2), ("q", lc.vertical_variable, LoadKind::Variable, "variable", "veränderlich", 0.2)] {
                if value > 1.0 {
                    let at = pose.at(0.0, shift, PEDESTAL_ABOVE_GROUND);
                    load(out, format!("{}:load-v-{tag}", f.id), loc(&format!("Vertical {en} load on {}", f.id), &format!("Vertikale {de} Last auf {}", f.id)), LoadGlyph::Point { at: at.clone(), direction: Vec3::new(0.0, 0.0, -1.0), magnitude: force(value), case: load_case(&lc.id, kind) }, &at);
                }
            }
        }
        mechanism(ctx, f, b_eff, cx, out);
        sliding(ctx, f, out);
        dimensions(ctx, f, b_eff, out);
    }
}

fn mechanism(ctx: &Ctx, f: &super::model::SpreadFoundation, b_eff: f64, cx: f64, out: &mut Out) {
    let (d, report) = (ctx.input.document, ctx.input.report);
    let pose = ctx.layout.pose(&f.id);
    let phi = mech::layer_at(&d.layers, f.embedment).map_or(30.0, |l| l.phi_prime_deg);
    let m = mech::mechanism(b_eff, phi);
    let binding = ctx.binding(&f.id, &["en1997.6.5.bearing"]);
    let strong = binding.as_ref().is_some_and(|b| b.ratio >= MECHANISM_FROM);
    let at = |p: &[f64; 2]| pose.at(cx + p[0], 0.0, -f.embedment + p[1]);
    let parts: [(&str, &str, &str, &Vec<[f64; 2]>); 5] = [
        ("wedge-active", "Active wedge", "Aktiver Keil", &m.active),
        ("fan-right", "Radial shear zone (right)", "Radiale Scherzone (rechts)", &m.fans[0]),
        ("fan-left", "Radial shear zone (left)", "Radiale Scherzone (links)", &m.fans[1]),
        ("passive-right", "Passive wedge (right)", "Passiver Keil (rechts)", &m.passive[0]),
        ("passive-left", "Passive wedge (left)", "Passiver Keil (links)", &m.passive[1]),
    ];
    for (tag, en, de, points) in parts {
        let ring: Vec<Vec3> = points.iter().map(at).collect();
        out.push(
            Spec::new(
                EntityKind::Surface,
                format!("{}:{tag}", f.id),
                "footings",
                Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&ring) } }),
                soil("mechanism"),
                loc(&format!("{en} of {}", f.id), &format!("{de} von {}", f.id)),
                LAYER_MECHANISMS,
            )
            .bound(binding.clone())
            .opacity(if strong { 0.55 } else { 0.25 })
            .detail(format!("phi' {phi:.1} deg, depth {:.2} m", m.depth), format!("phi' {phi:.1} Grad, Tiefe {:.2} m", m.depth)),
            &ring,
        );
    }
    if let Some(b) = binding.filter(|b| matches!(b.status, scene::CheckStatus::Fail | scene::CheckStatus::Warning)) {
        let anchor = at(&[0.0, -m.depth / 2.0]);
        out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-bearing", f.id), at: entity_ref(EntityKind::Solid, &f.id, "footings"), mode: FailureMode::Other, extent: None }, &anchor);
        out.annotate(Annotation::Label { id: format!("{}:failure-label", f.id), anchor: anchor.clone(), text: loc(&format!("Bearing resistance utilisation {:.2}", b.ratio), &format!("Grundbruch-Ausnutzung {:.2}", b.ratio)), tone: if b.status == scene::CheckStatus::Fail { Tone::Error } else { Tone::Warning } }, &anchor);
    }
}

fn sliding(ctx: &Ctx, f: &super::model::SpreadFoundation, out: &mut Out) {
    let pose = ctx.layout.pose(&f.id);
    let Some(b) = ctx.binding(&f.id, &["en1997.6.5.3.sliding"]) else { return };
    let z = -f.embedment - 0.01;
    let ring = [pose.at(-f.width / 2.0, -f.length / 2.0, z), pose.at(f.width / 2.0, -f.length / 2.0, z), pose.at(f.width / 2.0, f.length / 2.0, z), pose.at(-f.width / 2.0, f.length / 2.0, z)];
    out.push(
        Spec::new(
            EntityKind::Surface,
            format!("{}:sliding-plane", f.id),
            "footings",
            Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&ring) } }),
            soil("mechanism"),
            loc(&format!("Sliding plane of {}", f.id), &format!("Gleitfuge von {}", f.id)),
            LAYER_MECHANISMS,
        )
        .bound(Some(b))
        .opacity(0.3)
        .hatched(Hatch::Horizontal),
        &ring,
    );
}

fn dimensions(ctx: &Ctx, f: &super::model::SpreadFoundation, b_eff: f64, out: &mut Out) {
    let pose = ctx.layout.pose(&f.id);
    let z = -f.embedment;
    let dims = [
        ("width", pose.at(-f.width / 2.0, -f.length / 2.0 - 0.5, z), pose.at(f.width / 2.0, -f.length / 2.0 - 0.5, z), f.width, Some("width")),
        ("length", pose.at(f.width / 2.0 + 0.5, -f.length / 2.0, z), pose.at(f.width / 2.0 + 0.5, f.length / 2.0, z), f.length, Some("length")),
        ("embedment", pose.at(f.width / 2.0 + 0.5, f.length / 2.0 + 0.5, 0.0), pose.at(f.width / 2.0 + 0.5, f.length / 2.0 + 0.5, z), f.embedment, Some("embedment")),
        ("effective-width", pose.at(-b_eff / 2.0, -f.length / 2.0 - 1.0, z), pose.at(b_eff / 2.0, -f.length / 2.0 - 1.0, z), b_eff, None),
    ];
    for (tag, from, to, value, field) in dims {
        if from.distance(&to) < 1e-9 {
            continue;
        }
        out.annotate(
            Annotation::Dimension {
                id: format!("{}:dim-{tag}", f.id),
                from: from.clone(),
                to,
                offset: Vec3::ZERO,
                value: Quantity::new(QuantityKind::Length, value),
                binding: field.map(|n| FieldBinding::new(format!("footings[id={}].{n}", f.id), Affine::IDENTITY, "m")),
            },
            &from,
        );
    }
}

/// 🌡️ Contact pressure under the footing base as a utilisation field on a grid, sampled from the `soil-pressure` stations of the governing bearing check.
fn contact_pressure(ctx: &Ctx, f: &super::model::SpreadFoundation, out: &mut Out) {
    let prefix = format!("en1997.6.5.bearing.{}.", f.id);
    let Some(check) = ctx.input.report.checks.iter().filter(|c| c.id.starts_with(&prefix) && c.station_field == "soil-pressure" && !c.stations.is_empty()).max_by(|a, b| a.utilization.total_cmp(&b.utilization)) else { return };
    let pose = ctx.layout.pose(&f.id);
    let (columns, rows) = (17_u32, 5_u32);
    let mut local = Vec::new();
    for j in 0..rows {
        for i in 0..columns {
            local.push([-f.width / 2.0 + f.width * f64::from(i) / f64::from(columns - 1), -f.length / 2.0 + f.length * f64::from(j) / f64::from(rows - 1), 0.0]);
        }
    }
    let values = projection_kit::field_at_points(check, &local);
    let points: Vec<Vec3> = local.iter().map(|p| pose.at(p[0], p[1], -f.embedment - 0.03)).collect();
    out.push(
        Spec::new(EntityKind::Surface, format!("{}:contact-pressure", f.id), "footings", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows, columns, points: points.clone() } } }), soil("mechanism"), loc(&format!("Contact pressure under {}", f.id), &format!("Sohldruck unter {}", f.id)), LAYER_FOOTINGS)
            .bound(Some(projection_kit::binding_of_check(check)))
            .channel(ColourChannel::Fields { active: "contact-pressure".into(), fields: vec![ScalarField { id: "contact-pressure".into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } }] })
            .opacity(0.9)
            .detail(format!("peak utilisation {:.2} at the loaded edge", check.utilization), format!("Spitzenausnutzung {:.2} am belasteten Rand", check.utilization)),
        &points,
    );
    if !out.legends.iter().any(|l| l.id == "contact-pressure") {
        out.legends.push(Legend { id: "contact-pressure".into(), title: loc("Contact pressure utilisation", "Sohldruck-Ausnutzung"), scale: LegendScale::Scalar { field_id: "contact-pressure".into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: 1.2 }, unit: QuantityKind::Dimensionless } });
    }
}

const BULB_LEVELS: [(f64, &str, f64); 3] = [(0.5, "50", 0.30), (0.2, "20", 0.22), (0.1, "10", 0.14)];
const BULB_SEGMENTS: usize = 24;
const BULB_RINGS: usize = 10;

/// 🧅 Pressure bulb under the footing: the iso-stress surfaces σ_z = 0.5, 0.2 and 0.1 of the base pressure from the Boussinesq stress of the loaded rectangle (corner method), closed meshes below the base with a label for the depth of the 0.2 q isobar (the significant depth of the stress-strain settlement).
fn pressure_bulb(ctx: &Ctx, f: &super::model::SpreadFoundation, q: f64, out: &mut Out) {
    if q <= 1.0 {
        return;
    }
    let pose = ctx.layout.pose(&f.id);
    let base = -f.embedment;
    for (ratio, tag, opacity) in BULB_LEVELS {
        let (local, triangles) = mech::bulb_mesh(q, f.width, f.length, ratio, BULB_SEGMENTS, BULB_RINGS);
        let vertices: Vec<Vec3> = local.iter().map(|p| pose.at(p[0], p[1], base + p[2])).collect();
        let depth = -local[0][2];
        out.push(
            Spec::new(
                EntityKind::Solid,
                format!("{}:bulb-{tag}", f.id),
                "footings",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Mesh { vertices: vertices.clone(), triangles: triangles.iter().map(|t| Triangle { a: t[0], b: t[1], c: t[2] }).collect() } }),
                soil("mechanism"),
                loc(&format!("Pressure bulb {ratio} q of {}", f.id), &format!("Druckzwiebel {ratio} q von {}", f.id)),
                LAYER_MECHANISMS,
            )
            .channel(ColourChannel::Role)
            .opacity(opacity)
            .detail(format!("sigma_z = {ratio} q reaches {depth:.2} m below the base"), format!("sigma_z = {ratio} q reicht {depth:.2} m unter die Sohle")),
            &vertices,
        );
        if tag == "20" {
            let anchor = pose.at(0.0, 0.0, base - depth);
            out.annotate(Annotation::Label { id: format!("{}:bulb-label", f.id), anchor: anchor.clone(), text: loc(&format!("0.2 q at {depth:.2} m below the base"), &format!("0,2 q bei {depth:.2} m unter der Sohle")), tone: Tone::Info }, &anchor);
        }
    }
}

/// 🕳️ Ghost column of the required ground investigation z_a ≥ max(6 m, 2 B) below a footing (EN 1997-2 Annex B.3, GI-01), coloured by the utilisation of the investigation check `en1997.2.4.2.investigation.<footing>`; it passes when an investigation point reaches z_a (the declared depth of the document only counts without investigation points) and the detail names the points that do.
fn investigation_ghost(ctx: &Ctx, f: &super::model::SpreadFoundation, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    let required = super::checks::part_2::min_investigation_depth_for_gk(f.width.max(f.length), d.geotechnical_category);
    let pose = layout.pose(&f.id);
    let label = loc(&format!("Required investigation depth {required:.2} m"), &format!("Erforderliche Aufschlusstiefe {required:.2} m"));
    let reached = super::checks::ground_derivation::reached_depth(d);
    let covered = reached >= required;
    let reaching: Vec<&str> = d.investigations.iter().filter(|i| i.depth >= required).map(|i| i.id.as_str()).collect();
    let by = reaching.join(", ");
    out.push(
        Spec::new(
            EntityKind::Solid,
            format!("{}:investigation-ghost", f.id),
            "footings",
            Geometry::Solid(SolidGeometry { shape: SolidShape::StrataColumn { frame: pose.upright([0.0, 0.0, 0.0]), footprint: rect(f.width, f.length), strata: vec![Stratum { thickness: required, role: soil("borehole"), label: label.clone() }] } }),
            soil("borehole"),
            label,
            LAYER_GROUND,
        )
        .bound(projection_kit::binding_where(report, |c| c.id.starts_with(&format!("en1997.2.4.2.investigation.{}", f.id))))
        .channel(ColourChannel::Utilisation)
        .opacity(0.18)
        .detail(
            format!("z_a = {required:.2} m below the surface; investigation depth {reached:.2} m ({}{})", if covered { "covered" } else { "too shallow" }, if by.is_empty() { String::new() } else { format!(" by {by}") }),
            format!("z_a = {required:.2} m unter Gelände; Aufschlusstiefe {reached:.2} m ({}{})", if covered { "abgedeckt" } else { "zu gering" }, if by.is_empty() { String::new() } else { format!(" durch {by}") }),
        ),
        &[pose.at(0.0, 0.0, 0.0), pose.at(0.0, 0.0, -required)],
    );
}
