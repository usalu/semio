//! 📈️ Diagrams: k_mod over load-duration class and service class, embedment strength over the fastener diameter, Johansen failure-mode bars, required against provided spacings, the buckling factors k_c and k_crit with the member markers,
//! the deflection components (instantaneous, creep, final), the char depth over the fire duration, and the floor frequency and velocity response or the footbridge frequency over the span.

extern crate semio_s_artifact_norm_en1995_model as model;
extern crate semio_s_artifact_norm_en1995_scene_entities as entities;
extern crate semio_s_artifact_norm_en1995_scene_facts as facts;
extern crate semio_s_artifact_norm_en1995_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;
extern crate semio_s_plugin_norm_structural_scene as scene;

use entities::worst;
use facts::Dynamics;
use scene::projection_kit::loc;
use model::{CaseRow, En1995Snapshot, TimberConnection, TimberMember, TimberProperties};
use norm::CheckReport;
use schema::checks::timber as t;
use scene::diagram_kit::{curves, Xy};
use scene::*;
use scene::projection_kit::diagram_parts::{column, stroke};

const SAMPLES: usize = 60;
const LAMBDA_MAX: f64 = 3.0;

fn p(point: Xy) -> Vec2 {
    Vec2::new(point[0], point[1])
}

fn axis(en: &str, de: &str, quantity: QuantityKind, unit: &str, domain: Option<(f64, f64)>, ticks: TickRule) -> Axis {
    Axis { label: loc(en, de), quantity, unit: unit.to_string(), scale: AxisScale::Linear, domain: domain.map(|(lo, hi)| Interval::new(lo, hi)), ticks }
}

fn utilisation_tone(ratio: f64) -> Tone {
    scene::utilisation_tone(ratio)
}

fn polyline(id: &str, en: &str, de: &str, points: Vec<Xy>, style: Stroke) -> Series {
    Series::Polyline { id: id.into(), label: loc(en, de), points: points.into_iter().map(p).collect(), style }
}

fn bar(id: &str, en: &str, de: &str, x0: f64, x1: f64, height: f64, tone: Tone) -> Series {
    Series::Polygon { id: id.into(), label: loc(en, de), ring: vec![Vec2::new(x0, 0.0), Vec2::new(x1, 0.0), Vec2::new(x1, height), Vec2::new(x0, height)], fill: Fill { tone, opacity: 0.55 }, hatch: (tone == Tone::Error).then_some(Hatch::Diagonal) }
}

fn focus_member(m: &TimberMember) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Member, &m.id, format!("members[id={}]", m.id)))
}

fn focus_connection(c: &TimberConnection) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Group, &c.id, format!("connections[id={}]", c.id)))
}

fn binding(path: String, unit: &str, map: Affine, step: f64) -> FieldBinding {
    FieldBinding { path, map, unit: unit.to_string(), quantise: Some(Quantise::Round { step }) }
}

fn kmod(report: &CheckReport, m: &TimberMember, props: &TimberProperties) -> Option<Diagram2d> {
    let key = t::product_key(props.product);
    let series_of = |sc: u8| -> Vec<Xy> { t::DURATIONS.iter().enumerate().filter_map(|(i, d)| t::k_mod_for(key, sc, *d).map(|k| [(i + 1) as f64, k])).collect() };
    let classes = [1u8, 2, 3].map(|sc| (sc, series_of(sc)));
    if classes.iter().any(|(_, s)| s.len() < 2) {
        return None;
    }
    let duration = m.actions.iter().map(|a| t::load_duration_of(a.load_duration)).max()?;
    let index = t::DURATIONS.iter().position(|d| *d == duration)? + 1;
    let own = classes.iter().find(|(sc, _)| *sc == m.service_class.clamp(1, 3))?;
    let at = own.1.iter().find(|q| q[0] as usize == index)?;
    let check = worst(report, &m.id, "en1995.6.1.6.");
    let series = classes.iter().map(|(sc, points)| polyline(&format!("sc{sc}"), &format!("Service class {sc}"), &format!("Nutzungsklasse {sc}"), points.clone(), stroke(if *sc == own.0 { Tone::Info } else { Tone::Neutral }, if *sc == own.0 { 2.5 } else { 1.0 }, if *sc == own.0 { Dash::Solid } else { Dash::Dashed }))).collect();
    let mut table = vec![column("Load-duration class (1 permanent … 5 instantaneous)", "Klasse der Lasteinwirkungsdauer (1 ständig … 5 kurzzeitig)", "1", (1..=5).map(f64::from).collect())];
    table.extend(classes.iter().map(|(sc, points)| column(&format!("k_mod, service class {sc}"), &format!("k_mod, Nutzungsklasse {sc}"), "1", points.iter().map(|q| q[1]).collect())));
    Some(Diagram2d {
        id: format!("kmod:{}", m.id),
        title: loc(&format!("Modification factor k_mod – {}", m.label_en), &format!("Modifikationsbeiwert k_mod – {}", m.label_de)),
        description: loc("k_mod of EN 1995-1-1 Table 3.1 over the load-duration class for the three service classes; the marker is the shortest action of this member.", "k_mod nach EN 1995-1-1 Tabelle 3.1 über der Klasse der Lasteinwirkungsdauer für die drei Nutzungsklassen; der Marker ist die kürzeste Einwirkung dieses Bauteils."),
        axes: vec![axis("Load-duration class", "Klasse der Lasteinwirkungsdauer", QuantityKind::Dimensionless, "1", Some((1.0, 5.0)), TickRule::Explicit { values: vec![1.0, 2.0, 3.0, 4.0, 5.0] }), axis("k_mod", "k_mod", QuantityKind::Dimensionless, "1", Some((0.0, 1.2)), TickRule::Auto)],
        series,
        markers: vec![Marker { id: "marker-kmod".into(), at: p(*at), shape: MarkerShape::Circle, tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)), label: loc(&format!("k_mod = {:.2}", at[1]), &format!("k_mod = {:.2}", at[1])), utilisation: check.map(|c| c.utilization), bound_check: check.map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: true,
        table_twin: DataTable { columns: table },
    })
}

