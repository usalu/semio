//! 🌍️ Ground forms of the case drawing: spread footing, pile and pile group, retaining wall, sheet wall, slope, soil block, ground anchor, seepage path and pumped well, each with its soil context (stacked layers from a series input, the groundwater table from a depth or level input) and the grips that write the dimension and auxiliary inputs. See [`super`].

use super::*;
use norm::design_case::CaseForm;

/// 🌍️ The ground form of the declaration of `m`, `None` for the other forms.
pub(super) fn build<C: CaseCatalogue>(m: &Meas<'_, C>) -> Option<Built> {
    Some(match m.g.form {
        CaseForm::Footing => footing(m),
        CaseForm::Pile => pile(m),
        CaseForm::RetainingWall => retaining_wall(m),
        CaseForm::SheetWall => sheet_wall(m),
        CaseForm::Slope => slope(m),
        CaseForm::SoilBlock => soil_block(m),
        CaseForm::GroundAnchor => ground_anchor(m),
        CaseForm::Seepage => seepage(m),
        CaseForm::Well => well(m),
        _ => return None,
    })
}

const X: [f64; 3] = [1.0, 0.0, 0.0];
const NEG_X: [f64; 3] = [-1.0, 0.0, 0.0];
const Y: [f64; 3] = [0.0, 1.0, 0.0];
const Z: [f64; 3] = [0.0, 0.0, 1.0];
const COUNT_MAX: f64 = 12.0;

/// 🦶️ A foundation pad with a column stub at `embed` below ground on soil layers; the load bar stands on the stub.
fn footing<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (b, l, t) = (m.w(), m.l(), m.h());
    let embed = m.aux("embed", 1.0);
    let base = -embed;
    let (hx, hy) = (b / 2.0 + b.max(2.0), l / 2.0 + l.max(2.0));
    let depth = embed + t + 1.5 * b.max(2.0);
    let stub = (b.min(l) * 0.25).clamp(0.2, 0.6);
    let stub_top = (base + t).max(0.0) + 0.5;
    let water = m.has("water").then(|| -m.aux("water", 0.4 * depth));
    let stack = strata(m, depth);
    let mut parts = vec![Part::new("", solid(m.pose, [0.0, 0.0, base], [b, l, t]), m.role.clone(), ("Foundation pad", "Fundamentplatte")), Part::new("column", solid(m.pose, [0.0, 0.0, base + t], [stub, stub, stub_top - base - t]), m.role.clone(), ("Column stub", "Stützenstumpf"))];
    parts.extend(soil(m, 0.0, hx, hy, &stack, water));
    let plans: Vec<Plan> = vec![
        face("width", m.g.width, 2.0, X, [b / 2.0, 0.0, base + t / 2.0]),
        face("length", m.g.length, 2.0, Y, [0.0, l / 2.0, base + t / 2.0]),
        scalar("height", m.g.height, 1.0, Z, [0.0, 0.0, base + t]),
        scalar("embed", m.key("embed"), -1.0, Z, [0.0, 0.0, base]),
        scalar("water", m.key("water"), -1.0, Z, [hx, 0.0, water.unwrap_or(0.0)]),
    ]
    .into_iter()
    .chain(layer_plans(m, hx))
    .collect();
    Built { parts, top: stub_top, bottom: -stack.iter().sum::<f64>(), half: (hx, hy), plans }
}

