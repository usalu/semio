//! 🧰️ Entity kit: the layer names, the entity builder, the utilisation bindings from the report, the section helpers of the fire parts and the sink every entity renderer draws into; the scene crate re-exports them and implements [`Sink`] on its accumulator. See [`super`].

use super::*;
use super::scene::*;
pub use scene::projection_kit::{self, entity_ref, loc, tone_of, utilisation_words, v3};
use scene::projection_kit::{binding_of_subject, worst_of_subject};

pub const LAYER_MEMBERS: &str = "members";
pub const LAYER_CONNECTIONS: &str = "connections";
pub const LAYER_FIRE: &str = "fire";
pub const LAYER_ZONES: &str = "zones";
pub const LAYER_LOADS: &str = "loads";
pub const LAYER_SUPPORTS: &str = "supports";
pub const LAYER_ANNOTATIONS: &str = "annotations";

/// ⏳️ The ages in years of the creep keyframes of a timber floor and their layers.
pub const CREEP_YEARS: [u32; 7] = [0, 1, 2, 5, 10, 20, 50];
pub const CREEP_LAYERS: [&str; 7] = ["creep-0", "creep-1", "creep-2", "creep-5", "creep-10", "creep-20", "creep-50"];

/// 🔥️ The colour and the thickness of the glowing char front.
pub const FRONT_RGB: (u8, u8, u8) = (255, 112, 28);
pub const FRONT_THICKNESS: f64 = 0.004;

/// 🔥️ The fire keyframes in minutes and their layers.
pub const FIRE_MINUTES: [u32; 8] = [15, 30, 45, 60, 75, 90, 105, 120];
pub const FIRE_LAYERS: [&str; 8] = ["fire-15", "fire-30", "fire-45", "fire-60", "fire-75", "fire-90", "fire-105", "fire-120"];

/// 🎯️ What one entity renderer reads: the document and its settled check report.
#[derive(Clone, Copy)]
pub struct Input<'a> {
    pub document: &'a model::En1995Snapshot,
    pub report: &'a norm::CheckReport,
}

/// 📥️ Where an entity renderer draws: the append-only accumulator of one projection part. `home` names the collection of the entity being drawn (empty: the catalogue cases).
pub trait Sink {
    fn wants(&self, id: &str) -> bool;
    fn home(&self) -> &'static str;
    fn set_home(&mut self, home: &'static str);
    fn push(&mut self, spec: Spec, points: &[Vec3]);
    fn annotate(&mut self, annotation: Annotation, anchor: &Vec3);
    fn legend(&mut self, legend: Legend);
}

/// 📶️ The governing (largest finite utilisation) applicable check of subject `entity` whose id starts with `prefix` (empty: any check).
pub fn binding_of(report: &norm::CheckReport, entity: &str, prefix: &str) -> Option<UtilisationBinding> {
    let base = binding_of_subject(report, entity, &[prefix])?;
    if !prefix.is_empty() {
        return Some(base);
    }
    let mut merged: Vec<StationValue> = Vec::new();
    for check in report.checks.iter().filter(|c| c.subject.entity_id == entity && c.station_field.is_empty()) {
        for s in projection_kit::station_values(check) {
            match merged.iter_mut().find(|m| (m.at - s.at).abs() < 1e-9) {
                Some(m) => m.value = m.value.max(s.value),
                None => merged.push(s),
            }
        }
    }
    merged.sort_by(|a, b| a.at.total_cmp(&b.at));
    Some(UtilisationBinding { per_station: Some(merged).filter(|v| !v.is_empty()), ..base })
}

pub fn worst<'a>(report: &'a norm::CheckReport, entity: &str, prefix: &str) -> Option<&'a norm::CheckResult> {
    worst_of_subject(report, entity, &[prefix])
}

/// 🔲️ A centred rectangle polygon.
pub fn rect(width: f64, height: f64) -> Polygon2 {
    let (w, h) = (width / 2.0, height / 2.0);
    Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)])
}

/// 🔲️ An axis-aligned rectangle of a section in centred coordinates, `x` across the width and `y` up.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span2 {
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}

impl Span2 {
    pub fn full(width: f64, height: f64) -> Self {
        Self { x0: -width / 2.0, x1: width / 2.0, y0: -height / 2.0, y1: height / 2.0 }
    }

    /// 🔥️ What is left after `depth` metres are lost on every exposed side `[left, right, top, bottom]` (1 exposed, 0 protected); at least one millimetre of each extent stays.
    pub fn inset(self, depth: f64, faces: [f64; 4]) -> Self {
        let keep = |lo: f64, hi: f64| if hi - lo >= 1e-3 { (lo, hi) } else { ((lo + hi) / 2.0 - 5e-4, (lo + hi) / 2.0 + 5e-4) };
        let (x0, x1) = keep(self.x0 + depth * faces[0], self.x1 - depth * faces[1]);
        let (y0, y1) = keep(self.y0 + depth * faces[3], self.y1 - depth * faces[2]);
        Self { x0, x1, y0, y1 }
    }

