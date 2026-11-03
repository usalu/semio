#!/usr/bin/env python3
"""EN 1995 v6: moves the entities module of the schema checks from the `layup` string and the regular fastener pattern to the typed layer rows and explicit positions. Idempotent (guard: `PatternFacts`)."""
import re, sys

P = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚖️timber/🧱️entities/🦀️.rs"
t = open(P, encoding="utf-8").read()
if "pub struct PatternFacts" in t:
    sys.exit(0)


def rep(a, b, count=1):
    global t
    assert t.count(a) == count, (t.count(a), a[:100])
    t = t.replace(a, b)


rep("use semio_s_artifact_norm_en1995_model::{CaseNumber, CaseText};", "use semio_s_artifact_norm_en1995_model::{CaseNumber, CaseText, PanelLayer};")
rep("pub struct Home {\n    pub key: &'static str,", "pub struct Home {\n    pub key: String,")
rep("bindings.iter().map(|b| Home { key: b.key, path:", "bindings.iter().map(|b| Home { key: b.key.to_string(), path:")
rep("struct Dest<'a> {\n    collection: &'static str,\n    row: &'a str,\n    member: &'a str,\n    connection: &'a str,\n}", "struct Dest<'a> {\n    collection: &'static str,\n    row: &'a str,\n    member: &'a str,\n    connection: &'a str,\n    layers: &'a [PanelLayer],\n}")
for coll in ("tapers", "cutouts", "floors"):
    t = re.sub(r'(let home = Dest \{ collection: "%s", row: &\w\.id, member: &m\.id, connection: "")( \})' % coll, r"\1, layers: &[]\2", t)
rep('let home = Dest { collection: "fasteners", row: &f.id, member: &m.id, connection: &c.id };', 'let home = Dest { collection: "fasteners", row: &f.id, member: &m.id, connection: &c.id, layers: &[] };')
rep('let home = Dest { collection: "panels", row: &p.id, member: "", connection: "" };', 'let home = Dest { collection: "panels", row: &p.id, member: "", connection: "", layers: &p.layers };')
assert t.count("layers: &[] }") == 4, t.count("layers: &[] }")

rep("""        None => (format!("{}[id={}]", home.collection, home.row), None),
    }
}""", """        None => match key.and_then(|key| layer_thickness(key, home)) {
            Some((path, binding)) => (path, Some(binding)),
            None => (format!("{}[id={}]", home.collection, home.row), None),
        },
    }
}

static LAYER_THICKNESS: Binding = bind("t", "panels[id={row}].layers[id={layer}].thicknessM", MM);

/// 🧱️ The stored thickness of the layer a case input `t<k>` stands for (the k-th layer of the panel, bottom to top).
fn layer_thickness(key: &str, home: &Dest<'_>) -> Option<(String, &'static Binding)> {
    let layer = home.layers.get(key.strip_prefix('t')?.parse::<usize>().ok()?.checked_sub(1)?)?;
    Some((LAYER_THICKNESS.path.replace("{row}", home.row).replace("{layer}", &layer.id), &LAYER_THICKNESS))
}""")

rep('const PANEL: &[Binding] = &[', 'const FASTENER_EXPLICIT: &[Binding] = &[\n    bind("alpha_deg", "fasteners[id={row}].loadAngleRad", DEG),\n    bind("d", "connections[id={connection}].diameterM", MM),\n    bind("t1", "connections[id={connection}].t1M", MM),\n    bind("t2", "connections[id={connection}].t2M", MM),\n];\n\nconst PANEL: &[Binding] = &[')

