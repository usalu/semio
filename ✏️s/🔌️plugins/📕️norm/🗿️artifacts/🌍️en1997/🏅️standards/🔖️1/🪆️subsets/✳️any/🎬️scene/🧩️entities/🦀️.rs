//! 🧩️ Entities of the wave-2 snapshot: embedded walls with their pressure diagrams, ground anchors (free and fixed length), pile groups on a cap, laboratory specimens and the piezometric plane of a layer.

use super::kit::*;
use super::layout::WALL_LENGTH;
use super::scene::*;
use super::mechanics as mech;
use super::Ctx;
use super::checks::catalogue::seepage::FlowNet;
use super::checks::catalogue::walls_and_anchors::horizontal_coefficients;

const PRESSURE_SCALE: f64 = 40_000.0;
const GROUP_CAP: f64 = 0.8;
const RAFT_THICKNESS: f64 = 0.3;
const SPECIMEN_RADIUS: f64 = 0.2;

pub fn embedded_walls(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for w in d.embedded_walls.iter().filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6) {
        if !out.own("embeddedWalls", &w.id) {
            continue;
        }
        let pose = layout.pose(&w.id);
        let overall = ctx.binding(&w.id, &[]);
        let t = w.thickness.max(0.05);
        let depth = w.retained_height + w.embedment;
        out.push(
            Spec::new(
                EntityKind::Solid,
                w.id.clone(),
                "embeddedWalls",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.section([0.0, WALL_LENGTH / 2.0, 0.0]), base: polygon2(&[[0.0, -w.embedment], [t, -w.embedment], [t, w.retained_height], [0.0, w.retained_height]]), height: WALL_LENGTH } }),
                MaterialRole::Concrete,
                loc(&format!("Embedded wall {}", w.id), &format!("Eingespannte Wand {}", w.id)),
                LAYER_WALLS,
            )
            .bound(overall.clone())
            .detail(format!("retained {:.2} m, embedment {:.2} m, {}", w.retained_height, w.embedment, w.kind), format!("Geländesprung {:.2} m, Einbindung {:.2} m, {}", w.retained_height, w.embedment, w.kind))
            .handles(ctx.handles_of(&w.id)),
            &[pose.at(0.0, -WALL_LENGTH / 2.0, -w.embedment), pose.at(t, WALL_LENGTH / 2.0, w.retained_height)],
        );
        let reach = (2.0 * w.retained_height).max(3.0);
        out.push(
            Spec::new(
                EntityKind::Solid,
                format!("{}:retained-soil", w.id),
                "embeddedWalls",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.section([0.0, WALL_LENGTH / 2.0, 0.0]), base: polygon2(&[[t, 0.0], [t + reach, 0.0], [t + reach, w.retained_height], [t, w.retained_height]]), height: WALL_LENGTH } }),
                soil("backfill"),
                loc(&format!("Retained soil of {}", w.id), &format!("Anstehender Boden vor {}", w.id)),
                LAYER_WALLS,
            )
            .channel(ColourChannel::Role)
            .opacity(0.7)
            .detail(format!("phi' {:.1} deg, gamma {:.1} kN/m3", w.phi_deg, w.backfill_gamma / 1000.0), format!("phi' {:.1} Grad, gamma {:.1} kN/m3", w.phi_deg, w.backfill_gamma / 1000.0)),
            &[pose.at(t, -WALL_LENGTH / 2.0, 0.0), pose.at(t + reach, WALL_LENGTH / 2.0, w.retained_height)],
        );
        let phi = w.phi_deg.max(1.0);
        let (kah, kph) = horizontal_coefficients(phi, w.delta_active_deg.max(0.0), w.delta_passive_deg.max(0.0));
        let at = |x: f64, z: f64| pose.at(x, 0.0, z);
        let steps = 8;
        let mut ring = vec![at(t, w.retained_height)];
        for i in 0..=steps {
            let z = w.retained_height - depth * i as f64 / steps as f64;
            let depth_below_top = w.retained_height - z;
            ring.push(at(t + kah * (w.backfill_gamma * depth_below_top + w.surcharge) / PRESSURE_SCALE, z));
        }
        ring.push(at(t, -w.embedment));
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:active-pressure", w.id), "embeddedWalls", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&ring) } }), soil("mechanism"), loc(&format!("Active pressure on {}", w.id), &format!("Aktiver Erddruck auf {}", w.id)), LAYER_MECHANISMS)
                .bound(overall.clone())
                .opacity(0.3),
            &ring,
        );
        let mut passive = vec![at(0.0, 0.0)];
        for i in 0..=steps {
            let below = w.embedment * i as f64 / steps as f64;
            passive.push(at(-(kph * w.backfill_gamma * below) / PRESSURE_SCALE, -below));
        }
        passive.push(at(0.0, -w.embedment));
        out.push(
            Spec::new(EntityKind::Surface, format!("{}:passive-resistance", w.id), "embeddedWalls", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&passive) } }), soil("mechanism"), loc(&format!("Passive resistance of {}", w.id), &format!("Erdwiderstand von {}", w.id)), LAYER_MECHANISMS)
                .bound(overall.clone())
                .opacity(0.3),
            &passive,
        );
        let anchor = at(t, w.retained_height);
        out.annotate(Annotation::Label { id: format!("{}:label", w.id), anchor: anchor.clone(), text: loc(&format!("{} t = {:.2} m", w.id, w.embedment), &format!("{} t = {:.2} m", w.id, w.embedment)), tone: tone_of(&overall) }, &anchor);
    }
}