    pub fn solid(self) -> Polygon2 {
        Polygon2::solid(vec![Vec2::new(self.x0, self.y0), Vec2::new(self.x1, self.y0), Vec2::new(self.x1, self.y1), Vec2::new(self.x0, self.y1)])
    }

    fn turned(self, open: usize) -> Self {
        let corners = [(self.x0, self.y0), (self.x1, self.y1)].map(|(x, y)| match open {
            0 => (y, -x),
            1 => (y, x),
            3 => (x, -y),
            _ => (x, y),
        });
        Self { x0: corners[0].0.min(corners[1].0), x1: corners[0].0.max(corners[1].0), y0: corners[0].1.min(corners[1].1), y1: corners[0].1.max(corners[1].1) }
    }
}

/// 🕳️ The band between `outer` and the rectangle `inner` inside it: a ring with a hole when all four sides are exposed, a U-shaped single outline when exactly one side is protected (`faces[k] == 0`, order left, right, top, bottom); with two or more protected sides the band is drawn closed.
pub fn band(outer: Span2, inner: Span2, faces: [f64; 4]) -> Polygon2 {
    let open: Vec<usize> = (0..4).filter(|k| faces[*k] == 0.0).collect();
    let [side] = open[..] else {
        return Polygon2 { outer: outer.solid().outer, holes: vec![Ring2 { points: inner.solid().outer.points.into_iter().rev().collect() }] };
    };
    let (o, i) = (outer.turned(side), inner.turned(side));
    let ring = [(o.x0, o.y0), (o.x1, o.y0), (o.x1, o.y1), (i.x1, o.y1), (i.x1, i.y0), (i.x0, i.y0), (i.x0, o.y1), (o.x0, o.y1)];
    let unturn = |(u, v): (f64, f64)| match side {
        0 => Vec2::new(-v, u),
        1 => Vec2::new(v, u),
        3 => Vec2::new(u, -v),
        _ => Vec2::new(u, v),
    };
    let mut points: Vec<Vec2> = ring.iter().map(|p| unturn(*p)).collect();
    let area: f64 = points.iter().zip(points.iter().cycle().skip(1)).map(|(a, b)| a.x * b.y - b.x * a.y).sum();
    if area < 0.0 {
        points.reverse();
    }
    Polygon2::solid(points)
}

/// 🧱️ Everything one scene entity needs before sealing.
pub struct Spec {
    pub kind: EntityKind,
    pub id: String,
    pub collection: &'static str,
    pub geometry: Geometry,
    pub appearance: Option<Appearance>,
    pub role: MaterialRole,
    pub binding: Option<UtilisationBinding>,
    pub name: LocalizedCopy,
    pub detail: LocalizedCopy,
    pub layer: &'static str,
    pub handles: Vec<String>,
    pub deformation: Option<DeformationOverlay>,
}

impl Spec {
    pub fn build(self) -> SceneEntity {
        let (more_en, more_de) = utilisation_words(&self.binding);
        let failing = self.binding.as_ref().is_some_and(|b| b.status == scene::CheckStatus::Fail);
        let appearance = self.appearance.unwrap_or(Appearance {
            role: self.role,
            channel: if self.binding.is_some() { ColourChannel::Utilisation } else { ColourChannel::Role },
            style: if failing { StyleState::Error } else { StyleState::Normal },
            opacity: 1.0,
            hatch: failing.then_some(Hatch::Diagonal),
            deformation: self.deformation,
        });
        SceneEntity {
            entity: entity_ref(self.kind, &self.id, self.collection),
            geometry: self.geometry,
            appearance,
            utilisation: self.binding,
            label: Some(self.name.clone()),
            accessible_name: self.name,
            accessible_detail: LocalizedCopy::new(format!("{}{more_en}", self.detail.en), format!("{}{more_de}", self.detail.de)),
            layer: self.layer.to_string(),
            pick: PickPolicy::Whole,
            handles: self.handles,
            content_hash: String::new(),
        }
        .sealed()
    }
}

/// 🖍️ A flat appearance with a custom colour and opacity, for char, void and zone entities.
pub fn tinted(rgb: (u8, u8, u8), opacity: f64, style: StyleState) -> Appearance {
    Appearance { role: MaterialRole::Neutral, channel: ColourChannel::Custom { rgb: Rgb { r: rgb.0, g: rgb.1, b: rgb.2 } }, style, opacity, hatch: None, deformation: None }
}

