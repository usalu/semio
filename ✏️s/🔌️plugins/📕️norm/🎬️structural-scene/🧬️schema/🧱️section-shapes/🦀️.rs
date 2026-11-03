//! 🧱️ Section shapes: parametric profiles, free polygons, composites and thin-walled sections lowered to planar outlines.

use super::*;

records! {
    /// 🏷️ Named profile dimension in metres.
    pub struct SectionParam {
        pub name: String,
        pub value: f64,
    }

    /// 🧩️ One material part of a composite section, offset from the section origin.
    pub struct CompositePart {
        pub shape: Box<SectionShape>,
        pub role: MaterialRole,
        pub offset: Vec2,
    }

    /// 🪡️ Reinforcement bars of one diameter at section points.
    pub struct RebarLayout {
        pub bar_diameter: f64,
        pub positions: Vec<Vec2>,
        pub cover: f64,
    }
}

choices! {
    /// 🧰️ Parametric profile family; the parameters each family reads are listed in [`ProfileKind::required_params`].
    pub enum ProfileKind {
        IShape,
        Rect,
        Circle,
        Chs,
        Rhs,
        Channel,
        Tee,
        Angle,
        Cee,
        Zed,
    }
}

variants! {
    /// 🔩️ Where a member's cross-section outline comes from.
    pub enum SectionShape {
        Parametric { profile: ProfileKind, params: Vec<SectionParam> },
        Polygon { polygon: Polygon2 },
        Composite { parts: Vec<CompositePart>, rebar: Vec<RebarLayout> },
        ThinWalled { centreline: Polyline2, thickness: Vec<f64> },
    }
}

const CIRCLE_SEGMENTS: usize = 48;

impl ProfileKind {
    pub fn required_params(self) -> &'static [&'static str] {
        match self {
            Self::IShape | Self::Channel | Self::Tee => &["h", "b", "tw", "tf"],
            Self::Rect => &["h", "b"],
            Self::Circle => &["d"],
            Self::Chs => &["d", "t"],
            Self::Rhs | Self::Angle => &["h", "b", "t"],
            Self::Cee | Self::Zed => &["h", "b", "t", "c"],
        }
    }
}

impl SectionParam {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self { name: name.into(), value }
    }
}

fn pts(raw: &[(f64, f64)]) -> Vec<Vec2> {
    raw.iter().map(|&(x, y)| Vec2::new(x, y)).collect()
}

fn centred(points: Vec<Vec2>) -> Vec<Vec2> {
    let (x0, x1) = points.iter().fold((f64::MAX, f64::MIN), |a, p| (a.0.min(p.x), a.1.max(p.x)));
    let (y0, y1) = points.iter().fold((f64::MAX, f64::MIN), |a, p| (a.0.min(p.y), a.1.max(p.y)));
    let (cx, cy) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    points.into_iter().map(|p| Vec2::new(p.x - cx, p.y - cy)).collect()
}

fn circle(radius: f64) -> Vec<Vec2> {
    (0..CIRCLE_SEGMENTS).map(|i| {
        let a = std::f64::consts::TAU * i as f64 / CIRCLE_SEGMENTS as f64;
        Vec2::new(radius * a.cos(), radius * a.sin())
    }).collect()
}

fn rect(w: f64, h: f64) -> Vec<Vec2> {
    pts(&[(-w / 2.0, -h / 2.0), (w / 2.0, -h / 2.0), (w / 2.0, h / 2.0), (-w / 2.0, h / 2.0)])
}

impl SectionShape {
    pub fn parametric(profile: ProfileKind, params: &[(&str, f64)]) -> Self {
        Self::Parametric { profile, params: params.iter().map(|&(n, v)| SectionParam::new(n, v)).collect() }
    }

