//! 🖼️ Canvas lowering: `Diagram2d` to Canvas2d path and text layers with axes, ticks, grid, series, markers, notes and legend. See [`super`].

use super::*;

const MARGIN_LEFT: f64 = 72.0;
const MARGIN_RIGHT: f64 = 20.0;
const MARGIN_TOP: f64 = 40.0;
const MARGIN_BOTTOM: f64 = 56.0;
const LEGEND_WIDTH: f64 = 168.0;
const FONT: f64 = 12.0;
const MARKER_RADIUS: f64 = 6.0;

/// 👆️ The pick radius of a marker in pixels: its drawn radius plus two, never smaller than half the device's smallest hit edge (`min_target_px`, 0 keeps the drawn size), so a finger hits what it points at.
pub fn marker_hit_radius(min_target_px: u32) -> f64 {
    (MARKER_RADIUS + 2.0).max(f64::from(min_target_px) / 2.0)
}

/// 🎛️ Lowering inputs: canvas size, locale, theme, the emphasised elements and the smallest hit edge of the device (`0` keeps each target's own size).
#[derive(Clone, Debug, PartialEq)]
pub struct LoweringOptions {
    pub width: f64,
    pub height: f64,
    pub locale: Locale,
    pub dark: bool,
    pub selected: Option<String>,
    pub hovered: Option<String>,
    pub min_target_px: u32,
}

/// 🖼️ Lowered diagram: the Canvas2d layer JSON and the pick targets in the same pixel space.
#[derive(Clone, Debug, PartialEq)]
pub struct Lowered {
    pub layers_json: String,
    pub targets: Vec<PickTarget>,
    pub plot: PlotRect,
}

/// 🔲️ The plot rectangle in canvas pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlotRect {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

/// 🎨️ Colour-vision-safe Okabe-Ito colour of a tone; every tone is also told apart by dash, hatch and marker shape.
pub fn tone_rgb(tone: Tone) -> [u8; 3] {
    match tone {
        Tone::Neutral => [0, 114, 178],
        Tone::Info => [0, 158, 115],
        Tone::Warning => [230, 159, 0],
        Tone::Error => [213, 94, 0],
    }
}

/// 🪡️ Hatch used when a filled series names none, so that tone is never the only cue.
pub fn default_hatch(tone: Tone) -> Option<Hatch> {
    match tone {
        Tone::Neutral => None,
        Tone::Info => Some(Hatch::Horizontal),
        Tone::Warning => Some(Hatch::Diagonal),
        Tone::Error => Some(Hatch::CrossDiagonal),
    }
}

/// 🔷️ Marker shape default per tone for scatter points.
pub fn tone_marker(tone: Tone) -> MarkerShape {
    match tone {
        Tone::Neutral => MarkerShape::Circle,
        Tone::Info => MarkerShape::Square,
        Tone::Warning => MarkerShape::Triangle,
        Tone::Error => MarkerShape::Diamond,
    }
}

/// 📏️ Nice 1-2-5 tick values covering `[lo, hi]` with about `target` intervals.
pub fn nice_ticks(lo: f64, hi: f64, target: usize) -> Vec<f64> {
    let span = (hi - lo).abs().max(1e-300);
    let raw = span / target.max(1) as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let step = [1.0, 2.0, 5.0, 10.0].into_iter().map(|m| m * mag).find(|s| *s >= raw * 0.9999).unwrap_or(10.0 * mag);
    ticks_with_step(lo, hi, step)
}

/// 📏️ Multiples of `step` inside `[lo, hi]`.
pub fn ticks_with_step(lo: f64, hi: f64, step: f64) -> Vec<f64> {
    let first = (lo / step - 1e-9).ceil() as i64;
    let last = (hi / step + 1e-9).floor() as i64;
    (first..=last).map(|i| if i == 0 { 0.0 } else { i as f64 * step }).collect()
}

/// 📏️ Decade ticks of a logarithmic range, with 2..9 minor ticks when `minor` is set.
pub fn log_ticks(lo: f64, hi: f64, minor: bool) -> Vec<f64> {
    let (a, b) = (lo.log10().floor() as i32, hi.log10().ceil() as i32);
    (a..=b)
        .flat_map(|e| (1..10).filter(move |m| minor || *m == 1).map(move |m| m as f64 * 10f64.powi(e)))
        .filter(|v| *v >= lo * (1.0 - 1e-9) && *v <= hi * (1.0 + 1e-9))
        .collect()
}

/// 🧭️ Value-to-pixel mapping of one axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisMap {
    pub scale: AxisScale,
    pub lo: f64,
    pub hi: f64,
    pub p0: f64,
    pub p1: f64,
}

fn warp(scale: AxisScale, v: f64) -> f64 {
    match scale {
        AxisScale::Linear => v,
        AxisScale::Log10 => v.max(1e-300).log10(),
        AxisScale::Sqrt => v.max(0.0).sqrt(),
    }
}

