//! 🗺️ Diagrams of the uplift cases, the anchored walls, the pile groups and the seepage water rows, each a pure projection of the document and its report with a table twin: the stabilising and destabilising actions as bars (characteristic and design), the anchors of a wall as a section with free and fixed lengths coloured by utilisation, the plan of a pile group with the reaction of every pile, and the head along the flow path.

use super::*;

fn bar(id: &str, en: &str, de: &str, x: (f64, f64), y: (f64, f64), tone: Tone) -> Series {
    Series::Polygon { id: id.into(), label: loc(en, de), ring: vec![Vec2::new(x.0, y.0), Vec2::new(x.1, y.0), Vec2::new(x.1, y.1), Vec2::new(x.0, y.1)], fill: Fill { tone, opacity: 0.55 }, hatch: (tone == Tone::Error).then_some(Hatch::Diagonal) }
}

fn ratio_marker(id: String, at: [f64; 2], shape: MarkerShape, ratio: f64, label: LocalizedCopy, check: Option<String>) -> Marker {
    Marker { utilisation: Some(ratio), ..marker(id, at, shape, utilisation_tone(ratio), label, check) }
}

fn words(ratio: f64) -> (String, String) {
    (format!("utilisation {ratio:.2}"), format!("Ausnutzung {ratio:.2}"))
}

fn worst_ratio(report: &CheckReport, id: &str) -> f64 {
    report.checks.iter().filter(|k| k.subject.entity_id == id && k.utilization.is_finite()).map(|k| k.utilization).fold(0.0, f64::max)
}

/// 🧲️ Stabilising against destabilising actions of every uplift case as bars: the characteristic permanent stabilising force, the permanent and variable destabilising force stacked, and the design values of the UPL check.
pub(super) fn uplift_stack(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let kn = |v: f64| v / 1000.0;
    ctx.input.document.uplift_cases.iter().filter(|u| focus.is_none_or(|f| f == u.id)).map(|u| {
        let check = ctx.input.report.checks.iter().find(|k| k.id == format!("en1997.10.upl.{}", u.id));
        let status = ctx.binding(&u.id, &["en1997.10.upl."]);
        let (g, q, s) = (kn(u.permanent_destabilizing), kn(u.variable_destabilizing), kn(u.permanent_stabilizing));
        let mut series = vec![
            bar("stabilising-k", "Stabilising G_stb,k", "Stabilisierend G_stb,k", (0.6, 1.4), (0.0, s), Tone::Info),
            bar("destabilising-permanent-k", "Destabilising permanent G_dst,k", "Destabilisierend ständig G_dst,k", (1.6, 2.4), (0.0, g), Tone::Warning),
            bar("destabilising-variable-k", "Destabilising variable Q_dst,k", "Destabilisierend veränderlich Q_dst,k", (1.6, 2.4), (g, g + q), Tone::Error),
        ];
        let mut markers = vec![marker(format!("{}:stabilising", u.id), [1.0, s], MarkerShape::Square, Tone::Info, loc(&format!("G_stb,k = {s:.0} kN"), &format!("G_stb,k = {s:.0} kN")), status.as_ref().map(|s| s.check_id.clone())), marker(format!("{}:destabilising", u.id), [2.0, g + q], MarkerShape::Triangle, tone_of(&status), loc(&format!("G_dst,k + Q_dst,k = {:.0} kN", g + q), &format!("G_dst,k + Q_dst,k = {:.0} kN", g + q)), status.as_ref().map(|s| s.check_id.clone()))];
        let mut columns = vec![column("Action", "Einwirkung", "1", vec![1.0, 2.0]), column("Characteristic force", "Charakteristische Kraft", "kN", vec![s, g + q])];
        if let Some(k) = check {
            let (stab, dest) = (kn(k.computed.value), kn(k.limit.value));
            series.push(bar("stabilising-d", "Stabilising design value", "Stabilisierend Bemessungswert", (2.6, 3.4), (0.0, stab), Tone::Info));
            series.push(bar("destabilising-d", "Destabilising design value", "Destabilisierend Bemessungswert", (3.6, 4.4), (0.0, dest), utilisation_tone(k.utilization)));
            markers.push(ratio_marker(format!("{}:design", u.id), [4.0, dest], MarkerShape::Diamond, k.utilization, loc(&format!("V_dst,d = {dest:.0} kN against G_stb,d = {stab:.0} kN: {}", words(k.utilization).0), &format!("V_dst,d = {dest:.0} kN gegen G_stb,d = {stab:.0} kN: {}", words(k.utilization).1)), Some(k.id.clone())));
            columns.push(column("Design force", "Bemessungskraft", "kN", vec![stab, dest]));
        }
        Diagram2d {
            id: format!("uplift-stack:{}", u.id),
            title: loc(&format!("Uplift balance of {}", u.id), &format!("Auftriebsbilanz von {}", u.id)),
            description: loc("Permanent stabilising force against the stacked permanent and variable destabilising forces (EN 1997-1 10.2): characteristic bars first, then the design values with their partial factors.", "Ständige stabilisierende Kraft gegen die gestapelten ständigen und veränderlichen destabilisierenden Kräfte (EN 1997-1 10.2): zuerst die charakteristischen Balken, dann die Bemessungswerte mit ihren Teilsicherheitsbeiwerten."),
            axes: vec![axis("Action", "Einwirkung", QuantityKind::Dimensionless, "1", AxisScale::Linear), axis("Force", "Kraft", QuantityKind::Force, "kN", AxisScale::Linear)],
            series,
            markers,
            annotations: Vec::new(),
            focus: Some(entity_ref(EntityKind::Group, &u.id, "upliftCases")),
            legend: true,
            table_twin: DataTable { columns },
        }
    }).collect()
}