/// 🏗️ A pile, or a pile group under a cap, in soil layers: the first pile is the body.
fn pile<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (d, length) = (m.w(), m.h());
    let (rows, columns) = (m.count("rows", 1.0), m.count("columns", 1.0));
    let spacing = m.aux("spacing", 3.0 * d);
    let grouped = rows * columns > 1.0;
    let reach = |count: f64| (count - 1.0) * spacing / 2.0;
    let (hx, hy) = (reach(columns) + 3.0 * d.max(1.0), reach(rows) + 3.0 * d.max(1.0));
    let depth = 1.25 * length;
    let water = m.has("water").then(|| -m.aux("water", 0.3 * depth));
    let mut parts = Vec::new();
    for row in 0..rows as usize {
        for column in 0..columns as usize {
            let (x, y) = (column as f64 * spacing - reach(columns), row as f64 * spacing - reach(rows));
            let tag = if row == 0 && column == 0 { String::new() } else { format!("pile-{row}-{column}") };
            parts.push(Part::new(tag, pillar(m.pose, [x, y, -length], d / 2.0, length), m.role.clone(), ("Pile", "Pfahl")));
        }
    }
    let mut top = 0.0;
    if grouped {
        let cap = 0.6;
        parts.push(Part::new("cap", solid(m.pose, [0.0, 0.0, 0.0], [2.0 * reach(columns) + 2.0 * d, 2.0 * reach(rows) + 2.0 * d, cap]), m.role.clone(), ("Pile cap", "Pfahlkopfplatte")));
        top = cap;
    }
    let stack = strata(m, depth);
    parts.extend(soil(m, 0.0, hx, hy, &stack, water));
    let mut plans = vec![
        face("diameter", m.g.width, 2.0, X, [d / 2.0 - reach(columns), 0.0, -length / 2.0]),
        scalar("length", m.g.height, -1.0, Z, [-reach(columns), -reach(rows), -length]),
        scalar("spacing", m.key("spacing"), 2.0 / (columns - 1.0).max(1.0), X, [reach(columns), -reach(rows), 0.2]),
        scalar("water", m.key("water"), -1.0, Z, [hx, 0.0, water.unwrap_or(0.0)]),
        stepper("columns", m.key("columns"), (1.0, COUNT_MAX), [reach(columns) + spacing, -reach(rows), 0.2]),
        stepper("rows", m.key("rows"), (1.0, COUNT_MAX), [-reach(columns), reach(rows) + spacing, 0.2]),
    ];
    plans.extend(layer_plans(m, hx));
    Built { parts, top, bottom: -stack.iter().sum::<f64>(), half: (hx, hy), plans }
}

/// 🧗️ A gravity or cantilever retaining wall: stem on a base slab with toe and heel, backfill behind the stem and a groundwater sheet in the backfill.
fn retaining_wall<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (t, wall_length, h) = (m.w(), m.l(), m.h());
    let base_t = m.aux("base_t", (0.15 * h).clamp(0.3, 1.0));
    let base_w = m.aux("base", (0.6 * h).max(2.0 * t + 0.4)).max(t + 0.2);
    let toe = m.aux("toe", 0.25 * base_w).clamp(0.05, base_w - t - 0.05);
    let stem_x = -base_w / 2.0 + toe + t / 2.0;
    let back = (base_w - toe - t).max(0.6 * h);
    let back_x = stem_x + t / 2.0;
    let top = base_t + h;
    let (reach_left, reach_right) = (base_w / 2.0 + 1.5, back_x + back + 1.5);
    let (cx, hx) = ((reach_right - reach_left) / 2.0, (reach_left + reach_right) / 2.0);
    let hy = wall_length / 2.0 + 1.0;
    let mut parts = vec![
        Part::new("", solid(m.pose, [stem_x, 0.0, base_t], [t, wall_length, h]), m.role.clone(), ("Wall stem", "Wandschaft")),
        Part::new("base", solid(m.pose, [0.0, 0.0, 0.0], [base_w, wall_length, base_t]), m.role.clone(), ("Base slab", "Fundamentplatte")),
        Part::new("backfill", solid(m.pose, [back_x + back / 2.0, 0.0, base_t], [back, wall_length, h]), soil_role("backfill"), ("Backfill", "Hinterfüllung")).context(CONTEXT_OPACITY),
    ];
    let water = m.has("water").then(|| top - m.aux("water", 0.4 * h));
    parts.extend(soil(m, cx, hx, hy, &[1.5 * base_t.max(1.0) + 1.0], None));
    if let Some(level) = water {
        parts.push(Part::new("water", solid(m.pose, [back_x + back / 2.0, 0.0, level], [back, wall_length, 0.03]), soil_role("water"), ("Groundwater table", "Grundwasserspiegel")).context(WATER_OPACITY));
    }
    let plans = vec![
        face("width", m.g.width, 2.0, X, [stem_x + t / 2.0, 0.0, base_t + h / 2.0]),
        face("length", m.g.length, 2.0, Y, [0.0, wall_length / 2.0, base_t + h / 2.0]),
        scalar("height", m.g.height, 1.0, Z, [stem_x, 0.0, top]),
        face("base", m.key("base"), 1.0, X, [base_w / 2.0, 0.0, base_t / 2.0]),
        scalar("base-thickness", m.key("base_t"), 1.0, Z, [0.0, 0.0, base_t]),
        face("toe", m.key("toe"), 1.0, NEG_X, [-base_w / 2.0, 0.0, base_t / 2.0]),
        scalar("water", m.key("water"), -1.0, Z, [back_x + back, 0.0, water.unwrap_or(top)]),
    ];
    Built { parts, top, bottom: -(1.5 * base_t.max(1.0) + 1.0), half: (hx + cx.abs(), hy), plans }
}

