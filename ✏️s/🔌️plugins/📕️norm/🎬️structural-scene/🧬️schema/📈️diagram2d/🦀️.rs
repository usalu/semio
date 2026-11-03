//! 📈️ Diagram2d: locale-free 2D engineering diagrams (curves, polygons, bands, section stress blocks) with a mandatory table twin.

use super::*;

records! {
    /// 📏️ Closed numeric interval with `min < max`.
    pub struct Interval {
        pub min: f64,
        pub max: f64,
    }

    /// 📐️ One diagram axis; `unit` is a unit symbol of the compliance unit vocabulary.
    pub struct Axis {
        pub label: LocalizedCopy,
        pub quantity: QuantityKind,
        pub unit: String,
        pub scale: AxisScale,
        pub domain: Option<Interval>,
        pub ticks: TickRule,
    }

    /// ✒️ Line style; colour comes from the theme through `tone`.
    pub struct Stroke {
        pub width_px: f64,
        pub dash: Dash,
        pub tone: Tone,
    }

    /// 🎨️ Area fill; colour comes from the theme through `tone`.
    pub struct Fill {
        pub tone: Tone,
        pub opacity: f64,
    }

    /// 🔵️ One scatter point.
    pub struct ScatterPoint {
        pub at: Vec2,
        pub tone: Tone,
        pub label: Option<LocalizedCopy>,
    }

    /// ➖️ Infinite line through `origin` along `direction`.
    pub struct Line2 {
        pub origin: Vec2,
        pub direction: Vec2,
    }

    /// 📊️ Distribution across a section: `x` is the depth coordinate in metres, `y` the value in the series unit.
    pub struct SectionProfile {
        pub points: Vec<Vec2>,
        pub unit: String,
    }

    /// 🔺️ Triangle mesh for the optional 3D interaction surface.
    pub struct DiagramMesh {
        pub vertices: Vec<Vec3>,
        pub triangles: Vec<Triangle>,
    }

    /// 📌️ Text note anchored at a diagram coordinate.
    pub struct DiagramNote {
        pub id: String,
        pub at: Vec2,
        pub text: LocalizedCopy,
        pub tone: Tone,
    }

    /// 🏷️ Marked diagram point, optionally tied to a check and an editable field.
    pub struct Marker {
        pub id: String,
        pub at: Vec2,
        pub shape: MarkerShape,
        pub tone: Tone,
        pub label: LocalizedCopy,
        pub utilisation: Option<f64>,
        pub bound_check: Option<String>,
        pub binding: Option<FieldBinding>,
    }

    /// 🧮️ One table-twin column: the numbers a series shows, in `unit`.
    pub struct DataColumn {
        pub header: LocalizedCopy,
        pub unit: String,
        pub values: Vec<f64>,
    }

    /// 📋️ Accessible tabular twin of a diagram; columns have equal length.
    pub struct DataTable {
        pub columns: Vec<DataColumn>,
    }

    /// 📈️ A complete diagram as a function of entity focus and report.
    pub struct Diagram2d {
        pub id: String,
        pub title: LocalizedCopy,
        pub description: LocalizedCopy,
        pub axes: Vec<Axis>,
        pub series: Vec<Series>,
        pub markers: Vec<Marker>,
        pub annotations: Vec<DiagramNote>,
        pub focus: Option<EntityRef>,
        pub legend: bool,
        pub table_twin: DataTable,
    }
}

choices! {
    /// 🪜️ Axis scale.
    pub enum AxisScale {
        Linear,
        Log10,
        Sqrt,
    }

    /// 🧷️ Which axis a limit line is perpendicular to.
    pub enum AxisRef {
        X,
        Y,
    }

    /// 〰️ Dash pattern, a non-colour cue.
    pub enum Dash {
        Solid,
        Dashed,
        Dotted,
    }

    /// 🔷️ Marker glyph, a non-colour cue.
    pub enum MarkerShape {
        Circle,
        Square,
        Diamond,
        Triangle,
        Cross,
    }
}