fn buckling(report: &CheckReport, m: &TimberMember, props: &TimberProperties) -> Option<Diagram2d> {
    let s = facts::slenderness(m, props);
    let check = worst(report, &m.id, "en1995.6.3.2.");
    let sample = |beta: f64| curves::sample_with_breaks(|l| t::kc_factor(l, beta), 0.0, LAMBDA_MAX, SAMPLES, &[0.3]);
    let (solid, laminated) = (sample(0.2), sample(0.1));
    let own = if s.beta_c > 0.15 { 0.2 } else { 0.1 };
    let marker = |tag: &str, rel: f64, i: f64, leaf: &str, shape: MarkerShape| Marker {
        id: format!("marker-{tag}"),
        at: Vec2::new(rel.min(LAMBDA_MAX), t::kc_factor(rel, own)),
        shape,
        tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)),
        label: loc(&format!("{tag}–{tag}: λ_rel = {rel:.2}, k_c = {:.3}", t::kc_factor(rel, own)), &format!("{tag}–{tag}: λ_rel = {rel:.2}, k_c = {:.3}", t::kc_factor(rel, own))),
        utilisation: check.map(|c| c.utilization),
        bound_check: check.map(|c| c.id.clone()),
        binding: Some(binding(format!("members[id={}].{leaf}", m.id), "m", Affine { base: 0.0, k: i / s.scale }, 0.05)),
    };
    Some(Diagram2d {
        id: format!("kc:{}", m.id),
        title: loc(&format!("Column buckling factor k_c – {}", m.label_en), &format!("Knickbeiwert k_c – {}", m.label_de)),
        description: loc("k_c over the relative slenderness λ_rel (EN 1995-1-1, 6.3.2) for solid timber (β_c = 0.2) and glued products (β_c = 0.1); the markers are this member about both axes, draggable to change the buckling length.", "k_c über dem bezogenen Schlankheitsgrad λ_rel (EN 1995-1-1, 6.3.2) für Vollholz (β_c = 0,2) und Verbundprodukte (β_c = 0,1); die Marker sind dieses Bauteil um beide Achsen und ändern beim Ziehen die Knicklänge."),
        axes: vec![axis("Relative slenderness λ_rel", "Bezogener Schlankheitsgrad λ_rel", QuantityKind::Dimensionless, "1", Some((0.0, LAMBDA_MAX)), TickRule::Auto), axis("Buckling factor k_c", "Knickbeiwert k_c", QuantityKind::Dimensionless, "1", Some((0.0, 1.1)), TickRule::Auto)],
        series: vec![
            polyline("kc-solid", "Solid timber β_c = 0.2", "Vollholz β_c = 0,2", solid.clone(), stroke(if own > 0.15 { Tone::Info } else { Tone::Neutral }, if own > 0.15 { 2.5 } else { 1.0 }, Dash::Solid)),
            polyline("kc-glued", "Glulam, LVL β_c = 0.1", "Brettschichtholz, Furnierschichtholz β_c = 0,1", laminated.clone(), stroke(if own > 0.15 { Tone::Neutral } else { Tone::Info }, if own > 0.15 { 1.0 } else { 2.5 }, Dash::Solid)),
            polyline("euler", "Euler hyperbola 1/λ_rel²", "Euler-Hyperbel 1/λ_rel²", curves::sample(|l| 1.0 / (l * l), 1.0, LAMBDA_MAX, SAMPLES), stroke(Tone::Neutral, 1.0, Dash::Dotted)),
        ],
        markers: vec![marker("y", s.rel_y, s.i_y, "bucklingLengthYM", MarkerShape::Circle), marker("z", s.rel_z, s.i_z, "bucklingLengthZM", MarkerShape::Diamond)],
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Relative slenderness λ_rel", "Bezogener Schlankheitsgrad λ_rel", "1", solid.iter().map(|q| q[0]).collect()),
                column("k_c solid timber", "k_c Vollholz", "1", solid.iter().map(|q| q[1]).collect()),
                column("k_c glulam, LVL", "k_c Brettschichtholz, Furnierschichtholz", "1", laminated.iter().map(|q| q[1]).collect()),
            ],
        },
    })
}

fn kcrit(report: &CheckReport, m: &TimberMember, props: &TimberProperties) -> Option<Diagram2d> {
    let (rel, factor) = facts::lateral(m, props);
    let check = worst(report, &m.id, "en1995.6.1.6.");
    let points = curves::sample_with_breaks(t::kcrit_factor, 0.0, LAMBDA_MAX, SAMPLES, &[0.75, 1.4]);
    Some(Diagram2d {
        id: format!("kcrit:{}", m.id),
        title: loc(&format!("Lateral buckling factor k_crit – {}", m.label_en), &format!("Kippbeiwert k_crit – {}", m.label_de)),
        description: loc("k_crit over the relative slenderness in bending λ_rel,m (EN 1995-1-1, 6.3.3); the marker is this member with its restraint spacing.", "k_crit über dem bezogenen Biegeschlankheitsgrad λ_rel,m (EN 1995-1-1, 6.3.3); der Marker ist dieses Bauteil mit seinem Halterungsabstand."),
        axes: vec![axis("Relative slenderness λ_rel,m", "Bezogener Schlankheitsgrad λ_rel,m", QuantityKind::Dimensionless, "1", Some((0.0, LAMBDA_MAX)), TickRule::Auto), axis("Lateral buckling factor k_crit", "Kippbeiwert k_crit", QuantityKind::Dimensionless, "1", Some((0.0, 1.1)), TickRule::Auto)],
        series: vec![polyline("kcrit", "k_crit", "k_crit", points.clone(), stroke(Tone::Info, 2.0, Dash::Solid))],
        markers: vec![Marker { id: "marker-kcrit".into(), at: Vec2::new(rel.min(LAMBDA_MAX), factor), shape: MarkerShape::Diamond, tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)), label: loc(&format!("λ_rel,m = {rel:.2}, k_crit = {factor:.3}"), &format!("λ_rel,m = {rel:.2}, k_crit = {factor:.3}")), utilisation: check.map(|c| c.utilization), bound_check: check.map(|c| c.id.clone()), binding: None }],
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: false,
        table_twin: DataTable { columns: vec![column("Relative slenderness λ_rel,m", "Bezogener Schlankheitsgrad λ_rel,m", "1", points.iter().map(|q| q[0]).collect()), column("k_crit", "k_crit", "1", points.iter().map(|q| q[1]).collect())] },
    })
}