/// ⚓️ The anchors of every wall as a section: the wall as a vertical line, every anchor from its head along the free length (dashed) and the fixed length (solid, coloured by the anchor utilisation), with the head marked.
pub(super) fn anchor_layout(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.embedded_walls.iter().filter_map(|w| {
        let own: Vec<_> = d.anchors.iter().filter(|a| a.wall_id == w.id).collect();
        (!own.is_empty() && focus.is_none_or(|f| f == w.id || own.iter().any(|a| a.id == f))).then(|| {
            let depth = w.retained_height + w.embedment;
            let mut series = vec![polyline("wall", "Wall", "Wand", &[[0.0, 0.0], [0.0, -depth]], Tone::Neutral, Dash::Solid)];
            let mut markers = Vec::new();
            let mut columns: Vec<Vec<f64>> = vec![Vec::new(); 6];
            for (k, a) in own.iter().enumerate() {
                let (cos, sin) = (a.inclination_deg.to_radians().cos(), a.inclination_deg.to_radians().sin());
                let (head, joint) = ([0.0, -a.head_depth], [a.free_length * cos, -a.head_depth - a.free_length * sin]);
                let tip = [joint[0] + a.fixed_length * cos, joint[1] - a.fixed_length * sin];
                let ratio = worst_ratio(ctx.input.report, &a.id);
                series.push(polyline(&format!("{}:free", a.id), &format!("Anchor {} free length", a.id), &format!("Anker {} freie Länge", a.id), &[head, joint], Tone::Neutral, Dash::Dashed));
                series.push(polyline(&format!("{}:fixed", a.id), &format!("Anchor {} fixed length", a.id), &format!("Anker {} Verpresskörper", a.id), &[joint, tip], utilisation_tone(ratio), Dash::Solid));
                markers.push(ratio_marker(format!("{}:head", a.id), head, MarkerShape::Circle, ratio, loc(&format!("Anchor {}: head {:.1} m, {:.0} deg, A_d = {:.0} kN, {}", a.id, a.head_depth, a.inclination_deg, a.design_force / 1000.0, words(ratio).0), &format!("Anker {}: Kopf {:.1} m, {:.0} Grad, A_d = {:.0} kN, {}", a.id, a.head_depth, a.inclination_deg, a.design_force / 1000.0, words(ratio).1)), ctx.binding(&a.id, &[]).map(|b| b.check_id)));
                for (column, value) in columns.iter_mut().zip([(k + 1) as f64, a.head_depth, a.inclination_deg, a.free_length, a.fixed_length, ratio]) {
                    column.push(value);
                }
            }
            let [index, head, angle, free, fixed, ratio] = [0, 1, 2, 3, 4, 5].map(|i| std::mem::take(&mut columns[i]));
            Diagram2d {
                id: format!("anchor-layout:{}", w.id),
                title: loc(&format!("Anchors of wall {}", w.id), &format!("Anker der Wand {}", w.id)),
                description: loc("Section through the wall with every anchor: free length dashed, fixed length solid in the colour of the anchor utilisation; the markers sit at the anchor heads.", "Schnitt durch die Wand mit allen Ankern: freie Länge gestrichelt, Verpresskörper durchgezogen in der Farbe der Ankerauslastung; die Marken liegen an den Ankerköpfen."),
                axes: vec![axis("Distance from the wall", "Abstand von der Wand", QuantityKind::Length, "m", AxisScale::Linear), axis("Level below the wall top", "Höhe unter Wandkopf", QuantityKind::Length, "m", AxisScale::Linear)],
                series,
                markers,
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Group, &w.id, "embeddedWalls")),
                legend: true,
                table_twin: DataTable { columns: vec![column("Anchor", "Anker", "1", index), column("Head depth", "Kopftiefe", "m", head), column("Inclination", "Neigung", "°", angle), column("Free length", "Freie Länge", "m", free), column("Fixed length", "Verpresslänge", "m", fixed), column("Utilisation", "Ausnutzung", "1", ratio)] },
            }
        })
    }).collect()
}

