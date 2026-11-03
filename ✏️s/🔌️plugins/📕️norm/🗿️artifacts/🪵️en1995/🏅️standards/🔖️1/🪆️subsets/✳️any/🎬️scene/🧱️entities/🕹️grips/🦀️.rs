//! 🕹️ Grips of a case row drawn by the entity renderers: the dimension, notch, hole, taper, apex, fire time, span and layer grips of a timber block, the diameter, spacing, distance and thickness grips of a fastener row with its pattern editor (count steppers and a grip per fastener of the first row and column, one per coordinate for explicit positions) and those of the domain scenes of [`special`]; each writes the case inputs, and the typed entities re-point them to their own fields. See [`super`].

use super::facts;
use super::kit::{loc, v3};
use super::model::CaseRow;
use super::rows::special::handles as special_handles;
use super::rows::{elevation, extent, geo, product_of, regular, Geo, Profile};
use super::scene::projection_kit::handle_kit::{field_binding, grid, handle, round, HandleSpec as Spec};
use super::scene::*;

/// 🔩️ The most rods of a reinforcement with a count grip.
const ROD_COUNT_MAX: u32 = 24;

/// ✂️ The largest notch slope `i` a grip drags to (a drawing limit, not a normative bound).
const NOTCH_SLOPE_MAX: f64 = 2.0;

/// 🔩️ The most explicit positions with grips.
const EXPLICIT_GRIPS_MAX: usize = 24;

/// 🔩️ The pattern editor of a fastener case: count steppers for the columns `n` and the rows, a grip on every fastener of the first row (the column `i` stands at `a₃ + i·a₁`, so dragging it writes `a₁ = (x − a₃) / i`) and of the first column (the row `j` writes `a₂ = (y − a₄) / j`); a₃ and a₄ move with the first fastener. Only for a regular pattern, an explicit point list has no spacing to rewrite.
fn fastener_pattern_handles(c: &CaseRow, g: &Geo, o: &Vec3, entity: &EntityRef, out: &mut Vec<Handle>) {
    use super::schema::checks::timber::{PATTERN_COLUMNS_MAX, PATTERN_LINES_MAX};
    let id = c.id.as_str();
    let top = g.t1 + g.t2;
    let at = |x: f64, y: f64| v3(o.x + x, o.y + y, o.z + top);
    let present = |key: &str| c.numbers.iter().any(|n| n.id == key);
    let path = |key: &str| format!("cases[id={id}].numbers[id={key}].value");
    for (index, (x, y)) in g.points.iter().enumerate().take(if g.explicit { EXPLICIT_GRIPS_MAX } else { 0 }) {
        for (axis, key, limit, label) in [(Vec3::X, "px", 12.0, "x"), (Vec3::Y, "py", 1.0, "y")] {
            let tag = format!("{key}{}", index + 1);
            out.push(handle(Spec { entity, tag: &tag, kind: HandleKind::ScalarAlong { axis }, anchor: at(*x, *y), grip: GripStyle::Ring, writes: vec![field_binding(path(&tag), Affine { base: 0.0, k: 1000.0 }, "mm", round(0.005))], snap: grid(0.005), limits: (0.0, limit), name: loc(&format!("Fastener {} position {label}", index + 1), &format!("Verbindungsmittel {} Lage {label}", index + 1)) }));
        }
    }
    if !regular(g) {
        return;
    }
    let count = |key: &str, anchor: Vec3, max: u32, en: &str, de: &str| {
        handle(Spec { entity, tag: key, kind: HandleKind::CountStepper, anchor, grip: GripStyle::Diamond, writes: vec![FieldBinding { path: path(key), map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Int) }], snap: SnapRule::NONE, limits: (1.0, f64::from(max)), name: loc(&format!("{en} of {}", c.label_en), &format!("{de} von {}", c.label_de)) })
    };
    if present("n") {
        out.push(count("n", at(g.end + f64::from(g.ny) * g.a1, g.edge), PATTERN_COLUMNS_MAX, "Fasteners per row", "Verbindungsmittel je Reihe"));
    }
    if present("rows") {
        out.push(count("rows", at(g.end, g.edge + f64::from(g.nx) * g.a2), PATTERN_LINES_MAX, "Number of rows", "Anzahl Reihen"));
    }
    if present("a1") {
        for i in 1..g.ny {
            let k = f64::from(i);
            out.push(handle(Spec { entity, tag: &format!("col{i}"), kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: at(g.end + k * g.a1, g.edge), grip: GripStyle::Ring, writes: vec![field_binding(path("a1"), Affine { base: -g.end * 1000.0 / k, k: 1000.0 / k }, "mm", round(0.005))], snap: SnapRule::NONE, limits: (g.end + 0.005 * k, g.end + 0.6 * k), name: loc(&format!("Column {i} of {}", c.label_en), &format!("Spalte {i} von {}", c.label_de)) }));
        }
    }
    if present("a2") {
        for j in 1..g.nx {
            let k = f64::from(j);
            out.push(handle(Spec { entity, tag: &format!("row{j}"), kind: HandleKind::ScalarAlong { axis: Vec3::Y }, anchor: at(g.end, g.edge + k * g.a2), grip: GripStyle::Ring, writes: vec![field_binding(path("a2"), Affine { base: -g.edge * 1000.0 / k, k: 1000.0 / k }, "mm", round(0.005))], snap: SnapRule::NONE, limits: (g.edge + 0.005 * k, g.edge + 0.4 * k), name: loc(&format!("Row {j} of {}", c.label_en), &format!("Reihe {j} von {}", c.label_de)) }));
        }
    }
}