old_fast = t[t.index("    let (d, alpha) = (c.diameter_m * MM, f.load_angle_rad * DEG);"):t.index("    out\n}\n\n/// 🔩️ The largest pattern the scene draws")]
new_fast = '''    let (d, alpha) = (c.diameter_m * MM, f.load_angle_rad * DEG);
    let kind = c.fastener_type.as_str();
    let facts = pattern_facts(f, m.span_m * MM, m.h_m * MM);
    let bindings = if f.positions.is_empty() { FASTENER } else { FASTENER_EXPLICIT };
    let rows = vec![
        (case_row(&id("T-74"), "T-74", &label, &[("d", d), ("alpha_deg", alpha), ("rho_k", rho), ("a1", facts.a1), ("a2", facts.a2), ("a3t", facts.a3t), ("a4c", facts.a4c)], &[("kind", kind)]), bindings),
        (case_row(&id("T-75"), "T-75", &label, &[("n", f64::from(facts.per_row)), ("a1", facts.a1), ("d", d), ("alpha_deg", alpha)], &[("kind", kind)]), bindings),
    ];
    let mut out = run(annex, rows, &home);
    let pattern = u64::from(facts.count);
    if pattern != u64::from(c.number) {
        let subject = conn_subject(c, "number");
        let shape = if f.positions.is_empty() { format!("{} × {}", f.columns, f.lines) } else { format!("{} explicit positions", f.positions.len()) };
        let shape_de = if f.positions.is_empty() { format!("{} × {}", f.columns, f.lines) } else { format!("{} explizite Lagen", f.positions.len()) };
        out.push(
            CheckResult::assess(format!("en1995.8.pattern-count.{}", f.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "§8", "8.1"), subject.clone(), loc("Fastener count of the pattern", "Verbindungsmittelzahl des Bildes"))
                .annex(annex)
                .explanation(loc(&format!("The pattern ({shape}) holds {pattern} fasteners, the connection declares {}.", c.number), &format!("Das Bild ({shape_de}) enthält {pattern} Verbindungsmittel, die Verbindung gibt {} an.", c.number)))
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .remedy(Remedy::exactly(subject, Quantity::new(QuantityKind::Dimensionless, f64::from(c.number)), Quantity::new(QuantityKind::Dimensionless, pattern as f64), loc("Set the number of fasteners to the pattern count.", "Die Anzahl der Verbindungsmittel auf die Bildzahl setzen.")))
                .build(),
        );
    }
'''
t = t.replace(old_fast, new_fast)

a = t.index("/// 🔩️ The fastener positions of a pattern in mm")
b = t.index("fn with_extra<'a>")
new_pat = '''/// 🔩️ The most explicit fastener positions the scene draws.
pub const PATTERN_POINTS_MAX: usize = 72;

/// 🔩️ Two fasteners whose bottom-edge distances differ by at most this many mm stand in the same row.
pub const ROW_TOLERANCE_MM: f64 = 0.5;

/// 🔩️ The fastener positions of a pattern in mm from the member end and the bottom edge: the explicit `positions` of an irregular pattern, else row by row the regular pattern whose first fastener stands at the end distance a₃ and the edge distance a₄ and whose others follow at a₁ along and a₂ across the grain.
pub fn pattern_mm(f: &Fastener) -> Vec<(f64, f64)> {
    if !f.positions.is_empty() {
        return f.positions.iter().take(PATTERN_POINTS_MAX).map(|p| (p.x_m * MM, p.y_m * MM)).collect();
    }
    let (columns, lines) = (f.columns.clamp(1, PATTERN_COLUMNS_MAX), f.lines.clamp(1, PATTERN_LINES_MAX));
    (0..lines).flat_map(|j| (0..columns).map(move |i| ((f.end_distance_m + f64::from(i) * f.spacing_parallel_m) * MM, (f.edge_distance_m + f64::from(j) * f.spacing_perpendicular_m) * MM))).collect()
}

/// 🔩️ The count, the longest row and the governing spacings of a pattern in mm: the declared values of a regular pattern, the minima of the actual positions of an irregular one (a₁ between neighbours of one row, a₂ between neighbouring rows, a₃,t of the first fastener, a₄,c to the nearer edge); a missing neighbour never governs (the member span or depth stands in).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PatternFacts {
    pub count: u32,
    pub per_row: u32,
    pub a1: f64,
    pub a2: f64,
    pub a3t: f64,
    pub a4c: f64,
}

/// 🔩️ The positions of an irregular pattern grouped into rows (ascending bottom-edge distance, ascending end distance inside a row).
pub fn pattern_rows_mm(points: &[(f64, f64)]) -> Vec<Vec<(f64, f64)>> {
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.total_cmp(&b.0)));
    let mut rows: Vec<Vec<(f64, f64)>> = Vec::new();
    for point in sorted {
        match rows.last_mut() {
            Some(row) if (point.1 - row[0].1).abs() <= ROW_TOLERANCE_MM => row.push(point),
            _ => rows.push(vec![point]),
        }
    }
    for row in &mut rows {
        row.sort_by(|a, b| a.0.total_cmp(&b.0));
    }
    rows
}

/// 🔩️ The facts of the pattern of a fastener row; `span_mm` and `depth_mm` belong to the member it stands on.
pub fn pattern_facts(f: &Fastener, span_mm: f64, depth_mm: f64) -> PatternFacts {
    if f.positions.is_empty() {
        return PatternFacts { count: f.columns.saturating_mul(f.lines), per_row: f.columns, a1: f.spacing_parallel_m * MM, a2: f.spacing_perpendicular_m * MM, a3t: f.end_distance_m * MM, a4c: f.edge_distance_m * MM };
    }
    let points: Vec<(f64, f64)> = f.positions.iter().map(|p| (p.x_m * MM, p.y_m * MM)).collect();
    let rows = pattern_rows_mm(&points);
    let along = rows.iter().flat_map(|row| row.windows(2).map(|pair| pair[1].0 - pair[0].0)).fold(span_mm, f64::min);
    let across = rows.windows(2).map(|pair| pair[1][0].1 - pair[0][0].1).fold(depth_mm, f64::min);
    let (low, high) = points.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), p| (low.min(p.1), high.max(p.1)));
    let end = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    PatternFacts { count: u32::try_from(points.len()).unwrap_or(u32::MAX), per_row: u32::try_from(rows.iter().map(Vec::len).max().unwrap_or(0)).unwrap_or(u32::MAX), a1: along, a2: across, a3t: end, a4c: low.min(depth_mm - high) }
}

/// 🧱️ The layers of a CLT panel bottom to top as `(thickness in mm, along the span)`; `None` for a panel without layers, a layer of non-positive or non-finite thickness or an unknown orientation.
pub fn layers_mm(p: &Panel) -> Option<Vec<(f64, bool)>> {
    let layers = p
        .layers
        .iter()
        .map(|layer| {
            let along = match layer.orientation.as_str() {
                "longitudinal" => true,
                "transverse" => false,
                _ => return None,
            };
            let thickness = layer.thickness_m * MM;
            (thickness.is_finite() && thickness > 0.0).then_some((thickness, along))
        })
        .collect::<Option<Vec<_>>>()?;
    (!layers.is_empty()).then_some(layers)
}

/// 🧱️ The case inputs `n_layers`, `t<k>` (mm) and `o<k>` (1 along the span, 0 across) of the layers.
fn layer_numbers(layers: &[(f64, bool)]) -> Vec<(String, f64)> {
    let mut numbers: Vec<(String, f64)> = vec![("n_layers".into(), layers.len() as f64)];
    for (k, (thickness, along)) in layers.iter().enumerate() {
        numbers.push((format!("t{}", k + 1), *thickness));
        numbers.push((format!("o{}", k + 1), if *along { 1.0 } else { 0.0 }));
    }
    numbers
}

/// 🧱️ The lamella class the kernel evaluates a panel with: the grade of the first layer along the span (the outer layers carry the bending), the class of the panel when it has none.
fn panel_class(p: &Panel) -> &str {
    p.layers.iter().find(|layer| layer.orientation == "longitudinal").map_or(p.strength_class.as_str(), |layer| layer.grade.as_str())
}

'''
t = t[:a] + new_pat + t[b:]