/// 🏗️ Plan of every pile group: the piles on their grid with the reaction of every pile from the vertical load and the two moments on a rigid cap, N_i = V/n + Mx y_i / Σy² + My x_i / Σx², against the single-pile resistance.
pub(super) fn pile_group_plan(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.pile_groups.iter().filter(|g| g.rows > 0 && g.columns > 0 && g.spacing > 0.0 && focus.is_none_or(|f| f == g.id)).map(|g| {
        let diameter = d.piles.iter().find(|p| p.id == g.pile_id).map_or(0.0, |p| p.diameter);
        let (rows, columns) = (g.rows as usize, g.columns as usize);
        let at = |i: usize, n: usize| (i as f64 - (n as f64 - 1.0) / 2.0) * g.spacing;
        let piles: Vec<(f64, f64)> = (0..rows).flat_map(|r| (0..columns).map(move |c| (at(c, columns), at(r, rows)))).collect();
        let (sx, sy) = (piles.iter().map(|p| p.0 * p.0).sum::<f64>(), piles.iter().map(|p| p.1 * p.1).sum::<f64>());
        let share = |p: &(f64, f64)| g.cap_vertical / piles.len() as f64 + if sy > 0.0 { g.cap_moment_x * p.1 / sy } else { 0.0 } + if sx > 0.0 { g.cap_moment_y * p.0 / sx } else { 0.0 };
        let half = (columns as f64 * g.spacing / 2.0, rows as f64 * g.spacing / 2.0);
        let mut series = vec![polyline("cap", "Pile cap", "Pfahlkopfplatte", &[[-half.0, -half.1], [half.0, -half.1], [half.0, half.1], [-half.0, half.1], [-half.0, -half.1]], Tone::Neutral, Dash::Dashed)];
        let mut markers = Vec::new();
        let mut twin: Vec<Vec<f64>> = vec![Vec::new(); 5];
        for (k, p) in piles.iter().enumerate() {
            let (reaction, ratio) = (share(p), if g.single_resistance > 0.0 { share(p) / g.single_resistance } else { 0.0 });
            if diameter > 0.0 {
                let ring: Vec<Vec2> = (0..=16).map(|i| { let a = std::f64::consts::TAU * i as f64 / 16.0; Vec2::new(p.0 + 0.5 * diameter * a.cos(), p.1 + 0.5 * diameter * a.sin()) }).collect();
                series.push(Series::Polygon { id: format!("{}:pile{}", g.id, k + 1), label: loc(&format!("Pile {}", k + 1), &format!("Pfahl {}", k + 1)), ring, fill: Fill { tone: utilisation_tone(ratio), opacity: 0.45 }, hatch: None });
            }
            markers.push(ratio_marker(format!("{}:pile{}", g.id, k + 1), [p.0, p.1], MarkerShape::Circle, ratio, loc(&format!("Pile {}: N = {:.0} kN, {}", k + 1, reaction / 1000.0, words(ratio).0), &format!("Pfahl {}: N = {:.0} kN, {}", k + 1, reaction / 1000.0, words(ratio).1)), ctx.binding(&g.id, &[]).map(|b| b.check_id)));
            for (column, value) in twin.iter_mut().zip([(k + 1) as f64, p.0, p.1, reaction / 1000.0, ratio]) {
                column.push(value);
            }
        }
        let [index, x, y, reaction, ratio] = [0, 1, 2, 3, 4].map(|i| std::mem::take(&mut twin[i]));
        Diagram2d {
            id: format!("pile-group-plan:{}", g.id),
            title: loc(&format!("Pile group {}", g.id), &format!("Pfahlgruppe {}", g.id)),
            description: loc("Plan of the pile grid with the compression reaction of every pile from the vertical load and the cap moments on a rigid cap against the design resistance of one pile.", "Grundriss des Pfahlrasters mit der Druckreaktion jedes Pfahls aus Vertikallast und Kopfplattenmomenten auf starrer Platte gegen den Bemessungswiderstand eines Pfahls."),
            axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear), axis("Position y", "Lage y", QuantityKind::Length, "m", AxisScale::Linear)],
            series,
            markers,
            annotations: Vec::new(),
            focus: Some(entity_ref(EntityKind::Group, &g.id, "pileGroups")),
            legend: false,
            table_twin: DataTable { columns: vec![column("Pile", "Pfahl", "1", index), column("Position x", "Lage x", "m", x), column("Position y", "Lage y", "m", y), column("Pile reaction", "Pfahlreaktion", "kN", reaction), column("Utilisation", "Ausnutzung", "1", ratio)] },
        }
    }).collect()
}

