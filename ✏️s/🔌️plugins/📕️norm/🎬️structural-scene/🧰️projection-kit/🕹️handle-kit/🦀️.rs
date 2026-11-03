//! 🕹️ Handle kit: the one-field grip constructor shared by every artifact's handles.

use crate::*;

pub fn grid(step: f64) -> SnapRule {
    SnapRule { grid: Some(step), ..SnapRule::NONE }
}

pub fn round(step: f64) -> Option<Quantise> {
    Some(Quantise::Round { step })
}

/// 🔗️ A document leaf written as `base + k · s` with the given unit and quantisation.
pub fn field_binding(path: String, map: Affine, unit: &str, quantise: Option<Quantise>) -> FieldBinding {
    FieldBinding { path, map, unit: unit.to_string(), quantise }
}

/// 🔗️ A leaf written one to one.
pub fn identity_binding(path: String, unit: &str, quantise: Option<Quantise>) -> FieldBinding {
    field_binding(path, Affine::IDENTITY, unit, quantise)
}

/// 🔤️ A grip that steps through the tokens of an enum field, writing the token itself: `options` are `(token, English label, German label)` in enum order.
pub fn choice_handle(entity: &EntityRef, tag: &str, anchor: Vec3, path: String, options: &[(String, &str, &str)], name: LocalizedCopy) -> Handle {
    let tokens = options.iter().map(|(token, en, de)| ChoiceToken { token: token.clone(), label: LocalizedCopy::new(*en, *de) }).collect();
    let limits = (0.0, options.len().saturating_sub(1) as f64);
    handle(HandleSpec { entity, tag, kind: HandleKind::ChoiceStepper, anchor, grip: GripStyle::Diamond, writes: vec![FieldBinding::token(path, tokens)], snap: SnapRule { increment: Some(1.0), ..SnapRule::NONE }, limits, name })
}

/// 🧩️ Everything one grip needs; `writes` are the document leaves one release commits together.
pub struct HandleSpec<'a> {
    pub entity: &'a EntityRef,
    pub tag: &'a str,
    pub kind: HandleKind,
    pub anchor: Vec3,
    pub grip: GripStyle,
    pub writes: Vec<FieldBinding>,
    pub snap: SnapRule,
    pub limits: (f64, f64),
    pub name: LocalizedCopy,
}

/// 🕹️ The handle `<entity>:<tag>` of a spec.
pub fn handle(spec: HandleSpec<'_>) -> Handle {
    Handle { id: format!("{}:{}", spec.entity.id, spec.tag), entity: spec.entity.clone(), kind: spec.kind, anchor: spec.anchor, grip: spec.grip, writes: spec.writes, snap: spec.snap, limits: Some(Interval::new(spec.limits.0, spec.limits.1)), accessible_name: spec.name }
}

/// 🧲️ The move grips of the entity placed as `id` (`entity` is the scene entity that carries the grips): translate along X and Y and, when `yaw` is set, rotate about `+Z`. A drag writes `placements[id=ENTITY].x|y|yaw`; [`PlacementCommit::of_writes`] turns the first drag of an entity without a row into one insert and every later drag into one set, i.e. one undo step. The yaw grip is `Angular` about `+Z` through `anchor` (zero at `+X`), so its parameter is the absolute yaw in radians.
pub fn move_handles(entity: &EntityRef, id: &str, anchor: &Vec3, stored: &[Placement], standing_yaw: f64, yaw: bool) -> Vec<Handle> {
    let named = |en: &str, de: &str| LocalizedCopy::new(format!("{en} ({id})"), format!("{de} ({id})"));
    let current_yaw = placed(stored, id).map_or(standing_yaw, |row| row.yaw);
    let planar = [("move-x", Vec3::X, "x", "Move along X", "Entlang X verschieben"), ("move-y", Vec3::Y, "y", "Move along Y", "Entlang Y verschieben")].into_iter().map(|(tag, axis, leaf, en, de)| {
        let base = grip_base(stored, anchor, id, leaf);
        handle(HandleSpec { entity, tag, kind: HandleKind::Translate { axis }, anchor: anchor.clone(), grip: GripStyle::Ring, writes: vec![field_binding(format!("{PLACEMENTS_FIELD}[id={id}].{leaf}"), Affine { base, k: 1.0 }, "m", round(0.01))], snap: grid(0.05), limits: (base - 1.0e3, base + 1.0e3), name: named(en, de) })
    });
    let turn = yaw.then(|| {
        let arm = rotate_yaw(&Vec3::X, current_yaw);
        let tip = v3(anchor.x + arm.x, anchor.y + arm.y, anchor.z);
        handle(HandleSpec { entity, tag: "move-yaw", kind: HandleKind::Angular { axis: Vec3::Z, centre: anchor.clone() }, anchor: tip, grip: GripStyle::Ring, writes: vec![field_binding(format!("{PLACEMENTS_FIELD}[id={id}].yaw"), Affine::IDENTITY, "rad", round(0.001))], snap: grid(0.0087), limits: (-std::f64::consts::TAU, std::f64::consts::TAU), name: named("Rotate about Z", "Um Z drehen") })
    });
    planar.chain(turn).collect()
}

/// 🏹 The magnitude grip of one load glyph: the grip stands `|value| / per_metre` away from the glyph point `at` along `axis` (the direction the load arrow points, away from the structure), so dragging it outward by one metre changes the characteristic value by `per_metre`, in the sign of the value. Used for every nested action row (`<collection>[id=P].actions[id=A].<field>`).
#[allow(clippy::too_many_arguments)]
pub fn load_arrow(entity: &EntityRef, tag: &str, axis: Vec3, at: &Vec3, path: String, value: f64, per_metre: f64, unit: &str, name: LocalizedCopy) -> Handle {
    let sign = if value < 0.0 { -1.0 } else { 1.0 };
    let reach = value.abs() / per_metre;
    handle(HandleSpec {
        entity,
        tag,
        kind: HandleKind::ScalarAlong { axis: axis.clone() },
        anchor: along(at, &axis, reach),
        grip: GripStyle::Arrow,
        writes: vec![field_binding(path, Affine { base: value, k: sign * per_metre }, unit, round(per_metre / 200.0))],
        snap: grid(0.05),
        limits: (-reach, 100.0),
        name,
    })
}