fn deflection(d: &En1995Snapshot, report: &CheckReport, m: &TimberMember, props: &TimberProperties) -> Option<Diagram2d> {
    let w = facts::deflection(d.annex, d.reliability_class, m, props)?;
    let mm = |v: f64| v * 1e3;
    let values = [("inst-g", "Instantaneous permanent w_inst,G", "Sofortverformung ständig w_inst,G", w.w_g), ("inst-q", "Instantaneous variable w_inst,Q", "Sofortverformung veränderlich w_inst,Q", w.w_q), ("creep", "Creep w_creep", "Kriechen w_creep", w.creep), ("final", "Final w_fin", "Endverformung w_fin", w.w_fin)];
    let inst = worst(report, &m.id, "en1995.7.2.winst.");
    let fin = worst(report, &m.id, "en1995.7.2.wfin.");
    let tone_of = |index: usize| match index {
        1 => inst.map_or(Tone::Info, |c| utilisation_tone(c.utilization)),
        3 => fin.map_or(Tone::Info, |c| utilisation_tone(c.utilization)),
        _ => Tone::Neutral,
    };
    let series: Vec<Series> = values.iter().enumerate().map(|(i, (id, en, de, v))| bar(id, en, de, i as f64 + 0.6, i as f64 + 1.4, mm(*v), tone_of(i))).collect();
    let markers = [(1usize, inst), (3, fin)].into_iter().map(|(i, c)| Marker { id: format!("marker-{}", values[i].0), at: Vec2::new(i as f64 + 1.0, mm(values[i].3)), shape: MarkerShape::Triangle, tone: tone_of(i), label: loc(&format!("{:.1} mm", mm(values[i].3)), &format!("{:.1} mm", mm(values[i].3))), utilisation: c.map(|c| c.utilization), bound_check: c.map(|c| c.id.clone()), binding: None }).collect();
    let mut all = series;
    all.push(Series::Limit { id: "limit-inst".into(), label: loc("Limit w_inst,Q", "Grenze w_inst,Q"), axis: AxisRef::Y, value: mm(w.limit_inst), style: stroke(Tone::Warning, 1.5, Dash::Dashed) });
    all.push(Series::Limit { id: "limit-fin".into(), label: loc("Limit w_fin", "Grenze w_fin"), axis: AxisRef::Y, value: mm(w.limit_fin), style: stroke(Tone::Error, 1.5, Dash::Dashed) });
    let top = mm(w.w_fin.max(w.limit_fin).max(w.limit_inst)) * 1.15;
    Some(Diagram2d {
        id: format!("deflection:{}", m.id),
        title: loc(&format!("Deflection components – {}", m.label_en), &format!("Verformungsanteile – {}", m.label_de)),
        description: loc("Instantaneous deflection of the permanent and variable part, creep deformation and the final deflection w_fin = w_G(1 + k_def) + w_Q(1 + ψ₂ k_def) against the span limits (EN 1995-1-1, 7.2).", "Sofortverformung aus ständigem und veränderlichem Anteil, Kriechverformung und Endverformung w_fin = w_G(1 + k_def) + w_Q(1 + ψ₂ k_def) gegen die Stützweitengrenzen (EN 1995-1-1, 7.2)."),
        axes: vec![axis("Component (1 w_inst,G, 2 w_inst,Q, 3 w_creep, 4 w_fin)", "Anteil (1 w_inst,G, 2 w_inst,Q, 3 w_creep, 4 w_fin)", QuantityKind::Dimensionless, "1", Some((0.5, 4.5)), TickRule::Explicit { values: vec![1.0, 2.0, 3.0, 4.0] }), axis("Deflection", "Durchbiegung", QuantityKind::Length, "mm", Some((0.0, top.max(1.0))), TickRule::Auto)],
        series: all,
        markers,
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: true,
        table_twin: DataTable { columns: vec![column("Component", "Anteil", "1", (1..=4).map(f64::from).collect()), column("Deflection", "Durchbiegung", "mm", values.iter().map(|v| mm(v.3)).collect())] },
    })
}

fn fire(d: &En1995Snapshot, report: &CheckReport, m: &TimberMember, props: &TimberProperties) -> Option<Diagram2d> {
    let now = facts::fire_of(d.annex, m, props)?;
    let check = worst(report, &m.id, "en1995.1-2.");
    let end = (now.minutes * 1.5).max(90.0);
    let char_curve = curves::sample(|t| facts::fire_after(d.annex, props.product, m.b_m, m.h_m, t * 60.0).d_char * 1e3, 0.0, end, SAMPLES);
    let eff_curve = curves::sample(|t| facts::fire_after(d.annex, props.product, m.b_m, m.h_m, t * 60.0).d_ef * 1e3, 0.0, end, SAMPLES);
    let residual = curves::sample(|t| facts::fire_after(d.annex, props.product, m.b_m, m.h_m, t * 60.0).h_fi * 1e3, 0.0, end, SAMPLES);
    Some(Diagram2d {
        id: format!("fire:{}", m.id),
        title: loc(&format!("Char depth over fire duration – {}", m.label_en), &format!("Abbrandtiefe über der Branddauer – {}", m.label_de)),
        description: loc("Char depth d_char, effective depth d_ef with the zero-strength layer and the residual depth over the fire duration (EN 1995-1-2, 4.2); drag the marker to change the duration.", "Abbrandtiefe d_char, wirksame Tiefe d_ef mit Nullfestigkeitsschicht und Resthöhe über der Branddauer (EN 1995-1-2, 4.2); der Marker ändert beim Ziehen die Branddauer."),
        axes: vec![axis("Fire duration", "Branddauer", QuantityKind::Time, "min", Some((0.0, end)), TickRule::Auto), axis("Depth", "Tiefe", QuantityKind::Length, "mm", None, TickRule::Auto)],
        series: vec![
            polyline("d-char", "Char depth d_char", "Abbrandtiefe d_char", char_curve.clone(), stroke(Tone::Neutral, 1.5, Dash::Solid)),
            polyline("d-ef", "Effective depth d_ef", "Wirksame Tiefe d_ef", eff_curve.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)),
            polyline("h-fi", "Residual depth h_fi", "Resthöhe h_fi", residual.clone(), stroke(Tone::Info, 2.0, Dash::Dashed)),
        ],
        markers: vec![Marker { id: "marker-fire".into(), at: Vec2::new(now.minutes, now.d_ef * 1e3), shape: MarkerShape::Circle, tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)), label: loc(&format!("{:.0} min: d_ef = {:.1} mm", now.minutes, now.d_ef * 1e3), &format!("{:.0} min: d_ef = {:.1} mm", now.minutes, now.d_ef * 1e3)), utilisation: check.map(|c| c.utilization), bound_check: check.map(|c| c.id.clone()), binding: Some(binding(format!("members[id={}].fireDurationS", m.id), "s", Affine { base: 0.0, k: 60.0 }, 60.0)) }],
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: true,
        table_twin: DataTable {
            columns: vec![
                column("Fire duration", "Branddauer", "min", char_curve.iter().map(|q| q[0]).collect()),
                column("Char depth d_char", "Abbrandtiefe d_char", "mm", char_curve.iter().map(|q| q[1]).collect()),
                column("Effective depth d_ef", "Wirksame Tiefe d_ef", "mm", eff_curve.iter().map(|q| q[1]).collect()),
                column("Residual depth h_fi", "Resthöhe h_fi", "mm", residual.iter().map(|q| q[1]).collect()),
            ],
        },
    })
}