fn unwarp(scale: AxisScale, v: f64) -> f64 {
    match scale {
        AxisScale::Linear => v,
        AxisScale::Log10 => 10f64.powf(v),
        AxisScale::Sqrt => v * v,
    }
}

impl AxisMap {
    /// 📍️ Pixel of a data value.
    pub fn map(&self, v: f64) -> f64 {
        let (a, b) = (warp(self.scale, self.lo), warp(self.scale, self.hi));
        self.p0 + (warp(self.scale, v) - a) / (b - a) * (self.p1 - self.p0)
    }

    /// 🔙️ Data value of a pixel.
    pub fn unmap(&self, p: f64) -> f64 {
        let (a, b) = (warp(self.scale, self.lo), warp(self.scale, self.hi));
        unwarp(self.scale, a + (p - self.p0) / (self.p1 - self.p0) * (b - a))
    }

    /// 🏷️ Tick values for the axis rule, with minors only for grid use.
    pub fn ticks(&self, rule: &TickRule, minor: bool) -> Vec<f64> {
        match (rule, self.scale) {
            (TickRule::Explicit { values }, _) => values.iter().copied().filter(|v| *v >= self.lo && *v <= self.hi).collect(),
            (TickRule::Step { step }, _) => ticks_with_step(self.lo, self.hi, *step),
            (TickRule::Auto, AxisScale::Log10) => log_ticks(self.lo, self.hi, minor),
            (TickRule::Auto, _) => nice_ticks(self.lo, self.hi, ((self.p1 - self.p0).abs() / 64.0).round().max(2.0) as usize),
        }
    }
}

fn extent(values: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    values.filter(|v| v.is_finite()).fold(None, |acc, v| Some(acc.map_or((v, v), |(a, b)| (a.min(v), b.max(v)))))
}

fn series_points(series: &Series, axis: usize) -> Vec<f64> {
    let pick = |p: &Vec2| if axis == 0 { p.x } else { p.y };
    match series {
        Series::Polyline { points, .. } => points.iter().map(pick).collect(),
        Series::Polygon { ring, .. } => ring.iter().map(pick).collect(),
        Series::Band { lower, upper, .. } => lower.iter().chain(upper).map(pick).collect(),
        Series::Scatter { points, .. } => points.iter().map(|p| pick(&p.at)).collect(),
        Series::Limit { axis: a, value, .. } => if (*a == AxisRef::X) == (axis == 0) { vec![*value] } else { vec![] },
        Series::Ray { origin, through, .. } => vec![pick(origin), pick(through)],
        _ => vec![],
    }
}

/// 📐️ Resolves the data range of one axis: the declared domain or nice bounds around all data and markers.
pub fn resolve_domain(diagram: &Diagram2d, axis: usize) -> (f64, f64) {
    let spec = &diagram.axes[axis];
    if let Some(d) = &spec.domain {
        return (d.min, d.max);
    }
    let marker = diagram.markers.iter().map(|m| if axis == 0 { m.at.x } else { m.at.y });
    let note = diagram.plotted_notes().map(|n| if axis == 0 { n.at.x } else { n.at.y });
    let (lo, hi) = extent(diagram.series.iter().flat_map(|s| series_points(s, axis)).chain(marker).chain(note)).unwrap_or((0.0, 1.0));
    match spec.scale {
        AxisScale::Log10 => {
            let lo = if lo > 0.0 { lo } else { 1.0 };
            let (a, b) = (lo.log10().floor(), hi.max(lo * 1.0001).log10().ceil());
            (10f64.powf(a), 10f64.powf(if b <= a { a + 1.0 } else { b }))
        }
        scale => {
            let (lo, hi) = if hi - lo < 1e-12 { (lo - 0.5, hi + 0.5) } else { (lo, hi) };
            let t = nice_ticks(lo, hi, 6);
            let (nl, nh) = (t.first().copied().unwrap_or(lo).min(lo), t.last().copied().unwrap_or(hi).max(hi));
            let step = if t.len() > 1 { t[1] - t[0] } else { 1.0 };
            let (nl, nh) = (if nl > lo { nl - step } else { nl }, if nh < hi { nh + step } else { nh });
            (if scale == AxisScale::Sqrt { nl.max(0.0) } else { nl }, nh)
        }
    }
}

struct Json(String);

impl Json {
    fn num(v: f64) -> String {
        let s = format!("{v:.2}");
        let s = s.trim_end_matches('0').trim_end_matches('.');
        if s.is_empty() || s == "-" || s == "-0" { "0".into() } else { s.into() }
    }

    fn text(s: &str) -> String {
        let mut out = String::from("\"");
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
        out
    }

    fn rgba(c: [u8; 3], a: f64) -> String {
        format!("[{},{},{},{}]", c[0], c[1], c[2], (a.clamp(0.0, 1.0) * 255.0).round() as u8)
    }

    fn point(p: [f64; 2]) -> String {
        format!("[{},{}]", Self::num(p[0]), Self::num(p[1]))
    }