/// 📍️ An embedded sheet wall with the retained soil behind it, the excavation in front, an optional prop and a groundwater sheet.
fn sheet_wall<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (t, wall_length, retained) = (m.w(), m.l(), m.h());
    let embed = m.aux("embed", 0.6 * retained);
    let reach = retained.max(3.0) * 1.2;
    let depth = embed + 0.5 * retained.max(2.0) + 1.0;
    let hy = wall_length / 2.0 + 1.0;
    let mut parts = vec![
        Part::new("", solid(m.pose, [0.0, 0.0, -embed], [t, wall_length, retained + embed]), m.role.clone(), ("Sheet wall", "Spundwand")),
        Part::new("retained", solid(m.pose, [t / 2.0 + reach / 2.0, 0.0, 0.0], [reach, wall_length, retained]), soil_role("backfill"), ("Retained soil", "Anstehender Boden")).context(CONTEXT_OPACITY),
    ];
    parts.extend(soil(m, 0.0, reach + 1.0, hy, &[depth], None));
    let water = m.has("water").then(|| retained - m.aux("water", 0.4 * retained));
    if let Some(level) = water {
        parts.push(Part::new("water", solid(m.pose, [t / 2.0 + reach / 2.0, 0.0, level], [reach, wall_length, 0.03]), soil_role("water"), ("Groundwater table", "Grundwasserspiegel")).context(WATER_OPACITY));
    }
    let prop_depth = m.has("prop").then(|| m.aux("prop", 0.2 * retained));
    if let Some(prop) = prop_depth {
        let frame = m.pose.axes([t / 2.0, 0.0, retained - prop], [0.0, 0.0, -1.0], Y, X);
        parts.push(Part::new("prop", SolidShape::Cylinder { frame, radius: 0.08, height: reach * 0.8 }, MaterialRole::Steel, ("Prop", "Steife")));
    }
    let plans = vec![
        face("width", m.g.width, 2.0, X, [t / 2.0, 0.0, retained / 2.0]),
        face("length", m.g.length, 2.0, Y, [0.0, wall_length / 2.0, retained / 2.0]),
        scalar("height", m.g.height, 1.0, Z, [0.0, 0.0, retained]),
        scalar("embed", m.key("embed"), -1.0, Z, [0.0, 0.0, -embed]),
        scalar("prop", m.key("prop"), -1.0, Z, [t / 2.0 + reach * 0.8, 0.0, retained - prop_depth.unwrap_or(0.0)]),
        scalar("water", m.key("water"), -1.0, Z, [t / 2.0 + reach, 0.0, water.unwrap_or(retained)]),
    ];
    Built { parts, top: retained, bottom: -depth, half: (reach + 1.0, hy), plans }
}

