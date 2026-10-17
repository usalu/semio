//! 🧮️ The methods of the value types of the `sheets` kit; they live with the value types so the entity types and the model root stay above them.

use super::values::{IsoSize, Orientation, Paper};

/// 📐️ The standard sizes in the order they are offered, A4 first.
pub const ISO_SIZES: [IsoSize; 5] = [IsoSize::A4, IsoSize::A3, IsoSize::A2, IsoSize::A1, IsoSize::A0];

impl IsoSize {
    /// 📐️ The (long, short) sides in millimetres: 1189 by 841 for A0 down to 297 by 210 for A4.
    pub fn sides(self) -> (f64, f64) {
        match self {
            Self::A0 => (1189.0, 841.0),
            Self::A1 => (841.0, 594.0),
            Self::A2 => (594.0, 420.0),
            Self::A3 => (420.0, 297.0),
            Self::A4 => (297.0, 210.0),
        }
    }

    /// 🏷️ The name of the size.
    pub fn name(self) -> &'static str {
        match self {
            Self::A0 => "A0",
            Self::A1 => "A1",
            Self::A2 => "A2",
            Self::A3 => "A3",
            Self::A4 => "A4",
        }
    }

    /// 🔎️ The size a name spells, ignoring case.
    pub fn parse(text: &str) -> Option<Self> {
        ISO_SIZES.into_iter().find(|size| size.name().eq_ignore_ascii_case(text.trim()))
    }
}

impl Paper {
    /// 📄️ The standard paper of `size`.
    pub fn iso(size: IsoSize) -> Self {
        Self::Iso { size }
    }

    /// 📐️ The (long, short) sides in millimetres.
    pub fn sides(&self) -> (f64, f64) {
        match self {
            Self::Iso { size } => size.sides(),
            Self::Custom { width, height } => (width.max(*height), width.min(*height)),
        }
    }

    /// 🏷️ The name of the paper: `A3`, or `600 × 400` for a custom size.
    pub fn name(&self) -> String {
        match self {
            Self::Iso { size } => size.name().to_string(),
            Self::Custom { .. } => {
                let (long, short) = self.sides();
                format!("{long} × {short}")
            }
        }
    }
}

impl Orientation {
    /// 🔎️ The orientation a name spells, ignoring case.
    pub fn parse(text: &str) -> Option<Self> {
        [Self::Landscape, Self::Portrait].into_iter().find(|orientation| format!("{orientation:?}").eq_ignore_ascii_case(text.trim()))
    }
}
