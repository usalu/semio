//! 📍️ Layout: where every zone box and envelope panel stands. Zones are square boxes of side `sqrt(A)` and height `V / A` in a row along `+X`, `ZONE_GAP` apart; walls cover the face their compass orientation points to (`0` north is `+Y`, `90` east is `+X`), roofs and floors cover the top and the bottom, windows and doors stand on their face side by side.

use super::kit::v3;
use super::model::{Din18599Snapshot, ElementKind, EnvelopeElement, ThermalZone};
use super::scene::{Frame, Vec3};
use std::collections::BTreeMap;

pub const ZONE_GAP: f64 = 3.0;
pub const WALL_THICKNESS: f64 = 0.15;
pub const OPENING_THICKNESS: f64 = 0.3;
pub const SLAB_THICKNESS: f64 = 0.2;
pub const MIN_HEIGHT: f64 = 2.2;
pub const MAX_HEIGHT: f64 = 8.0;

/// 🏠️ One zone box: base centre, side and height.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneBox {
    pub centre: Vec3,
    pub side: f64,
    pub height: f64,
}

/// 🧱️ One envelope panel: the box frame (base centre, `x` outward normal, `y` tangent, `z` up) and its size, plus the outward normal and the face centre.
#[derive(Clone, Debug, PartialEq)]
pub struct Panel {
    pub frame: Frame,
    pub size: Vec3,
    pub normal: Vec3,
    pub centre: Vec3,
}

/// 📍️ All resolved boxes and panels of one projection.
#[derive(Clone, Debug, Default)]
pub struct Layout {
    zones: BTreeMap<String, ZoneBox>,
    panels: BTreeMap<String, Panel>,
}

fn zone_box(zone: &ThermalZone, cursor: f64) -> ZoneBox {
    let side = zone.area_m2.max(1.0).sqrt();
    let height = (zone.volume_m3.max(0.0) / zone.area_m2.max(1.0)).clamp(MIN_HEIGHT, MAX_HEIGHT);
    ZoneBox { centre: v3(cursor + side / 2.0, 0.0, 0.0), side, height }
}

fn quadrant(orientation_deg: f64) -> usize {
    (((orientation_deg.rem_euclid(360.0) + 45.0) / 90.0).floor() as usize) % 4
}

fn direction(quadrant: usize) -> Vec3 {
    let angle = (quadrant as f64) * std::f64::consts::FRAC_PI_2;
    v3(angle.sin().round(), angle.cos().round(), 0.0)
}

fn on_top(element: &EnvelopeElement) -> bool {
    element.kind == ElementKind::Roof || (matches!(element.kind, ElementKind::Window | ElementKind::Door) && element.tilt_deg < 45.0)
}

fn slab(zone: &ZoneBox, z: f64) -> Panel {
    let frame = Frame { origin: v3(zone.centre.x, zone.centre.y, z), ..Frame::WORLD };
    Panel { frame, size: v3(zone.side, zone.side, SLAB_THICKNESS), normal: v3(0.0, 0.0, if z < 0.0 { -1.0 } else { 1.0 }), centre: v3(zone.centre.x, zone.centre.y, z + SLAB_THICKNESS / 2.0) }
}

fn standing(zone: &ZoneBox, element: &EnvelopeElement, slot: usize, count: usize) -> Panel {
    let quadrant = quadrant(element.orientation_deg);
    let normal = direction(quadrant);
    let tangent = v3(-normal.y, normal.x, 0.0);
    let offset = zone.side * ((slot as f64 + 1.0) / (count as f64 + 1.0) - 0.5);
    let opening = matches!(element.kind, ElementKind::Window | ElementKind::Door);
    let (width, height, sill, thickness) = if opening {
        let height = (element.area_m2.max(0.01) / 1.5).sqrt().min(zone.height * 0.9);
        let width = (element.area_m2.max(0.01) / height).min(zone.side * 0.9 / count as f64);
        let sill = if element.kind == ElementKind::Door { 0.0 } else { (0.9_f64).min(zone.height - height).max(0.0) };
        (width, height, sill, OPENING_THICKNESS)
    } else {
        (zone.side, zone.height, 0.0, WALL_THICKNESS)
    };
    let base = v3(zone.centre.x + normal.x * zone.side / 2.0 + tangent.x * if opening { offset } else { 0.0 }, zone.centre.y + normal.y * zone.side / 2.0 + tangent.y * if opening { offset } else { 0.0 }, sill);
    let frame = Frame { origin: base.clone(), x: normal.clone(), y: tangent, z: Vec3::Z };
    Panel { frame, size: v3(thickness, width, height), centre: v3(base.x, base.y, sill + height / 2.0), normal }
}

impl Layout {
    pub fn of(document: &Din18599Snapshot) -> Self {
        let mut zones = BTreeMap::new();
        let mut cursor = 0.0;
        for zone in &document.zones {
            let placed = zone_box(zone, cursor);
            cursor += placed.side + ZONE_GAP;
            zones.entry(zone.id.clone()).or_insert(placed);
        }
        let mut panels = BTreeMap::new();
        for element in &document.elements {
            let Some(zone) = zones.get(&element.zone_id) else { continue };
            let panel = match element.kind {
                ElementKind::Floor => slab(zone, -SLAB_THICKNESS),
                _ if on_top(element) && element.kind == ElementKind::Roof => slab(zone, zone.height),
                _ if on_top(element) => Panel { size: v3((element.area_m2.max(0.01)).sqrt().min(zone.side * 0.9), (element.area_m2.max(0.01)).sqrt().min(zone.side * 0.9), OPENING_THICKNESS), ..slab(zone, zone.height) },
                _ => {
                    let same: Vec<&EnvelopeElement> = document.elements.iter().filter(|o| o.zone_id == element.zone_id && o.kind == element.kind && !on_top(o) && quadrant(o.orientation_deg) == quadrant(element.orientation_deg)).collect();
                    let slot = same.iter().position(|o| o.id == element.id).unwrap_or(0);
                    standing(zone, element, slot, same.len().max(1))
                }
            };
            panels.entry(element.id.clone()).or_insert(panel);
        }
        Self { zones, panels }
    }

    pub fn zone(&self, id: &str) -> Option<&ZoneBox> {
        self.zones.get(id)
    }

    pub fn panel(&self, id: &str) -> Option<&Panel> {
        self.panels.get(id)
    }
}