variants! {
    /// 📏️ Tick placement.
    pub enum TickRule {
        Auto,
        Step { step: f64 },
        Explicit { values: Vec<f64> },
    }

    /// 📉️ One drawable data series; every variant carries its legend and table label.
    pub enum Series {
        Polyline { id: String, label: LocalizedCopy, points: Vec<Vec2>, style: Stroke },
        Polygon { id: String, label: LocalizedCopy, ring: Vec<Vec2>, fill: Fill, hatch: Option<Hatch> },
        Band { id: String, label: LocalizedCopy, lower: Vec<Vec2>, upper: Vec<Vec2>, fill: Fill },
        Scatter { id: String, label: LocalizedCopy, points: Vec<ScatterPoint> },
        Limit { id: String, label: LocalizedCopy, axis: AxisRef, value: f64, style: Stroke },
        Section { id: String, label: LocalizedCopy, shape: SectionShape, strain: Option<SectionProfile>, stress: Option<SectionProfile>, neutral_axis: Option<Line2> },
        Ray { id: String, label: LocalizedCopy, origin: Vec2, through: Vec2, to_boundary: bool, style: Stroke },
        Surface3 { id: String, label: LocalizedCopy, mesh: DiagramMesh },
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn clamp(&self, value: f64) -> f64 {
        value.clamp(self.min, self.max)
    }
}

impl DataTable {
    pub fn rows(&self) -> usize {
        self.columns.first().map_or(0, |column| column.values.len())
    }
}

impl Series {
    pub fn id(&self) -> &str {
        match self {
            Self::Polyline { id, .. } | Self::Polygon { id, .. } | Self::Band { id, .. } | Self::Scatter { id, .. } | Self::Limit { id, .. } | Self::Section { id, .. } | Self::Ray { id, .. } | Self::Surface3 { id, .. } => id,
        }
    }
}

/// 🧭️ Id of the diagram note that carries the diagram's basis (the annex and the standard edition it was computed under); it belongs to the header, not to the plot.
pub const BASIS_NOTE_ID: &str = "basis:annex";

/// 🏷️ Id kind of a marker that is a pure reference point (a spectrum corner, a limit crossing) and therefore ties to no check; every other marker names its check.
pub const REFERENCE_MARKER_KIND: &str = "reference";

impl Marker {
    /// 🧷️ Whether this is a pure reference point: its id is `reference:<row>`.
    pub fn is_reference(&self) -> bool {
        self.id.split_once(':').is_some_and(|(kind, _)| kind == REFERENCE_MARKER_KIND)
    }
}

impl Diagram2d {
    /// 🧭️ The basis note of the header, if the diagram carries one.
    pub fn basis(&self) -> Option<&LocalizedCopy> {
        self.annotations.iter().find(|n| n.id == BASIS_NOTE_ID).map(|n| &n.text)
    }

    /// 📌️ The notes drawn inside the plot: every note except the basis note of the header.
    pub fn plotted_notes(&self) -> impl Iterator<Item = &DiagramNote> {
        self.annotations.iter().filter(|n| n.id != BASIS_NOTE_ID)
    }

    /// 🔎️ Series ids, marker ids and note ids share one namespace inside a diagram.
    pub fn ids(&self) -> Vec<&str> {
        self.series.iter().map(Series::id).chain(self.markers.iter().map(|m| m.id.as_str())).chain(self.annotations.iter().map(|n| n.id.as_str())).collect()
    }
}

fn finite_points(out: &mut Vec<Violation>, path: &str, points: &[Vec2], log: [bool; 2]) {
    for (index, point) in points.iter().enumerate() {
        let here = nth(path, "points", index);
        point.check(&here, out);
        expect(out, &here, !(log[0] && point.x <= 0.0) && !(log[1] && point.y <= 0.0), "positive-on-log-axis");
    }
}

fn ring_points(out: &mut Vec<Violation>, path: &str, name: &str, points: &[Vec2]) {
    for (index, point) in points.iter().enumerate() {
        point.check(&nth(path, name, index), out);
    }
}

impl Validate for Interval {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, path, &[self.min, self.max]);
        expect(out, path, self.min < self.max, "ordered");
    }
}

impl Validate for Axis {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.label.check(&at(path, "label"), out);
        non_empty(out, &at(path, "unit"), &self.unit);
        self.domain.check(&at(path, "domain"), out);
        if let (AxisScale::Log10, Some(domain)) = (self.scale, &self.domain) {
            expect(out, &at(path, "domain"), domain.min > 0.0, "positive-on-log-axis");
        }
        if let (AxisScale::Sqrt, Some(domain)) = (self.scale, &self.domain) {
            expect(out, &at(path, "domain"), domain.min >= 0.0, "non-negative-on-sqrt-axis");
        }
        self.ticks.check(&at(path, "ticks"), out);
    }
}

impl Validate for TickRule {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Auto => {}
            Self::Step { step } => positive(out, &at(path, "step"), *step),
            Self::Explicit { values } => {
                finite(out, &at(path, "values"), values);
                expect(out, &at(path, "values"), !values.is_empty() && values.windows(2).all(|w| w[0] < w[1]), "strictly-increasing");
            }
        }
    }
}

impl Validate for Stroke {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        positive(out, &at(path, "widthPx"), self.width_px);
    }
}

impl Validate for Fill {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, &at(path, "opacity"), self.opacity.is_finite() && (0.0..=1.0).contains(&self.opacity), "unit-interval");
    }
}

impl Validate for ScatterPoint {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.at.check(&at(path, "at"), out);
        self.label.check(&at(path, "label"), out);
    }
}

impl Validate for Line2 {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.origin.check(&at(path, "origin"), out);
        self.direction.check(&at(path, "direction"), out);
        expect(out, &at(path, "direction"), self.direction.x.hypot(self.direction.y) > TOLERANCE, "non-zero");
    }
}

