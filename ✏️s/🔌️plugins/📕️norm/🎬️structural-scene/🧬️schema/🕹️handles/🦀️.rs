//! 🕹️ Handles: viewport grips that bind a scalar drag parameter to document field paths.

use super::*;

records! {
    /// 🧮️ Affine drag law `field = base + k · s`.
    pub struct Affine {
        pub base: f64,
        pub k: f64,
    }

    /// 🔗️ One field a drag writes; `path` uses the `setField` path syntax and `unit` is the unit symbol shown while dragging.
    pub struct FieldBinding {
        pub path: String,
        pub map: Affine,
        pub unit: String,
        pub quantise: Option<Quantise>,
    }

    /// 🧲️ Snapping of the drag parameter; values are in handle units.
    pub struct SnapRule {
        pub grid: Option<f64>,
        pub increment: Option<f64>,
        pub targets: Vec<f64>,
        pub catalogue: Vec<f64>,
        pub tolerance: f64,
    }

    /// 🕹️ One draggable grip of a selected entity.
    pub struct Handle {
        pub id: String,
        pub entity: EntityRef,
        pub kind: HandleKind,
        pub anchor: Vec3,
        pub grip: GripStyle,
        pub writes: Vec<FieldBinding>,
        pub snap: SnapRule,
        pub limits: Option<Interval>,
        pub accessible_name: LocalizedCopy,
    }

    /// ✋️ A drag in progress or at commit: parameter `s` of handle `handle`.
    pub struct HandleDrag {
        pub handle: String,
        pub s: f64,
        pub started_seq: u64,
    }

    /// ✍️ One concrete field value produced by evaluating a handle at `s`; `token` is the enum token of a [`Quantise::Token`] binding, `value` then its index.
    pub struct ResolvedWrite {
        pub path: String,
        pub value: f64,
        pub token: Option<String>,
        pub unit: String,
    }

    /// 🔤️ One selectable enum token with its localized label.
    pub struct ChoiceToken {
        pub token: String,
        pub label: LocalizedCopy,
    }
}

choices! {
    /// 🔣️ Grip glyph, a non-colour cue.
    pub enum GripStyle {
        Arrow,
        Diamond,
        Ring,
    }
}

variants! {
    /// 🔢️ Rounding of a bound value after the affine map; `Decades` reads the mapped value as a base-10 exponent and rounds it to `per_decade` steps per decade, so a linear drag scales the field logarithmically.
    pub enum Quantise {
        Round { step: f64 },
        Choice { values: Vec<f64> },
        Int,
        Decades { per_decade: f64 },
        Token { tokens: Vec<ChoiceToken> },
    }

    /// 🧭️ How the pointer motion becomes the scalar `s`.
    pub enum HandleKind {
        Translate { axis: Vec3 },
        TranslatePlane { normal: Vec3 },
        ScalarAlong { axis: Vec3 },
        Radial { centre: Vec3 },
        Angular { axis: Vec3, centre: Vec3 },
        DimensionLine,
        CountStepper,
        ChoiceStepper,
        FaceOffset { normal: Vec3 },
    }
}

impl Affine {
    pub const IDENTITY: Self = Self { base: 0.0, k: 1.0 };

    pub fn apply(&self, s: f64) -> f64 {
        self.base + self.k * s
    }

    pub fn invert(&self, field: f64) -> f64 {
        (field - self.base) / self.k
    }
}

impl Quantise {
    pub fn apply(&self, value: f64) -> f64 {
        match self {
            Self::Round { step } => (value / step).round() * step,
            Self::Choice { values } => values.iter().copied().min_by(|a, b| (a - value).abs().total_cmp(&(b - value).abs())).unwrap_or(value),
            Self::Int => value.round(),
            Self::Decades { per_decade } => 10f64.powf((value * per_decade).round() / per_decade),
            Self::Token { tokens } => value.round().clamp(0.0, tokens.len().saturating_sub(1) as f64),
        }
    }

    /// 🔤️ The token that `value` selects (its rounded index, clamped); `None` for the numeric rules.
    pub fn token_at(&self, value: f64) -> Option<&ChoiceToken> {
        let Self::Token { tokens } = self else { return None };
        tokens.get(self.apply(value) as usize)
    }
}

impl FieldBinding {
    pub fn new(path: impl Into<String>, map: Affine, unit: impl Into<String>) -> Self {
        Self { path: path.into(), map, unit: unit.into(), quantise: None }
    }