fn vibration(report: &CheckReport, m: &TimberMember, dynamics: &Dynamics) -> Vec<Diagram2d> {
    let end = (m.span_m * 2.0).max(12.0);
    let start = 2.0;
    let f1 = curves::sample(|l| dynamics.f1(l), start, end, SAMPLES);
    let check = worst(report, &m.id, if dynamics.bridge { "en1995.2.b." } else { "en1995.7.3.f1." });
    let now = dynamics.f1(m.span_m.max(start));
    let span_binding = binding(format!("members[id={}].spanM", m.id), "m", Affine::IDENTITY, 0.05);
    let mut out = vec![Diagram2d {
        id: format!("frequency:{}", m.id),
        title: loc(&format!("Fundamental frequency over span – {}", m.label_en), &format!("Eigenfrequenz über der Stützweite – {}", m.label_de)),
        description: if dynamics.bridge {
            loc("First bending frequency f₁ of the footbridge over the span against the 5 Hz limit above which the vertical pedestrian check is waived (EN 1995-2, 7.3).", "Erste Biegeeigenfrequenz f₁ des Stegs über der Stützweite gegen die 5-Hz-Grenze, oberhalb derer der vertikale Fußgängernachweis entfällt (EN 1995-2, 7.3).")
        } else {
            loc("Fundamental floor frequency f₁ = π/(2ℓ²)·√((EI)_ℓ/m) over the span against the national limit (EN 1995-1-1, 7.3.3).", "Eigenfrequenz der Decke f₁ = π/(2ℓ²)·√((EI)_ℓ/m) über der Stützweite gegen die nationale Grenze (EN 1995-1-1, 7.3.3).")
        },
        axes: vec![axis("Span", "Stützweite", QuantityKind::Length, "m", Some((start, end)), TickRule::Auto), axis("Frequency f₁", "Eigenfrequenz f₁", QuantityKind::Frequency, "Hz", None, TickRule::Auto)],
        series: vec![polyline("f1", "f₁", "f₁", f1.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), Series::Limit { id: "f1-limit".into(), label: loc("Frequency limit", "Frequenzgrenze"), axis: AxisRef::Y, value: dynamics.f1_limit, style: stroke(Tone::Error, 1.5, Dash::Dashed) }],
        markers: vec![Marker { id: "marker-f1".into(), at: Vec2::new(m.span_m.max(start), now), shape: MarkerShape::Circle, tone: check.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)), label: loc(&format!("f₁ = {now:.2} Hz"), &format!("f₁ = {now:.2} Hz")), utilisation: check.map(|c| c.utilization), bound_check: check.map(|c| c.id.clone()), binding: Some(span_binding.clone()) }],
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: true,
        table_twin: DataTable { columns: vec![column("Span", "Stützweite", "m", f1.iter().map(|q| q[0]).collect()), column("Frequency f₁", "Eigenfrequenz f₁", "Hz", f1.iter().map(|q| q[1]).collect())] },
    }];
    if dynamics.bridge {
        return out;
    }
    let velocity = curves::sample(|l| dynamics.velocity(l) * 1e3, start, end, SAMPLES);
    let limit = curves::sample(|l| dynamics.velocity_limit(l) * 1e3, start, end, SAMPLES);
    let vcheck = worst(report, &m.id, "en1995.7.3.velocity.");
    let v_now = dynamics.velocity(m.span_m.max(start)) * 1e3;
    out.push(Diagram2d {
        id: format!("velocity:{}", m.id),
        title: loc(&format!("Unit impulse velocity response – {}", m.label_en), &format!("Einheitsimpuls-Geschwindigkeitsantwort – {}", m.label_de)),
        description: loc("Unit impulse velocity response v over the span against the limit b^(f₁ζ − 1) (EN 1995-1-1, 7.3.3, equations 7.4 to 7.6).", "Einheitsimpuls-Geschwindigkeitsantwort v über der Stützweite gegen die Grenze b^(f₁ζ − 1) (EN 1995-1-1, 7.3.3, Gleichungen 7.4 bis 7.6)."),
        axes: vec![axis("Span", "Stützweite", QuantityKind::Length, "m", Some((start, end)), TickRule::Auto), axis("Velocity v", "Geschwindigkeit v", QuantityKind::Dimensionless, "1", None, TickRule::Auto)],
        series: vec![polyline("v", "v", "v", velocity.clone(), stroke(Tone::Info, 2.0, Dash::Solid)), polyline("v-limit", "Limit b^(f₁ζ − 1)", "Grenze b^(f₁ζ − 1)", limit.clone(), stroke(Tone::Error, 1.5, Dash::Dashed))],
        markers: vec![Marker { id: "marker-v".into(), at: Vec2::new(m.span_m.max(start), v_now), shape: MarkerShape::Diamond, tone: vcheck.map_or(Tone::Neutral, |c| utilisation_tone(c.utilization)), label: loc(&format!("v = {v_now:.3}"), &format!("v = {v_now:.3}")), utilisation: vcheck.map(|c| c.utilization), bound_check: vcheck.map(|c| c.id.clone()), binding: Some(span_binding) }],
        annotations: Vec::new(),
        focus: focus_member(m),
        legend: true,
        table_twin: DataTable { columns: vec![column("Span", "Stützweite", "m", velocity.iter().map(|q| q[0]).collect()), column("Velocity v", "Geschwindigkeit v", "1", velocity.iter().map(|q| q[1]).collect()), column("Limit", "Grenze", "1", limit.iter().map(|q| q[1]).collect())] },
    });
    out
}

fn embedment(c: &TimberConnection, props: &TimberProperties) -> Diagram2d {
    let kind = facts::fastener_kind(c);
    let fh = |d: f64| t::embedment_fh(kind, "timber", props.rho_k, d, 0.0, "softwood", 0.0);
    let d_now = c.diameter_m * 1e3;
    let points = curves::sample(&fh, 3.0, 30.0, SAMPLES);
    Diagram2d {
        id: format!("embedment:{}", c.id),
        title: loc(&format!("Embedment strength – {}", c.label_en), &format!("Lochleibungsfestigkeit – {}", c.label_de)),
        description: loc("Characteristic embedment strength f_h,k over the fastener diameter for the timber of this connection (EN 1995-1-1, 8.2.2 and 8.3); the marker is the diameter of the connection.", "Charakteristische Lochleibungsfestigkeit f_h,k über dem Durchmesser des Verbindungsmittels für das Holz dieser Verbindung (EN 1995-1-1, 8.2.2 und 8.3); der Marker ist der Durchmesser der Verbindung."),
        axes: vec![axis("Diameter d", "Durchmesser d", QuantityKind::Length, "mm", Some((3.0, 30.0)), TickRule::Auto), axis("Embedment strength f_h,k", "Lochleibungsfestigkeit f_h,k", QuantityKind::Stress, "N/mm²", None, TickRule::Auto)],
        series: vec![polyline("fh", "f_h,k", "f_h,k", points.clone(), stroke(Tone::Info, 2.0, Dash::Solid))],
        markers: vec![Marker { id: "reference:marker-d".into(), at: Vec2::new(d_now.clamp(3.0, 30.0), fh(d_now)), shape: MarkerShape::Circle, tone: Tone::Info, label: loc(&format!("d = {d_now:.1} mm, f_h,k = {:.1} N/mm²", fh(d_now)), &format!("d = {d_now:.1} mm, f_h,k = {:.1} N/mm²", fh(d_now))), utilisation: None, bound_check: None, binding: Some(binding(format!("connections[id={}].diameterM", c.id), "m", Affine { base: 0.0, k: 1e-3 }, 0.0005)) }],
        annotations: Vec::new(),
        focus: focus_connection(c),
        legend: false,
        table_twin: DataTable { columns: vec![column("Diameter d", "Durchmesser d", "mm", points.iter().map(|q| q[0]).collect()), column("Embedment strength f_h,k", "Lochleibungsfestigkeit f_h,k", "N/mm²", points.iter().map(|q| q[1]).collect())] },
    }
}