    fn push(&mut self, layer: String) {
        if !self.0.is_empty() {
            self.0.push(',');
        }
        self.0.push_str(&layer);
    }
}

#[derive(Clone, Debug)]
enum Seg {
    Move([f64; 2]),
    Line([f64; 2]),
    Close,
}

struct Ink {
    text: [u8; 3],
    axis: [u8; 3],
    grid: [u8; 3],
}

struct Paint {
    fill: Option<([u8; 3], f64)>,
    stroke: Option<([u8; 3], f64, Dash, f64)>,
}

fn path_layer(id: &str, segs: &[Seg], paint: &Paint) -> String {
    let seg = |s: &Seg| match s {
        Seg::Move(p) => format!("{{\"kind\":\"move\",\"to\":{}}}", Json::point(*p)),
        Seg::Line(p) => format!("{{\"kind\":\"line\",\"to\":{}}}", Json::point(*p)),
        Seg::Close => "{\"kind\":\"close\"}".to_string(),
    };
    let mut out = format!("{{\"id\":{},\"transform\":[1,0,0,1,0,0],\"node\":{{\"kind\":\"path\",\"segments\":[{}]}}", Json::text(id), segs.iter().map(seg).collect::<Vec<_>>().join(","));
    if let Some((c, a)) = paint.fill {
        out.push_str(&format!(",\"fill\":{{\"kind\":\"solid\",\"color\":{}}}", Json::rgba(c, a)));
    }
    if let Some((c, w, dash, a)) = paint.stroke {
        let pattern = match dash {
            Dash::Solid => "[]",
            Dash::Dashed => "[8,5]",
            Dash::Dotted => "[1.5,4]",
        };
        out.push_str(&format!(",\"stroke\":{{\"color\":{},\"width\":{},\"cap\":\"butt\",\"join\":\"round\",\"dash\":{}}}", Json::rgba(c, a), Json::num(w), pattern));
    }
    out.push('}');
    out
}

fn text_layer(id: &str, at: [f64; 2], content: &str, size: f64, anchor: &str, baseline: &str, color: [u8; 3]) -> String {
    format!(
        "{{\"id\":{},\"transform\":[1,0,0,1,0,0],\"node\":{{\"kind\":\"text\",\"x\":{},\"y\":{},\"content\":{},\"size\":{},\"anchor\":\"{anchor}\",\"baseline\":\"{baseline}\"}},\"fill\":{{\"kind\":\"solid\",\"color\":{}}}}}",
        Json::text(id),
        Json::num(at[0]),
        Json::num(at[1]),
        Json::text(content),
        Json::num(size),
        Json::rgba(color, 1.0)
    )
}

fn polyline_segs(points: &[[f64; 2]], closed: bool) -> Vec<Seg> {
    let mut segs: Vec<Seg> = points.iter().enumerate().map(|(i, p)| if i == 0 { Seg::Move(*p) } else { Seg::Line(*p) }).collect();
    if closed && !points.is_empty() {
        segs.push(Seg::Close);
    }
    segs
}

fn clip_segment(a: [f64; 2], b: [f64; 2], r: &PlotRect) -> Option<([f64; 2], [f64; 2])> {
    let (mut t0, mut t1) = (0.0f64, 1.0f64);
    let d = [b[0] - a[0], b[1] - a[1]];
    for (p, q) in [(-d[0], a[0] - r.left), (d[0], r.right - a[0]), (-d[1], a[1] - r.top), (d[1], r.bottom - a[1])] {
        if p.abs() < 1e-12 {
            if q < 0.0 {
                return None;
            }
        } else {
            let t = q / p;
            if p < 0.0 { t0 = t0.max(t) } else { t1 = t1.min(t) }
        }
    }
    (t0 <= t1).then(|| ([a[0] + t0 * d[0], a[1] + t0 * d[1]], [a[0] + t1 * d[0], a[1] + t1 * d[1]]))
}

fn clip_path(points: &[[f64; 2]], r: &PlotRect) -> Vec<Vec<[f64; 2]>> {
    let mut runs: Vec<Vec<[f64; 2]>> = Vec::new();
    for w in points.windows(2) {
        match clip_segment(w[0], w[1], r) {
            Some((a, b)) => {
                let joins = runs.last().and_then(|run| run.last()).is_some_and(|l| (l[0] - a[0]).hypot(l[1] - a[1]) < 1e-6);
                if joins {
                    if let Some(run) = runs.last_mut() {
                        run.push(b);
                    }
                } else {
                    runs.push(vec![a, b]);
                }
            }
            None => {}
        }
    }
    runs
}