const SEEPAGE_CELLS: usize = 8;
const SEEPAGE_LINES: usize = 5;
const SEEPAGE_POINTS: usize = 14;

fn thinned(line: &[[f64; 2]]) -> Vec<[f64; 2]> {
    let step = line.len().div_ceil(SEEPAGE_POINTS).max(1);
    let mut kept: Vec<[f64; 2]> = line.iter().copied().step_by(step).collect();
    if let (Some(last), Some(end)) = (line.last(), kept.last()) {
        if last != end {
            kept.push(*last);
        }
    }
    kept
}

/// 💧 Flow net around the toe of an embedded wall from the finite-difference solution of the Laplace equation in a layer of the investigation depth (catalogue entry HYD-03): `SEEPAGE_LINES` streamlines (contours of the stream function) and as many equipotentials, each a chain of thin cylinders; the exit gradient of the net is the surface gradient next to the wall. Retained side at +x, depth below the dredge level.
pub fn seepage(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for w in d.embedded_walls.iter().filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6) {
        if !out.own("embeddedWalls", &w.id) {
            continue;
        }
        let head = w.retained_height - d.groundwater_level;
        if head <= 1e-6 {
            continue;
        }
        let thickness = (d.investigation_depth - w.retained_height).max(2.0 * w.embedment);
        let Ok(net) = FlowNet::solve(head, w.embedment, thickness, SEEPAGE_CELLS) else {
            continue;
        };
        let pose = layout.pose(&w.id);
        let overall = ctx.binding(&w.id, &["en1997.hyd-03"]);
        let middle = w.thickness.max(0.05) / 2.0;
        let exit = net.exit_gradient();
        let families = [("seepage", "Streamline", "Stromlinie", net.streamlines(SEEPAGE_LINES), 0.6), ("equipotential", "Equipotential", "Äquipotentiale", net.equipotentials(SEEPAGE_LINES), 0.35)];
        for (family, en, de, lines, opacity) in families {
            for (j, line) in lines.iter().enumerate() {
                let points: Vec<[f64; 2]> = thinned(line).into_iter().map(|[x, depth]| [middle - x, -depth]).collect();
                for (k, pair) in points.windows(2).enumerate() {
                    let (dx, dz) = (pair[1][0] - pair[0][0], pair[1][1] - pair[0][1]);
                    let length = (dx * dx + dz * dz).sqrt();
                    if length < 1e-6 {
                        continue;
                    }
                    let (ux, uz) = (dx / length, dz / length);
                    let frame = pose.frame([pair[0][0], 0.0, pair[0][1]], v3(-uz, 0.0, ux), v3(0.0, -1.0, 0.0), v3(ux, 0.0, uz));
                    out.push(
                        Spec::new(EntityKind::Solid, format!("{}:{family}-{}-{}", w.id, j + 1, k + 1), "embeddedWalls", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame, radius: 0.04, height: length } }), MaterialRole::Neutral, loc(&format!("{en} {} of {}", j + 1, w.id), &format!("{de} {} von {}", j + 1, w.id)), LAYER_WATER)
                            .bound(overall.clone())
                            .opacity(opacity)
                            .detail(format!("flow net exit gradient {exit:.3}, {} cells", net.nx * net.nz), format!("Strömungsnetz Austrittsgefälle {exit:.3}, {} Zellen", net.nx * net.nz)),
                        &[pose.at(pair[0][0], 0.0, pair[0][1]), pose.at(pair[1][0], 0.0, pair[1][1])],
                    );
                }
            }
        }
    }
}

const PORE_COLUMNS: usize = 33;
const PORE_ROWS: usize = 17;
const GRADIENT_PLANE: f64 = 0.1;