fn johansen(report: &CheckReport, c: &TimberConnection) -> Option<Diagram2d> {
    let j = facts::johansen(c)?;
    let check = worst(report, &c.id, "en1995.8.2");
    let series = j.modes.iter().enumerate().map(|(i, (mode, rk))| bar(&format!("mode-{mode}"), &format!("Failure mode {mode}"), &format!("Versagensform {mode}"), i as f64 + 0.6, i as f64 + 1.4, rk / 1e3, if i == j.governing { check.map_or(Tone::Info, |x| utilisation_tone(x.utilization)) } else { Tone::Neutral })).collect();
    let (mode, rk) = j.modes[j.governing];
    Some(Diagram2d {
        id: format!("johansen:{}", c.id),
        title: loc(&format!("Johansen failure modes – {}", c.label_en), &format!("Johansen-Versagensformen – {}", c.label_de)),
        description: loc("Characteristic load-carrying capacity per fastener and shear plane for every failure mode of the European Yield Model (EN 1995-1-1, 8.2); the lowest bar governs.", "Charakteristische Tragfähigkeit je Verbindungsmittel und Scherfuge für jede Versagensform des Europäischen Fließgelenkmodells (EN 1995-1-1, 8.2); der niedrigste Balken ist maßgebend."),
        axes: vec![axis("Failure mode", "Versagensform", QuantityKind::Dimensionless, "1", Some((0.5, j.modes.len() as f64 + 0.5)), TickRule::Explicit { values: (1..=j.modes.len()).map(|i| i as f64).collect() }), axis("Capacity F_v,Rk", "Tragfähigkeit F_v,Rk", QuantityKind::Force, "kN", None, TickRule::Auto)],
        series,
        markers: vec![Marker { id: "marker-governing".into(), at: Vec2::new(j.governing as f64 + 1.0, rk / 1e3), shape: MarkerShape::Triangle, tone: check.map_or(Tone::Info, |x| utilisation_tone(x.utilization)), label: loc(&format!("Mode {mode}: {:.2} kN", rk / 1e3), &format!("Form {mode}: {:.2} kN", rk / 1e3)), utilisation: check.map(|x| x.utilization), bound_check: check.map(|x| x.id.clone()), binding: None }],
        annotations: j.modes.iter().enumerate().map(|(i, (mode, rk))| DiagramNote { id: format!("note-{mode}"), at: Vec2::new(i as f64 + 1.0, rk / 1e3 * 0.5), text: loc(&format!("({mode})"), &format!("({mode})")), tone: Tone::Neutral }).collect(),
        focus: focus_connection(c),
        legend: true,
        table_twin: DataTable { columns: vec![column("Failure mode index", "Index der Versagensform", "1", (1..=j.modes.len()).map(|i| i as f64).collect()), column("Capacity F_v,Rk", "Tragfähigkeit F_v,Rk", "kN", j.modes.iter().map(|m| m.1 / 1e3).collect())] },
    })
}