/// 🌊️ Head along the flow path of every water row with a head difference and a flow length: the linear head h(x) = Δh (1 − x/L) with the hydraulic gradient i = Δh/L and, when the layer is known, the critical gradient i_crit = γ'/γ_w in the label.
pub(super) fn head_profile(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.layer_waters.iter().filter(|w| w.head_difference > 0.0 && w.flow_length > 0.0 && focus.is_none_or(|f| f == w.id)).map(|w| {
        let gradient = w.head_difference / w.flow_length;
        let critical = d.layers.iter().find(|l| l.id == w.layer_id).map(|l| l.gamma_prime / mech::GAMMA_W).filter(|c| *c > 0.0);
        let xs: Vec<f64> = (0..=STEPS).map(|i| w.flow_length * i as f64 / STEPS as f64).collect();
        let heads: Vec<f64> = xs.iter().map(|x| w.head_difference * (1.0 - x / w.flow_length)).collect();
        let raw: Vec<[f64; 2]> = xs.iter().zip(&heads).map(|(x, h)| [*x, *h]).collect();
        let status = ctx.binding(&w.id, &[]);
        let note = |de: bool| match (critical, de) {
            (Some(c), false) => format!("i = {gradient:.2}, i_crit = {c:.2}"),
            (Some(c), true) => format!("i = {gradient:.2}, i_krit = {c:.2}"),
            (None, _) => format!("i = {gradient:.2}"),
        };
        Diagram2d {
            id: format!("head-profile:{}", w.id),
            title: loc(&format!("Head along the flow path of {}", w.id), &format!("Standrohrspiegelhöhe entlang des Fließwegs von {}", w.id)),
            description: loc("Hydraulic head along the flow path of the water row: linear from the head difference to zero over the flow length; the slope is the hydraulic gradient.", "Standrohrspiegelhöhe entlang des Fließwegs der Wasserzeile: linear von der Potenzialdifferenz auf null über die Fließlänge; die Neigung ist das hydraulische Gefälle."),
            axes: vec![axis("Flow path", "Fließweg", QuantityKind::Length, "m", AxisScale::Linear), axis("Head", "Höhe", QuantityKind::Length, "m", AxisScale::Linear)],
            series: vec![polyline("head", "Hydraulic head h", "Standrohrspiegelhöhe h", &raw, tone_of(&status), Dash::Solid)],
            markers: vec![marker(format!("{}:gradient", w.id), [0.5 * w.flow_length, 0.5 * w.head_difference], MarkerShape::Diamond, tone_of(&status), loc(&note(false), &note(true)), status.as_ref().map(|s| s.check_id.clone()))],
            annotations: Vec::new(),
            focus: Some(entity_ref(EntityKind::Surface, &w.id, "layerWaters")),
            legend: false,
            table_twin: DataTable { columns: vec![column("Flow path", "Fließweg", "m", xs), column("Head", "Höhe", "m", heads)] },
        }
    }).collect()
}

