//! 🕹️ Handles: the grips of the RC entities. A drag parameter `s` is the displacement of the grip from its anchor along its kind (m or count), so `field = base + k · s` with the current field value as `base`; limits are the field range mapped back to `s`.

use super::kit::loc;
use super::scene::*;

pub const BAR_DIAMETERS: [f64; 11] = [0.006, 0.008, 0.010, 0.012, 0.014, 0.016, 0.020, 0.025, 0.028, 0.032, 0.040];

/// 🧩️ Everything one single-field grip needs.
pub(crate) struct Grip {
    pub id: String,
    pub entity: EntityRef,
    pub kind: HandleKind,
    pub style: GripStyle,
    pub anchor: Vec3,
    pub path: String,
    pub unit: &'static str,
    pub base: f64,
    pub k: f64,
    pub range: (f64, f64),
    pub step: f64,
    pub quantise: Option<Quantise>,
    pub name: LocalizedCopy,
}

impl Grip {
    pub(crate) fn build(self) -> Handle {
        let (a, b) = ((self.range.0 - self.base) / self.k, (self.range.1 - self.base) / self.k);
        let (lo, hi) = (a.min(b).min(0.0), a.max(b).max(0.0));
        let hi = if hi - lo < 1e-9 { lo + 1e-9 } else { hi };
        Handle {
            id: self.id,
            entity: self.entity,
            kind: self.kind,
            anchor: self.anchor,
            grip: self.style,
            writes: vec![FieldBinding { path: self.path, map: Affine { base: self.base, k: self.k }, unit: self.unit.to_string(), quantise: self.quantise }],
            snap: SnapRule { increment: Some(self.step), ..SnapRule::NONE },
            limits: Some(Interval::new(lo, hi)),
            accessible_name: self.name,
        }
    }
}

pub(crate) fn member_path(id: &str, field: &str) -> String {
    format!("members[id={id}].{field}")
}

pub(crate) fn length_grip(id: &str, entity: &EntityRef, name: &str, key: &str, anchor: Vec3, axis: Vec3, field: &str, base: f64, k: f64, range: (f64, f64), label: (&str, &str), kind: fn(Vec3) -> HandleKind) -> Grip {
    Grip {
        id: format!("{id}/{key}"),
        entity: entity.clone(),
        kind: kind(axis),
        style: GripStyle::Arrow,
        anchor,
        path: member_path(id, field),
        unit: "m",
        base,
        k,
        range,
        step: 0.025,
        quantise: Some(Quantise::Round { step: 0.005 }),
        name: loc(&format!("{name} {} handle", label.0), &format!("{name} Griff {}", label.1)),
    }
}

pub(crate) fn along(axis: Vec3) -> HandleKind {
    HandleKind::ScalarAlong { axis }
}

pub(crate) fn face(normal: Vec3) -> HandleKind {
    HandleKind::FaceOffset { normal }
}

pub(crate) fn count_grip(id: &str, entity: &EntityRef, layer: &str, anchor: Vec3, count: u32) -> Grip {
    Grip {
        id: format!("{id}/{layer}/count"),
        entity: entity.clone(),
        kind: HandleKind::CountStepper,
        style: GripStyle::Diamond,
        anchor,
        path: format!("members[id={id}].longitudinal[id={layer}].count"),
        unit: "1",
        base: f64::from(count),
        k: 1.0,
        range: (1.0, 40.0),
        step: 1.0,
        quantise: Some(Quantise::Int),
        name: loc(&format!("Bar count handle of layer {layer}"), &format!("Stabanzahl-Griff der Lage {layer}")),
    }
}