/// 💧 Pore-pressure section of the same finite-difference flow net: a vertical grid through the wall axis coloured by the total pore pressure `u = γw (h + depth)` (excess head `h` of the net, depth below the dredge level), so the hydrostatic increase with depth, the head loss around the wall toe and the lifted pressure under the excavation surface show beside the streamlines; the section is the hydraulic-failure (HYD) view of the embedded wall.
pub fn pore_pressure(ctx: &Ctx, out: &mut Out) {
    let (layout, d) = (ctx.layout, ctx.input.document);
    let gamma_w = super::checks::catalogue::Case::new("pore", d.annex).gamma_w();
    for w in d.embedded_walls.iter().filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6) {
        let head = w.retained_height - d.groundwater_level;
        if head <= 1e-6 || !out.own("embeddedWalls", &w.id) {
            continue;
        }
        let thickness = (d.investigation_depth - w.retained_height).max(2.0 * w.embedment);
        let Ok(net) = FlowNet::solve(head, w.embedment, thickness, SEEPAGE_CELLS) else {
            continue;
        };
        let (pose, middle) = (layout.pose(&w.id), w.thickness.max(0.05) / 2.0);
        let (columns, rows) = (PORE_COLUMNS.min(net.nx), PORE_ROWS.min(net.nz));
        let mut points = Vec::with_capacity(columns * rows);
        let mut values = Vec::with_capacity(columns * rows);
        for j in 0..rows {
            for i in 0..columns {
                let (ci, cj) = (i * (net.nx - 1) / (columns - 1).max(1), j * (net.nz - 1) / (rows - 1).max(1));
                let (x, depth) = ((ci as f64 + 0.5 - net.centre() as f64) * net.spacing, (cj as f64 + 0.5) * net.spacing);
                points.push(pose.at(middle - x, 0.0, -depth));
                values.push(gamma_w * (net.head_at(ci, cj) + depth));
            }
        }
        let peak = values.iter().copied().fold(0.0_f64, f64::max);
        let field = ScalarField { id: "pore-pressure".into(), unit: QuantityKind::Pressure, samples: FieldSamples::Vertex { values } };
        out.push(
            Spec::new(
                EntityKind::Surface,
                format!("{}:pore-pressure", w.id),
                "embeddedWalls",
                Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: rows as u32, columns: columns as u32, points: points.clone() } } }),
                soil("water"),
                loc(&format!("Pore-pressure section of {}", w.id), &format!("Porenwasserdruck-Schnitt von {}", w.id)),
                LAYER_WATER,
            )
            .channel(ColourChannel::Fields { active: "pore-pressure".into(), fields: vec![field] })
            .opacity(0.7)
            .detail(format!("total pore pressure up to {:.0} kPa from the flow net, excess head {head:.2} m", peak / 1000.0), format!("Gesamter Porenwasserdruck bis {:.0} kPa aus dem Strömungsnetz, Überdruckhöhe {head:.2} m", peak / 1000.0)),
            &points,
        );
        if !out.legends.iter().any(|legend| legend.id == "pore-pressure") {
            out.legends.push(Legend { id: "pore-pressure".into(), title: loc("Pore pressure", "Porenwasserdruck"), scale: LegendScale::Scalar { field_id: "pore-pressure".into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: (peak / 1000.0).max(1.0) }, unit: QuantityKind::Pressure } });
        }
    }
}

/// 🧭️ Hydraulic-gradient section of the same flow net (catalogue entry HYD-03): the grid of the pore-pressure section, offset by `GRADIENT_PLANE` along the wall axis, coloured per vertex by the gradient |∇h| of the net with the critical gradient γ′/γw of the layer at the wall toe as the top of the legend, so the zone around the toe where the gradient approaches the quick condition is visible in 3D.
pub fn hydraulic_gradient(ctx: &Ctx, out: &mut Out) {
    let (layout, d) = (ctx.layout, ctx.input.document);
    for w in d.embedded_walls.iter().filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6) {
        let head = w.retained_height - d.groundwater_level;
        if head <= 1e-6 || !out.own("embeddedWalls", &w.id) {
            continue;
        }
        let Some(layer) = mech::layer_at(&d.layers, w.retained_height + 0.5 * w.embedment) else { continue };
        let thickness = (d.investigation_depth - w.retained_height).max(2.0 * w.embedment);
        let Ok(net) = FlowNet::solve(head, w.embedment, thickness, SEEPAGE_CELLS) else {
            continue;
        };
        let critical = layer.gamma_prime / mech::GAMMA_W;
        let (pose, middle) = (layout.pose(&w.id), w.thickness.max(0.05) / 2.0);
        let (columns, rows) = (PORE_COLUMNS.min(net.nx), PORE_ROWS.min(net.nz));
        let mut points = Vec::with_capacity(columns * rows);
        let mut values = Vec::with_capacity(columns * rows);
        for j in 0..rows {
            for i in 0..columns {
                let (ci, cj) = (i * (net.nx - 1) / (columns - 1).max(1), j * (net.nz - 1) / (rows - 1).max(1));
                let (x, depth) = ((ci as f64 + 0.5 - net.centre() as f64) * net.spacing, (cj as f64 + 0.5) * net.spacing);
                points.push(pose.at(middle - x, GRADIENT_PLANE, -depth));
                values.push(net.gradient_at(ci, cj));
            }
        }
        let peak = values.iter().copied().fold(0.0_f64, f64::max);
        let field = ScalarField { id: "hydraulic-gradient".into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Vertex { values } };
        out.push(
            Spec::new(
                EntityKind::Surface,
                format!("{}:hydraulic-gradient", w.id),
                "embeddedWalls",
                Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: rows as u32, columns: columns as u32, points: points.clone() } } }),
                soil("water"),
                loc(&format!("Hydraulic-gradient section of {}", w.id), &format!("Strömungsgefälle-Schnitt von {}", w.id)),
                LAYER_WATER,
            )
            .channel(ColourChannel::Fields { active: "hydraulic-gradient".into(), fields: vec![field] })
            .opacity(0.7)
            .detail(format!("gradient up to {peak:.2} from the flow net against the critical gradient {critical:.2}, excess head {head:.2} m"), format!("Gefälle bis {peak:.2} aus dem Strömungsnetz gegen das kritische Gefälle {critical:.2}, Überdruckhöhe {head:.2} m")),
            &points,
        );
        if !out.legends.iter().any(|legend| legend.id == "hydraulic-gradient") {
            out.legends.push(Legend { id: "hydraulic-gradient".into(), title: loc("Hydraulic gradient (top: critical)", "Strömungsgefälle (oben: kritisch)"), scale: LegendScale::Scalar { field_id: "hydraulic-gradient".into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: critical.max(1e-3) }, unit: QuantityKind::Dimensionless } });
        }
    }
}

