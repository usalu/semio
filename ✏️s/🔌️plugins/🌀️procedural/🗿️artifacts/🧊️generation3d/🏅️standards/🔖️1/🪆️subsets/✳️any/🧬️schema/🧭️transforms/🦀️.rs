//! 🧭️ World-axis rotation composition shared by transform commands.

/// 📏️ Composes non-collapsing scales independently on each world axis.
pub fn compose_scale(current: [f64; 3], next: [f64; 3]) -> Result<[f64; 3], &'static str> {
    let result = std::array::from_fn(|axis| current[axis] * next[axis]);
    if result.iter().any(|value| !value.is_finite() || *value == 0.0) { return Err("Scale factors must remain finite and nonzero"); }
    Ok(result)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AxisAngle {
    pub axis: [f64; 3],
    pub angle: f64,
}

impl AxisAngle {
    pub const IDENTITY: Self = Self { axis: [0.0, 0.0, 1.0], angle: 0.0 };

    fn quaternion(self) -> Result<[f64; 4], &'static str> {
        if !self.angle.is_finite() || self.axis.iter().any(|value| !value.is_finite()) { return Err("Rotation values must be finite"); }
        let scale = self.axis.iter().fold(0.0_f64, |result, value| result.max(value.abs()));
        if scale == 0.0 { return Err("Rotation axis cannot be zero"); }
        let axis = self.axis.map(|value| value / scale);
        let length = axis[0].hypot(axis[1]).hypot(axis[2]);
        let (s, c) = (self.angle * 0.5).sin_cos();
        Ok([axis[0] / length * s, axis[1] / length * s, axis[2] / length * s, c])
    }

    /// 🔄️ Applies the next world-axis rotation after this rotation.
    pub fn then(self, next: Self) -> Result<Self, &'static str> {
        let [x, y, z, w] = self.quaternion()?;
        let [a, b, c, d] = next.quaternion()?;
        let mut q = [d * x + a * w + b * z - c * y, d * y - a * z + b * w + c * x, d * z + a * y - b * x + c * w, d * w - a * x - b * y - c * z];
        if q[3] < 0.0 { q = q.map(|value| -value); }
        let sine = q[0].hypot(q[1]).hypot(q[2]);
        if sine == 0.0 { return Ok(Self::IDENTITY); }
        Ok(Self { axis: [q[0] / sine, q[1] / sine, q[2] / sine], angle: 2.0 * sine.atan2(q[3]) })
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