    /// 🧱️ Canonical planar regions (section y, z as x, y), bounding-box centred; sharp corners, circles as 48-gons.
    pub fn outline(&self) -> Result<Vec<Polygon2>, Vec<Violation>> {
        self.validate()?;
        Ok(match self {
            Self::Parametric { profile, params } => {
                let p = |n: &str| params.iter().find(|q| q.name == n).map_or(0.0, |q| q.value);
                let (h, b, tw, tf, t, c, d) = (p("h"), p("b"), p("tw"), p("tf"), p("t"), p("c"), p("d"));
                vec![match profile {
                    ProfileKind::IShape => Polygon2::solid(centred(pts(&[(0.0, 0.0), (b, 0.0), (b, tf), ((b + tw) / 2.0, tf), ((b + tw) / 2.0, h - tf), (b, h - tf), (b, h), (0.0, h), (0.0, h - tf), ((b - tw) / 2.0, h - tf), ((b - tw) / 2.0, tf), (0.0, tf)]))),
                    ProfileKind::Rect => Polygon2::solid(rect(b, h)),
                    ProfileKind::Circle => Polygon2::solid(circle(d / 2.0)),
                    ProfileKind::Chs => Polygon2 { outer: Ring2 { points: circle(d / 2.0) }, holes: vec![Ring2 { points: circle(d / 2.0 - t) }] },
                    ProfileKind::Rhs => Polygon2 { outer: Ring2 { points: rect(b, h) }, holes: vec![Ring2 { points: rect(b - 2.0 * t, h - 2.0 * t) }] },
                    ProfileKind::Channel => Polygon2::solid(centred(pts(&[(0.0, 0.0), (b, 0.0), (b, tf), (tw, tf), (tw, h - tf), (b, h - tf), (b, h), (0.0, h)]))),
                    ProfileKind::Tee => Polygon2::solid(centred(pts(&[(0.0, h), (0.0, h - tf), ((b - tw) / 2.0, h - tf), ((b - tw) / 2.0, 0.0), ((b + tw) / 2.0, 0.0), ((b + tw) / 2.0, h - tf), (b, h - tf), (b, h)]))),
                    ProfileKind::Angle => Polygon2::solid(centred(pts(&[(0.0, 0.0), (b, 0.0), (b, t), (t, t), (t, h), (0.0, h)]))),
                    ProfileKind::Cee => Polygon2::solid(centred(pts(&[(0.0, 0.0), (b, 0.0), (b, c), (b - t, c), (b - t, t), (t, t), (t, h - t), (b - t, h - t), (b - t, h - c), (b, h - c), (b, h), (0.0, h)]))),
                    ProfileKind::Zed => Polygon2::solid(centred(pts(&[(0.0, 0.0), (b, 0.0), (b, c), (b - t, c), (b - t, t), (t, t), (t, h), (t - b, h), (t - b, h - c), (2.0 * t - b, h - c), (2.0 * t - b, h - t), (0.0, h - t)]))),
                }]
            }
            Self::Polygon { polygon } => vec![polygon.clone()],
            Self::Composite { parts, .. } => {
                let mut regions = Vec::new();
                for part in parts {
                    for mut region in part.shape.outline()? {
                        let shift = |r: &mut Ring2| r.points.iter_mut().for_each(|p| *p = Vec2::new(p.x + part.offset.x, p.y + part.offset.y));
                        shift(&mut region.outer);
                        region.holes.iter_mut().for_each(shift);
                        regions.push(region);
                    }
                }
                regions
            }
            Self::ThinWalled { centreline, thickness } => centreline.points.windows(2).zip(thickness).map(|(w, &t)| {
                let (dx, dy) = (w[1].x - w[0].x, w[1].y - w[0].y);
                let len = dx.hypot(dy);
                let (nx, ny) = (-dy / len * t / 2.0, dx / len * t / 2.0);
                Polygon2::solid(vec![Vec2::new(w[0].x + nx, w[0].y + ny), Vec2::new(w[1].x + nx, w[1].y + ny), Vec2::new(w[1].x - nx, w[1].y - ny), Vec2::new(w[0].x - nx, w[0].y - ny)])
            }).collect(),
        })
    }

    /// 📐️ Net area of the canonical outline, the figure the independent oracles compare against.
    pub fn area(&self) -> Result<f64, Vec<Violation>> {
        Ok(self.outline()?.iter().map(Polygon2::area).sum())
    }
}

impl Validate for SectionParam {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "name"), &self.name);
        positive(out, &at(path, "value"), self.value);
    }
}

impl Validate for CompositePart {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        self.shape.check(&at(path, "shape"), out);
        self.offset.check(&at(path, "offset"), out);
    }
}

impl Validate for RebarLayout {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        positive(out, &at(path, "barDiameter"), self.bar_diameter);
        non_negative(out, &at(path, "cover"), self.cover);
        expect(out, &at(path, "positions"), !self.positions.is_empty(), "non-empty");
        each(&self.positions, path, "positions", out);
    }
}

impl Validate for SectionShape {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Parametric { profile, params } => {
                each(params, path, "params", out);
                let value = |n: &str| params.iter().find(|q| q.name == n).map(|q| q.value);
                let missing: Vec<_> = profile.required_params().iter().filter(|n| value(n).is_none()).collect();
                expect(out, &at(path, "params"), missing.is_empty(), "required-params-present");
                if missing.is_empty() {
                    let g = |n: &str| value(n).unwrap_or(0.0);
                    let ok = match profile {
                        ProfileKind::IShape | ProfileKind::Channel | ProfileKind::Tee => g("tw") < g("b") && 2.0 * g("tf") < g("h"),
                        ProfileKind::Chs => 2.0 * g("t") < g("d"),
                        ProfileKind::Rhs => 2.0 * g("t") < g("b").min(g("h")),
                        ProfileKind::Angle => g("t") < g("b").min(g("h")),
                        ProfileKind::Cee | ProfileKind::Zed => g("t") < g("b") && 2.0 * g("t") < g("h") && g("c") < g("h") / 2.0 && g("t") < g("c"),
                        ProfileKind::Rect | ProfileKind::Circle => true,
                    };
                    expect(out, &at(path, "params"), ok, "consistent-dimensions");
                }
            }
            Self::Polygon { polygon } => polygon.check(&at(path, "polygon"), out),
            Self::Composite { parts, rebar } => {
                expect(out, &at(path, "parts"), !parts.is_empty(), "non-empty");
                each(parts, path, "parts", out);
                each(rebar, path, "rebar", out);
            }
            Self::ThinWalled { centreline, thickness } => {
                centreline.check(&at(path, "centreline"), out);
                expect(out, &at(path, "thickness"), thickness.len() + 1 == centreline.points.len(), "one-thickness-per-segment");
                for (i, t) in thickness.iter().enumerate() {
                    positive(out, &nth(path, "thickness", i), *t);
                }
                let degenerate = centreline.points.windows(2).any(|w| w[0] == w[1]);
                expect(out, &at(path, "centreline"), !degenerate, "non-degenerate-segments");
            }
        }
    }
}
