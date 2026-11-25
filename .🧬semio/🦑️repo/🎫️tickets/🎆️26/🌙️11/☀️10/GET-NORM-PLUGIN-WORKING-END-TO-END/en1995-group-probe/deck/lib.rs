//! 🪵 Concrete deck operations live in the deck owner.

use paid::{Budget, Projection, Refusal, Slot};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaf {
    Span(u16),
    Width(u16),
}

pub fn receive<'a>(
    local: usize,
    projection: Projection<'_>,
    budget: &'a Budget,
) -> Result<Slot<'a, Leaf>, Refusal> {
    let construct = match local {
        0 => Leaf::Span,
        1 => Leaf::Width,
        _ => return Err(Refusal::UnknownTag),
    };
    Slot::receive(budget, projection, construct)
}

impl Leaf {
    pub fn write_json(self) -> String {
        match self {
            Self::Span(value) => format!(r#"{{"ChangeDeckSpan":{{"newValue":{value}}}}}"#),
            Self::Width(value) => format!(r#"{{"ChangeDeckDeckWidth":{{"newValue":{value}}}}}"#),
        }
    }

    pub fn inverse(self, previous: u16) -> Self {
        match self {
            Self::Span(_) => Self::Span(previous),
            Self::Width(_) => Self::Width(previous),
        }
    }
}