const ARROW_COLUMNS: [f64; 4] = [-1.5, -0.75, 0.75, 1.5];
const ARROW_ROWS: [f64; 4] = [0.25, 0.75, 1.25, 1.75];

/// 🔺️ Terzaghi heave prism of an embedded wall (EN 1997-1 10.4, catalogue entry HYD-03): the strip of width d/2 and height d on the excavation side of the wall toe, bound to the `heave_prism` ratio of the check (seepage force γH·γw·h_m·d/2 against the submerged weight γH,stb·γ′·d²/2) so it takes the utilisation colour and turns red when the prism lifts.
pub fn heave_prism(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for w in d.embedded_walls.iter().filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6) {
        if d.groundwater_level >= w.retained_height || !out.own("embeddedWalls", &w.id) {
            continue;
        }
        let pose = layout.pose(&w.id);
        let bound = ctx.binding(&w.id, &["en1997.hyd-03.heave_prism"]).or_else(|| ctx.binding(&w.id, &["en1997.hyd-03"]));
        let ratio = bound.as_ref().map(|b| b.ratio).unwrap_or(0.0);
        let (reach, depth) = (w.embedment / 2.0, w.embedment);
        out.push(
            Spec::new(
                EntityKind::Solid,
                format!("{}:heave-prism", w.id),
                "embeddedWalls",
                Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.section([0.0, WALL_LENGTH / 2.0, 0.0]), base: polygon2(&[[-reach, -depth], [0.0, -depth], [0.0, 0.0], [-reach, 0.0]]), height: WALL_LENGTH } }),
                MaterialRole::Neutral,
                loc(&format!("Heave prism of {}", w.id), &format!("Aufbruchprisma von {}", w.id)),
                LAYER_WATER,
            )
            .bound(bound)
            .opacity(0.55)
            .detail(format!("Terzaghi prism {reach:.2} m wide, {depth:.2} m high: seepage force against submerged weight, utilisation {ratio:.2}"), format!("Terzaghi-Prisma {reach:.2} m breit, {depth:.2} m hoch: Strömungskraft gegen Auftriebsgewicht, Ausnutzung {ratio:.2}")),
            &[pose.at(-reach, 0.0, -depth), pose.at(0.0, 0.0, 0.0)],
        );
    }
}