pub fn row_handles(annex: super::norm::AnnexChoice, c: &CaseRow, o: &Vec3, entity: EntityRef) -> Vec<Handle> {
    let id = c.id.as_str();
    let g = geo(c);
    let at = |x: f64, y: f64, z: f64| v3(o.x + x, o.y + y, o.z + z);
    let present = |key: &str| c.numbers.iter().any(|n| n.id == key);
    let value = |key: &str| c.numbers.iter().find(|n| n.id == key).map(|n| n.value / 1e3);
    let mut out = Vec::new();
    let mut add = |key: &str, kind: HandleKind, anchor: Vec3, grip: GripStyle, k: f64, step: f64, limits: (f64, f64), en: &str, de: &str| {
        if present(key) {
            out.push(handle(Spec { entity: &entity, tag: key, kind, anchor, grip, writes: vec![field_binding(format!("cases[id={id}].numbers[id={key}].value"), Affine { base: 0.0, k }, "mm", round(step))], snap: grid(step), limits, name: loc(&format!("{en} of {}", c.label_en), &format!("{de} von {}", c.label_de)) }));
        }
    };
    if g.fasteners {
        let top = g.t1 + g.t2;
        let (length, width) = extent(c);
        add("d", HandleKind::Radial { centre: at(g.end, g.edge, top) }, at(g.end, g.edge + g.d / 2.0, top), GripStyle::Ring, 2000.0, 0.001, (0.002, 0.04), "Fastener diameter", "Durchmesser");
        add("a1", HandleKind::DimensionLine, at(g.end + g.a1 / 2.0, g.edge, top), GripStyle::Arrow, 1000.0, 0.005, (0.005, 0.6), "Spacing a₁", "Abstand a₁");
        add("a2", HandleKind::DimensionLine, at(g.end, g.edge + g.a2 / 2.0, top), GripStyle::Arrow, 1000.0, 0.005, (0.005, 0.4), "Spacing a₂", "Abstand a₂");
        add("a3t", HandleKind::ScalarAlong { axis: Vec3::X }, at(g.end, g.edge, top), GripStyle::Ring, 1000.0, 0.005, (0.01, 0.6), "End distance a₃", "Hirnholzabstand a₃");
        add("a4t", HandleKind::ScalarAlong { axis: Vec3::Y }, at(g.end, g.edge, top), GripStyle::Ring, 1000.0, 0.005, (0.01, 0.4), "Edge distance a₄", "Randabstand a₄");
        add("t1", HandleKind::FaceOffset { normal: Vec3::Z }, at(length / 2.0, 0.0, g.t1), GripStyle::Diamond, 1000.0, 0.005, (0.01, 0.4), "Thickness t₁", "Dicke t₁");
        add("t2", HandleKind::FaceOffset { normal: Vec3::Z }, at(length / 2.0, width, top), GripStyle::Diamond, 1000.0, 0.005, (0.005, 0.4), "Thickness t₂", "Dicke t₂");
        fastener_pattern_handles(c, &g, o, &entity, &mut out);
    } else {
        add("l", HandleKind::ScalarAlong { axis: Vec3::X }, at(g.len, g.b / 2.0, g.h / 2.0), GripStyle::Arrow, 1000.0 / g.spans, 0.01 * g.spans, (0.1 * g.spans, 12.0 * g.spans), "Length", "Länge");
        add("b", HandleKind::FaceOffset { normal: Vec3::Y }, at(g.len / 2.0, g.b, g.h / 2.0), GripStyle::Diamond, 1000.0, 0.01, (0.03, 0.6), "Width", "Breite");
        add("h", HandleKind::FaceOffset { normal: Vec3::Z }, at(g.len / 2.0, g.b / 2.0, g.h), GripStyle::Diamond, 1000.0, 0.01, (0.04, 1.4), "Depth", "Höhe");
        add("h_ef", HandleKind::FaceOffset { normal: Vec3::Z }, at(g.reach() / 2.0, g.b / 2.0, g.h - g.notch), GripStyle::Diamond, 1000.0, 0.005, (0.02, g.h), "Effective depth at the notch", "Wirksame Höhe an der Ausklinkung");
        add("notch_l", HandleKind::ScalarAlong { axis: Vec3::X }, at(g.reach(), g.b / 2.0, g.h - g.notch), GripStyle::Arrow, 1000.0, 0.005, (0.01, g.len), "Notch length", "Ausklinkungslänge");
        add("x", HandleKind::ScalarAlong { axis: Vec3::X }, at(g.notch_x.max(0.02), g.b / 2.0, g.h), GripStyle::Arrow, 1000.0, 0.005, (0.0, 1.0), "Notch distance x", "Ausklinkungsabstand x");
        if let (Some(d), Some(x), Some(z)) = (value("hole_d"), value("hole_x"), value("hole_z")) {
            let centre = at(x, g.b / 2.0, z);
            add("hole_d", HandleKind::Radial { centre: centre.clone() }, at(x + d / 2.0, g.b / 2.0, z), GripStyle::Ring, 2000.0, 0.001, (0.005, g.h / 2.0), "Hole diameter", "Lochdurchmesser");
            add("hole_x", HandleKind::ScalarAlong { axis: Vec3::X }, at(x, g.b / 2.0, z - d / 2.0), GripStyle::Arrow, 1000.0, 0.005, (0.0, g.len), "Hole position x", "Lochlage x");
            add("hole_z", HandleKind::FaceOffset { normal: Vec3::Z }, at(x - d / 2.0, g.b / 2.0, z), GripStyle::Diamond, 1000.0, 0.005, (0.0, g.h), "Hole height above the bottom", "Lochhöhe über der Unterkante");
        }
        let to_degrees = 180.0 / std::f64::consts::PI;
        let neg_y = Vec3::new(0.0, -1.0, 0.0);
        match g.profile {
            Profile::Tapered { alpha } => add("alpha_deg", HandleKind::Angular { axis: neg_y, centre: at(0.0, g.b / 2.0, g.h) }, at(g.len, g.b / 2.0, g.h + g.len * alpha.tan()), GripStyle::Ring, to_degrees, 0.002, (0.0, 0.6), "Taper angle", "Neigungswinkel"),
            Profile::Apex { .. } => {
                if let Some((_, _, anchors)) = elevation(&g) {
                    let point = |p: &Vec2| at(p.x, g.b / 2.0, p.y);
                    add("alpha_ap_deg", HandleKind::Angular { axis: Vec3::Y, centre: point(&anchors[0]) }, point(&anchors[2]), GripStyle::Ring, to_degrees, 0.002, (0.0, 0.6), "Apex angle", "Firstneigung");
                    add("r_in", HandleKind::Radial { centre: point(&anchors[0]) }, point(&anchors[1]), GripStyle::Ring, 1000.0, 0.05, (1.0, 80.0), "Inner radius", "Innenradius");
                    add("t_lam", HandleKind::FaceOffset { normal: Vec3::Z }, at(0.0, g.b, value("t_lam").unwrap_or(0.0)), GripStyle::Diamond, 1000.0, 0.001, (0.005, 0.1), "Lamella thickness", "Lamellendicke");
                }
            }
            Profile::Straight => {}
        }
        add("h_ap", HandleKind::FaceOffset { normal: Vec3::Z }, at(g.len / 2.0, g.b / 2.0, g.h), GripStyle::Diamond, 1000.0, 0.01, (0.1, 2.0), "Apex depth", "Firsthöhe");
        let per_minute = facts::fire_after(annex, product_of(c), g.b, g.h, 60.0).d_char;
        if per_minute > 0.0 {
            add("t_req", HandleKind::FaceOffset { normal: Vec3::Y }, at(g.len / 2.0, per_minute * c.numbers.iter().find(|n| n.id == "t_req").map_or(0.0, |n| n.value), g.h / 2.0), GripStyle::Diamond, 1.0 / per_minute, 5.0 * per_minute, (0.0, g.b / 2.0), "Fire resistance time", "Feuerwiderstandsdauer");
        }
        if g.vibration.is_some() {
            add("span_m", HandleKind::ScalarAlong { axis: Vec3::X }, at(g.len, g.b / 2.0, g.h / 2.0), GripStyle::Arrow, 1.0, 0.05, (0.5, 40.0), "Span", "Stützweite");
        }
        let mut cumulative = 0.0;
        for (index, (thickness, _)) in g.layers.iter().enumerate() {
            let key = format!("t{}", index + 1);
            let base = cumulative;
            cumulative += thickness;
            if present(&key) {
                out.push(handle(Spec { entity: &entity, tag: &key, kind: HandleKind::FaceOffset { normal: Vec3::Z }, anchor: at(g.len / 2.0, g.b / 2.0, cumulative), grip: GripStyle::Diamond, writes: vec![field_binding(format!("cases[id={id}].numbers[id={key}].value"), Affine { base: -base * 1000.0, k: 1000.0 }, "mm", round(0.001))], snap: grid(0.001), limits: (base + 0.005, 0.4), name: loc(&format!("Thickness of layer {} of {}", index + 1, c.label_en), &format!("Dicke der Lage {} von {}", index + 1, c.label_de)) }));
            }
        }
    }
    if !g.fasteners {
        let reach = g.reach();
        let count_anchor = if g.notch > 1e-6 { at(reach, g.b, g.h) } else { at(g.len / 2.0, g.b, g.h) };
        if present("n_rods") {
            out.push(handle(Spec { entity: &entity, tag: "n_rods", kind: HandleKind::CountStepper, anchor: count_anchor, grip: GripStyle::Diamond, writes: vec![FieldBinding { path: format!("cases[id={id}].numbers[id=n_rods].value"), map: Affine::IDENTITY, unit: "1".into(), quantise: Some(Quantise::Int) }], snap: SnapRule::NONE, limits: (0.0, f64::from(ROD_COUNT_MAX)), name: loc(&format!("Number of rods of {}", c.label_en), &format!("Anzahl Stäbe von {}", c.label_de)) }));
        }
        if present("incl") && g.notch > 1e-3 {
            let slope = value("incl").unwrap_or(0.0) * 1e3;
            out.push(handle(Spec { entity: &entity, tag: "incl", kind: HandleKind::ScalarAlong { axis: Vec3::X }, anchor: at(reach + slope * g.notch, g.b / 2.0, g.h), grip: GripStyle::Ring, writes: vec![field_binding(format!("cases[id={id}].numbers[id=incl].value"), Affine { base: -reach / g.notch, k: 1.0 / g.notch }, "1", round(0.01))], snap: grid(0.01), limits: (reach, reach + NOTCH_SLOPE_MAX * g.notch), name: loc(&format!("Notch slope i of {}", c.label_en), &format!("Ausklinkungsneigung i von {}", c.label_de)) }));
        }
    }
    out.extend(special_handles(c, &g, o, &entity));
    for handle in out.iter_mut().filter(|h| h.id.ends_with(":alpha_ap_deg")) {
        let quarter = std::f64::consts::FRAC_PI_2;
        handle.writes[0].map.base = 90.0;
        handle.limits = Some(Interval::new(-quarter, -quarter + 0.6));
    }
    out
}