    /// 🔤️ A token-valued binding: the written value is the enum token at the rounded index `s`.
    pub fn token(path: impl Into<String>, tokens: Vec<ChoiceToken>) -> Self {
        Self { path: path.into(), map: Affine::IDENTITY, unit: "token".into(), quantise: Some(Quantise::Token { tokens }) }
    }

    /// 🔤️ The enum token produced by drag parameter `s`; `None` for a numeric binding.
    pub fn token_at(&self, s: f64) -> Option<&str> {
        self.quantise.as_ref()?.token_at(self.map.apply(s)).map(|t| t.token.as_str())
    }

    /// 🎯️ The field value produced by drag parameter `s`.
    pub fn value_at(&self, s: f64) -> f64 {
        let value = self.map.apply(s);
        self.quantise.as_ref().map_or(value, |q| q.apply(value))
    }
}

impl SnapRule {
    pub const NONE: Self = Self { grid: None, increment: None, targets: Vec::new(), catalogue: Vec::new(), tolerance: 0.0 };

    /// 🧲️ Pulls `s` to the nearest target or catalogue value within tolerance, else rounds to increment or grid.
    pub fn apply(&self, s: f64) -> f64 {
        let nearest = self.targets.iter().chain(&self.catalogue).copied().filter(|v| (v - s).abs() <= self.tolerance).min_by(|a, b| (a - s).abs().total_cmp(&(b - s).abs()));
        match (nearest, self.increment.or(self.grid)) {
            (Some(v), _) => v,
            (None, Some(step)) => (s / step).round() * step,
            (None, None) => s,
        }
    }
}

impl Handle {
    /// 🧱️ Clamps `s` into the geometric limits, then snaps, then clamps again so snapping never escapes the limits.
    pub fn resolve_s(&self, s: f64) -> f64 {
        let clamp = |v: f64| self.limits.as_ref().map_or(v, |l| l.clamp(v));
        clamp(self.snap.apply(clamp(s)))
    }

    /// ✍️ Every field write for drag parameter `s`, one entry per binding.
    pub fn writes_at(&self, s: f64) -> Vec<ResolvedWrite> {
        let s = self.resolve_s(s);
        self.writes.iter().map(|w| ResolvedWrite { path: w.path.clone(), value: w.value_at(s), token: w.token_at(s).map(str::to_string), unit: w.unit.clone() }).collect()
    }
}

impl Validate for Affine {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        finite(out, path, &[self.base, self.k]);
        expect(out, &at(path, "k"), self.k.abs() > TOLERANCE * TOLERANCE, "non-zero");
    }
}

impl Validate for ChoiceToken {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "token"), &self.token);
        self.label.check(&at(path, "label"), out);
    }
}

impl Validate for Quantise {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        match self {
            Self::Round { step } => positive(out, &at(path, "step"), *step),
            Self::Choice { values } => {
                finite(out, &at(path, "values"), values);
                expect(out, &at(path, "values"), !values.is_empty(), "non-empty");
            }
            Self::Int => {}
            Self::Decades { per_decade } => positive(out, &at(path, "perDecade"), *per_decade),
            Self::Token { tokens } => {
                expect(out, &at(path, "tokens"), !tokens.is_empty(), "non-empty");
                expect(out, &at(path, "tokens"), tokens.iter().enumerate().all(|(i, t)| tokens[..i].iter().all(|u| u.token != t.token)), "unique");
                each(tokens, path, "tokens", out);
            }
        }
    }
}

impl Validate for FieldBinding {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "path"), &self.path);
        non_empty(out, &at(path, "unit"), &self.unit);
        self.map.check(&at(path, "map"), out);
        self.quantise.check(&at(path, "quantise"), out);
    }
}

impl Validate for SnapRule {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        for (name, value) in [("grid", self.grid), ("increment", self.increment)] {
            if let Some(v) = value {
                positive(out, &at(path, name), v);
            }
        }
        finite(out, &at(path, "targets"), &self.targets);
        finite(out, &at(path, "catalogue"), &self.catalogue);
        non_negative(out, &at(path, "tolerance"), self.tolerance);
    }
}