fn clip_ring(ring: &[[f64; 2]], r: &PlotRect) -> Vec<[f64; 2]> {
    let edges: [(usize, f64, bool); 4] = [(0, r.left, true), (0, r.right, false), (1, r.top, true), (1, r.bottom, false)];
    let mut out = ring.to_vec();
    for (axis, bound, keep_above) in edges {
        let inside = |p: [f64; 2]| if keep_above { p[axis] >= bound } else { p[axis] <= bound };
        let input = std::mem::take(&mut out);
        for i in 0..input.len() {
            let (cur, prev) = (input[i], input[(i + input.len() - 1) % input.len()]);
            let cross = || {
                let t = (bound - prev[axis]) / (cur[axis] - prev[axis]);
                [prev[0] + t * (cur[0] - prev[0]), prev[1] + t * (cur[1] - prev[1])]
            };
            if inside(cur) {
                if !inside(prev) {
                    out.push(cross());
                }
                out.push(cur);
            } else if inside(prev) {
                out.push(cross());
            }
        }
    }
    out
}

/// 🪡️ Hatch segments clipped to a ring: parallel lines every `spacing` px, or dot centres for `Dots`.
pub fn hatch_lines(ring: &[[f64; 2]], hatch: Hatch, spacing: f64) -> Vec<[[f64; 2]; 2]> {
    let angles: &[f64] = match hatch {
        Hatch::Diagonal => &[45.0],
        Hatch::CrossDiagonal => &[45.0, -45.0],
        Hatch::Horizontal => &[0.0],
        Hatch::Dots => &[0.0],
        Hatch::Vertical => &[90.0],
    };
    let mut out = Vec::new();
    for deg in angles {
        let (s, c) = deg.to_radians().sin_cos();
        let rot: Vec<[f64; 2]> = ring.iter().map(|p| [p[0] * c + p[1] * s, -p[0] * s + p[1] * c]).collect();
        let (lo, hi) = extent(rot.iter().map(|p| p[1])).unwrap_or((0.0, 0.0));
        let mut y = (lo / spacing).ceil() * spacing;
        while y <= hi {
            let mut xs: Vec<f64> = (0..rot.len())
                .filter_map(|i| {
                    let (a, b) = (rot[i], rot[(i + 1) % rot.len()]);
                    ((a[1] > y) != (b[1] > y)).then(|| a[0] + (y - a[1]) / (b[1] - a[1]) * (b[0] - a[0]))
                })
                .collect();
            xs.sort_by(f64::total_cmp);
            for pair in xs.chunks_exact(2) {
                let back = |x: f64| [x * c - y * s, x * s + y * c];
                if hatch == Hatch::Dots {
                    let mut x = (pair[0] / spacing).ceil() * spacing;
                    while x <= pair[1] {
                        let p = back(x);
                        out.push([p, p]);
                        x += spacing;
                    }
                } else {
                    out.push([back(pair[0]), back(pair[1])]);
                }
            }
            y += spacing;
        }
    }
    out
}

fn hatch_layer(id: &str, ring: &[[f64; 2]], hatch: Hatch, color: [u8; 3]) -> String {
    let lines = hatch_lines(ring, hatch, 7.0);
    let dots = hatch == Hatch::Dots;
    let mut segs = Vec::new();
    for [a, b] in lines {
        if dots {
            segs.push(Seg::Move([a[0] - 0.6, a[1]]));
            segs.push(Seg::Line([a[0] + 0.6, a[1]]));
        } else {
            segs.push(Seg::Move(a));
            segs.push(Seg::Line(b));
        }
    }
    path_layer(id, &segs, &Paint { fill: None, stroke: Some((color, if dots { 2.4 } else { 1.0 }, Dash::Solid, 0.7)) })
}

fn marker_segs(shape: MarkerShape, c: [f64; 2], r: f64) -> Vec<Seg> {
    let p = |dx: f64, dy: f64| [c[0] + dx, c[1] + dy];
    match shape {
        MarkerShape::Square => polyline_segs(&[p(-r, -r), p(r, -r), p(r, r), p(-r, r)], true),
        MarkerShape::Diamond => polyline_segs(&[p(0.0, -r * 1.3), p(r * 1.3, 0.0), p(0.0, r * 1.3), p(-r * 1.3, 0.0)], true),
        MarkerShape::Triangle => polyline_segs(&[p(0.0, -r * 1.2), p(r * 1.1, r * 0.9), p(-r * 1.1, r * 0.9)], true),
        MarkerShape::Cross => vec![Seg::Move(p(-r, -r)), Seg::Line(p(r, r)), Seg::Move(p(-r, r)), Seg::Line(p(r, -r))],
        MarkerShape::Circle => polyline_segs(&(0..24).map(|i| { let a = i as f64 / 24.0 * std::f64::consts::TAU; p(r * a.cos(), r * a.sin()) }).collect::<Vec<_>>(), true),
    }
}

fn shape_name(shape: MarkerShape) -> (&'static str, &'static str) {
    match shape {
        MarkerShape::Circle => ("circle", "Kreis"),
        MarkerShape::Square => ("square", "Quadrat"),
        MarkerShape::Diamond => ("diamond", "Raute"),
        MarkerShape::Triangle => ("triangle", "Dreieck"),
        MarkerShape::Cross => ("cross", "Kreuz"),
    }
}