impl Validate for SectionProfile {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "unit"), &self.unit);
        expect(out, &at(path, "points"), self.points.len() >= 2, "at-least-two");
        ring_points(out, path, "points", &self.points);
    }
}

impl Validate for DiagramMesh {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        each(&self.vertices, path, "vertices", out);
        let count = self.vertices.len() as u32;
        for (index, t) in self.triangles.iter().enumerate() {
            expect(out, &nth(path, "triangles", index), t.a < count && t.b < count && t.c < count, "index-in-range");
        }
    }
}

impl Validate for DiagramNote {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.at.check(&at(path, "at"), out);
        self.text.check(&at(path, "text"), out);
    }
}

impl Validate for Marker {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.at.check(&at(path, "at"), out);
        self.label.check(&at(path, "label"), out);
        if let Some(u) = self.utilisation {
            non_negative(out, &at(path, "utilisation"), u);
        }
        self.binding.check(&at(path, "binding"), out);
    }
}

impl Validate for DataColumn {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.header.check(&at(path, "header"), out);
        non_empty(out, &at(path, "unit"), &self.unit);
        finite(out, &at(path, "values"), &self.values);
    }
}

impl Validate for DataTable {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        expect(out, &at(path, "columns"), !self.columns.is_empty(), "non-empty");
        each(&self.columns, path, "columns", out);
        expect(out, &at(path, "columns"), self.columns.iter().all(|c| c.values.len() == self.rows()), "equal-length");
    }
}

impl Series {
    fn check_in(&self, path: &str, log: [bool; 2], out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), self.id());
        match self {
            Self::Polyline { label, points, style, .. } => {
                label.check(&at(path, "label"), out);
                style.check(&at(path, "style"), out);
                expect(out, &at(path, "points"), points.len() >= 2, "at-least-two");
                finite_points(out, path, points, log);
            }
            Self::Polygon { label, ring, fill, .. } => {
                label.check(&at(path, "label"), out);
                fill.check(&at(path, "fill"), out);
                expect(out, &at(path, "ring"), ring.len() >= 3, "at-least-three");
                ring_points(out, path, "ring", ring);
            }
            Self::Band { label, lower, upper, fill, .. } => {
                label.check(&at(path, "label"), out);
                fill.check(&at(path, "fill"), out);
                expect(out, &at(path, "lower"), lower.len() >= 2 && lower.len() == upper.len(), "equal-length-at-least-two");
                ring_points(out, path, "lower", lower);
                ring_points(out, path, "upper", upper);
            }
            Self::Scatter { label, points, .. } => {
                label.check(&at(path, "label"), out);
                each(points, path, "points", out);
            }
            Self::Limit { label, value, style, axis, .. } => {
                label.check(&at(path, "label"), out);
                style.check(&at(path, "style"), out);
                finite(out, &at(path, "value"), &[*value]);
                let log_axis = match axis {
                    AxisRef::X => log[0],
                    AxisRef::Y => log[1],
                };
                expect(out, &at(path, "value"), !log_axis || *value > 0.0, "positive-on-log-axis");
            }
            Self::Section { label, shape, strain, stress, neutral_axis, .. } => {
                label.check(&at(path, "label"), out);
                shape.check(&at(path, "shape"), out);
                strain.check(&at(path, "strain"), out);
                stress.check(&at(path, "stress"), out);
                neutral_axis.check(&at(path, "neutralAxis"), out);
            }
            Self::Ray { label, origin, through, style, .. } => {
                label.check(&at(path, "label"), out);
                style.check(&at(path, "style"), out);
                origin.check(&at(path, "origin"), out);
                through.check(&at(path, "through"), out);
                expect(out, &at(path, "through"), (through.x - origin.x).hypot(through.y - origin.y) > TOLERANCE, "distinct-from-origin");
            }
            Self::Surface3 { label, mesh, .. } => {
                label.check(&at(path, "label"), out);
                mesh.check(&at(path, "mesh"), out);
            }
        }
    }
}

impl Validate for Series {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.check_in(path, [false, false], out);
    }
}

impl Validate for Diagram2d {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.title.check(&at(path, "title"), out);
        self.description.check(&at(path, "description"), out);
        expect(out, &at(path, "axes"), self.axes.len() == 2, "exactly-two");
        each(&self.axes, path, "axes", out);
        let log = |i: usize| self.axes.get(i).is_some_and(|a| a.scale == AxisScale::Log10);
        for (index, series) in self.series.iter().enumerate() {
            series.check_in(&nth(path, "series", index), [log(0), log(1)], out);
        }
        each(&self.markers, path, "markers", out);
        each(&self.annotations, path, "annotations", out);
        self.focus.check(&at(path, "focus"), out);
        self.table_twin.check(&at(path, "tableTwin"), out);
        let ids = self.ids();
        let unique: std::collections::BTreeSet<_> = ids.iter().collect();
        expect(out, &at(path, "series"), unique.len() == ids.len(), "unique-ids");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