impl HandleKind {
    /// 🧭️ **Angular zero direction (one convention for every artifact):** the unit vector from which `s` is measured, `+X` projected perpendicular to `axis` (`+Z` projected when `axis` is `±X`). `s` is the signed right-handed angle about `axis` from this direction to `anchor − centre`; the viewport solver is [`HandleKind::angular_s`] and its inverse [`HandleKind::angular_point`]. `None` for other kinds or a zero axis.
    pub fn angular_zero_direction(&self) -> Option<Vec3> {
        let Self::Angular { axis, .. } = self else { return None };
        let length = axis.length();
        (length > TOLERANCE).then_some(())?;
        let a = Vec3::new(axis.x / length, axis.y / length, axis.z / length);
        let perpendicular = |v: Vec3| {
            let d = v.dot(&a);
            Vec3::new(v.x - d * a.x, v.y - d * a.y, v.z - d * a.z)
        };
        let mut reference = perpendicular(Vec3::X);
        if reference.length() < 1e-9 {
            reference = perpendicular(Vec3::Z);
        }
        let norm = reference.length();
        Some(Vec3::new(reference.x / norm, reference.y / norm, reference.z / norm))
    }

    /// 🎯️ The drag parameter `s` (radians in `(−π, π]`) of a point about an `Angular` handle, by the zero-direction convention of [`HandleKind::angular_zero_direction`]; `None` for other kinds or a point on the axis.
    pub fn angular_s(&self, point: &Vec3) -> Option<f64> {
        let Self::Angular { axis, centre } = self else { return None };
        let reference = self.angular_zero_direction()?;
        let length = axis.length();
        let a = Vec3::new(axis.x / length, axis.y / length, axis.z / length);
        let v = Vec3::new(point.x - centre.x, point.y - centre.y, point.z - centre.z);
        let along = v.dot(&a);
        let radial = Vec3::new(v.x - along * a.x, v.y - along * a.y, v.z - along * a.z);
        (radial.length() > 1e-12).then(|| reference.cross(&radial).dot(&a).atan2(reference.dot(&radial)))
    }

    /// ↩️ The point at angle `s` and `radius` about an `Angular` handle (in the plane through `centre` perpendicular to the axis), the inverse of [`HandleKind::angular_s`].
    pub fn angular_point(&self, s: f64, radius: f64) -> Option<Vec3> {
        let Self::Angular { axis, centre } = self else { return None };
        let reference = self.angular_zero_direction()?;
        let length = axis.length();
        let a = Vec3::new(axis.x / length, axis.y / length, axis.z / length);
        let quarter = a.cross(&reference);
        let (sin, cos) = s.sin_cos();
        Some(Vec3::new(
            centre.x + radius * (cos * reference.x + sin * quarter.x),
            centre.y + radius * (cos * reference.y + sin * quarter.y),
            centre.z + radius * (cos * reference.z + sin * quarter.z),
        ))
    }
}

impl Validate for HandleKind {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        let direction = |v: &Vec3, name: &str, out: &mut Vec<Violation>| {
            v.check(&at(path, name), out);
            expect(out, &at(path, name), v.length() > TOLERANCE, "non-zero");
        };
        match self {
            Self::Translate { axis } | Self::ScalarAlong { axis } => direction(axis, "axis", out),
            Self::TranslatePlane { normal } | Self::FaceOffset { normal } => direction(normal, "normal", out),
            Self::Radial { centre } => centre.check(&at(path, "centre"), out),
            Self::Angular { axis, centre } => {
                direction(axis, "axis", out);
                centre.check(&at(path, "centre"), out);
            }
            Self::DimensionLine | Self::CountStepper | Self::ChoiceStepper => {}
        }
    }
}

impl Validate for Handle {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "id"), &self.id);
        self.entity.check(&at(path, "entity"), out);
        self.kind.check(&at(path, "kind"), out);
        self.anchor.check(&at(path, "anchor"), out);
        expect(out, &at(path, "writes"), !self.writes.is_empty(), "non-empty");
        each(&self.writes, path, "writes", out);
        self.snap.check(&at(path, "snap"), out);
        self.limits.check(&at(path, "limits"), out);
        self.accessible_name.check(&at(path, "accessibleName"), out);
    }
}

impl Validate for HandleDrag {
    fn check(&self, path: &str, out: &mut Vec<Violation>) {
        non_empty(out, &at(path, "handle"), &self.handle);
        finite(out, &at(path, "s"), &[self.s]);
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