/// ⛰️ A slope or embankment: crest block and wedge of the slope face on ground with an optional soft layer and a water level above the toe.
fn slope<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (crest, extrusion, h) = (m.w(), m.l(), m.h());
    let angle = match m.ratio("ratio") {
        Some(ratio) if m.has("ratio") => (1.0 / ratio).atan(),
        _ => m.angle("angle", 30.0_f64.to_radians()),
    };
    let run = h / angle.tan();
    let soft = m.has("soft").then(|| m.aux("soft", 2.0));
    let stack: Vec<f64> = soft.map_or_else(|| vec![2.0], |s| vec![s, 2.0]);
    let (reach_left, reach_right) = (crest + 2.0, run + 4.0);
    let (cx, hx) = ((reach_right - reach_left) / 2.0, (reach_left + reach_right) / 2.0);
    let hy = extrusion / 2.0 + 1.0;
    let level = m.has("level").then(|| m.aux("level", 0.25 * h));
    let frame = m.pose.frame([0.0, -extrusion / 2.0, 0.0]);
    let mut parts = vec![
        Part::new("", SolidShape::Wedge { frame, base: Vec2::new(run, extrusion), height: h }, soil_role("fill"), ("Slope", "Böschung")).glass(0.85),
        Part::new("crest", solid(m.pose, [-crest / 2.0, 0.0, 0.0], [crest, extrusion, h]), soil_role("fill"), ("Crest", "Krone")).glass(0.85),
    ];
    parts.extend(soil(m, cx, hx, hy, &stack, None));
    if let Some(level) = level {
        parts.push(Part::new("water", solid(m.pose, [cx, 0.0, level], [2.0 * hx, 2.0 * hy, 0.03]), soil_role("water"), ("Water level", "Wasserspiegel")).context(WATER_OPACITY));
    }
    let plans: Vec<Plan> = vec![
        scalar("height", m.g.height, 1.0, Z, [0.0, 0.0, h]),
        face("crest", m.g.width, 1.0, NEG_X, [-crest, 0.0, h / 2.0]),
        face("length", m.g.length, 2.0, Y, [0.0, extrusion / 2.0, h / 2.0]),
        scalar("soft", m.key("soft"), -1.0, Z, [run + 2.0, 0.0, -soft.unwrap_or(0.0)]),
        scalar("level", m.key("level"), 1.0, Z, [reach_right, 0.0, level.unwrap_or(0.0)]),
        super::angle("slope-angle", m.key("angle"), Y, [run, 0.0, 0.0], [0.0, 0.0, h], std::f64::consts::PI, (0.05 - std::f64::consts::PI, 1.5 - std::f64::consts::PI)),
    ]
    .into_iter()
    .chain(m.ratio("ratio").filter(|_| m.has("ratio")).map(|ratio| Plan { kind: GripKind::Gauge { k: 1.0 / h, step: 0.05, limits: ((0.2 - ratio) * h, (20.0 - ratio) * h), follow: Vec::new() }, ..scalar("slope-ratio", m.key("ratio"), 1.0, X, [run, 0.0, 0.0]) }))
    .collect();
    Built { parts, top: h, bottom: -stack.iter().sum::<f64>(), half: (hx + cx.abs(), hy), plans }
}

/// 🌍️ A soil block with stacked layers shown by interface sheets, a groundwater sheet and an optional test rod pushed from the surface.
fn soil_block<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (w, l, depth) = (m.w(), m.l(), m.h());
    let mut parts = vec![Part::new("", solid(m.pose, [0.0, 0.0, -depth], [w, l, depth]), soil_role("layer-0"), ("Soil block", "Bodenblock")).glass(0.55)];
    let mut top = 0.0;
    for (index, thickness) in m.layers().iter().enumerate() {
        top -= thickness;
        parts.push(Part::new(format!("interface-{index}"), solid(m.pose, [0.0, 0.0, top.max(-depth)], [w, l, 0.02]), soil_role(&format!("layer-{}", (index + 1) % 4)), ("Layer interface", "Schichtgrenze")).context(WATER_OPACITY));
    }
    let water = m.has("water").then(|| -m.aux("water", 0.4 * depth));
    if let Some(level) = water {
        parts.push(Part::new("water", solid(m.pose, [0.0, 0.0, level], [w, l, 0.03]), soil_role("water"), ("Groundwater table", "Grundwasserspiegel")).context(WATER_OPACITY));
    }
    let rod = m.has("rod").then(|| m.aux("rod", 0.8 * depth));
    if let Some(penetration) = rod {
        parts.push(Part::new("rod", pillar(m.pose, [0.0, 0.0, -penetration], 0.04, penetration + 0.8), MaterialRole::Steel, ("Test rod", "Sondiergestänge")));
    }
    let plans: Vec<Plan> = vec![
        face("width", m.g.width, 2.0, X, [w / 2.0, 0.0, -depth / 2.0]),
        face("length", m.g.length, 2.0, Y, [0.0, l / 2.0, -depth / 2.0]),
        scalar("depth", m.g.height, -1.0, Z, [w / 2.0, l / 2.0, -depth]),
        scalar("water", m.key("water"), -1.0, Z, [w / 2.0, 0.0, water.unwrap_or(0.0)]),
        scalar("rod", m.key("rod"), -1.0, Z, [0.0, 0.0, -rod.unwrap_or(0.0)]),
    ]
    .into_iter()
    .chain(layer_plans(m, w / 2.0))
    .collect();
    Built { parts, top: if rod.is_some() { 0.8 } else { 0.0 }, bottom: -depth, half: (w / 2.0, l / 2.0), plans }
}