fn spacing(report: &CheckReport, c: &TimberConnection, props: &TimberProperties) -> Diagram2d {
    let kind = facts::fastener_kind(c);
    let table = t::spacing_table(if kind == "screw" { "nail" } else { kind }, c.diameter_m * 1e3, 0.0, props.rho_k);
    let required = |name: &str| table.iter().find(|r| r.0 == name).map_or(0.0, |r| r.1);
    let rows = [("a1", "spacingM", "a1", "Spacing a₁", "Abstand a₁", c.spacing_m * 1e3, "en1995.8.spacing.a1."), ("a3t", "endDistanceM", "a3t", "End distance a₃,t", "Hirnholzabstand a₃,t", c.end_distance_m * 1e3, "en1995.8.spacing.a3t."), ("a4t", "edgeDistanceM", "a4t", "Edge distance a₄,t", "Randabstand a₄,t", c.edge_distance_m * 1e3, "en1995.8.spacing.a4t.")];
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (i, (id, leaf, name, en, de, provided, prefix)) in rows.iter().enumerate() {
        let need = required(name);
        let check = worst(report, &c.id, prefix);
        let tone = if *provided >= need * (1.0 - norm::UTILIZATION_TOLERANCE) { Tone::Info } else { Tone::Error };
        let x = i as f64 + 1.0;
        series.push(bar(&format!("required-{id}"), &format!("Required {en}"), &format!("Erforderlich {de}"), x - 0.4, x, need, Tone::Neutral));
        series.push(bar(&format!("provided-{id}"), &format!("Provided {en}"), &format!("Vorhanden {de}"), x, x + 0.4, *provided, tone));
        markers.push(Marker { id: format!("marker-{id}"), at: Vec2::new(x + 0.2, *provided), shape: MarkerShape::Diamond, tone, label: loc(&format!("{en}: {provided:.0} mm (min. {need:.0} mm)"), &format!("{de}: {provided:.0} mm (min. {need:.0} mm)")), utilisation: check.map(|x| x.utilization), bound_check: check.map(|x| x.id.clone()), binding: Some(binding(format!("connections[id={}].{leaf}", c.id), "m", Affine { base: 0.0, k: 1e-3 }, 0.005)) });
    }
    Diagram2d {
        id: format!("spacing:{}", c.id),
        title: loc(&format!("Spacings and distances – {}", c.label_en), &format!("Abstände und Randabstände – {}", c.label_de)),
        description: loc("Required minimum spacing a₁ and end and edge distances a₃,t and a₄,t (EN 1995-1-1, Tables 8.2 to 8.6) against the provided values; the markers drag the provided values.", "Erforderliche Mindestabstände a₁ sowie Hirnholz- und Randabstände a₃,t und a₄,t (EN 1995-1-1, Tabellen 8.2 bis 8.6) gegen die vorhandenen Werte; die Marker ändern beim Ziehen die vorhandenen Werte."),
        axes: vec![axis("Distance (1 a₁, 2 a₃,t, 3 a₄,t)", "Abstand (1 a₁, 2 a₃,t, 3 a₄,t)", QuantityKind::Dimensionless, "1", Some((0.4, 3.6)), TickRule::Explicit { values: vec![1.0, 2.0, 3.0] }), axis("Distance", "Abstand", QuantityKind::Length, "mm", None, TickRule::Auto)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus_connection(c),
        legend: true,
        table_twin: DataTable { columns: vec![column("Distance index", "Index des Abstands", "1", vec![1.0, 2.0, 3.0]), column("Required", "Erforderlich", "mm", ["a1", "a3t", "a4t"].iter().map(|n| required(n)).collect()), column("Provided", "Vorhanden", "mm", rows.iter().map(|r| r.5).collect())] },
    }
}

const N_EF_MAX: u32 = 12;

/// 🔩️ The effective number of fasteners of one row, `n_ef(n)` (EN 1995-1-1, 8.3.1.1 and 8.5.1.1), over the number of fasteners in the row at the provided spacing `a₁`, against the ideal `n_ef = n`; the marker is this connection and drags the number of fasteners.
fn n_ef(report: &CheckReport, c: &TimberConnection) -> Diagram2d {
    let kind = facts::fastener_kind(c);
    let g = facts::grid(c);
    let (a1, d) = (g.a1 * 1e3, c.diameter_m * 1e3);
    let n_max = N_EF_MAX.max(g.per_row + 2);
    let counts: Vec<f64> = (1..=n_max).map(f64::from).collect();
    let effective: Vec<f64> = counts.iter().map(|n| t::n_ef_value(kind, *n, a1, d, 0.0, false)).collect();
    let now = t::n_ef_value(kind, f64::from(g.per_row), a1, d, 0.0, false);
    let check = worst(report, &c.id, "en1995.8.2");
    let points = |values: &[f64]| -> Vec<Vec2> { counts.iter().zip(values).map(|(n, v)| Vec2::new(*n, *v)).collect() };
    Diagram2d {
        id: format!("n-ef:{}", c.id),
        title: loc(&format!("Effective number of fasteners – {}", c.label_en), &format!("Wirksame Anzahl der Verbindungsmittel – {}", c.label_de)),
        description: loc("Effective number n_ef of the fasteners in a row along the grain (EN 1995-1-1, 8.3.1.1 for nails and screws, 8.5.1.1 for bolts and dowels) over the number n in the row at the provided spacing a₁, against the ideal n_ef = n; the marker is this connection and drags the number of fasteners.", "Wirksame Anzahl n_ef der Verbindungsmittel in einer Reihe in Faserrichtung (EN 1995-1-1, 8.3.1.1 für Nägel und Schrauben, 8.5.1.1 für Bolzen und Stabdübel) über der Anzahl n in der Reihe bei vorhandenem Abstand a₁, gegen den Idealwert n_ef = n; der Marker ist diese Verbindung und ändert beim Ziehen die Anzahl der Verbindungsmittel."),
        axes: vec![axis("Fasteners per row n", "Verbindungsmittel je Reihe n", QuantityKind::Dimensionless, "1", Some((1.0, f64::from(n_max))), TickRule::Auto), axis("Effective number n_ef", "Wirksame Anzahl n_ef", QuantityKind::Dimensionless, "1", Some((0.0, f64::from(n_max))), TickRule::Auto)],
        series: vec![
            Series::Polyline { id: "n-ef".into(), label: loc("n_ef(n)", "n_ef(n)"), points: points(&effective), style: stroke(Tone::Info, 2.5, Dash::Solid) },
            Series::Polyline { id: "n-ideal".into(), label: loc("Ideal n_ef = n", "Ideal n_ef = n"), points: points(&counts), style: stroke(Tone::Neutral, 1.0, Dash::Dotted) },
        ],
        markers: vec![Marker { id: "marker-n-ef".into(), at: Vec2::new(f64::from(g.per_row), now), shape: MarkerShape::Diamond, tone: check.map_or(Tone::Neutral, |x| utilisation_tone(x.utilization)), label: loc(&format!("n = {} per row, n_ef = {now:.2}", g.per_row), &format!("n = {} je Reihe, n_ef = {now:.2}", g.per_row)), utilisation: check.map(|x| x.utilization), bound_check: check.map(|x| x.id.clone()), binding: Some(binding(format!("connections[id={}].number", c.id), "1", Affine { base: 0.0, k: f64::from(g.rows) }, 1.0)) }],
        annotations: Vec::new(),
        focus: focus_connection(c),
        legend: true,
        table_twin: DataTable { columns: vec![column("Fasteners per row n", "Verbindungsmittel je Reihe n", "1", counts.clone()), column("Effective number n_ef", "Wirksame Anzahl n_ef", "1", effective.clone())] },
    }
}

fn focus_case(c: &CaseRow) -> Option<EntityRef> {
    Some(EntityRef::new(EntityKind::Group, &c.id, format!("cases[id={}]", c.id)))
}

fn case_binding(c: &CaseRow, key: &str) -> String {
    format!("cases[id={}].numbers[id={key}].value", c.id)
}

/// 📈️ Utilisation of every limit result of a case as bars (one bar per named result), with the table twin.
fn case_results(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let rows: Vec<(&str, f64, String)> = report.checks.iter().filter(|k| k.subject.entity_id == c.id && k.utilization.is_finite() && k.utilization > 0.0).map(|k| (k.id.as_str(), k.utilization, k.title.en.clone())).collect();
    if rows.is_empty() {
        return None;
    }
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (i, (id, ratio, title)) in rows.iter().enumerate() {
        let x = i as f64 + 1.0;
        let tone = utilisation_tone(*ratio);
        series.push(bar(&format!("result-{i}"), title, title, x - 0.4, x + 0.4, *ratio, tone));
        markers.push(Marker { id: format!("marker-result-{i}"), at: Vec2::new(x, *ratio), shape: MarkerShape::Diamond, tone, label: loc(&format!("{title}: {:.0} %", ratio * 100.0), &format!("{title}: {:.0} %", ratio * 100.0)), utilisation: Some(*ratio), bound_check: Some((*id).to_string()), binding: None });
    }
    let n = rows.len() as f64;
    Some(Diagram2d {
        id: format!("case-results:{}", c.id),
        title: loc(&format!("Utilisation of {} ({})", c.label_en, c.entry_id), &format!("Ausnutzung von {} ({})", c.label_de, c.entry_id)),
        description: loc("Demand over resistance of every named result of the catalogue verification; a bar above 1 fails.", "Beanspruchung zu Beanspruchbarkeit jedes benannten Ergebnisses des Katalognachweises; ein Balken über 1 versagt."),
        axes: vec![axis("Result", "Ergebnis", QuantityKind::Dimensionless, "1", Some((0.4, n + 0.6)), TickRule::Auto), axis("Utilisation", "Ausnutzung", QuantityKind::Dimensionless, "1", Some((0.0, rows.iter().map(|r| r.1).fold(1.2_f64, f64::max))), TickRule::Auto)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus_case(c),
        legend: false,
        table_twin: DataTable { columns: vec![column("Result", "Ergebnis", "1", (1..=rows.len()).map(|i| i as f64).collect()), column("Utilisation", "Ausnutzung", "1", rows.iter().map(|r| r.1).collect())] },
    })
}

/// 🔥️ Char depth progression of a fire case over the fire duration; the marker drags `t_req`.
fn case_fire(d: &En1995Snapshot, report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let g = entities::geo(c);
    if g.t_req_min <= 0.0 {
        return None;
    }
    let product = entities::product_of(c);
    let end = (g.t_req_min * 1.5).max(90.0);
    let after = |t: f64| facts::fire_after(d.annex, product, g.b, g.h, t * 60.0);
    let char_curve = curves::sample(|t| after(t).d_char * 1e3, 0.0, end, SAMPLES);
    let eff_curve = curves::sample(|t| after(t).d_ef * 1e3, 0.0, end, SAMPLES);
    let residual = curves::sample(|t| after(t).h_fi * 1e3, 0.0, end, SAMPLES);
    let now = after(g.t_req_min);
    let check = worst(report, &c.id, "");
    Some(Diagram2d {
        id: format!("case-fire:{}", c.id),
        title: loc(&format!("Char depth progression – {}", c.label_en), &format!("Abbrandfortschritt – {}", c.label_de)),
        description: loc("Char depth, effective depth with the zero-strength layer and residual depth over the fire duration (EN 1995-1-2, 4.2); the marker drags the required fire resistance time.", "Abbrandtiefe, wirksame Tiefe mit Nullfestigkeitsschicht und Resthöhe über der Branddauer (EN 1995-1-2, 4.2); der Marker ändert die geforderte Feuerwiderstandsdauer."),
        axes: vec![axis("Fire duration", "Branddauer", QuantityKind::Time, "min", Some((0.0, end)), TickRule::Auto), axis("Depth", "Tiefe", QuantityKind::Length, "mm", None, TickRule::Auto)],
        series: vec![
            polyline("d-char", "Char depth d_char", "Abbrandtiefe d_char", char_curve.clone(), stroke(Tone::Neutral, 1.5, Dash::Solid)),
            polyline("d-ef", "Effective depth d_ef", "Wirksame Tiefe d_ef", eff_curve.clone(), stroke(Tone::Warning, 2.0, Dash::Solid)),
            polyline("h-fi", "Residual depth h_fi", "Resthöhe h_fi", residual.clone(), stroke(Tone::Info, 2.0, Dash::Dashed)),
        ],
        markers: vec![Marker { id: "marker-t-req".into(), at: Vec2::new(g.t_req_min, now.d_ef * 1e3), shape: MarkerShape::Circle, tone: check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), label: loc(&format!("{:.0} min: d_ef = {:.1} mm", g.t_req_min, now.d_ef * 1e3), &format!("{:.0} min: d_ef = {:.1} mm", g.t_req_min, now.d_ef * 1e3)), utilisation: check.map(|k| k.utilization), bound_check: check.map(|k| k.id.clone()), binding: Some(binding(case_binding(c, "t_req"), "min", Affine::IDENTITY, 5.0)) }],
        annotations: Vec::new(),
        focus: focus_case(c),
        legend: true,
        table_twin: DataTable { columns: vec![column("Fire duration", "Branddauer", "min", char_curve.iter().map(|q| q[0]).collect()), column("Char depth d_char", "Abbrandtiefe d_char", "mm", char_curve.iter().map(|q| q[1]).collect()), column("Effective depth d_ef", "Wirksame Tiefe d_ef", "mm", eff_curve.iter().map(|q| q[1]).collect()), column("Residual depth h_fi", "Resthöhe h_fi", "mm", residual.iter().map(|q| q[1]).collect())] },
    })
}

/// 🎵️ First vertical mode shape of a floor or footbridge case with its frequency; the marker drags the span.
fn case_mode(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let g = entities::geo(c);
    let (span, ei, mass) = g.vibration?;
    let mode = entities::cached_mode(span, ei, mass)?;
    let points: Vec<Xy> = mode.shape.iter().map(|&(t, u)| [t * span, u]).collect();
    let check = worst(report, &c.id, "");
    Some(Diagram2d {
        id: format!("case-mode:{}", c.id),
        title: loc(&format!("First vertical mode – {}", c.label_en), &format!("Erste vertikale Eigenform – {}", c.label_de)),
        description: loc(&format!("Normalised mode shape with f₁ = {:.2} Hz from the shared mechanics kernel; the marker drags the span.", mode.frequency), &format!("Normierte Eigenform mit f₁ = {:.2} Hz aus dem gemeinsamen Mechanikkern; der Marker ändert die Stützweite.", mode.frequency)),
        axes: vec![axis("Position", "Lage", QuantityKind::Length, "m", Some((0.0, span)), TickRule::Auto), axis("Mode shape", "Eigenform", QuantityKind::Dimensionless, "1", Some((-1.2, 1.2)), TickRule::Auto)],
        series: vec![polyline("mode-1", "Mode 1", "Eigenform 1", points.clone(), stroke(Tone::Info, 2.0, Dash::Solid))],
        markers: vec![Marker { id: "marker-span".into(), at: Vec2::new(span, 0.0), shape: MarkerShape::Diamond, tone: check.map_or(Tone::Neutral, |k| utilisation_tone(k.utilization)), label: loc(&format!("span {span:.2} m, f₁ = {:.2} Hz", mode.frequency), &format!("Stützweite {span:.2} m, f₁ = {:.2} Hz", mode.frequency)), utilisation: check.map(|k| k.utilization), bound_check: check.map(|k| k.id.clone()), binding: Some(binding(case_binding(c, "span_m"), "m", Affine::IDENTITY, 0.05)) }],
        annotations: Vec::new(),
        focus: focus_case(c),
        legend: false,
        table_twin: DataTable { columns: vec![column("Position", "Position", "m", points.iter().map(|q| q[0]).collect()), column("Mode shape", "Eigenform", "1", points.iter().map(|q| q[1]).collect())] },
    })
}

/// 🌳️ CLT layup: thickness of every layer, longitudinal layers neutral and cross layers (rolling shear) in warning tone.
fn case_layup(report: &CheckReport, c: &CaseRow) -> Option<Diagram2d> {
    let g = entities::geo(c);
    if g.layers.is_empty() {
        return None;
    }
    let check = worst(report, &c.id, "en1995.t-97.rolling_shear");
    let mut series = Vec::new();
    let mut markers = Vec::new();
    for (i, (t, parallel)) in g.layers.iter().enumerate() {
        let x = i as f64 + 1.0;
        let tone = if *parallel { Tone::Neutral } else { Tone::Warning };
        let mut layer = bar(&format!("layer-{}", i + 1), &format!("Layer {} {}", i + 1, if *parallel { "longitudinal" } else { "cross" }), &format!("Lage {} {}", i + 1, if *parallel { "längs" } else { "quer" }), x - 0.4, x + 0.4, t * 1e3, tone);
        if let Series::Polygon { hatch, .. } = &mut layer {
            *hatch = Some(if *parallel { Hatch::Horizontal } else { Hatch::Vertical });
        }
        series.push(layer);
        markers.push(Marker { id: format!("marker-t{}", i + 1), at: Vec2::new(x, t * 1e3), shape: MarkerShape::Diamond, tone, label: loc(&format!("t{} = {:.0} mm", i + 1, t * 1e3), &format!("t{} = {:.0} mm", i + 1, t * 1e3)), utilisation: check.filter(|_| !parallel).map(|k| k.utilization), bound_check: check.filter(|_| !parallel).map(|k| k.id.clone()), binding: Some(binding(case_binding(c, &format!("t{}", i + 1)), "mm", Affine::IDENTITY, 1.0)) });
    }
    let n = g.layers.len() as f64;
    Some(Diagram2d {
        id: format!("case-layup:{}", c.id),
        title: loc(&format!("CLT layup – {}", c.label_en), &format!("BSP-Aufbau – {}", c.label_de)),
        description: loc("Layer thicknesses of the CLT element; cross layers carry the rolling shear. The markers drag the thicknesses.", "Lagendicken des BSP-Elements; Querlagen tragen den Rollschub. Die Marker ändern die Dicken."),
        axes: vec![axis("Layer", "Lage", QuantityKind::Dimensionless, "1", Some((0.4, n + 0.6)), TickRule::Explicit { values: (1..=g.layers.len()).map(|i| i as f64).collect() }), axis("Thickness", "Dicke", QuantityKind::Length, "mm", None, TickRule::Auto)],
        series,
        markers,
        annotations: Vec::new(),
        focus: focus_case(c),
        legend: true,
        table_twin: DataTable { columns: vec![column("Layer", "Lage", "1", (1..=g.layers.len()).map(|i| i as f64).collect()), column("Thickness", "Dicke", "mm", g.layers.iter().map(|l| l.0 * 1e3).collect()), column("Longitudinal (1) or cross (0)", "Längs (1) oder quer (0)", "1", g.layers.iter().map(|l| f64::from(u8::from(l.1))).collect())] },
    })
}

#[path = "🧩️special/🦀️.rs"]
mod special;

#[path = "🧩️failure-modes/🦀️.rs"]
mod failure_modes;

#[path = "🧩️response-curves/🦀️.rs"]
mod response_curves;

#[path = "🧩️components/🦀️.rs"]
mod components;

#[path = "🧩️entity-curves/🦀️.rs"]
mod entity_curves;

/// 📈️ The diagrams of the document, narrowed to the entity `focus` (a member or connection id) when given.
pub fn diagrams(document: &En1995Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    ::semio_s_plugin_norm_structural_scene::projection::with_basis(diagrams_unbased(document, report, focus), &::semio_s_plugin_norm_structural_scene::projection::Basis::new(::semio_s_plugin_norm_compliance::NormFamilyId::En1995, document.annex))
}

fn diagrams_unbased(document: &En1995Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let matches = |id: &str| focus.is_none_or(|f| f == id);
    let mut out = Vec::new();
    for member in document.members.iter().filter(|m| matches(&m.id)) {
        let Some(props) = facts::properties(member.strength_class.id()) else { continue };
        out.extend(kmod(report, member, &props));
        out.extend(buckling(report, member, &props));
        out.extend(kcrit(report, member, &props));
        out.extend(deflection(document, report, member, &props));
        out.extend(fire(document, report, member, &props));
        if let Some(dynamics) = facts::dynamics(document.annex, member, &props) {
            out.extend(vibration(report, member, &dynamics));
        }
        out.extend(entity_curves::bridge_fatigue(report, member));
        out.extend(entity_curves::member_results(report, member));
    }
    for connection in document.connections.iter().filter(|c| matches(&c.id)) {
        let Some(props) = facts::properties(connection.strength_class.id()) else { continue };
        out.push(embedment(connection, &props));
        out.extend(johansen(report, connection));
        out.extend(failure_modes::connection(report, connection));
        out.push(spacing(report, connection, &props));
        out.push(n_ef(report, connection));
    }
    for case in document.cases.iter().filter(|c| matches(&c.id)) {
        out.extend(case_diagrams(document, report, case));
    }
    out.extend(entity_diagrams(document, report, focus));
    out
}

/// 📈️ The diagrams of the typed entities, narrowed to the entity `focus` when given: those of the case row each is drawn as, re-pointed to the entity fields, plus the creep time curve of a floor.
fn entity_diagrams(document: &En1995Snapshot, report: &CheckReport, focus: Option<&str>) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    for table in t::ENTITY_TABLES {
        for index in 0..t::entity_ids(document, table).len() {
            let Some((row, homes)) = t::drawing_row(document, table, index) else { continue };
            if focus.is_some_and(|f| f != row.id) {
                continue;
            }
            out.extend(case_diagrams(document, report, &row).into_iter().map(|d| entities::rehome(d, table, &row, &homes)));
            if let Some(floor) = document.floors.iter().find(|f| table == "floors" && f.id == row.id) {
                out.extend(entities::creep_diagram(document, report, floor));
            }
        }
    }
    out
}

/// 📈️ Every diagram of one case row: the results, fire, mode, layup, failure-mode and entry-specific diagrams; the typed entities reuse it for the case row they are drawn as.
pub fn case_diagrams(document: &En1995Snapshot, report: &CheckReport, case: &CaseRow) -> Vec<Diagram2d> {
    let mut out = Vec::new();
    out.extend(case_results(report, case));
    out.extend(case_fire(document, report, case));
    out.extend(case_mode(report, case));
    out.extend(case_layup(report, case));
    out.extend(failure_modes::case(document, report, case));
    out.extend(special::case_diagrams(document, report, case));
    out.extend(components::case_diagrams(document, report, case));
    out.extend(response_curves::case_diagrams(document, report, case));
    out.extend(entity_curves::case_diagrams(document, report, case));
    out
}