pub(crate) fn stud_row_count_grip(id: &str, entity: &EntityRef, row: &str, anchor: Vec3, count: u32) -> Grip {
    Grip {
        id: format!("{id}/stud-{row}-count"),
        entity: entity.clone(),
        kind: HandleKind::CountStepper,
        style: GripStyle::Diamond,
        anchor,
        path: format!("members[id={id}].studRows[id={row}].count"),
        unit: "1",
        base: f64::from(count),
        k: 1.0,
        range: (1.0, 16.0),
        step: 1.0,
        quantise: Some(Quantise::Int),
        name: loc(&format!("Stud count handle of row {row}"), &format!("Dübelanzahl-Griff der Reihe {row}")),
    }
}

pub(crate) fn diameter_grip(id: &str, entity: &EntityRef, layer: &str, centre: Vec3, anchor: Vec3, diameter: f64) -> Grip {
    Grip {
        id: format!("{id}/{layer}/diameter"),
        entity: entity.clone(),
        kind: HandleKind::Radial { centre },
        style: GripStyle::Ring,
        anchor,
        path: format!("members[id={id}].longitudinal[id={layer}].diameter"),
        unit: "m",
        base: diameter,
        k: 2.0,
        range: (BAR_DIAMETERS[0], BAR_DIAMETERS[BAR_DIAMETERS.len() - 1]),
        step: 0.002,
        quantise: Some(Quantise::Choice { values: BAR_DIAMETERS.to_vec() }),
        name: loc(&format!("Bar diameter handle of layer {layer}"), &format!("Stabdurchmesser-Griff der Lage {layer}")),
    }
}

pub(crate) fn anchor_grip(id: &str, entity: &EntityRef, key: &str, kind: HandleKind, anchor: Vec3, field: &str, base: f64, k: f64, range: (f64, f64), label: (&str, &str)) -> Grip {
    Grip {
        id: format!("{id}/{key}"),
        entity: entity.clone(),
        kind,
        style: GripStyle::Arrow,
        anchor,
        path: format!("anchors[id={id}].{field}"),
        unit: "m",
        base,
        k,
        range,
        step: 0.005,
        quantise: Some(Quantise::Round { step: 0.001 }),
        name: loc(&format!("Anchor {id} {} handle", label.0), &format!("Dübel {id} Griff {}", label.1)),
    }
}

pub(crate) fn punching_grip(id: &str, entity: &EntityRef, key: &str, normal: Vec3, anchor: Vec3, field: &str, base: f64, label: (&str, &str)) -> Grip {
    Grip {
        id: format!("{id}/punching/{key}"),
        entity: entity.clone(),
        kind: HandleKind::FaceOffset { normal },
        style: GripStyle::Diamond,
        anchor,
        path: member_path(id, &format!("punching.{field}")),
        unit: "m",
        base,
        k: 2.0,
        range: (0.15, 1.5),
        step: 0.025,
        quantise: Some(Quantise::Round { step: 0.005 }),
        name: loc(&format!("Column {} handle", label.0), &format!("Stützen-Griff {}", label.1)),
    }
}

pub(crate) fn punching_moment_grip(id: &str, entity: &EntityRef, key: &str, axis: Vec3, anchor: Vec3, field: &str, base: f64, label: (&str, &str)) -> Grip {
    Grip {
        id: format!("{id}/punching/{key}"),
        entity: entity.clone(),
        kind: HandleKind::ScalarAlong { axis },
        style: GripStyle::Arrow,
        anchor,
        path: member_path(id, &format!("punching.{field}")),
        unit: "N·m",
        base,
        k: PUNCHING_MOMENT_PER_METRE,
        range: (-PUNCHING_MOMENT_LIMIT, PUNCHING_MOMENT_LIMIT),
        step: 5.0e3,
        quantise: Some(Quantise::Round { step: 1.0e3 }),
        name: loc(&format!("Unbalanced column moment {} handle", label.0), &format!("Stützenmoment-Griff {}", label.1)),
    }
}

pub(crate) const PUNCHING_MOMENT_PER_METRE: f64 = 1.0e5;
pub(crate) const PUNCHING_MOMENT_LIMIT: f64 = 2.0e6;