/// ⚓️ A ground anchor: wall, inclined free tendon and fixed bore of the declared diameter and length in the retained soil.
fn ground_anchor<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (d, fixed) = (m.w(), m.l());
    let free = m.aux("free", 6.0);
    let theta = m.angle("angle", 15.0_f64.to_radians());
    let (cos, sin) = (theta.cos(), theta.sin());
    let reach = (free + fixed) * cos;
    let drop = (free + fixed) * sin;
    let wall_height = drop + 3.0;
    let start = [0.2, 0.0, -0.25 * wall_height];
    let junction = [start[0] + free * cos, 0.0, start[2] - free * sin];
    let along = [cos, 0.0, -sin];
    let across = [-sin, 0.0, -cos];
    let tip = [junction[0] + fixed * cos, 0.0, junction[2] - fixed * sin];
    let hy = 3.0;
    let mut parts = vec![
        Part::new("", SolidShape::Cylinder { frame: m.pose.axes(junction, across, Y, along), radius: d / 2.0, height: fixed }, MaterialRole::Concrete, ("Fixed length", "Haftstrecke")),
        Part::new("free", SolidShape::Cylinder { frame: m.pose.axes(start, across, Y, along), radius: 0.03, height: free }, MaterialRole::Steel, ("Free length", "Freie Stahllänge")),
        Part::new("wall", solid(m.pose, [0.0, 0.0, -wall_height], [0.4, 2.0 * hy, wall_height]), m.role.clone(), ("Wall", "Wand")),
    ];
    parts.extend(soil(m, 0.2 + (reach + 3.0) / 2.0, (reach + 3.0) / 2.0, hy, &[wall_height + 1.0], None));
    let mid = [(junction[0] + tip[0]) / 2.0, 0.0, (junction[2] + tip[2]) / 2.0];
    let plans = vec![
        scalar("length", m.g.length, 1.0, along, tip),
        scalar("free", m.key("free"), 1.0, along, junction),
        face("diameter", m.g.width, 2.0, across, [mid[0] + across[0] * d / 2.0, 0.0, mid[2] + across[2] * d / 2.0]),
        angle("anchor-angle", m.key("angle"), Y, start, tip, 0.0, (0.05, 1.5)),
    ];
    Built { parts, top: 0.0, bottom: -(wall_height + 1.0), half: (0.2 + reach + 3.0, hy), plans }
}

/// 🧱️ The thickness grips of the declared soil layers: one per layer on the interface below it at `x`, dragging the interface down thickens the layer.
fn layer_plans<C: CaseCatalogue>(m: &Meas<'_, C>, x: f64) -> Vec<Plan> {
    let key = m.key("layers");
    m.layers().iter().enumerate().scan(0.0, |depth, (index, thickness)| {
        *depth += thickness;
        Some(scalar(&format!("layer-{}", index + 1), &design_case::series_key(key, index), -1.0, Z, [x, 0.0, -*depth]))
    }).collect()
}