/// 🔷️ Localised name of a marker shape for descriptions.
pub fn marker_shape_name(shape: MarkerShape, locale: Locale) -> &'static str {
    let (en, de) = shape_name(shape);
    match locale {
        Locale::En => en,
        Locale::De => de,
    }
}

fn boost(options: &LoweringOptions, id: &str) -> f64 {
    let on = |o: &Option<String>| o.as_deref() == Some(id);
    1.0 + if on(&options.selected) { 1.5 } else { 0.0 } + if on(&options.hovered) { 0.75 } else { 0.0 }
}

struct Frame {
    plot: PlotRect,
    x: AxisMap,
    y: AxisMap,
}

impl Frame {
    fn px(&self, p: &Vec2) -> [f64; 2] {
        [self.x.map(p.x), self.y.map(p.y)]
    }
}

fn axis_text(axis: &Axis, locale: Locale) -> String {
    format!("{} [{}]", copy_text(&axis.label, locale), axis.unit)
}

/// 🖼️ Lowers one diagram to Canvas2d layers and pick targets.
pub fn lower_diagram(diagram: &Diagram2d, options: &LoweringOptions) -> Lowered {
    let ink = if options.dark { Ink { text: [235, 235, 235], axis: [200, 200, 200], grid: [90, 90, 90] } } else { Ink { text: [20, 20, 20], axis: [60, 60, 60], grid: [205, 205, 205] } };
    let locale = options.locale;
    let section_only = !diagram.series.is_empty() && diagram.series.iter().all(|s| matches!(s, Series::Section { .. } | Series::Surface3 { .. }));
    let right = MARGIN_RIGHT + if diagram.legend { LEGEND_WIDTH } else { 0.0 };
    let plot = PlotRect { left: MARGIN_LEFT, top: MARGIN_TOP, right: (options.width - right).max(MARGIN_LEFT + 40.0), bottom: (options.height - MARGIN_BOTTOM).max(MARGIN_TOP + 40.0) };
    let mut layers = Json(String::new());
    let mut targets = Vec::new();
    layers.push(text_layer("title", [plot.left, MARGIN_TOP / 2.0], copy_text(&diagram.title, locale), FONT + 2.0, "start", "middle", ink.text));
    if section_only || diagram.axes.len() < 2 {
        let frame = Frame { plot, x: AxisMap { scale: AxisScale::Linear, lo: 0.0, hi: 1.0, p0: plot.left, p1: plot.right }, y: AxisMap { scale: AxisScale::Linear, lo: 0.0, hi: 1.0, p0: plot.bottom, p1: plot.top } };
        for series in &diagram.series {
            lower_section(series, &frame, &ink, options, &mut layers, &mut targets);
        }
        lower_legend(diagram, options, &ink, &plot, &mut layers, &mut targets);
        return Lowered { layers_json: format!("[{}]", layers.0), targets, plot };
    }
    let (xs, ys) = (resolve_domain(diagram, 0), resolve_domain(diagram, 1));
    let frame = Frame {
        plot,
        x: AxisMap { scale: diagram.axes[0].scale, lo: xs.0, hi: xs.1, p0: plot.left, p1: plot.right },
        y: AxisMap { scale: diagram.axes[1].scale, lo: ys.0, hi: ys.1, p0: plot.bottom, p1: plot.top },
    };
    lower_axes(diagram, &frame, &ink, locale, &mut layers);
    for series in &diagram.series {
        lower_series(series, &frame, &ink, options, &mut layers, &mut targets);
    }
    for marker in &diagram.markers {
        let at = frame.px(&marker.at);
        let rgb = tone_rgb(marker.tone);
        let w = boost(options, &marker.id);
        layers.push(path_layer(&format!("marker:{}", marker.id), &marker_segs(marker.shape, at, MARKER_RADIUS * (1.0 + (w - 1.0) * 0.25)), &Paint { fill: Some((rgb, 0.85)), stroke: Some((ink.text, 1.5 * w, Dash::Solid, 1.0)) }));
        layers.push(text_layer(&format!("marker-label:{}", marker.id), [at[0] + MARKER_RADIUS + 4.0, at[1] - MARKER_RADIUS], &marker_text(marker, locale), FONT, "start", "bottom", ink.text));
        targets.push(PickTarget { element: marker.id.clone(), kind: ElementKind::Marker, index: None, shape: PickShape::Point { at, radius: marker_hit_radius(options.min_target_px) }, data: Some([marker.at.x, marker.at.y]) });
    }
    for note in diagram.plotted_notes() {
        let at = frame.px(&note.at);
        layers.push(text_layer(&format!("note:{}", note.id), at, copy_text(&note.text, locale), FONT, "start", "middle", tone_rgb(note.tone)));
        let w = copy_text(&note.text, locale).chars().count() as f64 * FONT * 0.6;
        targets.push(PickTarget { element: note.id.clone(), kind: ElementKind::Note, index: None, shape: PickShape::Rect { min: [at[0], at[1] - FONT], max: [at[0] + w, at[1] + FONT] }, data: Some([note.at.x, note.at.y]) });
    }
    lower_legend(diagram, options, &ink, &plot, &mut layers, &mut targets);
    Lowered { layers_json: format!("[{}]", layers.0), targets, plot }
}

