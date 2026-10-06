//! 🎯️ Owned raster target shape and private backend resource ownership.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetError {
    EmptyTarget,
    ByteOverflow,
}

impl TargetError {
    pub fn code(self) -> &'static str {
        match self {
            Self::EmptyTarget => "empty-target",
            Self::ByteOverflow => "target-byte-overflow",
        }
    }
}

impl std::fmt::Display for TargetError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for TargetError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target {
    width: u32,
    height: u32,
    bytes: u64,
}

impl Target {
    pub fn try_new(width: u32, height: u32) -> Result<Self, TargetError> {
        if width == 0 || height == 0 {
            return Err(TargetError::EmptyTarget);
        }
        let bytes = u64::from(width).checked_mul(u64::from(height)).and_then(|value| value.checked_mul(4)).ok_or(TargetError::ByteOverflow)?;
        Ok(Self { width, height, bytes })
    }

    pub fn width(self) -> u32 { self.width }
    pub fn height(self) -> u32 { self.height }
    pub fn bytes(self) -> u64 { self.bytes }
}

#[cfg(all(feature = "wgpu-engine", not(all(target_arch = "wasm32", target_env = "p2"))))]
#[path = "🧊️gpu/🦀️.rs"]
pub mod gpu;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
