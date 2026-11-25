//! 📐 Concrete taper operations live in the taper owner.

use paid::{Budget, Projection, Refusal, Slot};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaf {
    Angle(u16),
    Depth(u16),
}

pub fn receive<'a>(
    local: usize,
    projection: Projection<'_>,
    budget: &'a Budget,
) -> Result<Slot<'a, Leaf>, Refusal> {
    let construct = match local {
        0 => Leaf::Angle,
        1 => Leaf::Depth,
        _ => return Err(Refusal::UnknownTag),
    };
    Slot::receive(budget, projection, construct)
}

impl Leaf {
    pub fn write_json(self) -> String {
        match self {
            Self::Angle(value) => format!(r#"{{"ChangeTaperTaperAngle":{{"newValue":{value}}}}}"#),
            Self::Depth(value) => format!(r#"{{"ChangeTaperApexDepth":{{"newValue":{value}}}}}"#),
        }
    }

    pub fn inverse(self, previous: u16) -> Self {
        match self {
            Self::Angle(_) => Self::Angle(previous),
            Self::Depth(_) => Self::Depth(previous),
        }
    }
}