fn marker_text(marker: &Marker, locale: Locale) -> String {
    match marker.utilisation {
        Some(u) => format!("{} {}", copy_text(&marker.label, locale), format_percent(u, locale)),
        None => copy_text(&marker.label, locale).to_string(),
    }
}

fn lower_axes(diagram: &Diagram2d, frame: &Frame, ink: &Ink, locale: Locale, layers: &mut Json) {
    let plot = frame.plot;
    let mut grid = Vec::new();
    let mut minor = Vec::new();
    let mut ticks = Vec::new();
    for v in frame.x.ticks(&diagram.axes[0].ticks, true) {
        let px = frame.x.map(v);
        let major = frame.x.ticks(&diagram.axes[0].ticks, false).iter().any(|m| (m - v).abs() <= 1e-9 * v.abs().max(1.0));
        let target = if major { &mut grid } else { &mut minor };
        target.extend([Seg::Move([px, plot.top]), Seg::Line([px, plot.bottom])]);
        if major {
            ticks.extend([Seg::Move([px, plot.bottom]), Seg::Line([px, plot.bottom + 5.0])]);
            layers.push(text_layer(&format!("tick-x:{}", Json::num(v)), [px, plot.bottom + 8.0], &format_number(v, locale), FONT, "middle", "top", ink.text));
        }
    }
    for v in frame.y.ticks(&diagram.axes[1].ticks, true) {
        let py = frame.y.map(v);
        let major = frame.y.ticks(&diagram.axes[1].ticks, false).iter().any(|m| (m - v).abs() <= 1e-9 * v.abs().max(1.0));
        let target = if major { &mut grid } else { &mut minor };
        target.extend([Seg::Move([plot.left, py]), Seg::Line([plot.right, py])]);
        if major {
            ticks.extend([Seg::Move([plot.left - 5.0, py]), Seg::Line([plot.left, py])]);
            layers.push(text_layer(&format!("tick-y:{}", Json::num(v)), [plot.left - 8.0, py], &format_number(v, locale), FONT, "end", "middle", ink.text));
        }
    }
    layers.push(path_layer("grid-minor", &minor, &Paint { fill: None, stroke: Some((ink.grid, 0.5, Dash::Dotted, 0.6)) }));
    layers.push(path_layer("grid", &grid, &Paint { fill: None, stroke: Some((ink.grid, 1.0, Dash::Solid, 1.0)) }));
    layers.push(path_layer("axes", &polyline_segs(&[[plot.left, plot.top], [plot.left, plot.bottom], [plot.right, plot.bottom]], false), &Paint { fill: None, stroke: Some((ink.axis, 1.5, Dash::Solid, 1.0)) }));
    layers.push(path_layer("ticks", &ticks, &Paint { fill: None, stroke: Some((ink.axis, 1.0, Dash::Solid, 1.0)) }));
    layers.push(text_layer("axis-x", [(plot.left + plot.right) / 2.0, plot.bottom + 32.0], &axis_text(&diagram.axes[0], locale), FONT, "middle", "top", ink.text));
    layers.push(text_layer("axis-y", [plot.left, plot.top - 8.0], &axis_text(&diagram.axes[1], locale), FONT, "start", "bottom", ink.text));
}