old = t[t.index("    let Some(layers) = layup_mm(&p.layup) else {"):t.index("    let m_d = p.load_pa * p.span_m * p.span_m / 8.0;")]
new = '''    let Some(layers) = layers_mm(p) else {
        let subject = SubjectRef::new(&p.id, format!("panels[id={}].layers", p.id), loc(&label.0, &label.1));
        return vec![CheckResult::assess(format!("en1995.clt.layers.{}", p.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "Annex CLT", "layers"), subject, loc("CLT layers", "BSP-Lagen"))
            .annex(annex)
            .explanation(loc("A panel needs at least one layer; every layer needs a positive thickness and the orientation longitudinal or transverse.", "Eine Platte braucht mindestens eine Lage; jede Lage braucht eine positive Dicke und die Faserrichtung längs oder quer."))
            .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
            .build()];
    };
    let id = |entry: &str| format!("panels:{}:{entry}", p.id);
    let numbers = layer_numbers(&layers);
'''
t = t.replace(old, new)
rep('''    let texts = [("class", p.strength_class.as_str())];
    let mut rows = vec![(case_row(&id("T-97")''', '''    let texts = [("class", panel_class(p))];
    let mut rows = vec![(case_row(&id("T-97")''')
rep("    let mut out = Vec::new();\n    if p.fire_duration_s > 0.0 {\n        rows.push", "    let mut out = mixed_grade(annex, p, &label);\n    if p.fire_duration_s > 0.0 {\n        rows.push")
rep('''            result.ndps.push("ndp.en1995.panel.fire-load-ratio".to_string());
        }
        out.push(result);
    }
    out
}''', '''            result.ndps.push("ndp.en1995.panel.fire-load-ratio".to_string());
        }
        out.push(result);
    }
    out
}

/// 🧱️ A warning when the layers carry different grades: the kernel evaluates the section with one lamella class.
fn mixed_grade(annex: AnnexChoice, p: &Panel, label: &(String, String)) -> Vec<CheckResult> {
    let class = panel_class(p);
    if p.layers.iter().all(|layer| layer.grade == class) {
        return Vec::new();
    }
    let subject = SubjectRef::new(&p.id, format!("panels[id={}].layers", p.id), loc(&label.0, &label.1));
    vec![CheckResult::assess(format!("en1995.clt.mixed-grade.{}", p.id), "EN 1995-1-1", ClauseId::new("EN 1995-1-1", "Annex CLT", "layers"), subject, loc("CLT layer grades", "BSP-Lagenklassen"))
        .annex(annex)
        .explanation(loc(&format!("The layers carry different grades; the section is evaluated with the grade {class} of the first longitudinal layer."), &format!("Die Lagen haben verschiedene Klassen; der Querschnitt wird mit der Klasse {class} der ersten Längslage berechnet.")))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Warning)
        .build()]
}''')