/// 🏹️ Seepage arrows around the wall toe: a coarse grid of the flow net, each cell drawn as a shaft with a thick head along `−∇h` (retained side down, round the toe, up on the excavation side), the length growing with the gradient of the cell up to the largest gradient of the net.
pub fn flow_arrows(ctx: &Ctx, out: &mut Out) {
    let (layout, d) = (ctx.layout, ctx.input.document);
    for w in d.embedded_walls.iter().filter(|w| w.retained_height > 1e-6 && w.embedment > 1e-6) {
        let head = w.retained_height - d.groundwater_level;
        if head <= 1e-6 || !out.own("embeddedWalls", &w.id) {
            continue;
        }
        let thickness = (d.investigation_depth - w.retained_height).max(2.0 * w.embedment);
        let Ok(net) = FlowNet::solve(head, w.embedment, thickness, SEEPAGE_CELLS) else {
            continue;
        };
        let (pose, middle, peak) = (layout.pose(&w.id), w.thickness.max(0.05) / 2.0, net.max_gradient().max(1e-9));
        for (r, depth_ratio) in ARROW_ROWS.iter().enumerate() {
            for (c, column_ratio) in ARROW_COLUMNS.iter().enumerate() {
                let (x, depth) = (column_ratio * w.embedment, depth_ratio * w.embedment);
                let (ci, cj) = (((net.centre() as f64 + x / net.spacing).floor().max(0.0) as usize).min(net.nx - 1), ((depth / net.spacing).floor().max(0.0) as usize).min(net.nz - 1));
                let (gradient, [vx, vd]) = (net.gradient_at(ci, cj), net.flow_direction_at(ci, cj));
                let norm = vx.hypot(vd);
                if gradient < 1e-9 || norm < 1e-12 {
                    continue;
                }
                let (ux, uz) = (-vx / norm, -vd / norm);
                let length = w.embedment * (0.15 + 0.55 * (gradient / peak).min(1.0));
                let (tail_x, tail_z) = (middle - x, -depth);
                let (shaft, tip) = (0.75 * length, 0.25 * length);
                let frame = |along: f64| pose.frame([tail_x + ux * along, 0.0, tail_z + uz * along], v3(-uz, 0.0, ux), v3(0.0, -1.0, 0.0), v3(ux, 0.0, uz));
                let name = loc(&format!("Seepage arrow {}-{} of {}", r + 1, c + 1, w.id), &format!("Sickerpfeil {}-{} von {}", r + 1, c + 1, w.id));
                let detail = (format!("flow direction, gradient {gradient:.3}"), format!("Strömungsrichtung, Gefälle {gradient:.3}"));
                out.push(
                    Spec::new(EntityKind::Solid, format!("{}:flow-arrow-{}-{}", w.id, r + 1, c + 1), "embeddedWalls", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: frame(0.0), radius: 0.03, height: shaft } }), MaterialRole::Neutral, name.clone(), LAYER_WATER)
                        .opacity(0.85)
                        .detail(detail.0.clone(), detail.1.clone()),
                    &[pose.at(tail_x, 0.0, tail_z), pose.at(tail_x + ux * length, 0.0, tail_z + uz * length)],
                );
                out.push(
                    Spec::new(EntityKind::Solid, format!("{}:flow-arrow-{}-{}-head", w.id, r + 1, c + 1), "embeddedWalls", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: frame(shaft), radius: 0.09, height: tip } }), MaterialRole::Neutral, name, LAYER_WATER)
                        .opacity(0.85)
                        .detail(detail.0, detail.1),
                    &[pose.at(tail_x + ux * shaft, 0.0, tail_z + uz * shaft), pose.at(tail_x + ux * length, 0.0, tail_z + uz * length)],
                );
            }
        }
    }
}