fn lower_series(series: &Series, frame: &Frame, ink: &Ink, options: &LoweringOptions, layers: &mut Json, targets: &mut Vec<PickTarget>) {
    let plot = frame.plot;
    let id = series.id();
    let w = boost(options, id);
    let layer_id = format!("series:{id}");
    match series {
        Series::Polyline { points, style, .. } => {
            let px: Vec<[f64; 2]> = points.iter().map(|p| frame.px(p)).collect();
            let segs: Vec<Seg> = clip_path(&px, &plot).iter().flat_map(|run| polyline_segs(run, false)).collect();
            layers.push(path_layer(&layer_id, &segs, &Paint { fill: None, stroke: Some((tone_rgb(style.tone), style.width_px * w, style.dash, 1.0)) }));
            targets.push(PickTarget { element: id.to_string(), kind: ElementKind::Series, index: None, shape: PickShape::Path { points: px }, data: None });
        }
        Series::Polygon { ring, fill, hatch, .. } => lower_area(&layer_id, id, &ring.iter().map(|p| frame.px(p)).collect::<Vec<_>>(), fill, *hatch, w, frame, layers, targets),
        Series::Band { lower, upper, fill, .. } => {
            let ring: Vec<[f64; 2]> = lower.iter().chain(upper.iter().rev()).map(|p| frame.px(p)).collect();
            lower_area(&layer_id, id, &ring, fill, None, w, frame, layers, targets);
        }
        Series::Scatter { points, .. } => {
            for (i, p) in points.iter().enumerate() {
                let at = frame.px(&p.at);
                if at[0] < plot.left || at[0] > plot.right || at[1] < plot.top || at[1] > plot.bottom {
                    continue;
                }
                layers.push(path_layer(&format!("{layer_id}:{i}"), &marker_segs(tone_marker(p.tone), at, 4.5 * w.min(1.6)), &Paint { fill: Some((tone_rgb(p.tone), 0.9)), stroke: Some((ink.text, 1.0, Dash::Solid, 1.0)) }));
                targets.push(PickTarget { element: id.to_string(), kind: ElementKind::Series, index: Some(i), shape: PickShape::Point { at, radius: 7.0 }, data: Some([p.at.x, p.at.y]) });
            }
        }
        Series::Limit { axis, value, style, .. } => {
            let line = match axis {
                AxisRef::X => [[frame.x.map(*value), plot.top], [frame.x.map(*value), plot.bottom]],
                AxisRef::Y => [[plot.left, frame.y.map(*value)], [plot.right, frame.y.map(*value)]],
            };
            layers.push(path_layer(&layer_id, &polyline_segs(&line, false), &Paint { fill: None, stroke: Some((tone_rgb(style.tone), style.width_px * w, style.dash, 1.0)) }));
            targets.push(PickTarget { element: id.to_string(), kind: ElementKind::Series, index: None, shape: PickShape::Path { points: line.to_vec() }, data: None });
        }
        Series::Ray { origin, through, to_boundary, style, .. } => {
            let (o, t) = (frame.px(origin), frame.px(through));
            let far = if *to_boundary {
                let d = [t[0] - o[0], t[1] - o[1]];
                let len = d[0].hypot(d[1]).max(1e-9);
                let reach = (plot.right - plot.left).hypot(plot.bottom - plot.top) * 2.0;
                [o[0] + d[0] / len * reach, o[1] + d[1] / len * reach]
            } else {
                t
            };
            let line = clip_segment(o, far, &plot).map(|(a, b)| vec![a, b]).unwrap_or_default();
            layers.push(path_layer(&layer_id, &polyline_segs(&line, false), &Paint { fill: None, stroke: Some((tone_rgb(style.tone), style.width_px * w, style.dash, 1.0)) }));
            targets.push(PickTarget { element: id.to_string(), kind: ElementKind::Series, index: None, shape: PickShape::Path { points: line }, data: None });
        }
        Series::Section { .. } | Series::Surface3 { .. } => {}
    }
}

#[allow(clippy::too_many_arguments)]
fn lower_area(layer_id: &str, id: &str, px: &[[f64; 2]], fill: &Fill, hatch: Option<Hatch>, w: f64, frame: &Frame, layers: &mut Json, targets: &mut Vec<PickTarget>) {
    let ring = clip_ring(px, &frame.plot);
    if ring.len() < 3 {
        return;
    }
    let rgb = tone_rgb(fill.tone);
    layers.push(path_layer(layer_id, &polyline_segs(&ring, true), &Paint { fill: Some((rgb, fill.opacity)), stroke: Some((rgb, 1.5 * w, Dash::Solid, 1.0)) }));
    if let Some(h) = hatch.or_else(|| default_hatch(fill.tone)) {
        layers.push(hatch_layer(&format!("{layer_id}:hatch"), &ring, h, rgb));
    }
    targets.push(PickTarget { element: id.to_string(), kind: ElementKind::Series, index: None, shape: PickShape::Area { ring }, data: None });
}