rep("""            let layers = layup_mm(&p.layup)?;
            let label = (format!("CLT panel {}", p.id), format!("BSP-Platte {}", p.id));
            let mut numbers: Vec<(String, f64)> = vec![("n_layers".into(), layers.len() as f64), ("l".into(), p.span_m * MM), ("width_m".into(), p.width_m)];
            for (k, t) in layers.iter().enumerate() {
                numbers.push((format!("t{}", k + 1), *t));
                numbers.push((format!("o{}", k + 1), if k % 2 == 0 { 1.0 } else { 0.0 }));
            }
""", '''            let layers = layers_mm(p)?;
            let label = (format!("CLT panel {}", p.id), format!("BSP-Platte {}", p.id));
            let mut numbers = layer_numbers(&layers);
            numbers.extend([("l".to_string(), p.span_m * MM), ("width_m".to_string(), p.width_m)]);
''')
rep("""            let row = drawing(&p.id, "T-97", &label, &flat, &[("class", p.strength_class.as_str())]);
            let homes = homes(PANEL, &HomeIds { row: &p.id, member: "", connection: "" }, &row);
            Some((row, homes))""", """            let row = drawing(&p.id, "T-97", &label, &flat, &[("class", panel_class(p))]);
            let mut homes = homes(PANEL, &HomeIds { row: &p.id, member: "", connection: "" }, &row);
            homes.extend(p.layers.iter().enumerate().map(|(k, layer)| Home { key: format!("t{}", k + 1), path: format!("panels[id={}].layers[id={}].thicknessM", p.id, layer.id), scale: MM, offset: 0.0 }));
            Some((row, homes))""")

rep("""            let mut numbers: Vec<(String, f64)> = [("d", c.diameter_m * MM), ("t1", c.t1_m * MM), ("t2", (if c.steel_plate { c.steel_plate_thickness_m } else { c.t2_m }) * MM), ("a1", f.spacing_parallel_m * MM), ("a2", f.spacing_perpendicular_m * MM), ("a3t", f.end_distance_m * MM), ("a4t", f.edge_distance_m * MM), ("n", f64::from(f.columns)), ("rows", f64::from(f.lines)), ("alpha_deg", f.load_angle_rad * DEG)]""", '''            let facts = pattern_facts(f, m.span_m * MM, m.h_m * MM);
            let mut numbers: Vec<(String, f64)> = [("d", c.diameter_m * MM), ("t1", c.t1_m * MM), ("t2", (if c.steel_plate { c.steel_plate_thickness_m } else { c.t2_m }) * MM), ("a1", facts.a1), ("a2", facts.a2), ("a3t", facts.a3t), ("a4t", facts.a4c), ("n", f64::from(facts.per_row)), ("rows", if f.positions.is_empty() { f64::from(f.lines) } else { 0.0 }), ("explicit", if f.positions.is_empty() { 0.0 } else { 1.0 }), ("alpha_deg", f.load_angle_rad * DEG)]''')
rep("""            let homes = homes(FASTENER, &HomeIds { row: &f.id, member: &m.id, connection: &c.id }, &row);
            Some((row, homes))""", """            let mut homes = homes(if f.positions.is_empty() { FASTENER } else { FASTENER_EXPLICIT }, &HomeIds { row: &f.id, member: &m.id, connection: &c.id }, &row);
            for (k, point) in f.positions.iter().take(PATTERN_POINTS_MAX).enumerate() {
                homes.push(Home { key: format!("px{}", k + 1), path: format!("fasteners[id={}].positions[id={}].xM", f.id, point.id), scale: MM, offset: 0.0 });
                homes.push(Home { key: format!("py{}", k + 1), path: format!("fasteners[id={}].positions[id={}].yM", f.id, point.id), scale: MM, offset: 0.0 });
            }
            Some((row, homes))""")
open(P, "w", encoding="utf-8").write(t)
print("ok")