/// ⏳️ Settlement against time of every footing on a logarithmic day axis: the final oedometric settlement under the footing centre times the Terzaghi degree of consolidation, with t50, t90 and the final settlement marked; the coefficient comes from the oedometer series (`cv = T90 h² / t90`, smallest governs) and falls back to the nominal clay permeability only without one.
pub(super) fn consolidation_time(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    use crate::tracks::{degree, laboratory_cv, consolidation, DAY};
    let d = ctx.input.document;
    let moduli: Vec<f64> = d.layers.iter().map(|l| l.oedometric_modulus).collect();
    let lab = laboratory_cv(&d.lab_tests);
    d.footings.iter().filter(|f| f.width > 1e-6 && f.length > 1e-6 && focus.is_none_or(|x| x == f.id)).filter_map(|f| {
        let (cv, path) = consolidation(f, &moduli, lab.as_ref().map(|l| l.0))?;
        let pose = ctx.layout.pose(&f.id);
        let s_final = crate::settlement::settlement_of(ctx, f, pose.origin.x, pose.origin.y) * 1000.0;
        (s_final > 1e-9).then(|| {
            let day = |tv: f64| tv * path * path / cv / DAY;
            let tv: Vec<f64> = (0..=60).map(|i| 10f64.powf(-3.0 + 3.6 * f64::from(i) / 60.0)).collect();
            let u: Vec<f64> = tv.iter().map(|t| degree(*t)).collect();
            let time: Vec<f64> = tv.iter().map(|t| day(*t)).collect();
            let curve: Vec<[f64; 2]> = time.iter().zip(&u).map(|(t, u)| [*t, -s_final * u]).collect();
            let status = ctx.binding(&f.id, &["en1997.6.6"]);
            let mut series = vec![polyline("settlement-time", "Settlement s(t)", "Setzung s(t)", &curve, tone_of(&status), Dash::Solid), Series::Limit { id: "final".into(), label: loc("Final settlement", "Endsetzung"), axis: AxisRef::Y, value: -s_final, style: stroke(Tone::Neutral, Dash::Dashed) }];
            if f.settlement_limit > 0.0 {
                series.push(Series::Limit { id: "limit".into(), label: loc("Settlement limit", "Setzungsgrenze"), axis: AxisRef::Y, value: -f.settlement_limit * 1000.0, style: stroke(Tone::Error, Dash::Dashed) });
            }
            let markers = Vec::from([(0.197, 0.5, "t50"), (crate::tracks::t90(), 0.9, "t90")].map(|(t, share, name)| marker(format!("{}:{name}", f.id), [day(t), -s_final * share], MarkerShape::Diamond, Tone::Info, loc(&format!("{name} = {:.0} d, s = {:.1} mm", day(t), s_final * share), &format!("{name} = {:.0} d, s = {:.1} mm", day(t), s_final * share)), status.as_ref().map(|s| s.check_id.clone()))));
            let source = lab.as_ref().map_or_else(|| ("the nominal clay permeability (unverified)".to_string(), "der nominellen Tonpermeabilität (ungeprüft)".to_string()), |(cv, id)| (format!("oedometer series {id} (c_v = {cv:.2e} m2/s)"), format!("Ödometerreihe {id} (c_v = {cv:.2e} m2/s)")));
            Diagram2d {
                id: format!("consolidation-time:{}", f.id),
                title: loc(&format!("Consolidation settlement of {} over time", f.id), &format!("Konsolidationssetzung von {} über die Zeit", f.id)),
                description: loc(&format!("Settlement under the footing centre against time (logarithmic days) with the Terzaghi series, drainage path {path:.1} m, coefficient from {}; t50 and t90 are marked.", source.0), &format!("Setzung unter dem Fundamentmittelpunkt über die Zeit (logarithmische Tage) mit der Terzaghi-Reihe, Dränweg {path:.1} m, Beiwert aus {}; t50 und t90 sind markiert.", source.1)),
                axes: vec![axis("Time", "Zeit", QuantityKind::Time, "d", AxisScale::Log10), axis("Settlement (downwards negative)", "Setzung (nach unten negativ)", QuantityKind::Length, "mm", AxisScale::Linear)],
                series,
                markers,
                annotations: Vec::new(),
                focus: Some(entity_ref(EntityKind::Solid, &f.id, "footings")),
                legend: true,
                table_twin: DataTable { columns: vec![column("Time factor", "Zeitbeiwert", "1", tv), column("Time", "Zeit", "d", time), column("Degree of consolidation", "Konsolidationsgrad", "1", u.clone()), column("Settlement (downwards negative)", "Setzung (nach unten negativ)", "mm", u.iter().map(|u| -s_final * u).collect())] },
            }
        })
    }).collect()
}