pub fn anchors(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for a in d.anchors.iter().filter(|a| a.free_length > 1e-6 && a.fixed_length > 1e-6) {
        if !out.own("anchors", &a.id) {
            continue;
        }
        let wall = d.embedded_walls.iter().find(|w| w.id == a.wall_id);
        let pose = layout.pose(&a.wall_id);
        let overall = ctx.binding(&a.id, &[]);
        let theta = a.inclination_deg.to_radians();
        let (c, s) = (theta.cos(), theta.sin());
        let (top, t) = wall.map_or((0.0, 0.3), |w| (w.retained_height, w.thickness.max(0.05)));
        let head = [t, 0.0, top - a.head_depth];
        let dir = [c, 0.0, -s];
        let along = |l: f64| pose.at(head[0] + dir[0] * l, head[1], head[2] + dir[2] * l);
        let frame = |l: f64| pose.frame([head[0] + dir[0] * l, head[1], head[2] + dir[2] * l], v3(s, 0.0, c), v3(0.0, -1.0, 0.0), v3(c, 0.0, -s));
        let name = loc(&format!("Anchor {}", a.id), &format!("Anker {}", a.id));
        let free_id = format!("{}:free", a.id);
        let fixed_id = format!("{}:fixed", a.id);
        out.push(
            Spec::new(EntityKind::Solid, free_id.clone(), "anchors", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: frame(0.0), radius: 0.03, height: a.free_length } }), MaterialRole::Support, loc(&format!("Free length of {}", a.id), &format!("Freie Länge von {}", a.id)), LAYER_WALLS).bound(overall.clone()),
            &[along(0.0), along(a.free_length)],
        );
        out.push(
            Spec::new(EntityKind::Solid, fixed_id.clone(), "anchors", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: frame(a.free_length), radius: (a.fixed_diameter / 2.0).max(0.03), height: a.fixed_length } }), MaterialRole::Concrete, loc(&format!("Fixed length of {}", a.id), &format!("Verpresskörper von {}", a.id)), LAYER_WALLS).bound(overall.clone()),
            &[along(a.free_length), along(a.free_length + a.fixed_length)],
        );
        out.push(
            Spec::new(EntityKind::Group, a.id.clone(), "anchors", Geometry::Group { members: vec![entity_ref(EntityKind::Solid, &free_id, "anchors"), entity_ref(EntityKind::Solid, &fixed_id, "anchors")] }, MaterialRole::Support, name, LAYER_WALLS)
                .bound(overall.clone())
                .detail(format!("A_d {:.0} kN, free {:.1} m, fixed {:.1} m, {:.0} deg", a.design_force / 1000.0, a.free_length, a.fixed_length, a.inclination_deg), format!("A_d {:.0} kN, frei {:.1} m, Verpresskörper {:.1} m, {:.0} Grad", a.design_force / 1000.0, a.free_length, a.fixed_length, a.inclination_deg))
                .handles(ctx.handles_of(&a.id)),
            &[],
        );
        let at = along(a.free_length + a.fixed_length);
        out.push(
            Spec::new(EntityKind::Load, format!("{}:force", a.id), "anchors", Geometry::Load(LoadGlyph::Point { at: along(0.0), direction: pose.dir(&v3(-c, 0.0, s)), magnitude: force(a.design_force), case: load_case("anchor-force", LoadKind::Permanent) }), MaterialRole::Load { kind: LoadKind::Permanent }, loc(&format!("Anchor force of {}", a.id), &format!("Ankerkraft von {}", a.id)), LAYER_LOADS),
            &[along(0.0)],
        );
        out.annotate(Annotation::Label { id: format!("{}:label", a.id), anchor: at.clone(), text: loc(&format!("{} R_a = {:.0} kN", a.id, a.tests.iter().map(|t| t.pull_out_resistance).fold(f64::INFINITY, f64::min).min(1.0e12) / 1000.0), &format!("{} R_a = {:.0} kN", a.id, a.tests.iter().map(|t| t.pull_out_resistance).fold(f64::INFINITY, f64::min).min(1.0e12) / 1000.0)), tone: tone_of(&overall) }, &at);
    }
}