fn lower_section(series: &Series, frame: &Frame, ink: &Ink, options: &LoweringOptions, layers: &mut Json, targets: &mut Vec<PickTarget>) {
    let Series::Section { id, shape, strain, stress, neutral_axis, .. } = series else { return };
    let Ok(regions) = shape.outline() else { return };
    let pts: Vec<Vec2> = regions.iter().flat_map(|r| r.outer.points.iter().cloned()).collect();
    let (Some((x0, x1)), Some((y0, y1))) = (extent(pts.iter().map(|p| p.x)), extent(pts.iter().map(|p| p.y))) else { return };
    let plot = frame.plot;
    let (w, h) = (plot.right - plot.left, plot.bottom - plot.top);
    let scale = (w * 0.4 / (x1 - x0).max(1e-9)).min(h * 0.9 / (y1 - y0).max(1e-9));
    let origin = [plot.left + w * 0.3 - (x0 + x1) / 2.0 * scale, plot.top + h * 0.5 + (y0 + y1) / 2.0 * scale];
    let to_px = |p: &Vec2| [origin[0] + p.x * scale, origin[1] - p.y * scale];
    let boost_w = boost(options, id);
    for (i, region) in regions.iter().enumerate() {
        let ring: Vec<[f64; 2]> = region.outer.points.iter().map(to_px).collect();
        layers.push(path_layer(&format!("series:{id}:section:{i}"), &polyline_segs(&ring, true), &Paint { fill: Some((tone_rgb(Tone::Neutral), 0.12)), stroke: Some((ink.axis, 1.5 * boost_w, Dash::Solid, 1.0)) }));
        targets.push(PickTarget { element: id.clone(), kind: ElementKind::Series, index: Some(i), shape: PickShape::Area { ring }, data: None });
    }
    let top = origin[1] - y1 * scale;
    let bottom = origin[1] - y0 * scale;
    let profile = |profile: &SectionProfile, tag: &str, tone: Tone, base_x: f64, layers: &mut Json| {
        let vmax = profile.points.iter().map(|p| p.y.abs()).fold(1e-12, f64::max);
        let depth = profile.points.iter().map(|p| p.x).fold(1e-12, f64::max);
        let amp = w * 0.22 / vmax;
        let ring: Vec<[f64; 2]> = std::iter::once([base_x, top]).chain(profile.points.iter().map(|p| [base_x + p.y * amp, top + p.x / depth * (bottom - top)])).chain(std::iter::once([base_x, top + (bottom - top)])).collect();
        layers.push(path_layer(&format!("series:{id}:{tag}"), &polyline_segs(&ring, true), &Paint { fill: Some((tone_rgb(tone), 0.35)), stroke: Some((tone_rgb(tone), 1.5, Dash::Solid, 1.0)) }));
    };
    if let Some(p) = strain {
        profile(p, "strain", Tone::Info, plot.left + w * 0.6, layers);
    }
    if let Some(p) = stress {
        profile(p, "stress", Tone::Warning, plot.left + w * 0.82, layers);
    }
    if let Some(n) = neutral_axis {
        let (a, b) = (to_px(&n.origin), [origin[0] + (n.origin.x + n.direction.x) * scale, origin[1] - (n.origin.y + n.direction.y) * scale]);
        let d = [b[0] - a[0], b[1] - a[1]];
        let len = d[0].hypot(d[1]).max(1e-9);
        let line = [[a[0] - d[0] / len * w, a[1] - d[1] / len * w], [a[0] + d[0] / len * w, a[1] + d[1] / len * w]];
        layers.push(path_layer(&format!("series:{id}:neutral-axis"), &polyline_segs(&line, false), &Paint { fill: None, stroke: Some((ink.text, 1.5, Dash::Dashed, 1.0)) }));
    }
}

fn lower_legend(diagram: &Diagram2d, options: &LoweringOptions, ink: &Ink, plot: &PlotRect, layers: &mut Json, targets: &mut Vec<PickTarget>) {
    if !diagram.legend {
        return;
    }
    let x = plot.right + 16.0;
    for (i, series) in diagram.series.iter().enumerate() {
        let y = plot.top + 6.0 + i as f64 * 22.0;
        let (swatch, text_x) = ([x, y + 6.0], x + 34.0);
        let id = series.id();
        let (segs, paint) = match series {
            Series::Polyline { style, .. } | Series::Limit { style, .. } | Series::Ray { style, .. } => (polyline_segs(&[[swatch[0], swatch[1]], [swatch[0] + 26.0, swatch[1]]], false), Paint { fill: None, stroke: Some((tone_rgb(style.tone), style.width_px.max(2.0), style.dash, 1.0)) }),
            Series::Polygon { fill, .. } | Series::Band { fill, .. } => (polyline_segs(&[[swatch[0], swatch[1] - 6.0], [swatch[0] + 26.0, swatch[1] - 6.0], [swatch[0] + 26.0, swatch[1] + 6.0], [swatch[0], swatch[1] + 6.0]], true), Paint { fill: Some((tone_rgb(fill.tone), fill.opacity.max(0.35))), stroke: Some((tone_rgb(fill.tone), 1.5, Dash::Solid, 1.0)) }),
            Series::Scatter { points, .. } => {
                let tone = points.first().map_or(Tone::Neutral, |p| p.tone);
                (marker_segs(tone_marker(tone), [swatch[0] + 13.0, swatch[1]], 5.0), Paint { fill: Some((tone_rgb(tone), 0.9)), stroke: Some((ink.text, 1.0, Dash::Solid, 1.0)) })
            }
            Series::Section { .. } | Series::Surface3 { .. } => (polyline_segs(&[[swatch[0], swatch[1] - 6.0], [swatch[0] + 26.0, swatch[1] - 6.0], [swatch[0] + 26.0, swatch[1] + 6.0], [swatch[0], swatch[1] + 6.0]], true), Paint { fill: Some((tone_rgb(Tone::Neutral), 0.12)), stroke: Some((ink.axis, 1.5, Dash::Solid, 1.0)) }),
        };
        layers.push(path_layer(&format!("legend:swatch:{id}"), &segs, &paint));
        let label = copy_text(series_label(series), options.locale);
        layers.push(text_layer(&format!("legend:text:{id}"), [text_x, y + 6.0], label, FONT, "start", "middle", ink.text));
        targets.push(PickTarget { element: id.to_string(), kind: ElementKind::Legend, index: None, shape: PickShape::Rect { min: [x, y - 2.0], max: [x + LEGEND_WIDTH - 20.0, y + 16.0] }, data: None });
    }
}