/// 🪵️ Section through every pile group with its equivalent raft: the piles, the load spread lines from the outer pile edges at the pile head to the raft edges at two thirds of the pile length, the raft, the layer interfaces and the group settlement under the raft centre.
pub(super) fn pile_group_raft(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let d = ctx.input.document;
    d.pile_groups.iter().filter(|g| g.rows > 0 && g.columns > 0 && g.spacing > 0.0 && focus.is_none_or(|f| f == g.id)).filter_map(|g| {
        let pile = d.piles.iter().find(|p| p.id == g.pile_id).filter(|p| p.length > 0.0 && p.diameter > 0.0)?;
        let raft = mech::equivalent_raft(g, pile);
        let s = mech::group_settlement(g, pile, &d.layers) * 1000.0;
        let pressure = g.cap_vertical / (raft.width * raft.length).max(1e-9);
        let outer = ((f64::from(g.columns) - 1.0) * g.spacing + pile.diameter) / 2.0;
        let ratio = worst_ratio(ctx.input.report, &g.id);
        let mut series = vec![polyline("cap", "Pile head", "Pfahlkopf", &[[-outer, 0.0], [outer, 0.0]], Tone::Neutral, Dash::Solid)];
        for c in 0..g.columns {
            let x = (f64::from(c) - (f64::from(g.columns) - 1.0) / 2.0) * g.spacing;
            series.push(polyline(&format!("{}:pile{}", g.id, c + 1), &format!("Pile column {}", c + 1), &format!("Pfahlspalte {}", c + 1), &[[x, 0.0], [x, -pile.length]], Tone::Neutral, Dash::Solid));
        }
        series.push(polyline("spread-left", "Load spread (left)", "Lastausbreitung (links)", &[[-outer, 0.0], [-raft.width / 2.0, -raft.depth]], Tone::Info, Dash::Dashed));
        series.push(polyline("spread-right", "Load spread (right)", "Lastausbreitung (rechts)", &[[outer, 0.0], [raft.width / 2.0, -raft.depth]], Tone::Info, Dash::Dashed));
        series.push(polyline("raft", "Equivalent raft", "Ersatzfundament", &[[-raft.width / 2.0, -raft.depth], [raft.width / 2.0, -raft.depth]], utilisation_tone(ratio), Dash::Solid));
        for (k, layer) in d.layers.iter().enumerate().filter(|(_, l)| l.depth_bottom > 0.0).take(8) {
            series.push(Series::Limit { id: format!("layer{}", k + 1), label: loc(&format!("Bottom of layer {}", layer.id), &format!("Unterkante Schicht {}", layer.id)), axis: AxisRef::Y, value: -layer.depth_bottom, style: stroke(Tone::Neutral, Dash::Dotted) });
        }
        let markers = vec![ratio_marker(format!("{}:raft-centre", g.id), [0.0, -raft.depth], MarkerShape::Triangle, ratio, loc(&format!("Group settlement {s:.1} mm under {:.0} kPa, {}", pressure / 1000.0, words(ratio).0), &format!("Gruppensetzung {s:.1} mm unter {:.0} kPa, {}", pressure / 1000.0, words(ratio).1)), ctx.binding(&g.id, &[]).map(|b| b.check_id))];
        Some(Diagram2d {
            id: format!("pile-group-raft:{}", g.id),
            title: loc(&format!("Equivalent raft of pile group {}", g.id), &format!("Ersatzfundament der Pfahlgruppe {}", g.id)),
            description: loc("Section through the pile group: load spread 1:4 from the outer pile edges to two thirds of the pile length (textbook convention, not an EN 1997 value) and the Boussinesq settlement of the ground below the raft.", "Schnitt durch die Pfahlgruppe: Lastausbreitung 1:4 von den äußeren Pfahlkanten bis zu zwei Dritteln der Pfahllänge (Lehrbuchkonvention, kein EN-1997-Wert) und die Boussinesq-Setzung des Bodens unter dem Ersatzfundament."),
            axes: vec![axis("Position x", "Lage x", QuantityKind::Length, "m", AxisScale::Linear), axis("Depth below the pile head (downwards negative)", "Tiefe unter Pfahlkopf (nach unten negativ)", QuantityKind::Length, "m", AxisScale::Linear)],
            series,
            markers,
            annotations: Vec::new(),
            focus: Some(entity_ref(EntityKind::Group, &g.id, "pileGroups")),
            legend: false,
            table_twin: DataTable { columns: vec![column("Raft width", "Breite Ersatzfundament", "m", vec![raft.width]), column("Raft length", "Länge Ersatzfundament", "m", vec![raft.length]), column("Raft depth", "Tiefe Ersatzfundament", "m", vec![raft.depth]), column("Raft pressure", "Pressung Ersatzfundament", "kPa", vec![pressure / 1000.0]), column("Group settlement", "Gruppensetzung", "mm", vec![s])] },
        })
    }).collect()
}