pub fn pile_groups(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for g in d.pile_groups.iter().filter(|g| g.rows > 0 && g.columns > 0) {
        if !out.own("pileGroups", &g.id) {
            continue;
        }
        let pile = d.piles.iter().find(|p| p.id == g.pile_id);
        let (diameter, length) = pile.map_or((0.6, 10.0), |p| (p.diameter.max(0.05), p.length.max(0.5)));
        let pose = layout.pose(&g.id);
        let overall = ctx.binding(&g.id, &[]);
        let mut members = Vec::new();
        for r in 0..g.rows {
            for c in 0..g.columns {
                let x = (f64::from(c) - (f64::from(g.columns) - 1.0) / 2.0) * g.spacing;
                let y = (f64::from(r) - (f64::from(g.rows) - 1.0) / 2.0) * g.spacing;
                let id = format!("{}:pile-{}-{}", g.id, r + 1, c + 1);
                members.push(entity_ref(EntityKind::Solid, &id, "pileGroups"));
                out.push(
                    Spec::new(EntityKind::Solid, id, "pileGroups", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: pose.upright([x, y, -length]), radius: diameter / 2.0, height: length } }), MaterialRole::Concrete, loc(&format!("Pile {}/{} of {}", r + 1, c + 1, g.id), &format!("Pfahl {}/{} von {}", r + 1, c + 1, g.id)), LAYER_PILES).bound(overall.clone()),
                    &[pose.at(x, y, -length), pose.at(x, y, 0.0)],
                );
            }
        }
        let ex = (f64::from(g.columns) - 1.0) * g.spacing / 2.0 + diameter;
        let ey = (f64::from(g.rows) - 1.0) * g.spacing / 2.0 + diameter;
        out.push(
            Spec::new(EntityKind::Solid, format!("{}:cap", g.id), "pileGroups", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([0.0, 0.0, 0.2 - GROUP_CAP]), base: polygon2(&[[-ex, -ey], [ex, -ey], [ex, ey], [-ex, ey]]), height: GROUP_CAP } }), MaterialRole::Concrete, loc(&format!("Cap of {}", g.id), &format!("Kopfplatte von {}", g.id)), LAYER_PILES).bound(overall.clone()),
            &[pose.at(-ex, -ey, 0.2 - GROUP_CAP), pose.at(ex, ey, 0.2)],
        );
        if let Some(pile) = pile.filter(|p| p.length > 0.0 && g.cap_vertical > 0.0) {
            let raft = mech::equivalent_raft(g, pile);
            let s = mech::group_settlement(g, pile, &d.layers) * 1000.0;
            let (hx, hy) = (raft.width / 2.0, raft.length / 2.0);
            out.push(
                Spec::new(EntityKind::Solid, format!("{}:equivalent-raft", g.id), "pileGroups", Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: pose.upright([0.0, 0.0, -raft.depth - RAFT_THICKNESS / 2.0]), base: polygon2(&[[-hx, -hy], [hx, -hy], [hx, hy], [-hx, hy]]), height: RAFT_THICKNESS } }), MaterialRole::Neutral, loc(&format!("Equivalent raft of {}", g.id), &format!("Ersatzfundament von {}", g.id)), LAYER_PILES)
                    .bound(overall.clone())
                    .channel(ColourChannel::Utilisation)
                    .opacity(0.18)
                    .detail(format!("{:.2} x {:.2} m at {:.1} m below the pile head, load spread 1:4 (textbook convention); group settlement {s:.1} mm", raft.width, raft.length, raft.depth), format!("{:.2} x {:.2} m in {:.1} m unter Pfahlkopf, Lastausbreitung 1:4 (Lehrbuchkonvention); Gruppensetzung {s:.1} mm", raft.width, raft.length, raft.depth)),
                &[pose.at(-hx, -hy, -raft.depth - RAFT_THICKNESS / 2.0), pose.at(hx, hy, -raft.depth + RAFT_THICKNESS / 2.0)],
            );
        }
        out.push(
            Spec::new(EntityKind::Group, g.id.clone(), "pileGroups", Geometry::Group { members }, MaterialRole::Concrete, loc(&format!("Pile group {}", g.id), &format!("Pfahlgruppe {}", g.id)), LAYER_PILES)
                .bound(overall.clone())
                .detail(format!("{} x {} piles, spacing {:.2} m, V_d {:.0} kN", g.rows, g.columns, g.spacing, g.cap_vertical / 1000.0), format!("{} x {} Pfähle, Achsabstand {:.2} m, V_d {:.0} kN", g.rows, g.columns, g.spacing, g.cap_vertical / 1000.0))
                .handles(ctx.handles_of(&g.id)),
            &[],
        );
        let cap_top = pose.at(0.0, 0.0, 0.2);
        out.push(
            Spec::new(EntityKind::Load, format!("{}:load", g.id), "pileGroups", Geometry::Load(LoadGlyph::Point { at: cap_top.clone(), direction: Vec3::new(0.0, 0.0, -1.0), magnitude: force(g.cap_vertical), case: load_case("group-load", LoadKind::Permanent) }), MaterialRole::Load { kind: LoadKind::Permanent }, loc(&format!("Load on {}", g.id), &format!("Last auf {}", g.id)), LAYER_LOADS),
            &[cap_top],
        );
    }
}

pub fn lab_tests(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for t in &d.lab_tests {
        if !out.own("labTests", &t.id) {
            continue;
        }
        let layer = d.layers.iter().find(|l| l.id == t.layer_id);
        let (top, bottom, role) = layer.map_or((0.0, 1.0, soil("soil")), |l| (l.depth_top, l.depth_bottom.max(l.depth_top + 0.1), soil(l.soil_type.as_str())));
        let pose = layout.pose(&t.id);
        let overall = ctx.binding(&t.id, &[]);
        out.push(
            Spec::new(EntityKind::Solid, t.id.clone(), "labTests", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: pose.upright([0.0, 0.0, -bottom]), radius: SPECIMEN_RADIUS, height: bottom - top } }), role, loc(&format!("Laboratory series {} ({})", t.id, t.kind), &format!("Laborversuchsreihe {} ({})", t.id, t.kind)), LAYER_GROUND)
                .bound(overall.clone())
                .channel(ColourChannel::Utilisation)
                .handles(ctx.handles_of(&t.id))
                .detail(format!("{} points in layer {}", t.points.len(), t.layer_id), format!("{} Punkte in Schicht {}", t.points.len(), t.layer_id)),
            &[pose.at(0.0, 0.0, -bottom), pose.at(0.0, 0.0, -top)],
        );
        let anchor = pose.at(0.0, 0.0, -(top + bottom) / 2.0);
        out.annotate(Annotation::Label { id: format!("{}:label", t.id), anchor: anchor.clone(), text: loc(&format!("{} {}", t.id, t.kind), &format!("{} {}", t.id, t.kind)), tone: tone_of(&overall) }, &anchor);
    }
}