/// 💧 A seepage path: a soil block of the flow length with water on both sides at levels that differ by the head, and an optional cut-off wall.
fn seepage<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (w, flow, head) = (m.w(), m.l(), m.h());
    let layer = m.aux("layer", 4.0);
    let embed = m.has("embed").then(|| m.aux("embed", 0.5 * layer));
    let base_level = 1.0;
    let upstream = base_level + head;
    let mut parts = vec![
        Part::new("", solid(m.pose, [0.0, 0.0, -layer], [flow, w, layer]), soil_role("layer-0"), ("Soil layer", "Bodenschicht")).glass(0.55),
        Part::new("upstream", solid(m.pose, [-flow / 4.0, 0.0, 0.0], [flow / 2.0 - 0.1, w, base_level]), soil_role("water"), ("Upstream water", "Oberwasser")).context(WATER_OPACITY),
        Part::new("head", solid(m.pose, [-flow / 4.0, 0.0, base_level], [flow / 2.0 - 0.1, w, head]), soil_role("water"), ("Hydraulic head", "Wasserstandsdifferenz")).context(WATER_OPACITY),
        Part::new("downstream", solid(m.pose, [flow / 4.0, 0.0, 0.0], [flow / 2.0 - 0.1, w, base_level]), soil_role("water"), ("Downstream water", "Unterwasser")).context(WATER_OPACITY),
    ];
    if let Some(cut) = embed {
        parts.push(Part::new("cut-off", solid(m.pose, [0.0, 0.0, -cut], [0.1, w, cut + upstream]), MaterialRole::Steel, ("Cut-off wall", "Dichtwand")));
    }
    let plans = vec![
        scalar("head", m.g.height, 1.0, Z, [-flow / 4.0, 0.0, upstream]),
        face("length", m.g.length, 2.0, X, [flow / 2.0, 0.0, -layer / 2.0]),
        face("width", m.g.width, 2.0, Y, [0.0, w / 2.0, -layer / 2.0]),
        scalar("embed", m.key("embed"), -1.0, Z, [0.0, 0.0, -embed.unwrap_or(0.0)]),
        scalar("layer", m.key("layer"), -1.0, Z, [flow / 2.0, 0.0, -layer]),
    ];
    Built { parts, top: upstream, bottom: -layer, half: (flow / 2.0, w / 2.0), plans }
}

/// 🚰 A pumped well in an aquifer: the aquifer cylinder to its radius of influence, the casing, and three stepped water-table discs that fall from the undisturbed level to the level in the well along the logarithmic drawdown curve.
fn well<C: CaseCatalogue>(m: &Meas<'_, C>) -> Built {
    let (influence, thickness) = (m.w(), m.h());
    let radius = m.aux("radius", 0.3);
    let level = m.aux("level", 0.6 * thickness);
    let mut parts = vec![
        Part::new("", pillar(m.pose, [0.0, 0.0, -thickness], influence, thickness), soil_role("layer-0"), ("Aquifer", "Grundwasserleiter")).glass(0.55),
        Part::new("casing", pillar(m.pose, [0.0, 0.0, -thickness], radius.max(0.05), thickness + 0.5), MaterialRole::Steel, ("Well casing", "Brunnenrohr")),
    ];
    let ratio = (influence / radius).ln().max(1e-6);
    for (index, fraction) in [1.0, 2.0 / 3.0, 1.0 / 3.0].into_iter().enumerate() {
        let at = (influence * fraction).max(radius);
        let surface = level + (thickness - level) * (at / radius).ln() / ratio;
        parts.push(Part::new(format!("water-{index}"), pillar(m.pose, [0.0, 0.0, surface - thickness], at, 0.05), soil_role("water"), ("Water table", "Wasserspiegel")).context(WATER_OPACITY));
    }
    let plans = vec![
        face("influence", m.g.width, 1.0, X, [influence, 0.0, -thickness / 2.0]),
        scalar("thickness", m.g.height, -1.0, Z, [0.0, 0.0, -thickness]),
        face("radius", m.key("radius"), 1.0, X, [radius, 0.0, 0.3]),
        scalar("level", m.key("level"), 1.0, Z, [0.0, 0.0, level - thickness]),
    ];
    Built { parts, top: 0.5, bottom: -thickness, half: (influence, influence), plans }
}