/// 📊️ The outcome diagram of every row owning checks of a family no dedicated diagram draws ([`OUTCOME_FAMILIES`](crate::kit::OUTCOME_FAMILIES)): one bar per check at its utilisation against the limit line at 1, coloured by the shared utilisation ramp, with the check titles as legend and table twin.
pub(super) fn family_outcomes(ctx: &Ctx, focus: Option<&str>) -> Vec<Diagram2d> {
    let (d, report) = (ctx.input.document, ctx.input.report);
    let mut rows: std::collections::BTreeMap<String, Vec<&crate::norm::CheckResult>> = std::collections::BTreeMap::new();
    for check in report.checks.iter().filter(|c| c.utilization.is_finite() && c.status != crate::norm::CheckStatus::NotApplicable && crate::kit::OUTCOME_FAMILIES.iter().any(|p| c.id.starts_with(p))) {
        if let Some(id) = crate::kit::owner_entity(d, check).filter(|id| focus.is_none_or(|f| f == id)) {
            rows.entry(id).or_default().push(check);
        }
    }
    rows.into_iter().filter_map(|(id, checks)| {
        let spec = crate::kit::entity_kind_of(d, &id)?;
        let worst = checks.iter().map(|c| c.utilization).fold(0.0, f64::max);
        let mut series: Vec<Series> = checks.iter().enumerate().map(|(i, c)| bar(&format!("outcome-{}", i + 1), &c.title.en, &c.title.de, (i as f64 + 0.6, i as f64 + 1.4), (0.0, c.utilization), utilisation_tone(c.utilization))).collect();
        series.push(Series::Limit { id: "limit".into(), label: loc("Limit of the check (utilisation 1)", "Grenze des Nachweises (Ausnutzung 1)"), axis: AxisRef::Y, value: 1.0, style: stroke(Tone::Error, Dash::Dashed) });
        let markers = checks.iter().enumerate().map(|(i, c)| ratio_marker(format!("{id}:outcome-{}", i + 1), [i as f64 + 1.0, c.utilization], MarkerShape::Diamond, c.utilization, loc(&format!("{}: {}", c.title.en, words(c.utilization).0), &format!("{}: {}", c.title.de, words(c.utilization).1)), Some(c.id.clone()))).collect();
        Some(Diagram2d {
            id: format!("check-outcomes:{id}"),
            title: loc(&format!("Check outcomes of {id}, worst utilisation {worst:.2}"), &format!("Nachweisergebnisse von {id}, größte Ausnutzung {worst:.2}")),
            description: loc("Utilisation of every check of this row whose family has no dedicated diagram, as bars in check order against the limit at 1; the legend names each check.", "Ausnutzung jedes Nachweises dieser Zeile, dessen Familie kein eigenes Diagramm hat, als Balken in Nachweisreihenfolge gegen die Grenze bei 1; die Legende benennt jeden Nachweis."),
            axes: vec![axis("Check", "Nachweis", QuantityKind::Dimensionless, "1", AxisScale::Linear), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", AxisScale::Linear)],
            series,
            markers,
            annotations: Vec::new(),
            focus: Some(entity_ref(spec.scene_kind, &id, spec.collection)),
            legend: true,
            table_twin: DataTable { columns: vec![column("Check", "Nachweis", "1", (1..=checks.len()).map(|i| i as f64).collect()), column("Utilisation", "Ausnutzung", "1", checks.iter().map(|c| c.utilization).collect())] },
        })
    }).collect()
}