pub fn layer_waters(ctx: &Ctx, out: &mut Out) {
    let (layout, d, report) = (ctx.layout, ctx.input.document, ctx.input.report);
    for w in &d.layer_waters {
        if !out.own("layerWaters", &w.id) {
            continue;
        }
        let z = -w.piezometric_depth;
        let (x0, y0, x1, y1) = (layout.min[0], layout.min[1], layout.max[0], layout.max[1]);
        let plane = [v3(x0, y0, z), v3(x1, y0, z), v3(x1, y1, z), v3(x0, y1, z)];
        let overall = ctx.binding(&w.id, &[]);
        out.push(
            Spec::new(EntityKind::Surface, w.id.clone(), "layerWaters", Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: polygon3(&plane) } }), MaterialRole::Neutral, loc(&format!("Piezometric level of {}", w.layer_id), &format!("Standrohrspiegel von {}", w.layer_id)), LAYER_WATER)
                .bound(overall.clone())
                .opacity(0.25)
                .detail(format!("{:.2} m below ground, dh {:.2} m over {:.2} m", w.piezometric_depth, w.head_difference, w.flow_length), format!("{:.2} m unter Gelände, dh {:.2} m auf {:.2} m", w.piezometric_depth, w.head_difference, w.flow_length))
                .handles(ctx.handles_of(&w.id)),
            &plane,
        );
        let anchor = v3(x1, y0, z);
        out.annotate(Annotation::Label { id: format!("{}:label", w.id), anchor: anchor.clone(), text: loc(&format!("{} k = {:.1e} m/s", w.id, w.permeability), &format!("{} k = {:.1e} m/s", w.id, w.permeability)), tone: tone_of(&overall) }, &anchor);
    }
}

const BASIS_RADIUS: f64 = 0.25;
const BASIS_THICKNESS: f64 = 0.1;
const BASIS_PITCH: f64 = 0.7;

/// 🧭️ Plan position of the marker of the `index`-th characteristic-value basis: a column along the west edge of the site, level with the middle of its layer.
pub fn basis_pose(layout: &super::layout::Layout, index: usize) -> super::layout::Pose {
    super::layout::Pose { origin: v3(layout.min[0] + 2.0 * BASIS_RADIUS, layout.min[1] + 2.0 * BASIS_RADIUS + index as f64 * BASIS_PITCH, 0.0), yaw: 0.0 }
}

/// 🏷️ The English and German name of the soil parameter a basis governs.
pub fn parameter_words(parameter: super::model::CharacteristicParameter) -> (&'static str, &'static str) {
    match parameter {
        super::model::CharacteristicParameter::UndrainedStrength => ("undrained shear strength c_u", "undränierte Scherfestigkeit c_u"),
        super::model::CharacteristicParameter::FrictionAngle => ("friction angle φ′", "Reibungswinkel φ′"),
    }
}

/// 🏷️ The English and German name of the source of a characteristic value.
pub fn basis_words(basis: super::model::ParameterBasis) -> (&'static str, &'static str) {
    match basis {
        super::model::ParameterBasis::Declared => ("declared on the layer", "auf der Schicht angegeben"),
        super::model::ParameterBasis::Derived => ("derived from investigations", "aus Erkundungen abgeleitet"),
    }
}

/// ⚖️ The characteristic-value bases: one disc per row at the middle of the layer it governs, toned by the checks of the row (GI-03, GI-13).
pub fn characteristic_bases(ctx: &Ctx, out: &mut Out) {
    let (layout, d) = (ctx.layout, ctx.input.document);
    for (i, b) in d.characteristic_bases.iter().enumerate() {
        if !out.own("characteristicBases", &b.id) {
            continue;
        }
        let layer = d.layers.iter().find(|l| l.id == b.layer_id);
        let (mid, role) = layer.map_or((0.5, soil("soil")), |l| ((l.depth_top + l.depth_bottom) / 2.0, soil(l.soil_type.as_str())));
        let pose = basis_pose(layout, i);
        let overall = ctx.binding(&b.id, &[]);
        let (parameter, basis) = (parameter_words(b.parameter), basis_words(b.basis));
        out.push(
            Spec::new(EntityKind::Solid, b.id.clone(), "characteristicBases", Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: pose.upright([0.0, 0.0, -mid - BASIS_THICKNESS / 2.0]), radius: BASIS_RADIUS, height: BASIS_THICKNESS } }), role, loc(&format!("Characteristic value basis {} of layer {}", b.id, b.layer_id), &format!("Grundlage {} des charakteristischen Werts der Schicht {}", b.id, b.layer_id)), LAYER_GROUND)
                .bound(overall.clone())
                .channel(ColourChannel::Utilisation)
                .handles(ctx.handles_of(&b.id))
                .detail(format!("{}, basis {}", parameter.0, basis.0), format!("{}, Grundlage {}", parameter.1, basis.1)),
            &[pose.at(-BASIS_RADIUS, -BASIS_RADIUS, -mid - BASIS_THICKNESS / 2.0), pose.at(BASIS_RADIUS, BASIS_RADIUS, -mid + BASIS_THICKNESS / 2.0)],
        );
    }
}
